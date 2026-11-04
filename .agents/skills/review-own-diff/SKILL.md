---
name: review-own-diff
description: Check your own Rust diff in kreatyna against the rules the user keeps catching in review, before every guided review. Use right before submit_guide, for a slice change, a cooldown change, or a round of review fixes.
---

# Review your own diff

Introspection 1 (2026-11-04) read every comment the user had made. In 9 of
them, a skill already held the rule, and the user still caught the mistake in
review: a primitive where a newtype belongs, a `bool` for a mode, a value that
belongs in one state only. The rule was there, but it was not applied while the
code was written. This checklist applies it before the user sees the diff.

Do it before every `submit_guide` (see
[use-plannotator-inbox](../use-plannotator-inbox/SKILL.md)). Clippy finds some
of these mistakes, but not all. Read the patch that you will send:

```sh
jj diff --git -r @ 'glob:"**/*.rs" | glob:"**/Cargo.toml" | glob:"**/migrations/**"'
```

## The checklist

Go through each added or changed line. For each item, fix the code, or write
why it stays.

1. **Primitives.** For each `u32`, `i64`, `String`, `Uuid` or `f64` field or
   argument, ask: is it an id, a unit, or a count? Use a newtype for an id or a
   unit, and put the unit in the name (`StepDurationS`, not `StepDuration`).
   A database local id gets a newtype too (`LocalRecipeId`).
   *Caught before:* "IngredientId? Seriously?", `RecipeId` as `String`,
   `local_id: i64`.
2. **Modes and shapes.** For each `bool`, `Option` and tuple, ask:
   - A `bool` that chooses a behavior is a mode. Use an enum with two named
     variants. For a "did it change" result, use `CanvasChange`.
   - An `Option` that is `None` in one mode and `Some` in another is a variant
     of that mode.
   - A tuple return holds two values. Give them a named struct.
   *Caught before:* `apply(...) -> bool`, `opened_with: String` outside
   `Saved`, `-> (&str, &[Step])`.
   Follow [designing-types](../designing-types/SKILL.md).
3. **Existing helpers.** For each new helper, loop, lookup, wrapper or
   `id: CommandId` field, search for one that already exists, in this
   workspace and in krucyfiks:

   ```sh
   rg -n '<what it does>' crates/ ../krucyfiks/
   ```

   Use it if it exists. If the same code is now in two places, make one
   helper.
   *Caught before:* `Scoped<T>`, `ScopedChild`, `Scoped::map`, `Latest<T>`,
   `tokio::test` in place of a copied `run()`, nutype in place of a
   hand-written `parse`, a step lookup.
4. **Names.** Read each new name without its doc comment. Is it clear what the
   name means? Do two names that are near each other say how they differ?
   *Caught before:* `previous` and `switch_to`, `before` in place of
   `from`/`to`.
5. **Tests.** Check the test layout of
   [write-rust-tests](../write-rust-tests/SKILL.md) (green path, validation,
   red cases, support). Check `anyhow::Result` with `?`, and no `unwrap`,
   `expect` or `panic!`.
6. **Work in the right layer.** Does Rust sort, filter or count something
   that the database query can do? Does Swift decide something that Rust
   should own?
   *Caught before:* sorting the recipe list in Rust.

## In the guide

End the guide's body with one line: `Self-review: done.` If an item stays as
it is on purpose, name it and say why, for example `Self-review: dirty: bool
stays, because the FFI view mirrors a two-state flag that Swift only
shows.`
