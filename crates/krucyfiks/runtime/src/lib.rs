use flume::{Receiver, Sender, unbounded};
use futures::{
    StreamExt,
    stream::{BoxStream, SelectAll},
};
use parking_lot::Mutex;
use tokio_util::sync::{CancellationToken, DropGuard};

use krucyfiks::{Message, Widget};
use tracing::Span;

#[derive(Debug, thiserror::Error)]
pub enum RuntimeError {
    #[error("Runtime is terminated")]
    Terminated,

    #[error("Runtime has already been started")]
    AlreadyStarted,
}

pub struct Runtime<W: Widget> {
    widget: Mutex<W>,
    state: Mutex<State<W>>,
}

enum State<W: Widget> {
    Created {
        tx: Sender<Message<W>>,
        rx: Receiver<Message<W>>,
    },
    Running {
        token: CancellationToken,
        tx: Sender<Message<W>>,
    },
    Terminating,
    Terminated,
}

impl<W> Runtime<W>
where
    W: Widget,
    W::Event: std::fmt::Debug,
    W::Outcome: std::fmt::Debug,
{
    pub fn new(widget: W) -> Self {
        let (tx, rx) = unbounded();
        tracing::info!("Create runtime");
        Self {
            widget: Mutex::new(widget),
            state: Mutex::new(State::Created { tx, rx }),
        }
    }

    #[tracing::instrument(skip(self))]
    pub fn dispatch(&self, message: Message<W>) -> Result<(), RuntimeError> {
        let tx = {
            let state = self.state.lock();
            match &*state {
                State::Terminating | State::Terminated => {
                    tracing::error!("Runtime terminated");
                    return Err(RuntimeError::Terminated);
                }
                State::Created { tx, .. } | State::Running { tx, .. } => tx.clone(),
            }
        };

        tracing::info!("Dispatching");
        if tx.send(message).is_err() {
            tracing::error!("Terminated");
            return Err(RuntimeError::Terminated);
        }

        Ok(())
    }

    pub fn view(&self) -> W::View {
        self.widget.lock().view()
    }

    #[tracing::instrument(skip_all)]
    pub fn shutdown(&self) {
        let token = {
            let mut state = self.state.lock();
            match &*state {
                State::Created { .. } => {
                    tracing::warn!("Terminated");
                    *state = State::Terminated;
                    return;
                }
                State::Terminating | State::Terminated => {
                    tracing::warn!("Already terminated");
                    return;
                }
                State::Running { token, .. } => {
                    let token = token.clone();
                    *state = State::Terminating;
                    tracing::warn!("Terminating");
                    token
                }
            }
        };
        token.cancel();
    }

    pub async fn run(&self) -> Result<(), RuntimeError>
    where
        W::Event: Send + 'static,
        W::Outcome: Send + 'static,
        W: 'static,
    {
        let (token, rx) = {
            let mut state = self.state.lock();
            match &*state {
                State::Running { .. } => {
                    tracing::error!("Already started");
                    return Err(RuntimeError::AlreadyStarted);
                }
                State::Terminating | State::Terminated => {
                    tracing::error!("Can't start: Terminated before");
                    return Err(RuntimeError::Terminated);
                }
                State::Created { tx, rx } => {
                    let token = CancellationToken::new();

                    let tx = tx.clone();
                    let rx = rx.clone();
                    *state = State::Running {
                        token: token.clone(),
                        tx,
                    };
                    (token, rx)
                }
            }
        };

        let _guard = RunCleanup {
            _drop_guard: token.clone().drop_guard(),
            state: &self.state,
        };
        let rx = rx.into_stream();
        let mut active: SelectAll<BoxStream<'static, SpannedMessage<W>>> = SelectAll::new();

        {
            let span = tracing::info_span!("Init");
            let initial = span.in_scope(|| self.widget.lock().init());
            let initial = initial.into_stream().map(Message::<W>::Outcome).map({
                let span = span.clone();
                move |message| SpannedMessage {
                    span: tracing::info_span!(parent: span.clone(), "Command", message = ?message),
                    message,
                }
            });
            active.push(Box::pin(initial));
        }

        let rx = rx.map(|message| SpannedMessage {
            span: tracing::info_span!("Event", message = ?message),
            message,
        });
        active.push(Box::pin(rx));

        token
            .run_until_cancelled(async move {
                while let Some(msg) = active.next().await {
                    let follow_up = {
                        let mut widget = self.widget.lock();
                        msg.span.in_scope(|| widget.update(msg.message))
                    };
                    let follow_up = follow_up.into_stream().map(Message::<W>::Outcome).map({
                        let span = msg.span.clone();
                        move |message| SpannedMessage {
                            span: tracing::info_span!(parent: span.clone(), "Outcome",message= ?message),
                            message,
                        }
                    });

                    active.push(Box::pin(follow_up));
                }
            })
            .await;

        Ok(())
    }
}

struct SpannedMessage<W: Widget> {
    message: Message<W>,
    span: Span,
}
impl<W> std::fmt::Debug for SpannedMessage<W>
where
    Message<W>: std::fmt::Debug,
    W: Widget,
{
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        self.message.fmt(f)
    }
}

struct RunCleanup<'a, W: Widget> {
    state: &'a Mutex<State<W>>,
    _drop_guard: DropGuard,
}
impl<W: Widget> Drop for RunCleanup<'_, W> {
    fn drop(&mut self) {
        *self.state.lock() = State::Terminated;
    }
}
