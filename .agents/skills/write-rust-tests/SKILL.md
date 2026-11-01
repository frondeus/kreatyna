---
name: write-rust-tests
description: Lay out and write any Rust test module in kreatyna (core, db, ffi, widget, or integration tests) with a fixed section order, test_case/test_matrix parameterization, insta snapshots, and anyhow::Result with ? instead of unwrap or expect. Use whenever you add, move, refactor, or review a Rust test.
---

# Write Rust tests

This skill applies to every Rust test in the workspace: `#[cfg(test)]`
modules, `tests.rs` files next to a module, and files in a crate's `tests/`
folder. Widget tests also follow
[write-widget-tests](../write-widget-tests/SKILL.md) for the krucyfiks parts.

## Module order

Put the imports first. Then write these sections in this order. Start each
section with a plain comment line (`// Green path`). Omit a section that has no
tests; do not invent tests to fill it.

1. **Green path.** The ordinary success: an empty list, stored rows, a
   successful save, an upgrade that keeps the data.
2. **Validation.** Input or state that the code refuses on purpose: a required
   field, a duplicate name, a table constraint, an event that is ignored in the
   current state, a stale result.
3. **Red cases.** Things that went wrong: storage and transport failures,
   corrupted rows, dropped responders, and the retries after them. A successful
   retry stays here.
4. **Test support.** Every constant, fixture, builder, helper function, and
   test-only type, with its trait implementations. Nothing of this goes above
   the tests.

Classify by intent. A table that refuses a duplicate is validation. A stored
row that fails to load is a red case.

## Parameterization

- Give each test one claim. Its name states the expected result.
- Use `#[test_case(...; "name")]` for several examples of one rule.
- Use `#[test_matrix(...)]` for independent dimensions where every combination
  is meaningful.
- Give every case a short descriptive name. Each case starts from fresh state.
- Do not use loops or `bool` switches that combine independent stories.
- Keep success and failure cases in separate tests, not in one table.

## Snapshots with insta

- Use `insta` for a structured value that is long or has many fields: a view, a
  list of rows with several fields, an error report, generated text.
- Prefer `assert_debug_snapshot!` or `assert_yaml_snapshot!` with an inline
  snapshot (`@"..."`) when it is shorter than about 20 lines. Use a file
  snapshot when it is longer.
- Keep `assert_eq!` for a small value that reads clearly in the test (a count,
  one id, `Ok(vec![])`, one error variant).
- Review every new or changed snapshot with `cargo insta review` before you
  accept it. Never accept a snapshot only to make a test pass.
- Add `insta` to a crate's `[dev-dependencies]` when the first snapshot test
  in that crate needs it.

## Async tests

- Write an async test as `#[tokio::test] async fn`. Enable the `macros`
  feature of `tokio` in the crate's `[dev-dependencies]`.
- Put `#[test_case(...)]` lines above `#[tokio::test]`; they work together.
- Do not write a helper that builds a runtime and calls `block_on`. Use
  `#[tokio::test(flavor = "multi_thread")]` only when the test needs threads
  that run at the same time, and say why in a comment.

## Errors: no unwrap, no expect

Every test returns `anyhow::Result<()>` and uses `?` for each step that can
fail. Never call `unwrap()` or `expect()` in a test or a test helper. A test
helper that can fail returns `anyhow::Result<T>` too.

- `Option`: `.context("what was missing")?`.
- A wrong enum variant: `let ... else { anyhow::bail!("expected ...") };`.
- A value that must be an error: `assert_eq!(result, Err(...))`, or
  `assert!(matches!(result, Err(...)))`. Do not `unwrap_err()`.
- Assertions stay `assert!` and `assert_eq!`: they state the claim.
- `#[test_case]` and `#[tokio::test]` work with `-> anyhow::Result<()>`.
- Add `anyhow` to the crate's `[dev-dependencies]` when the first test in that
  crate needs it.

Why: `?` shows which step failed and why. An `unwrap` panic hides it behind a
line number (the user's review of REC-02, 2026-11-01).

## Style

- Write short comments that explain an invariant or a surprise, in the spirit
  of ASD-STE100. Do not narrate the syntax.
- Test behavior that the app can reach. Do not test a scenario only because an
  API allows it.
- Let each test own its claim. Do not repeat assertions that another test
  owns.
- Run the tests of the module after each change.
