# R09a — Command scopes for child lifetimes

Status: approved. Framework change: `oxtsnomz`; documentation change: `txlxmntq`.
R09 is approved. R10 depends on R09a.

## Contract and implementation

Add framework `CommandScope` with new, Default, id, and accepts. The scope is not
Clone or Copy. Opaque `CommandScopeId` derives Debug, Clone, Copy, PartialEq, Eq
and uses checked process-local identity allocation.

`CommandExt::scoped(self, &mut scope)` wraps outcomes in `Scoped<T>`, pairing the
value with its scope ID. The same public wrapper is used when routing scoped
events. Multiple independent commands can share one lifetime. The scope owns a
`CancellationToken` with a drop guard. Each scoped stream watches a clone of that
token and releases its wrapped stream when cancelled or completed. Scope drop
wakes pending commands and cancels unpolled commands before they run. Completing
or dropping one command does not affect its siblings. No handle registry, mutex,
or registration counter is needed. `CommandExt::abortable(self)` remains the
separate, fluent way to cancel an individual command and returns its abort handle.

Queued outcomes remain valid while the scope lives; parent routing checks their
ID against the current child scope. Replacements receive fresh scopes. The same
ID can tag child events. Existing latest-command behavior is unchanged and
composes with scoping. Cancellation stops local futures, not necessarily external
operations already dispatched. No widget wrapper, reset API, bindings, or
navigation policy belongs in the framework utility.

## Validation

Test concurrent and multi-item commands, sibling independence, unpolled
cancellation, queued identity checks, isolated scopes, pending cancellation,
wakeups and stream release, commands without outcomes, and composition with
fluent abortable and latest commands.
Run framework tests and Clippy with warnings denied, consumer core tests, FFI
compilation, and formatting checks. Report unrelated formatting failures.

## R10 decisions

- Apply accepted saves/deletions to the retained list cache, preserving search.
- Back dismisses a delete confirmation before leaving detail.
- Each child instance owns a scope; replacements reject stale events/outcomes.
- Derive navigation from owned child state, without a second mutable route.

Coordinator implementation remains R10. Each repository has one R09a change;
mark awaiting review only after validation.

## Results

Framework workspace tests (including eight scope tests), workspace Clippy with
warnings denied, consumer core tests, and FFI compilation passed offline.
Framework formatting passed. Consumer workspace formatting reports only existing
import-order differences in core tests/background.rs, core tests/composition.rs,
and kreatyna-ffi/src/lib.rs. No coordinator code was changed.
