use std::sync::atomic::{AtomicUsize, Ordering::Relaxed};

use futures::{
    Stream, StreamExt,
    stream::{AbortHandle, Abortable, BoxStream},
};

pub mod effects;

pub trait Widget: Sized {
    type View;
    type Event: Send + 'static;
    type Outcome: Send + 'static;

    fn init(&self) -> Command<Self::Outcome> {
        Command::empty()
    }

    fn update(&mut self, message: Message<Self>) -> Command<Self::Outcome>;

    fn view(&self) -> Self::View;
}

pub enum Message<W: Widget> {
    Event(W::Event),
    Outcome(W::Outcome),
}

impl<W: Widget> std::fmt::Debug for Message<W>
where
    W::Event: std::fmt::Debug,
    W::Outcome: std::fmt::Debug,
{
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Event(arg0) => f.debug_tuple("Event").field(arg0).finish(),
            Self::Outcome(arg0) => f.debug_tuple("Outcome").field(arg0).finish(),
        }
    }
}

pub struct Command<Outcome> {
    stream: BoxStream<'static, Outcome>,
}

impl<Outcome> Command<Outcome> {
    pub fn new(stream: impl Stream<Item = Outcome> + Send + 'static) -> Self {
        // Capture while the widget's init/update span is still current.
        let span = tracing::Span::current();
        let mut stream = Box::pin(stream);

        let stream =
            futures::stream::poll_fn(move |cx| span.in_scope(|| stream.as_mut().poll_next(cx)));

        Self {
            stream: Box::pin(stream),
        }
    }

    pub fn map<Outcome2, F>(self, map: F) -> Command<Outcome2>
    where
        F: FnMut(Outcome) -> Outcome2 + Send + 'static,
        Outcome: 'static,
    {
        let stream = self.stream.map(map);
        Command::new(stream)
    }

    pub fn into_stream(self) -> BoxStream<'static, Outcome> {
        self.stream
    }
}

pub trait CommandExt<Outcome>: Sized {
    fn empty() -> Self;
    fn without_outcome(fut: impl Future<Output = ()> + Send + 'static) -> Self;
    fn abortable(stream: impl Stream<Item = Outcome> + Send + 'static) -> (Self, AbortHandle);
    fn single(fut: impl Future<Output = Outcome> + Send + 'static) -> Self;
    fn select_all(commands: Vec<Self>) -> Self;
}

impl<Outcome: Send + 'static> CommandExt<Outcome> for Command<Outcome> {
    fn empty() -> Self {
        Self::new(futures::stream::empty())
    }
    fn without_outcome(fut: impl Future<Output = ()> + Send + 'static) -> Self {
        Self::new(
            futures::stream::once(async move {
                fut.await;
                None::<Outcome>
            })
            .filter_map(futures::future::ready),
        )
    }

    fn abortable(stream: impl Stream<Item = Outcome> + Send + 'static) -> (Self, AbortHandle) {
        let (handle, registration) = AbortHandle::new_pair();

        let mut state = Some((
            Box::pin(Abortable::new(stream, registration)),
            AbortOnDrop(handle.clone()),
        ));

        let stream = futures::stream::poll_fn(move |cx| {
            let Some((inner, _guard)) = state.as_mut() else {
                return std::task::Poll::Ready(None);
            };

            let result = inner.as_mut().poll_next(cx);

            if matches!(&result, std::task::Poll::Ready(None)) {
                // Release the inner stream and abort guard immediately
                // when completion is observed.
                state = None;
            }

            result
        });

        (Self::new(stream), handle)
    }

    fn single(fut: impl Future<Output = Outcome> + Send + 'static) -> Self {
        Self::new(futures::stream::once(fut))
    }

    fn select_all(commands: Vec<Self>) -> Self {
        let streams: Vec<_> = commands.into_iter().map(Command::into_stream).collect();
        let stream = futures::stream::select_all(streams);
        Self::new(stream)
    }
}

pub struct AbortOnDrop(AbortHandle);

impl Drop for AbortOnDrop {
    fn drop(&mut self) {
        self.0.abort();
    }
}

#[derive(Clone, Copy, PartialEq, Eq)]
pub struct CommandId {
    generation: u64,
    slot_id: usize,
}

impl std::fmt::Debug for CommandId {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "CommandId({}v{})", self.slot_id, self.generation)
    }
}

pub struct CommandSlot {
    id: usize,
    generation: u64,
    handle: Option<AbortHandle>,
}

static COMMAND_SLOT_ID_COUNTER: AtomicUsize = AtomicUsize::new(0);

impl CommandSlot {
    pub fn new() -> Self {
        let id = COMMAND_SLOT_ID_COUNTER
            .try_update(Relaxed, Relaxed, |id| id.checked_add(1))
            .expect("command slot ids exhausted");
        Self {
            id,
            generation: 0,
            handle: None,
        }
    }

    pub fn is_running(&self) -> bool {
        self.handle
            .as_ref()
            .is_some_and(|handle| !handle.is_aborted())
    }

    pub fn accepts(&self, id: CommandId) -> bool {
        self.is_running() && id.slot_id == self.id && id.generation == self.generation
    }

    pub fn start<Outcome, F, S>(&mut self, make_stream: F) -> Command<Outcome>
    where
        Outcome: Send + 'static,
        F: FnOnce(CommandId) -> S,
        S: Stream<Item = Outcome> + 'static + Send,
    {
        if self.is_running() {
            return Command::empty();
        }
        // NOTE: This obviously only works if old outcome does not survive cycle of
        // 2 t 64 starts on the same slot. Even at a billlion starts per second, that would
        // roughly take 585 years. I think we are fine **for now**.
        self.generation = self.generation.wrapping_add(1);
        let generation = self.generation;
        let slot_id = self.id;

        let (command, handle) = Command::abortable(make_stream(CommandId {
            generation,
            slot_id,
        }));
        if let Some(old_handle) = self.handle.replace(handle) {
            old_handle.abort();
        }
        command
    }

    pub fn stop(&mut self) {
        if let Some(handle) = self.handle.take() {
            handle.abort();
        }
    }
}

impl Default for CommandSlot {
    fn default() -> Self {
        Self::new()
    }
}
impl Drop for CommandSlot {
    fn drop(&mut self) {
        self.stop();
    }
}
