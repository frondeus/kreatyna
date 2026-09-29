---
name: write-widget-tests
description: Write, refactor, or review Rust widget tests in kreatyna using krucyfiks test drivers, readable behavioral scenarios, and parameterized cases.
---

# Write readable widget tests

Tests explain the widget's behavior. Keep them linear and readable like plain
English: starting condition, interaction, visible state, then command behavior.
Respect the requested working mode; coaching or review does not authorize edits.

## Module order

Keep imports first, then these sections in order, marked with comments. Omit empty
sections rather than inventing tests to fill them.

1. **Ordinary behavior:** ordinary success, valid boundaries, and validation
   feedback. An empty catalog or a required-field error belongs here.
2. **Edge cases and guards:** cancellation, repeated actions, stale results,
   invalid event sequences, invalid indices, and relevant lifecycle boundaries.
3. **Failures and recovery:** rejected service operations, storage and transport
   failures, dropped responders, and retries. A successful retry stays here.
4. **Test support:** all fixtures, builders, helper functions, test-only types,
   and their trait implementations. No support boilerplate above the tests.

Classify by intent. Choosing Cancel is ordinary behavior for a confirmation
widget; cancelling pending work is an edge case.

## Scenarios and parameterization

- Give each test one behavioral claim and a name that states the expected result.
- Prefer `test_case` for examples of one rule and `test_matrix` for independent
  dimensions where every combination is meaningful. Use descriptive case names
  where supported and start each case with fresh state.
- Avoid loops or boolean switches that combine independent stories. Keep ordered
  interactions together when their history is the behavior under test.
- Separate ordinary behavior from failures instead of mixing them in one table.
- Use short, direct comments to explain invariants or surprising behavior, in the
  spirit of ASD-STE100. Do not narrate obvious syntax.
- Test reachable application behavior and meaningful guards. An API permitting a
  call does not justify an impossible scenario. Establish relevance before adding
  speculative edge cases.
- Let other tests own their claims. Do not repeat assertions for unrelated setup
  behavior merely to make each test exhaustive.

## State before command behavior

Dispatch the event with `update`, then assert the relevant view **before** advancing
its returned command. Next assert the command's effects, outcomes, and completion.
If the scenario requires applying an outcome, dispatch it explicitly and assert
its resulting view before advancing the next command.

Widget state changes through `update`; advancing a command does not itself apply
its outcomes. Make that distinction visible in the test's order. For repeated
or ignored events, assert unchanged state directly after that interaction, before
advancing earlier pending work. Check relevant status fields as well as values.

## Use krucyfiks test utilities

Avoid `.into_stream()` and manual stream polling or collection in widget tests.
Use the available krucyfiks command drivers: `.driver(...)` for effects,
`.pure_driver()` for pure commands, and their current stepping/assertion APIs
(such as `.step()` or `.drive()`). Inspect the installed API instead of assuming
method names. Use existing helpers such as `expect_noop_init` where appropriate;
keep view assertions explicit because command completion does not prove unchanged
state.

Assert the effects, request arguments, outcomes, and completion relevant to the
claim. Use exact routes when routing is under test. Ignoring outcomes while
finishing a setup command is appropriate when their contents belong to another
test's claim.

Keep local helpers narrow and honestly named. They may assemble fixtures or wrap
supported driver steps, but must not inject unrelated events, hide edge-case
assertions, or silently apply outcomes.

If krucyfiks lacks a needed testing capability, **stop the affected work and ask
for permission to extend its test utilities**. Explain the missing capability and
proposed extension. Do not bypass this boundary with manual streams or a local
replacement testing framework. Continue independent work only if unaffected.

Run the relevant module tests after changes. Assess coverage against the widget's
behavioral contracts, not the number of tests or assertions.
