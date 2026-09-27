# App FFI: problem inventory

Status: for discussion. Analysis change: `rrvynuoq`.

This document examines the Counter-like R13 baseline in change `uqmquulk`.
It describes problems and their causes. It does not select solutions or authorize
implementation. Discuss each item separately before changing code.

The findings come from source inspection. The examples show possible execution
orders; they are not results from new concurrency tests. Conditional findings
state the condition that is necessary to cause the problem.

## Severity scale

Problem IDs such as **P01** identify findings. Severity labels such as **P2**
describe urgency. They are separate values.

These ratings apply to the current development baseline. They consider the
effect, the conditions needed to trigger it, and the evidence available today.
They do not measure implementation effort. The discussion order also respects
the dependencies listed below.

| Severity | Meaning |
| --- | --- |
| P1 | Immediate. Stop affected use: continued use presents a demonstrated critical risk, such as serious data loss. |
| P2 | High. Resolve before relying on the affected feature with real user data. A normal use sequence or expected failure can give an incorrect result. |
| P3 | Medium. Resolve before the affected integration or lifecycle is treated as reliable. The effect is significant, but needs a specific condition. |
| P4 | Low. Track it. The current effect is limited or needs sustained load or repeated use that has not been demonstrated. |
| P5 | Acceptable quirk. We can probably retain the behavior if its limits are clearly documented and callers follow them. |

No finding currently has severity P1. Source inspection has not established an
immediate critical incident. This does not establish that the baseline is ready
for production. Reassess ratings when the Swift integration, logging destination,
and actual workload are known.

| Problem | Severity | Main effect |
| --- | --- | --- |
| P01 — Database startup | P2 | Expected setup failures leave an app with no usable store and no startup error result. |
| P07 — Counter write order | P2 | A later database write can restore an older value without reporting an error. |
| P02 — Early callbacks | P3 | A callback can run before its owner has the app reference. |
| P03 — Shutdown completion | P3 | Callers cannot establish when app work and callbacks have stopped. |
| P08 — Callback reference cycle | P3 | A strong reference cycle prevents destruction and retains app resources. |
| P11 — Payload logging | P3 | Enabled tracing can disclose user-entered content to the configured logger. |
| P04 — Retained connection | P4 | Retained stopped facades keep idle workers and connections alive. |
| P05 — Write completion at shutdown | P2 | A save can be lost before enqueueing or commit without its result reaching the view. |
| P06 — Task failure status | P3 | A failed worker can leave a partially working app with no coherent failure signal. |
| P10 — Pending-work limits | P4 | Sustained excess input can increase memory use without a fixed bound. |
| P09 — Callback progress | P5 | Awaited callbacks require a clear rule about what they may wait for. |

## Scope and source files

- [FFI construction, callbacks, dispatch, and shutdown](../../../crates/kreatyna-ffi/src/lib.rs)
- [SQLite setup and workers](../../../crates/kreatyna-ffi/src/database/sqlite.rs)
- [Current facade test](../../../crates/kreatyna-ffi/src/app_runtime_tests.rs)
- [Counter commands](../../../crates/kreatyna/src/lib.rs)
- Framework: `krucyfix/runtime/src/lib.rs`, `Runtime::run`, `dispatch`, and `shutdown`.
- Framework: `krucyfix/macros/src/capability.rs`, generated clients and mutable resolvers.

Most lifecycle problems also apply to `CounterFfi`. The app makes their effect
more visible because it owns more commands and more persistent data.

## P01 — Database setup failure has no startup result

**Severity: P2.** Database setup can fail in ordinary operation. The factory
still returns an app whose persistence cannot work. This blocks reliable use
with real data, but does not itself prove that existing database data is lost.

### What is a problem

`app()` returns an `AppFfi` before SQLite opens and migrations finish. The factory
cannot report a database setup error. The database task returns the error, but
the factory discards its task handle. That error is not inspected or logged at
this boundary.

```rust
RUNTIME.spawn(async move {
    let mut database = SqliteDatabase::new(&database_path).await?;
    database.app_worker(database_rx, ingredient_rx).await;
    Ok::<(), anyhow::Error>(())
}); // The task result is not observed.
```

