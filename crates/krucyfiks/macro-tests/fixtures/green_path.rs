use framework::effects::{Envelope, TransportError};

#[framework::capability(crate = "::framework")]
pub trait Store {
    async fn put(
        &self,
        key: String,
        value: String,
        revision: u64,
    ) -> Result<String, TransportError>;

    #[krucyfiks(panic_on_transport)]
    async fn notify(&self, key: String, revision: u64);
}

#[derive(framework::Effect)]
#[krucyfiks(crate = "::framework")]
pub enum StoreEffect {
    Store(Envelope<StoreRequest, StoreResponse>),
}

#[derive(framework::Effect)]
#[krucyfiks(crate = "::framework")]
pub enum AppEffect {
    Left(StoreEffect),
    Right(StoreEffect),
}
