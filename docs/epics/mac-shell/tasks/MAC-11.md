# MAC-11 — Counter becomes a `krucyfiks` example

Tracked in beads: `vibetyna-4rn.3`. Change in `vibetyna`: `swxlwslq` (this plan, the
folder rename paths, epic status, brief). Change in `krucyfiks`: `usmwykuw`
(on top of `ypqzonus`). Dependencies: none.

## Open questions

None. The user answered MAC-11/Q1 and MAC-11/Q2 and asked for the folder
rename (MAC-11/Q3) in review (see Decisions).

## Decisions

### MAC-11/Q1 — Move Counter's behavior unchanged, with a self-contained warning

*Context:* Counter saves its value after each change. Each change returns a
command. The command captures the new value, waits for `render()`, and then
writes the value. The runtime runs these commands concurrently, so a later
write can store an older value (P07 in
`docs/epics/ingredient-catalog/R13-ffi-problems.md`). The test
`delayed_commands_persist_each_events_captured_value`
(`crates/kreatyna/tests/counter.rs:110`) pins the current behavior.

*Decision:* Option A ("Looks good"). The example keeps the behavior and the
test. The widget doc comment explains the defect in a few sentences, with no
path into `vibetyna`. A fix can be a later `krucyfiks` change.

### MAC-11/Q2 — Capability routing tests move into `krucyfiks/macro-tests` with neutral fixtures

*Context:* `crates/kreatyna/src/capabilities/tests.rs` has 17 unit tests.
They test code that the `krucyfiks` macros generate (exact and recursive
routing, `try_into_*`/`expect_*`, mutable resolvers, transport failures, the
panic policy). They use Counter's types (`Render`, `Clock`,
`CounterDatabase`, `CounterEffect`, `CounterPairEffect`). MAC-03 deletes these
types from `kreatyna`.

*Decision:* Option B ("Looks good"). The 17 tests move into the framework's
consumer test crate `krucyfiks/macro-tests`. A new fixture module declares
capabilities that do not belong to Counter. The test logic stays the same;
only type and method names change (mapping in "Changes by layer").