### Why is it a problem

For example, a missing parent directory, a file that SQLite cannot open, or a
migration error stops the database task. Swift still receives an app object.
Ingredient loading then reports `Transport`, which does not identify the setup
failure. Counter loading reports Ready with zero after its load error.

An ingredient Retry uses the same disconnected client. It does not repeat
database setup, even if the original path problem has been corrected. The
caller cannot distinguish a usable app from this failed startup through the
factory result.

## P07 — Counter write order can differ from event order

**Severity: P2.** Ordinary counter events can persist the wrong final value
under the stated callback order. The failure is silent. Its demonstrated scope
is Counter; do not infer ingredient data loss from this finding.

### What is a problem

The runtime handles events one at a time, but polls their returned commands
concurrently. Each Counter change captures a value, waits for `render()`, and
then sends that value to the database.

### Why is it a problem

Consider two Increase events with an initial count of zero:

| Step | First command | Second command |
| --- | --- | --- |
| State update | Captures 1 | Captures 2 |
| Render callback | Still pending | Completes |
| Database request | Not yet sent | Writes 2 |
| Later completion | Writes 1 | Already finished |

The visible counter is 2, but SQLite ends with 1. Serial database execution and
FIFO order within an inbox do not prevent this: the requests reached the inbox
in the reverse order.

This needs different callback completion times. It does not require shutdown
or database failure. Ingredient form guards prevent this particular sequence
of overlapping saves in one form; it is not a claim that all app writes reorder.

## P02 — Callbacks can start before the caller receives the facade

**Severity: P3.** This can affect the first displayed state or callback access
to the app. Its effect depends on how the Swift callback handles early calls;
no failing Swift implementation has yet been established.

### What is a problem

The factory schedules `runtime.run()` before it returns the `Arc<AppFfi>`.
The executor can start child initialization and call `render()` before Swift
stores the returned facade.

### Why is it a problem

A render handler commonly reads `app.view()`. On the first callback, its app
reference may not yet exist. If the handler ignores that callback, the contract
does not guarantee a replacement notification after the reference is stored.

This can happen with a healthy database. It is a publication-order problem,
separate from database setup failure. The caller can read the current view, but
the present API does not define the order between facade delivery and callbacks.

## P03 — Shutdown has no observable completion

**Severity: P3.** Resource disposal and app replacement cannot rely on shutdown
returning. This matters when the caller needs complete shutdown. It is not, by
itself, evidence of memory corruption or a failed database transaction.

### What is a problem

`AppFfi::shutdown()` requests runtime cancellation and returns immediately. It
does not wait for the runtime task, command futures, or database worker to finish.
Repeated calls also return without waiting for completion.

### Why is it a problem

After this method returns, a command or callback can still be in progress. The
caller cannot use that return as proof that callbacks have stopped or that the
last state change has finished. An immediate `view()` is not guaranteed to be a
final, stable snapshot while runtime work is still stopping.

```rust
app.shutdown();
let view = app.view()?;
// This sequence does not wait for all app work to finish.
```

This affects disposal of callback state and reopening the same database. It
also prevents a test from observing complete shutdown through the public API.

## P08 — Callback ownership can prevent facade destruction

**Severity: P3.** The failure requires a strong callback-to-facade reference.
That is a plausible integration choice and can retain the entire app, but the
cycle is not present in the current callback test fixture.

### What is a problem

Rust retains the render and clock callback objects through widget capabilities.
If a Swift callback object strongly retains the facade, these references form
a cycle:

```text
AppFfi → Runtime → Widget → callback object → AppFfi
```

### Why is it a problem

Removing the external Swift reference then does not drop the final facade
reference. Its Rust `Drop` implementation cannot start cleanup. Explicit
shutdown also leaves the widget and callbacks retained.

This is a conditional integration hazard, not evidence that the current test
fixture has a cycle. It matters because a render handler needs some way to read
the app view, and the FFI contract does not describe callback ownership.

## P11 — Runtime tracing can expose complete ingredient data

