use flume::{Receiver, Sender, unbounded};
use futures::{
    StreamExt,
    stream::{BoxStream, SelectAll},
};
use parking_lot::Mutex;
use tokio_util::sync::{CancellationToken, DropGuard};

use crate::widget::{Message, Widget};

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

impl<W: Widget> Runtime<W> {
    pub fn new(widget: W) -> Self {
        let (tx, rx) = unbounded();
        Self {
            widget: Mutex::new(widget),
            state: Mutex::new(State::Created { tx, rx }),
        }
    }

    pub fn dispatch(&self, message: Message<W>) -> Result<(), RuntimeError> {
        let tx = {
            let state = self.state.lock();
            match &*state {
                State::Terminating | State::Terminated => {
                    return Err(RuntimeError::Terminated);
                }
                State::Created { tx, .. } | State::Running { tx, .. } => tx.clone(),
            }
        };

        if tx.send(message).is_err() {
            return Err(RuntimeError::Terminated);
        }

        Ok(())
    }

    pub fn view(&self) -> W::View {
        self.widget.lock().view()
    }

    pub fn shutdown(&self) {
        let token = {
            let mut state = self.state.lock();
            match &*state {
                State::Created { .. } => {
                    *state = State::Terminated;
                    return;
                }
                State::Terminating | State::Terminated => {
                    return;
                }
                State::Running { token, .. } => {
                    let token = token.clone();
                    *state = State::Terminating;
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
                State::Running { .. } => return Err(RuntimeError::AlreadyStarted),
                State::Terminating | State::Terminated => return Err(RuntimeError::Terminated),
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
        let mut active: SelectAll<BoxStream<'static, Message<W>>> = SelectAll::new();

        {
            let initial = self.widget.lock().init();
            let initial = initial.into_stream().map(Message::<W>::Outcome);
            active.push(Box::pin(initial));
        }

        active.push(Box::pin(rx));

        token
            .run_until_cancelled(async move {
                while let Some(msg) = active.next().await {
                    let follow_up = {
                        let mut widget = self.widget.lock();
                        widget.update(msg)
                    };
                    let follow_up = follow_up.into_stream().map(Message::<W>::Outcome);
                    active.push(Box::pin(follow_up));
                }
            })
            .await;

        Ok(())
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