*Follow-up decided by the agent:* the three compile-fail doctests on
`CounterEffect` and `CounterPairEffect` also test a macro contract ("raw
composition enums do not implement the resolver traits"). They move to
`macro-tests/src/compile_contracts.md` with the same fixture names. The
example keeps the explanation as plain doc text, without the test blocks.

### MAC-11/Q3 — Rename the folder `krucyfix` to `krucyfiks`

*Context:* The crates are named `krucyfiks` and `krucyfiks-runtime`. The
remote is already `ssh://git@codeberg.org/frondeus/krucyfiks.git`. Only the
local folder `/Users/frondeus/Code/krucyfix` has the other spelling.

*Decision:* The user asked for the rename in review. This slice renames the
folder to `/Users/frondeus/Code/krucyfiks` before it adds the example. All
references in `vibetyna` change with it (list in "Changes by layer"). This is
epic decision E-16.

Facts checked on 2026-10-11:

- The folder has no `.envrc`, so `direnv` does not need a new `allow`.
- `vibetyna/Cargo.lock` stores no paths for path dependencies, so it does
  not change.
- No Claude settings or memory files reference the folder.
- Both `target/` folders contain absolute paths. Cargo rebuilds the
  `krucyfiks` crates once after the rename.

## Outcome

The sibling repository folder is `/Users/frondeus/Code/krucyfiks`. Kreatyna
builds against it.

`krucyfiks` has a new folder `examples/counter/`. It is a Cargo crate named
`counter-example`. It contains:

- the Counter widget: Increase, Decrease, Reset, and a background tick
  every 1000 ms that you start and stop;
- `CounterPair`, which composes two counters and routes their events,
  outcomes and effects;
- their behavior tests, adapted from `kreatyna`.

`cargo test -p counter-example` runs 89 tests. Example: the test
`a_left_tick_changes_only_its_child_and_routes_render_and_write` shows a left
tick that changes the pair view from `(10, 20)` to `(11, 20)`.

`krucyfiks/macro-tests` gains 17 routing tests and 2 compile-fail doctests.

The Kreatyna app does not change. Counter stays in the app until MAC-03.

## Terms

| Term | Meaning |
|---|---|
| `krucyfiks` | The sibling repository (today the folder `../krucyfix`) and its main crate (`core`). Kreatyna's widgets are built on it. |
| `macro-tests` | A `krucyfiks` crate that uses the framework as an outside consumer would. It depends on it under the name `framework`. |
| Widget | A `krucyfiks` trait. A widget has `init`, `update(Message)`, and `view`. `update` changes state and returns a `Command`. |
| Command | Async work that a widget returns. The runtime runs it. It can request effects and produce outcomes. |
| Capability | A trait such as `Render` or `Clock`. The `#[krucyfiks::capability]` macro generates a client, a request enum, and resolvers for it. |
| Effect composition | An enum with `#[derive(Effect)]` that wraps child effects, for example `Left(...)` and `Right(...)`. |
| Effect stream | The stream of capability requests that a test receives and resolves by hand. |
| `CommandSlot` | Holds one running background command. It rejects outcomes from commands that it no longer runs. |
| Workspace member | A crate listed in the `members` array of the root `Cargo.toml`. `cargo test --workspace` tests it. |
| Path dependency | A dependency given as a folder (`path = "../../core"`) instead of a version. |
| Compile-fail doctest | A code block in a doc comment marked ` ```compile_fail `. The test passes when the code does not compile. |
| jj change | One reviewable unit of work in jj (the version control tool used here). Each change has a short ID such as `swxlwslq`. |
| `jj new` | Creates a new empty change on top of a given change. Your edits go into it. |

## Design

This slice has no UI change. No mockup applies.

## Changes by layer

### db

None. The `counter` table stays. MAC-03 drops it (E-05).

### tooling — folder rename

Run once: `mv /Users/frondeus/Code/krucyfix /Users/frondeus/Code/krucyfiks`.

Search result for references in `vibetyna` (`rg -n krucyfix`, 2026-10-11,
`target/` excluded). Every line changes `krucyfix` to `krucyfiks`:

| File | Lines | Kind |
|---|---|---|
| `crates/kreatyna/Cargo.toml` | 11 | path dependency |
| `crates/kreatyna-db/Cargo.toml` | 17 | path dependency |
| `crates/kreatyna-ffi/Cargo.toml` | 20, 21 | path dependencies |
| `.agents/skills/plan-reviewable-epics/SKILL.md` | 76 | example repository name |
| `docs/epics/ingredient-catalog/R13-ffi-problems.md` | 92, 93 | source paths |
| `docs/epics/ingredient-catalog/tasks/R12-uniffi-types.md` | 51, 52, 53 | "in krucyfix" |
| `docs/epics/ingredient-catalog/tasks/R06-list-widget.md` | 18, 33 | repository name |
| `docs/tasks/recipes/brief.md` | 53, 106 | repository name and path |
| `docs/epics/mac-shell/epic.html` | 198, 236, 259, 262, 265, 328, 362, 363, 364, 367, 420 (not 270: E-16 records the old name) | repository name and path |

After the edit, `rg -n krucyfix -g '!target'` in `vibetyna` finds only the
E-16 row in `epic.html` and the history in this plan.

Path dependencies after the rename (*sketch*,
`crates/kreatyna-ffi/Cargo.toml`):

```toml
krucyfiks = { path = "../../../krucyfiks/core" }
krucyfiks-runtime = { path = "../../../krucyfiks/runtime" }
```

### rust — `krucyfiks/examples/counter` (new crate)

This crate adds no new behavior. It copies the Counter code and its behavior
tests and changes only import paths and doc comments.

| New file in `krucyfiks/examples/counter/` | Source in `vibetyna/crates/kreatyna/` | Change |
|---|---|---|
| `Cargo.toml` | — | New (sketch below) |
| `src/lib.rs` | `src/lib.rs` lines 1–15 (imports), 17–213 (`Counter`, `CounterStatus`, `CounterView`, `CounterEvent`, `CounterOutcome`, `test_fixture`) | Drop app imports, `pub mod app`/`ingredients`/`confirmation`, and `CLIENT_CAPABILITY`. New crate doc and widget doc (Q1). |
| `src/capabilities.rs` | `src/capabilities.rs` | Remove `#[cfg(test)] mod tests;` and the compile-fail block (Q2). |
| `src/composability.rs` | `src/composability.rs` | Remove the two compile-fail blocks (Q2). |
| `tests/counter.rs` | `tests/counter.rs` | `use kreatyna::` → `use counter_example::` (lines 1, 4). |
| `tests/background.rs` | `tests/background.rs` | `use kreatyna::` → `use counter_example::` (lines 1, 4). |
| `tests/composition.rs` | `tests/composition.rs` | `use kreatyna::` → `use counter_example::` (lines 2, 3, 6). |

Search result for `kreatyna` references in the moved behavior tests (`rg -n
"kreatyna"`, 2026-10-11): `tests/counter.rs:1,4`, `tests/background.rs:1,4`,
`tests/composition.rs:2,3,6`. The doctest references
(`src/capabilities.rs:50`, `src/composability.rs:31,38`) leave with their
blocks.

Why a workspace member crate and not Cargo's `examples/` target: the root
`krucyfiks/Cargo.toml` is a virtual workspace, so Cargo finds no examples
there. A Cargo example target is one binary and cannot have a `tests/`
folder. A crate keeps the three test files as they are. The folder
`examples/` matches D-45.

`Cargo.toml` (*sketch*; versions from `crates/kreatyna/Cargo.toml`):

```toml
[package]
name = "counter-example"
version = "0.1.0"
edition = "2024"
publish = false

[dependencies]
krucyfiks = { path = "../../core" }
async-stream = "0.3.6"
async-trait = "0.1.92"
futures = "0.3.34"
thiserror = "2.0.20"
tracing = "0.1.44"

[dev-dependencies]
anyhow = "1.0.104"
test-case = "3"
tokio = { version = "1", features = ["macros", "rt"] }
```

Root `krucyfiks/Cargo.toml` (*sketch*):

```toml
members = [ "core", "macros", "macro-tests", "runtime", "examples/counter" ]
```

`src/lib.rs` head (*sketch*):

```rust
//! Counter: a small `krucyfiks` widget example.
//!
//! - `Counter` loads its count, changes it on events, renders, and saves.
//! - `StartBackground` ticks every 1000 ms through the `Clock` capability.
//! - `composability::CounterPair` composes two counters and routes their effects.
//!
//! Capabilities exist only as test fixtures (`CounterCapabilities::testing`).
//! The tests in `tests/` drive each flow by resolving effects by hand.

pub mod capabilities;
pub mod composability;

pub use capabilities::CounterCapabilities;
use capabilities::CounterEffectStream;
use krucyfiks::{Command, CommandExt, CommandId, CommandSlot, Message, Widget};

/// A counter that saves each new value.
///
/// Known limitation: each change captures its value, waits for `render()`,
/// then writes. The runtime runs these commands concurrently, so a later
/// write can store an older value. Do not copy this save pattern into a
/// widget that needs the last value to win.
pub struct Counter {
    // unchanged
}
```

Public types stay as they are (*real*, `crates/kreatyna/src/lib.rs`):

```rust
pub enum CounterEvent { Increase, Decrease, Reset, StartBackground, StopBackground }
pub enum CounterOutcome { Tick { id: CommandId }, Loaded { counter: usize } }
pub struct CounterView { pub count: usize, pub status: CounterStatus }
```

### rust — `krucyfiks/macro-tests` (routing tests, Q2)

New file `macro-tests/src/routing.rs`, declared in `src/lib.rs` as
`#[cfg(test)] mod routing;`. It holds a fixture and the 17 tests from
`crates/kreatyna/src/capabilities/tests.rs`. It follows the style of
`macro-tests/fixtures/green_path.rs`: the dependency is named `framework`,
and each macro gets `crate = "::framework"`.

Fixture (*sketch*):

```rust
use framework::effects::{Envelope, TransportError};

#[framework::capability(crate = "::framework")]
trait Render {
    #[krucyfiks(panic_on_transport)]
    async fn render(&self);
}

#[framework::capability(crate = "::framework")]
trait Clock {
    #[krucyfiks(panic_on_transport)]
    async fn sleep_ms(&self, ms: u64);
}

#[framework::capability(crate = "::framework")]
trait Store {
    async fn get(&self) -> Result<Option<usize>, StoreError>;
    async fn set(&self, value: usize) -> Result<(), StoreError>;
}

#[derive(Debug)]
enum StoreError { OutOfRange, Storage, Transport }

impl From<TransportError> for StoreError {
    fn from(_: TransportError) -> Self { Self::Transport }
}

#[derive(Debug, framework::Effect)]
#[krucyfiks(crate = "::framework")]
enum LeafEffect {
    Render(Envelope<RenderRequest>),
    Clock(Envelope<ClockRequest>),
    Store(Envelope<StoreRequest>),
}

#[derive(Debug, framework::Effect)]
#[krucyfiks(crate = "::framework")]
enum PairEffect {
    Left(LeafEffect),
    Right(LeafEffect),
}

const CAPACITY: usize = 32;
```

Name mapping, applied to the 17 tests. Test names and logic do not change.

| In `kreatyna` | In `macro-tests/src/routing.rs` |
|---|---|
| `CounterDatabase`, `CounterDatabaseClient`, `CounterDatabaseMut`, `CounterDatabaseRequest` | `Store`, `StoreClient`, `StoreMut`, `StoreRequest` |
| `count()` / `resolve_count` | `get()` / `resolve_get` |
| `set_count(n)` / `resolve_set_count` | `set(n)` / `resolve_set` |
| `DatabaseError` (`OutOfRange`, `Storage`, `Transport`) | `StoreError` (`OutOfRange`, `Storage`, `Transport`) |
| `CounterEffect` (`Database` variant, `expect_database`, `try_into_database`) | `LeafEffect` (`Store` variant, `expect_store`, `try_into_store`) |
| `CounterPairEffect` | `PairEffect` |
| `CLIENT_CAPABILITY` | `CAPACITY` |
| `#[async_trait::async_trait]` on the `RenderMut`, `ClockMut` and `CounterDatabaseMut` impls (lines 190, 197, 204) | `#[framework::__private::async_trait]`, as in `green_path_checks.rs:49` |

`macro-tests` must keep "no direct dependency on async-trait" (its crate doc).
So it does not add `async-trait`. It adds no new dependencies: the tests use
`futures` (already a dev-dependency) and `std::cell::Cell`.

New blocks in `macro-tests/src/compile_contracts.md` (*sketch* of one; each
block declares its own fixture, because each doctest compiles alone):

````markdown
Raw composition enums do not implement the resolver traits. Select a branch
explicitly or opt into recursive resolution with `any_route()`:

```compile_fail,E0599
#[framework::capability(crate = "::framework")]
trait Render {
    #[krucyfiks(panic_on_transport)]
    async fn render(&self);
}

#[derive(framework::Effect)]
#[krucyfiks(crate = "::framework")]
enum LeafEffect {
    Render(framework::effects::Envelope<RenderRequest>),
}

async fn resolve(effect: LeafEffect) {
    effect.resolve_render(|| async {}).await.unwrap();
}
```
````

The three blocks correspond to the three blocks in `kreatyna`:

| `kreatyna` source | Call that must not compile |
|---|---|
| `src/capabilities.rs:49` | `LeafEffect.resolve_render(...)` |
| `src/composability.rs:30` | `PairEffect.resolve_render(...)` |
| `src/composability.rs:37` | `PairEffect.expect_left().resolve_render(...)` |

### rust — `vibetyna`

Only the four path dependency lines (see "tooling — folder rename").
`kreatyna`, `kreatyna-db` and `kreatyna-ffi` keep their Counter code until
MAC-03.

### ffi

None.

### swift

None.

### docs (`vibetyna`)

| File | Change |
|---|---|
| `docs/tasks/recipes/brief.md:143` | F-08 row: mark as done in MAC-11, with the `krucyfiks` change ID. |
| `docs/epics/mac-shell/epic.html` | MAC-11 `status`; `krucyfix` → `krucyfiks`. E-16 is already added (planning). |
| `docs/epics/mac-shell/tasks/MAC-11.md` | This plan; Validation after implementation. |
| Files in "tooling — folder rename" | `krucyfix` → `krucyfiks`. |

`docs/epics/ingredient-catalog/R13-ffi-problems.md` P07 keeps its status
("deferred until Counter removal"). MAC-03 resolves it for the app.

## Steps

The `krucyfix` working copy `@` is `ypqzonus` ("Split effects into cohesive
modules"). It has a description and is not on `main`. `kreatyna` compiles
against this working copy through its path dependencies. So the new change
goes on top of it, and `kreatyna` keeps compiling against the same `core`.

Folder rename:

1. Confirm that no process uses `/Users/frondeus/Code/krucyfix` (`lsof +D`).
2. Run `mv /Users/frondeus/Code/krucyfix /Users/frondeus/Code/krucyfiks`.
3. In `vibetyna`, replace `krucyfix` with `krucyfiks` in the files listed in
   "tooling — folder rename".
4. Run `rg -n krucyfix -g '!target'` in `vibetyna`. Expect only E-16 and this plan.
5. Run `cargo test --workspace` in `vibetyna` (check 6).

Example crate, in `/Users/frondeus/Code/krucyfiks`:

6. Run `jj new ypqzonus -m "Add Counter example; move routing tests to macro-tests"`.
7. Create `examples/counter/` with `Cargo.toml`, `src/lib.rs`,
   `src/capabilities.rs`, `src/composability.rs` and the three test files,
   as listed in "rust — `krucyfiks/examples/counter`".
8. Add `"examples/counter"` to `members` in `Cargo.toml`.

Routing tests, in `/Users/frondeus/Code/krucyfiks/macro-tests`:

9. Create `src/routing.rs` with the fixture and the 17 tests, renamed by the
   mapping table.
10. Add `#[cfg(test)] mod routing;` to `src/lib.rs`.
11. Add the three compile-fail blocks to `src/compile_contracts.md`.

Check and close:

12. Run checks 1–5 in `krucyfiks`. Fix only names, imports, and lint
    errors. Do not change test logic.
13. Run `jj diff --summary` in `krucyfiks`. Confirm that it lists only
    `examples/counter/**`, `macro-tests/src/routing.rs`,
    `macro-tests/src/lib.rs`, `macro-tests/src/compile_contracts.md`,
    `Cargo.toml` and `Cargo.lock`.
14. In `vibetyna`, mark F-08 as done in the brief.
15. Fill in Validation in this plan.
16. Set MAC-11 `status` to `awaiting-review` in the epic data object.
17. Report both change IDs, then stop for review.

## Checks

Run checks 1–5 in `/Users/frondeus/Code/krucyfiks`, check 6 in `vibetyna`.

| # | Command | Expected result |
|---|---|---|
| 1 | `cargo test -p counter-example` | `counter` 35 passed, `background` 23 passed, `composition` 31 passed (89 in total). No doctests. |
| 2 | `cargo test -p krucyfiks-macro-tests` | The existing tests pass, plus 17 new `routing::` tests and 2 new compile-fail doctests. |
| 3 | `cargo test --workspace` | All `krucyfiks` crates pass. |
| 4 | `cargo clippy --workspace --all-targets -- -D warnings` | No warnings. |
| 5 | `cargo fmt --all --check` | No output. |
| 6 | `cargo test --workspace` | All `vibetyna` crates pass against the renamed folder. |

Baseline in `kreatyna`, run on 2026-10-11:

```text
cargo test -p kreatyna --test counter      → 35 passed
cargo test -p kreatyna --test background   → 23 passed
cargo test -p kreatyna --test composition  → 31 passed
cargo test -p kreatyna --lib capabilities  → 17 passed
cargo test -p kreatyna --doc               → 4 passed (3 are Counter compile-fail doctests)
```

Step 12 also records the `macro-tests` count before the change, so check 2
can show "+17 tests, +3 doctests".

## Stubs and exclusions

No stubs.

This slice does not:

- remove anything from `vibetyna`. MAC-03 removes Counter, `AppTab`, the
  composition example, the counter table, the Swift Counter screen, and the
  `kreatyna` copies of the routing tests;
- move the app-level Counter tests (`crates/kreatyna/tests/app.rs`, for
  example `counter_departure_cancels_ticks_but_preserves_accepted_writes`).
  They test `AppWidget` tab and shutdown rules, which MAC-03 removes;
- move Counter's SQLite store (`crates/kreatyna-db/src/sqlite.rs`) or FFI
  code (`crates/kreatyna-ffi/src/app_types/counter.rs`, `app_shell.rs`). The
  example has no real capabilities (E-11);
- add a runnable app or `main` (E-11);
- fix the write-order defect (Q1);
- check or remove overlap between the new routing tests and the existing
  `macro-tests` tests;
- rename anything inside `krucyfiks` (crate names already use `krucyfiks`);
- push the `krucyfiks` change to `origin`.

## Validation

Run on 2026-10-11.

### Deviation from the plan

The plan moved three compile-fail doctests into `compile_contracts.md`. Only
two are new. The first one (a raw leaf enum such as `CounterEffect` has no
`resolve_render`) was already in `compile_contracts.md` as "Deriving the tree
does not grant raw enums recursive resolvers". The two new blocks cover a
composed enum (`Pair`) and an exact branch (`pair.expect_left()`).

Each new block imports `Effect` and `Envelope`, so the only difference from a
compiling program is the missing `.any_route()`. Positive control: with
`.any_route()` added and the blocks marked `no_run`, both compiled (2 passed).
The blocks were then restored to `compile_fail,E0599`.

The test names `database_clients_*` in `routing.rs` stay unchanged, as the
mapping table says, although the fixture capability is now `Store`.

### Checks

| # | Where | Command | Result |
|---|---|---|---|
| — | `vibetyna` | `rg -n krucyfix -g '!target'` | Only the E-16 row in `epic.html` and this plan. |
| 1 | `krucyfiks` | `cargo test -p counter-example` | `counter` 35, `background` 23, `composition` 31 passed (89). 0 unit tests, 0 doctests. |
| 2 | `krucyfiks` | `cargo test -p krucyfiks-macro-tests` | Unit 32 passed (15 before + 17 `routing::`). Doctests 14 passed (12 before + 2). |
| 3 | `krucyfiks` | `cargo test --workspace` | 14 test binaries, all ok. |
| 4 | `krucyfiks` | `cargo clippy --workspace --all-targets -- -D warnings` | No warnings. |
| 5 | `krucyfiks` | `cargo fmt --all --check` | No output, after one `cargo fmt --all` (the renames changed line lengths). |
| 6 | `vibetyna` | `cargo test --workspace` | All pass against `../krucyfiks`. |
| — | `vibetyna` | `cargo clippy --workspace --all-targets -- -D warnings` | No warnings. |
| — | `krucyfiks` | `jj diff --summary` | `Cargo.toml`, `Cargo.lock`, `examples/counter/**` (7 files), `macro-tests/src/{lib.rs,routing.rs,compile_contracts.md}`. `Cargo.lock` only adds packages. |

### Not run

- Swift builds and tests: this slice changes no Swift, Xcode, or FFI code.
  The only `vibetyna` code change is the folder in four path dependencies,
  and `cargo test --workspace` covers it.
- The `krucyfiks` change is not pushed to `origin`.
