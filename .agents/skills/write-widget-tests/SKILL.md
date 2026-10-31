---
name: write-widget-tests
description: Write, refactor, or review Rust widget tests in kreatyna using krucyfiks test drivers and readable behavioral scenarios. Use together with write-rust-tests, which sets the module order and parameterization.
---

# Write readable widget tests

Tests explain the widget's behavior. Keep them linear and readable like plain
English: starting condition, interaction, visible state, then command behavior.
Respect the requested working mode; coaching or review does not authorize edits.

## Module order and parameterization

Follow [write-rust-tests](../write-rust-tests/SKILL.md): green path,
validation, red cases, then test support at the bottom, with `test_case`,
`test_matrix`, and `insta` snapshots. For widgets:

- An ignored event, a repeated action, a cancelled command, or a stale result
  is validation.
- A rejected service operation, a storage or transport failure, a dropped
  responder, and a retry are red cases.
- Classify by intent. Choosing Cancel is the green path for a confirmation
  widget; cancelling pending work is validation.
- Keep ordered interactions in one test when their history is the behavior
  under test.

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
