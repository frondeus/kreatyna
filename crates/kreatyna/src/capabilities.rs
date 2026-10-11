#[krucyfiks::capability]
pub trait Render: Send + Sync {
    #[krucyfiks(panic_on_transport)]
    async fn render(&self);
}