**Severity: P3.** Payload logging occurs when the relevant tracing is enabled.
The consequence depends on who can access the configured logs and how long
they are retained. There is no evidence here of external log upload. Reassess
before enabling payload-bearing logs in a production logging service.

### What is a problem

`Runtime::dispatch` uses `#[tracing::instrument(skip(self))]`, which records its
message argument. Runtime event and outcome spans also record `message =
?message`. App events and outcomes derive `Debug` for their nested payloads.

### Why is it a problem

A form SetField event can put a name or notes into tracing data. A load or save
outcome can put complete ingredient records into tracing data. With the existing
logger enabled at the relevant level, these fields can cross the FFI logger
callback boundary.

This occurs above the database layer even when database diagnostics omit record
payloads. It increases log volume and makes user-entered content part of the
logging contract without an explicit decision. It is independent of shutdown.

## P04 — Runtime shutdown does not release the database connection

**Severity: P4.** The known effect is retention of resources while a stopped
facade remains referenced. An idle SQLite connection does not by itself prove
that a write lock is held. Raise the rating if repeated app replacement causes
measurable resource exhaustion or prevents required file operations.

### What is a problem

The facade retains `Runtime<AppWidget>`. The runtime retains the widget after
`run()` ends. The widget retains capabilities, including database request senders.
The database worker waits for both inbox streams to end. Runtime shutdown does
not close those senders.

The database task has no handle in `AppFfi`. Dropping the facade requests runtime
shutdown and aborts its task, but does not observe database-worker completion.

### Why is it a problem

An app that is shut down but still retained can keep an idle SQLite connection
open indefinitely. This is different from P03: waiting for the runtime task
alone would not make the retained widget release its clients.

Dropping all owners can eventually close the channels, but there is no public
completion point for that cleanup. Repeated construction and shutdown can retain
multiple database workers if old facades remain referenced.

## P05 — Accepted work has different meanings during shutdown

**Severity: P2.** Closing the app during a save is a realistic sequence. The
caller cannot establish whether the requested write happened. This is a data
reliability concern; it does not mean that an acknowledged successful commit
is rolled back or that SQLite transactions are corrupt.

### What is a problem

There are several distinct stages of a write:

1. `dispatch()` enqueues an event.
2. The widget handles the event and returns a command.
3. The command enqueues a database request.
4. SQLite commits the transaction.
5. The widget receives and applies the result.

Shutdown can occur between any two stages. The API has no completion contract
that tells the caller which of these stages must finish.

### Why is it a problem

An event accepted by `dispatch()` can still be dropped before its update runs.
A command cancelled before it sends its request performs no write. A request
already in a database inbox can still run after its caller is cancelled: the
mutable resolver calls the backend before it sends the response.

Thus a save can commit without its success reaching the widget. Its final
retained view can still say Saving while the database contains the new record.
Cancellation is not proof of rollback, and dispatch success is not proof of a
commit. An immediate shutdown after Save cannot establish either result.

Within the normal worker loop, queued requests are still processed even when
their responders have been dropped. This fact does not establish completion
for the facade, and does not cover a worker failure.

## P06 — Task failure does not produce one coherent app status

**Severity: P3.** A worker must first stop or startup must race with shutdown.
The resulting partial failure can prevent recovery and confuse the caller.
No recurring worker panic has been demonstrated in the current baseline.

### What is a problem

The database worker and widget runtime run as separate tasks. Their exits are
not monitored together. A database-worker failure leaves the widget runtime
running. A runtime panic is not observed through its stored `JoinHandle`.

### Why is it a problem

If the database worker stops, `dispatch()` can still accept events. Store calls
fail through disconnected channels, but the facade does not report that its
database service is gone.

If the widget runtime panics after its cleanup guard is installed, the framework
does mark it Terminated. This is useful, but there is no app-level failure
notification, and the database worker can remain alive because of P04.

There is also a normal-start race: shutdown can set a new runtime to Terminated
before its scheduled `run()` begins. `run()` then returns `Terminated`, and the
FFI task's `expect("Runtime to run correctly")` turns that return into a panic.
An early shutdown is therefore treated as an unexpected failure in that task.

## P10 — Bounded database inboxes do not bound total pending work

