use std::sync::atomic::{AtomicUsize, Ordering::Relaxed};

use futures::{
    Stream, StreamExt,
    stream::{AbortHandle, Abortable, BoxStream},
};

pub trait Widget: Sized {
    type View;
    type Event;
    type Outcome: Send + 'static;

    fn update(&mut self, message: Message<Self::Event, Self::Outcome>) -> Command<Self::Outcome>;

    fn view(&self) -> Self::View;
}

pub enum Message<Event, Outcome> {
    Event(Event),
    Outcome(Outcome),
}

pub struct Command<Outcome> {
    stream: BoxStream<'static, Outcome>,
}

impl<Outcome> Command<Outcome> {
    pub fn new(stream: impl Stream<Item = Outcome> + Send + 'static) -> Self {
        Self {
            stream: Box::pin(stream),
        }
    }

    pub fn into_stream(self) -> BoxStream<'static, Outcome> {
        self.stream
    }
}

pub trait CommandExt<Outcome>: Sized {
    fn empty() -> Self;
    fn without_outcome(fut: impl Future<Output = ()> + Send + 'static) -> Self;
    fn abortable(stream: impl Stream<Item = Outcome> + Send + 'static) -> (Self, AbortHandle);
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
        let guard = AbortOnDrop(handle.clone());

        let stream = async_stream::stream!({
            let _guard = guard;
            let mut stream = Box::pin(Abortable::new(stream, registration));

            while let Some(outcome) = stream.next().await {
                yield outcome;
            }
        });
        (Self::new(stream), handle)
    }
}

pub struct AbortOnDrop(AbortHandle);

impl Drop for AbortOnDrop {
    fn drop(&mut self) {
        self.0.abort();
    }
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct CommandId {
    generation: u64,
    slot_id: usize,
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
