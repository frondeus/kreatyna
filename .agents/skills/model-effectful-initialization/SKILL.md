---
name: model-effectful-initialization
description: Design and review initialization in any krucyfiks application. Keep widget, runtime, and facade constructors free of operational startup work; model fallible preparation and recovery through Rust state, commands, events/outcomes, and views.
---

# Model effectful initialization in krucyfiks

Construction assembles an application; initialization is observable application
behavior. A constructed widget or runtime must not imply that its dependencies
are operational. Apply this boundary to app startup and lazily initialized
features, independently of UI toolkit, host language, or storage backend.

## DO

- Keep constructors limited to in-memory state, dependency wiring, and pure
  validation. Represent required preparation explicitly, for example with
  `Initializing`, `Failed`, and `Ready` states at the responsible widget level.
- Perform operational work through commands and capabilities: opening storage,
  migrations, network handshakes, loading resources, or starting service workers.
  Return typed outcomes; let Rust updates decide readiness and recovery.
- Expose initialization and failure through view data. The host renders those
  views and forwards events. Enable dependent features only after preparation
  succeeds; hand them the prepared resources rather than recreating them.
- Separate construction from activation. If callbacks need the published facade,
  attach it before an explicit start step drives the initialization command.
- Make Start and Retry state-dependent. Prevent duplicate attempts; retry failed
  preparation with fresh resources. Where attempts can outlive their scope, use
  scope/attempt identity to reject stale results and release their resources.
- Test observable boundaries: construction performs no startup effects, failure
  remains visible, premature events cannot use unavailable services, and retry
  can recover without duplicate workers. Add progress view data only when needed
  and backed by meaningful measurements or phases.

Example (Rust design sketch, not a prescribed framework API):

```rust
enum State {
    Initializing,
    Failed(SetupError),
    Ready(Feature),
}

// Constructor: no I/O, worker launch, or callbacks.
fn new(setup: SetupClient) -> Self {
    Self { state: State::Initializing, setup, started: false }
}

// Accepted Start event (or activation-time init hook):
self.started = true; // Reject duplicate starts while preparation is pending.
let setup = self.setup.clone();
Command::single(async move {
    Outcome::Prepared(setup.prepare().await)
})

// update handles Prepared(Result<Services, SetupError>), changes state,
// and requests rendering. view exposes Initializing, Failed, or Ready.
```

## DON'T

- Hide fallible work in `new()`, an FFI factory, a blocking constructor, or a
  detached task. An async factory alone does not replace modeled startup state.
- Discard worker setup results, panic on expected setup failures, or turn a failed
  load into a successful empty/default state. Do not silently reset user data.
- Make the UI host infer readiness, parse diagnostic strings, or own recovery
  policy. Do not invoke callbacks before their host-side dependencies exist.
- Retry through a disconnected client, reopen a resource after declaring it
  prepared, or assume a timeout/cancelled waiter proves underlying work stopped.
- Require one global startup shell for every feature, block startup on unrelated
  content queries, or add speculative progress machinery. Model preparation at
  the narrowest owner that can enforce the dependency boundary.

```rust
// DON'T: construction reports success before operational setup succeeds.
fn new() -> Self {
    tokio::spawn(async {
        let service = Service::open().await?;
        service.run().await;
        Ok::<(), SetupError>(())
    }); // Unobserved result; no state transition on failure.
    Self { state: State::Ready }
}
```
