---
name: designing-types
description: Design Rust and UniFFI types in kreatyna so that invalid states cannot be represented. Use when adding or changing a view, event, outcome, widget state, domain model, or FFI type, and when writing or reviewing a slice plan that shows types.
---

# Designing types

Make invalid states unrepresentable. If two values must agree, one type holds
both. If a field exists only in some states, it lives inside those states. The
compiler then checks the rule, and Swift's exhaustive `switch` checks it again.

## Rules

1. **One enum variant per state, with that state's data.** Do not write a
   struct of `Option` fields and flags that must agree.
2. **No `Option` for "this does not apply here".** `Option` means "the value
   may be missing" in a state where it belongs (an ingredient's notes). If a
   field is always `None` in one mode and always `Some` in another, split the
   modes into variants.
3. **No `bool` for a mode.** A two-state `bool` becomes an enum with named
   variants once the states carry different data, or when its name needs a
   comment.
4. **No parallel fields that repeat a fact.** A `section` and a `sections`
   list can disagree; a `selected` id can point at a missing row. Derive the
   fact, or put it where only valid values fit.
5. **Newtypes for identity and units.** `IngredientId`, `RecipeId`, `Grams`.
   Never a bare `Uuid`, `String`, or `Decimal` that a caller can mix up.
6. **Parse at the boundary.** Constructors and `TryFrom` check the input once
   (a draft becomes an `IngredientWrite`, FFI strings become ids). Code inside
   trusts the type and does not check again.
   A newtype with a validation or sanitization rule uses the
   [`nutype`](https://docs.rs/nutype) crate, not a hand-written constructor:
   `#[nutype(sanitize(trim), validate(not_empty), derive(...))]` gives
   `try_new` and a typed error such as `RecipeTitleError::NotEmptyViolated`.
   Rust code and the FFI use that error; do not wrap it in a second enum
   (the user, REC-02/Q2, 2026-11-01). Bead `vibetyna-nci` moves the older
   newtypes (`IngredientId`, `RecipeId`, `Grams`, …) to nutype.
7. **Match exhaustively.** Avoid `_ =>` over your own enums, so a new variant
   makes the compiler list every place to update. A catch-all is fine for
   "ignore every other event", and its comment says so.
8. **The FFI mirrors the Rust shape.** A Rust enum with data becomes a
   `uniffi::Enum` with named fields. Do not flatten it into a record of
   optionals for Swift's convenience; Swift `switch`es on it.

## Examples in this codebase

*Good — real:* the delete state of a detail view
(`crates/kreatyna/src/ingredients/detail.rs`). The error exists only while
the user decides; `Deleting` has no error to show.

```rust
pub enum IngredientDetailDeletion {
    Idle,
    Confirming { confirmation: ConfirmationView, error: Option<IngredientDeleteError> },
    Deleting,
    Deleted,
}
```

*Bad — first REC-01 draft:* the stack layout (iOS) has no recipes and one
section, but this type allows recipes on iOS, a missing recipe list on the
Mac, and a selected section that is not in `sections`.

```rust
pub struct AppView {
    pub sections: Vec<AppSection>,
    pub section: AppSection,
    pub ingredients: IngredientsView, // Stack or Split
    pub recipes: Option<RecipeListView>, // None in the stack layout
}
```

*Good — the REC-01 plan:* each layout has exactly its own data.

```rust
pub enum AppView {
    /// iOS: the ingredient catalog on a stack. No sections (E-04).
    Stack(IngredientCatalogView),
    /// Mac: a sidebar with the Recipes and Ingredients sections.
    Split(SplitAppView),
}

pub struct SplitAppView {
    pub section: AppSection,
    pub recipes: RecipeListView,
    pub ingredients: IngredientSplitCatalogView,
}
```

## Checklist for a plan or a review

- For each `Option` field: in which state is it `None`? If the answer names a
  mode, a layout, or a phase, use an enum.
- For each `bool`: would two named variants read better?
- For each pair of fields: can they disagree? If yes, merge them.
- For each `Vec`: may it be empty, or contain duplicates? If not, say why the
  type prevents it, or use a type that does.
- For each FFI type: does Swift receive the same enum shape as Rust?

When a rule costs more than it saves (for example, a non-empty list type for a
list that is validated once and never changes), keep the simple type and write
one sentence in the plan or the doc comment that says why.
