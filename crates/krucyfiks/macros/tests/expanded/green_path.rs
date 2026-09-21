use framework::effects::{Envelope, TransportError};
#[::framework::__private::async_trait]
pub trait Store: ::core::marker::Send + ::core::marker::Sync {
    async fn put(
        &self,
        key: String,
        value: String,
        revision: u64,
    ) -> Result<String, TransportError>;
    async fn notify(&self, key: String, revision: u64);
}
#[::framework::__private::async_trait]
pub trait StoreMut: ::core::marker::Send {
    async fn put(
        &mut self,
        key: String,
        value: String,
        revision: u64,
    ) -> Result<String, TransportError>;
    async fn notify(&mut self, key: String, revision: u64);
}
pub enum StoreRequest {
    Put {
        key: String,
        value: String,
        revision: u64,
    },
    Notify {
        key: String,
        revision: u64,
    },
}
pub enum StoreResponse {
    Put(Result<String, TransportError>),
    Notify,
}
impl ::core::fmt::Debug for StoreRequest {
    fn fmt(&self, formatter: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        match self {
            Self::Put { .. } => formatter.write_str("StoreRequest::Put"),
            Self::Notify { .. } => formatter.write_str("StoreRequest::Notify"),
        }
    }
}
impl ::core::fmt::Debug for StoreResponse {
    fn fmt(&self, formatter: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        match self {
            Self::Put(..) => formatter.write_str("StoreResponse::Put"),
            Self::Notify => formatter.write_str("StoreResponse::Notify"),
        }
    }
}
impl StoreRequest {
    pub async fn dispatch_mut<__KrucyfiksBackend>(
        self,
        backend: &mut __KrucyfiksBackend,
    ) -> StoreResponse
    where
        __KrucyfiksBackend: StoreMut + ?::core::marker::Sized,
    {
        match self {
            StoreRequest::Put {
                key: __krucyfiks_arg_0,
                value: __krucyfiks_arg_1,
                revision: __krucyfiks_arg_2,
            } => {
                let __krucyfiks_result = StoreMut::put(
                    backend,
                    __krucyfiks_arg_0,
                    __krucyfiks_arg_1,
                    __krucyfiks_arg_2,
                )
                .await;
                StoreResponse::Put(__krucyfiks_result)
            }
            StoreRequest::Notify {
                key: __krucyfiks_arg_0,
                revision: __krucyfiks_arg_1,
            } => {
                StoreMut::notify(backend, __krucyfiks_arg_0, __krucyfiks_arg_1).await;
                StoreResponse::Notify
            }
        }
    }
}
pub struct StoreClient {
    tx: ::framework::effects::RequestOutbox<StoreRequest, StoreResponse>,
    envelope_size: usize,
}
impl StoreClient {
    #[doc = r" Creates a client with explicit request queue and per-request response capacities."]
    pub fn new(
        request_capacity: usize,
        envelope_size: usize,
    ) -> (
        Self,
        ::framework::effects::RequestInbox<StoreRequest, StoreResponse>,
    ) {
        let (tx, rx) = ::framework::effects::request_channel(request_capacity);
        (Self { tx, envelope_size }, rx)
    }
}
#[::framework::__private::async_trait]
impl Store for StoreClient {
    async fn put(
        &self,
        __krucyfiks_arg_0: String,
        __krucyfiks_arg_1: String,
        __krucyfiks_arg_2: u64,
    ) -> Result<String, TransportError> {
        let (__krucyfiks_envelope, __krucyfiks_receiver) = ::framework::effects::Envelope::new(
            StoreRequest::Put {
                key: __krucyfiks_arg_0,
                value: __krucyfiks_arg_1,
                revision: __krucyfiks_arg_2,
            },
            self.envelope_size,
        );
        self.tx
            .send_async(__krucyfiks_envelope)
            .await
            .map_err(|_| ::framework::effects::TransportError::RequestChannelClosed)?;
        match __krucyfiks_receiver
            .recv_async()
            .await
            .map_err(|_| ::framework::effects::TransportError::ResponseChannelClosed)?
        {
            StoreResponse::Put(__krucyfiks_result) => __krucyfiks_result,
            #[allow(unreachable_patterns)]
            _ => ::core::result::Result::Err(
                ::framework::effects::TransportError::UnexpectedResponse,
            )?,
        }
    }
    async fn notify(&self, __krucyfiks_arg_0: String, __krucyfiks_arg_1: u64) {
        let (__krucyfiks_envelope, __krucyfiks_receiver) = ::framework::effects::Envelope::new(
            StoreRequest::Notify {
                key: __krucyfiks_arg_0,
                revision: __krucyfiks_arg_1,
            },
            self.envelope_size,
        );
        self.tx
            .send_async(__krucyfiks_envelope)
            .await
            .map_err(|_| ::framework::effects::TransportError::RequestChannelClosed)
            .unwrap_or_else(|error| ::core::panic!("Store::notify transport failed: {error}"));
        match __krucyfiks_receiver
            .recv_async()
            .await
            .map_err(|_| ::framework::effects::TransportError::ResponseChannelClosed)
            .unwrap_or_else(|error| ::core::panic!("Store::notify transport failed: {error}"))
        {
            StoreResponse::Notify => (),
            #[allow(unreachable_patterns)]
            _ => {
                let error = ::framework::effects::TransportError::UnexpectedResponse;
                ::core::panic!("Store::notify transport failed: {error}")
            }
        }
    }
}
#[::framework::__private::async_trait]
pub trait ResolveStore: ::core::marker::Send {
    type ResolveError;
    async fn resolve_mut<__KrucyfiksBackend>(
        self,
        backend: &mut __KrucyfiksBackend,
    ) -> ::core::result::Result<(), Self::ResolveError>
    where
        __KrucyfiksBackend: StoreMut + ?::core::marker::Sized;
    async fn resolve_put<__KrucyfiksHandle, __KrucyfiksFuture>(
        self,
        handle: __KrucyfiksHandle,
    ) -> ::core::result::Result<(), Self::ResolveError>
    where
        __KrucyfiksHandle:
            ::core::ops::FnOnce(String, String, u64) -> __KrucyfiksFuture + ::core::marker::Send,
        __KrucyfiksFuture:
            ::core::future::Future<Output = Result<String, TransportError>> + ::core::marker::Send;
    async fn resolve_notify<__KrucyfiksHandle, __KrucyfiksFuture>(
        self,
        handle: __KrucyfiksHandle,
    ) -> ::core::result::Result<(), Self::ResolveError>
    where
        __KrucyfiksHandle:
            ::core::ops::FnOnce(String, u64) -> __KrucyfiksFuture + ::core::marker::Send,
        __KrucyfiksFuture: ::core::future::Future<Output = ()> + ::core::marker::Send;
}
#[::framework::__private::async_trait]
impl ResolveStore for ::framework::effects::Envelope<StoreRequest, StoreResponse> {
    type ResolveError = ::framework::effects::EnvelopeError<StoreResponse>;
    async fn resolve_mut<__KrucyfiksBackend>(
        self,
        backend: &mut __KrucyfiksBackend,
    ) -> ::core::result::Result<(), Self::ResolveError>
    where
        __KrucyfiksBackend: StoreMut + ?::core::marker::Sized,
    {
        let (request, responder) = self.into_parts();
        responder
            .answer_async(request.dispatch_mut(backend).await)
            .await
    }
    async fn resolve_put<__KrucyfiksHandle, __KrucyfiksFuture>(
        self,
        handle: __KrucyfiksHandle,
    ) -> ::core::result::Result<(), Self::ResolveError>
    where
        __KrucyfiksHandle:
            ::core::ops::FnOnce(String, String, u64) -> __KrucyfiksFuture + ::core::marker::Send,
        __KrucyfiksFuture:
            ::core::future::Future<Output = Result<String, TransportError>> + ::core::marker::Send,
    {
        let (__krucyfiks_request, __krucyfiks_responder) = self.into_parts();
        match __krucyfiks_request {
            StoreRequest::Put {
                key: __krucyfiks_arg_0,
                value: __krucyfiks_arg_1,
                revision: __krucyfiks_arg_2,
            } => {
                let __krucyfiks_result =
                    handle(__krucyfiks_arg_0, __krucyfiks_arg_1, __krucyfiks_arg_2).await;
                __krucyfiks_responder
                    .answer_async(StoreResponse::Put(__krucyfiks_result))
                    .await
            }
            #[allow(unreachable_patterns)]
            _ => ::core::result::Result::Err(::framework::effects::EnvelopeError::Mismatch),
        }
    }
    async fn resolve_notify<__KrucyfiksHandle, __KrucyfiksFuture>(
        self,
        handle: __KrucyfiksHandle,
    ) -> ::core::result::Result<(), Self::ResolveError>
    where
        __KrucyfiksHandle:
            ::core::ops::FnOnce(String, u64) -> __KrucyfiksFuture + ::core::marker::Send,
        __KrucyfiksFuture: ::core::future::Future<Output = ()> + ::core::marker::Send,
    {
        let (__krucyfiks_request, __krucyfiks_responder) = self.into_parts();
        match __krucyfiks_request {
            StoreRequest::Notify {
                key: __krucyfiks_arg_0,
                revision: __krucyfiks_arg_1,
            } => {
                handle(__krucyfiks_arg_0, __krucyfiks_arg_1).await;
                __krucyfiks_responder
                    .answer_async(StoreResponse::Notify)
                    .await
            }
            #[allow(unreachable_patterns)]
            _ => ::core::result::Result::Err(::framework::effects::EnvelopeError::Mismatch),
        }
    }
}
#[::framework::__private::async_trait]
impl<__KrucyfiksTree> ResolveStore for ::framework::effects::AnyRoute<__KrucyfiksTree>
where
    __KrucyfiksTree: ::framework::effects::Effect + ::core::marker::Send,
{
    type ResolveError = ::framework::effects::EnvelopeError<StoreResponse>;
    async fn resolve_mut<__KrucyfiksBackend>(
        self,
        backend: &mut __KrucyfiksBackend,
    ) -> ::core::result::Result<(), Self::ResolveError>
    where
        __KrucyfiksBackend: StoreMut + ?::core::marker::Sized,
    {
        let envelope = ::framework::effects::Effect::extract::<
            ::framework::effects::Envelope<StoreRequest, StoreResponse>,
        >(self.into_inner())
        .map_err(|_| ::framework::effects::EnvelopeError::Mismatch)?;
        ResolveStore::resolve_mut(envelope, backend).await
    }
    async fn resolve_put<__KrucyfiksHandle, __KrucyfiksFuture>(
        self,
        handle: __KrucyfiksHandle,
    ) -> ::core::result::Result<(), Self::ResolveError>
    where
        __KrucyfiksHandle:
            ::core::ops::FnOnce(String, String, u64) -> __KrucyfiksFuture + ::core::marker::Send,
        __KrucyfiksFuture:
            ::core::future::Future<Output = Result<String, TransportError>> + ::core::marker::Send,
    {
        let envelope = ::framework::effects::Effect::extract::<
            ::framework::effects::Envelope<StoreRequest, StoreResponse>,
        >(self.into_inner())
        .map_err(|_| ::framework::effects::EnvelopeError::Mismatch)?;
        ResolveStore::resolve_put(envelope, handle).await
    }
    async fn resolve_notify<__KrucyfiksHandle, __KrucyfiksFuture>(
        self,
        handle: __KrucyfiksHandle,
    ) -> ::core::result::Result<(), Self::ResolveError>
    where
        __KrucyfiksHandle:
            ::core::ops::FnOnce(String, u64) -> __KrucyfiksFuture + ::core::marker::Send,
        __KrucyfiksFuture: ::core::future::Future<Output = ()> + ::core::marker::Send,
    {
        let envelope = ::framework::effects::Effect::extract::<
            ::framework::effects::Envelope<StoreRequest, StoreResponse>,
        >(self.into_inner())
        .map_err(|_| ::framework::effects::EnvelopeError::Mismatch)?;
        ResolveStore::resolve_notify(envelope, handle).await
    }
}
pub enum StoreEffect {
    Store(Envelope<StoreRequest, StoreResponse>),
}
impl ::framework::effects::Effect for StoreEffect {
    fn extract<T: 'static>(self) -> ::core::result::Result<T, Self> {
        match self {
            Self::Store(child) => {
                ::framework::effects::Effect::extract::<T>(child).map_err(Self::Store)
            }
        }
    }
}
impl StoreEffect {
    pub fn try_into_store(
        self,
    ) -> ::core::result::Result<Envelope<StoreRequest, StoreResponse>, Self> {
        match self {
            Self::Store(child) => ::core::result::Result::Ok(child),
            #[allow(unreachable_patterns)]
            other => ::core::result::Result::Err(other),
        }
    }
    #[track_caller]
    pub fn expect_store(self) -> Envelope<StoreRequest, StoreResponse> {
        match self {
            Self::Store(child) => child,
        }
    }
}
pub enum AppEffect {
    Left(StoreEffect),
    Right(StoreEffect),
}
impl ::framework::effects::Effect for AppEffect {
    fn extract<T: 'static>(self) -> ::core::result::Result<T, Self> {
        match self {
            Self::Left(child) => {
                ::framework::effects::Effect::extract::<T>(child).map_err(Self::Left)
            }
            Self::Right(child) => {
                ::framework::effects::Effect::extract::<T>(child).map_err(Self::Right)
            }
        }
    }
}
impl AppEffect {
    pub fn try_into_left(self) -> ::core::result::Result<StoreEffect, Self> {
        match self {
            Self::Left(child) => ::core::result::Result::Ok(child),
            #[allow(unreachable_patterns)]
            other => ::core::result::Result::Err(other),
        }
    }
    #[track_caller]
    pub fn expect_left(self) -> StoreEffect {
        match self {
            Self::Left(child) => child,
            Self::Right(_) => ::core::panic!("expected AppEffect::Left, got AppEffect::Right"),
        }
    }
    pub fn try_into_right(self) -> ::core::result::Result<StoreEffect, Self> {
        match self {
            Self::Right(child) => ::core::result::Result::Ok(child),
            #[allow(unreachable_patterns)]
            other => ::core::result::Result::Err(other),
        }
    }
    #[track_caller]
    pub fn expect_right(self) -> StoreEffect {
        match self {
            Self::Right(child) => child,
            Self::Left(_) => ::core::panic!("expected AppEffect::Right, got AppEffect::Left"),
        }
    }
}
