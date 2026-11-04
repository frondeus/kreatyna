// On the module, because the krucyfiks capability macro accepts `allow` but
// not `expect` (vibetyna-5ts2).
#![expect(
    clippy::panic,
    reason = "a render returns nothing, so a closed transport has no error path; krucyfiks asks for this panic policy (P09)"
)]

#[krucyfiks::capability]
pub trait Render: Send + Sync {
    #[krucyfiks(panic_on_transport)]
    async fn render(&self);
}
