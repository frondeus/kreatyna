use std::{future::poll_fn, task::Poll};

use flume::{Receiver, RecvError, SendError, Sender};
use futures::stream::BoxStream;

use crate::Command;

pub type RequestInbox<Request, Response> = Receiver<Envelope<Request, Response>>;
pub type RequestOutbox<Request, Response> = Sender<Envelope<Request, Response>>;

pub fn request_channel<Req, Res>(size: usize) -> (RequestOutbox<Req, Res>, RequestInbox<Req, Res>) {
    flume::bounded(size)
}

enum PendingCommand<Outcome> {
    Outcome(Outcome),
    Done,
}

#[derive(Debug, thiserror::Error)]
pub enum DriverError {
    #[error("Inbox closed")]
    InboxClosed,
}

pub struct CommandDriver<'a, Effect, Outcome> {
    stream: BoxStream<'static, Outcome>,
    inbox: &'a mut BoxStream<'static, Effect>,
    pending: Option<PendingCommand<Outcome>>,
    done: bool,
}

impl<'a, Effect, Outcome> CommandDriver<'a, Effect, Outcome> {
    pub fn new(command: Command<Outcome>, inbox: &'a mut BoxStream<'static, Effect>) -> Self {
        Self {
            stream: command.into_stream(),
            inbox,

            pending: None,
            done: false,
        }
    }

    pub async fn process_effect(&mut self) -> Result<CommandEffect<Effect, Outcome>, DriverError> {
        if self.done {
            return Ok(CommandEffect::Done);
        }

        let Self {
            stream,
            inbox,
            pending,
            done,
        } = self;

        poll_fn(|cx| {
            if pending.is_none() {
                match stream.as_mut().poll_next(cx) {
                    Poll::Ready(Some(outcome)) => {
                        *pending = Some(PendingCommand::Outcome(outcome));
                    }
                    Poll::Ready(None) => {
                        *pending = Some(PendingCommand::Done);
                    }
                    Poll::Pending => {}
                }
            }

            match inbox.as_mut().poll_next(cx) {
                Poll::Ready(Some(envelope)) => {
                    return Poll::Ready(Ok(CommandEffect::Effect(envelope)));
                }
                Poll::Ready(None) if pending.is_none() => {
                    return Poll::Ready(Err(DriverError::InboxClosed));
                }
                Poll::Ready(None) | Poll::Pending => {}
            }

            match pending.take() {
                Some(PendingCommand::Done) => {
                    *done = true;
                    Poll::Ready(Ok(CommandEffect::Done))
                }
                Some(PendingCommand::Outcome(outcome)) => {
                    Poll::Ready(Ok(CommandEffect::Outcome(outcome)))
                }
                None => Poll::Pending,
            }
        })
        .await
    }
}

#[derive(Debug)]
pub enum CommandEffect<Effect, Outcome> {
    /// Command has finished
    Done,
    Outcome(Outcome),
    Effect(Effect),
}

#[derive(Debug)]
pub struct Envelope<Request, Response> {
    pub request: Request,
    response: Responder<Response>,
}

#[derive(Debug)]
pub struct Responder<Response> {
    tx: Sender<Response>,
}

impl<Request, Response> Envelope<Request, Response> {
    pub fn new(request: Request, capacity: usize) -> (Self, Receiver<Response>) {
        let (tx, rx) = flume::bounded(capacity);
        let response = Responder { tx };
        (Self { request, response }, rx)
    }

    pub fn into_parts(self) -> (Request, Responder<Response>) {
        (self.request, self.response)
    }

    pub async fn answer_async(self, response: Response) -> Result<(), EnvelopeError<Response>> {
        self.response.answer_async(response).await
    }

    pub fn answer(self, response: Response) -> Result<(), EnvelopeError<Response>> {
        self.response.answer(response)
    }
}
impl<Response> Responder<Response> {
    pub async fn answer_async(self, response: Response) -> Result<(), EnvelopeError<Response>> {
        self.tx
            .send_async(response)
            .await
            .map_err(EnvelopeError::Send)
    }

    pub fn answer(self, response: Response) -> Result<(), EnvelopeError<Response>> {
        self.tx.send(response).map_err(EnvelopeError::Send)
    }
}

#[derive(Debug, thiserror::Error)]
pub enum EnvelopeError<Payload> {
    #[error(transparent)]
    Send(#[from] SendError<Payload>),
    #[error(transparent)]
    Receive(#[from] RecvError),
    #[error("There is a mismatch between request and response types")]
    Mismatch,
}

pub trait CommandTestExt<Outcome> {
    fn driver<'a, Effect>(
        self,
        effects: &'a mut BoxStream<'static, Effect>,
    ) -> CommandDriver<'a, Effect, Outcome>;
}

impl<Outcome> CommandTestExt<Outcome> for Command<Outcome> {
    fn driver<'a, Effect>(
        self,
        effects: &'a mut BoxStream<'static, Effect>,
    ) -> CommandDriver<'a, Effect, Outcome> {
        CommandDriver::new(self, effects)
    }
}