**Severity: P4.** The risk needs input to exceed processing capacity for long
enough to matter. The current source permits that growth, but no normal-use
memory exhaustion has been demonstrated.

### What is a problem

Each database request inbox has capacity 32, but the runtime event channel is
unbounded. The runtime also retains concurrent command streams. Commands that
cannot yet send a database request can remain pending outside its inbox.

### Why is it a problem

If events arrive faster than callbacks or SQLite complete, memory use can grow.
The inbox capacity does not impose a maximum on queued events or waiting
commands. A slow database worker services both stores, so a long operation can
delay requests from either child.

This is a load-dependent risk. The normal tests do not establish a production
limit on pending work. It does not imply that sharing one connection is wrong.

## P09 — Callback progress controls command progress

**Severity: P5.** Waiting for a callback is not inherently incorrect. The
deadlock example requires the callback to wait for work that depends on its
own return. We can probably accept this constraint as a documented caller
obligation. This rating does not reduce P07's separate write-order risk.

### What is a problem

Core commands await the platform render callback before they perform some store
operations. The FFI shim directly awaits the foreign callback. The contract does
not state which other app work that callback may wait for.

### Why is it a problem

If a save's render callback waits for that save to complete, the save cannot
start its store request. Each operation waits for the other. Even without a
cycle, a callback that never completes leaves the command pending.

Render callbacks can overlap because commands run concurrently. Their completion
order also affects P07. A render notification contains no snapshot or revision;
reading `view()` returns the state at the time of the read, which can be newer
than the state that caused the notification.

Calling `view()` from the ordinary render callback is not itself the problem:
the runtime releases the widget lock before polling commands. The missing
contract concerns progress, overlap, and foreign-side work after cancellation.

## Discussion order and dependencies

The sections and both issue tables follow the same discussion order. At each
step, choose the most severe issue whose prerequisites have already been
discussed. For equal severity, use the lower problem ID first. Keep the original
IDs so existing references remain valid.

P05 and P06 wait for P04 even though P04 has lower severity. Once P04 has been
discussed, P05 and P06 take priority over the remaining P4 and P5 issues. These
are discussion prerequisites; they do not require prior implementation.

Each item describes a different observable problem. A shared cause does not
mean the items must be implemented together.

| Item | Earlier facts needed for discussion | Reason |
| --- | --- | --- |
| P01 | None | Establish what database startup currently means. |
| P07 | None | Write order is wrong under the stated callback schedule. |
| P02 | None | Facade publication is distinct from database readiness. |
| P03 | None | Establish what shutdown completion must mean. |
| P08 | None | Cross-language ownership is a separate concern. |
| P11 | None | Data in logs is a separate concern. |
| P04 | None | Identify which retained objects keep the worker alive. |
| P05 | P03, P04 | Write completion spans runtime and worker lifetimes. |
| P06 | P01, P03, P04 | Failure handling needs the startup and lifetime distinctions. |
| P10 | None | Pending-work limits are a separate concern. |
| P09 | None | Callback progress needs its own contract. |

These are discussion dependencies, not an implementation design. P09 helps
explain the trigger for P07, but either problem can be examined on its own.
P08 can prevent cleanup from starting; P04 can prevent database cleanup even
when explicit shutdown has occurred.

Start discussion with P01. No solution is selected here.

## Boundaries that are already sound

Event conversion checks input before enqueueing and preserves supplied scope
IDs. Core parents reject stale scope IDs. SQLite setup enables foreign keys,
keeps the busy timeout, and runs embedded migrations. Ingredient writes already
use transactions and typed mappings. These are not missing features in R13.

The path comes from the platform while Rust owns SQLite. Supplying a filesystem
path through FFI is not itself a problem. The failure and lifetime contracts
around that path are the concerns above.

## Validation limits

The current facade test checks successful startup, child readiness, malformed
input, and dispatch rejection after shutdown. It does not prove worker exit,
resource release, callback completion, write order, or startup error reporting.
Its timeout detects a stalled test; it does not establish lifecycle ordering.

This documentation change adds no runtime code and no tests. Findings are based
on the current source paths listed above.
