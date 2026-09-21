use std::{any::TypeId, future::poll_fn, task::Poll};

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

    pub async fn drive(&mut self) -> Result<CommandEffect<Effect, Outcome>, DriverError> {
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

impl<Effect, Outcome> CommandEffect<Effect, Outcome> {
    pub fn try_into_effect(self) -> Result<Effect, Self> {
        match self {
            CommandEffect::Effect(e) => Ok(e),
            _ => Err(self),
        }
    }
    pub fn try_into_outcome(self) -> Result<Outcome, Self> {
        match self {
            CommandEffect::Outcome(e) => Ok(e),
            _ => Err(self),
        }
    }
    pub fn try_into_done(self) -> Result<(), Self> {
        match self {
            CommandEffect::Done => Ok(()),
            _ => Err(self),
        }
    }

    #[track_caller]
    pub fn expect_effect(self) -> Effect {
        match self {
            Self::Effect(effect) => effect,
            Self::Outcome(_) => {
                panic!("expected CommandEffect::Effect, got CommandEffect::Outcome")
            }
            Self::Done => panic!("expected CommandEffect::Effect, got CommandEffect::Done"),
        }
    }
    #[track_caller]
    pub fn expect_outcome(self) -> Outcome {
        match self {
            Self::Outcome(outcome) => outcome,
            Self::Effect(_) => panic!("expected CommandEffect::Outcome, got CommandEffect::Effect"),
            Self::Done => panic!("expected CommandEffect::Outcome, got CommandEffect::Done"),
        }
    }
    #[track_caller]
    pub fn expect_done(self) {
        match self {
            Self::Done => (),
            Self::Effect(_) => panic!("expected CommandEffect::Done, got CommandEffect::Effect"),
            Self::Outcome(_) => panic!("expected CommandEffect::Done, got CommandEffect::Outcome"),
        }
    }
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

/// Payload-independent failures at the capability client's transport boundary.
///
/// Clients map channel failures into this type and use `?` to propagate them.
/// Rust checks the conversion into the method's error type, including through
/// return-type aliases; no associated transport error type is required.
/// Non-unit responses carry the method's complete declared return type;
/// unit-returning methods use unit response variants.
///
/// ```
/// use krucyfiks::effects::TransportError;
///
/// #[derive(Debug)]
/// struct AppError(TransportError);
/// impl From<TransportError> for AppError {
///     fn from(error: TransportError) -> Self { Self(error) }
/// }
/// type Reply = Result<u32, AppError>;
/// fn receive(response: Result<Reply, TransportError>) -> Reply {
///     response?
/// }
/// assert_eq!(receive(Ok(Ok(7))).unwrap(), 7);
/// assert!(matches!(
///     receive(Err(TransportError::ResponseChannelClosed)),
///     Err(AppError(TransportError::ResponseChannelClosed))
/// ));
/// ```
///
/// A return type that cannot propagate transport errors needs an explicit
/// panic policy in the client (the future `#[krucyfiks(panic_on_transport)]`).
///
/// ```compile_fail,E0277
/// use krucyfiks::effects::TransportError;
/// fn infallible_operation() {
///     Err::<(), _>(TransportError::RequestChannelClosed)?;
/// }
/// ```
///
/// Returning a `Result` is insufficient unless its error supports conversion.
///
/// ```compile_fail,E0277
/// use krucyfiks::effects::TransportError;
/// struct AppError;
/// fn operation() -> Result<(), AppError> {
///     Err::<(), _>(TransportError::RequestChannelClosed)?;
///     Ok(())
/// }
/// ```
#[derive(Debug, Clone, Copy, PartialEq, Eq, thiserror::Error)]
pub enum TransportError {
    #[error("request channel closed")]
    RequestChannelClosed,
    #[error("response channel closed")]
    ResponseChannelClosed,
    #[error("unexpected response variant")]
    UnexpectedResponse,
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

/// Consuming extraction of an exact envelope leaf from the active branch.
/// A mismatch returns the original tree with its responder intact. Intermediate
/// tree nodes are selected with branch helpers, not extracted as leaves.
pub trait EffectTree: Sized {
    fn extract<T: 'static>(self) -> Result<T, Self>;

    fn any_route(self) -> AnyRoute<Self> {
        AnyRoute { effect: self }
    }
}

impl<Request, Response> EffectTree for Envelope<Request, Response>
where
    Request: 'static,
    Response: 'static,
{
    fn extract<T: 'static>(self) -> Result<T, Self> {
        if TypeId::of::<Self>() != TypeId::of::<T>() {
            return Err(self);
        }

        let value: Box<dyn std::any::Any> = Box::new(self);
        match value.downcast::<T>() {
            Ok(value) => Ok(*value),
            Err(_) => unreachable!("matching type IDs must downcast"),
        }
    }
}

pub struct AnyRoute<E> {
    effect: E,
}

impl<E> AnyRoute<E> {
    pub fn into_inner(self) -> E {
        self.effect
    }
}

#[cfg(test)]
mod tests;
