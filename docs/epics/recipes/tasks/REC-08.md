# REC-08 — Amount, known unit and note on a line

Tracked in beads: `vibetyna-hrx.8`. Changes: eight, listed in Steps.
Dependencies: REC-07 (`vibetyna-hrx.7`), closed. This plan builds on its
lines, their store table and the Mac list.

## Open questions

None. You answered the questions in the REC-08 Inbox thread. They are
in Decisions.

## Decisions

| ID | Question | Answer |
|---|---|---|
| REC-08/Q1 | How do you edit a line's amount, unit, name and note? | The row shows a summary, and a click opens a popover with Amount, Unit ▾, Name and Note, as in the mockup. ＋ Line opens the popover for the new line, with the cursor in Name. REC-09 (custom unit) and REC-10 (catalog link) add their fields to the same popover. |
| REC-08/Q3 | What name do a recipe's ingredients get, instead of "ingredient line"? (It replaces Q2, which asked only about the prefix.) | `RecipeIngredient…`: `RecipeIngredient`, `RecipeIngredientId`, `RecipeIngredientName`, `RecipeIngredientAmount`, `RecipeIngredientQuantity`, `RecipeIngredientNote`, `RecipeIngredientsEvent`. The catalog keeps its names. The table keeps its name, `recipe_ingredient_lines`. The button text stays "＋ Line". |
| REC-08/Q4 | Should an existing crate turn the text into a decimal? | No. Keep the check of the characters and the decimal places, then `rust_decimal`'s `from_str_exact`. `rust_decimal` alone refuses "1,5", accepts "1_000", ".5", "1." and "+2", and has no typed error. No crate parses "1.5" and "1,5" with a place limit. |

## Decided in the plan

| Rule | Follows from |
|---|---|
| The amount is an exact decimal: "1.5" or "1,5", at most 6 decimal places, more than 0. An empty amount means "no amount". | The epic card: "catalog rules". The catalog's parser already has these rules. |
| A unit needs an amount: "g flour" is a problem. An amount without a unit is fine: "2 eggs". | The epic card. |
| The note is optional. Rust trims it, and an empty note is no note. | D-12 ("a pinch", "to taste"). |
| The unit picker offers "No unit", then g, kg, ml, l, tsp, tbsp, cup and piece, in that order. Rust gives the list, as it gives the duration chips. | D-25; the duration chips (REC-17). |
| The problems (name, amount) show from the first Save on, under the row's summary and under the field in the popover. Any problem blocks Save. | The name rule (REC-07). |
| Rust formats the read view's amount: "1,50" shows as "1.5". "cup" and "piece" get a plural for an amount other than 1: "2 cups", "3 pieces". The other units are symbols and never change. | The read view shows what is stored, not what was typed. |
| The editor's summary shows the amount as typed, trimmed, and the unit: "1,50 kg". The popover shows the fields as typed. | The editor keeps raw text until Save (REC-07, the name). |
| An unnamed line's summary shows the amount and the note, and the grey text "No name" in place of the name. | The step's "Untitled step" (REC-14). |
| Swift owns which popover is open, and for which line, like the keyboard focus. Rust does not know that a popover is open. When ＋ Line adds a line, Swift opens its popover. | AGENTS.md: Swift may own transient presentation details. The popover only shows fields; every edit is still a Rust event. |
| A click on the summary opens the popover. The handle ≡ still drags, and − still removes at once. | REC-07. |

## Outcome

You edit "Roasted tomato pasta". The Ingredients list shows
"penne pasta" from REC-07. You click it, and a popover opens. You type 300
in Amount and pick g in Unit. The row now reads **300 g** penne pasta. You
click ＋ Line. A popover opens for the new line, with the cursor in Name. You
type "olive oil", then 1 in Amount, tbsp in Unit and "extra virgin" in Note.
You add "salt" with the note "to taste", and Save. The read view shows:

```
Ingredients
  300 g   penne pasta
  1 tbsp  olive oil, extra virgin
          salt, to taste
Steps
  ...
```

If you pick g for a line with no amount and Save, the recipe does not save,
and that line shows "Amount needed for a unit".

## Terms

| Term | Meaning |
|---|---|
| Amount | The number on a line, such as 300 in "300 g penne pasta". It is an exact decimal (`rust_decimal::Decimal`), not a float, so 0.1 stays 0.1. |
| Known unit | One of the 8 units that the app knows: g, kg, ml, l, tsp, tbsp, cup, piece (D-25). Only known units can convert later. REC-09 adds custom units such as "knob". |
| Note | Free text after the name, such as "extra virgin" or "to taste" (D-12). |
| Popover | A small floating panel that points at the control that opened it. SwiftUI shows one with `.popover`. |
| `nutype` | The crate that makes a validated newtype from a field, such as `RecipeIngredientName`. `try_new` returns the value or a generated error. |

## Design

[REC-08-design.html](REC-08-design.html) shows, in light and dark:

1. The editor with the popover open for "olive oil" and the Unit menu open.
2. Rows and a popover with problems after the first Save.
3. The read view, with the amounts in a column on the left.

The popover follows the mockup's line editor (section 3), without the catalog
link and the custom unit, which come in REC-10 and REC-09. The read view
follows the mockup's "How lines look in the list".

## Changes by layer

### db (`crates/kreatyna-db`)

A new forward-only migration,
`2026-11-07-120000-0000_add_line_amounts` (*sketch*):

```sql
-- An ingredient's amount, known unit and note (REC-08). The amount is exact
-- decimal text, as in the catalog. A unit needs an amount.
ALTER TABLE recipe_ingredient_lines ADD COLUMN amount TEXT;
ALTER TABLE recipe_ingredient_lines ADD COLUMN unit TEXT
    CHECK (unit IS NULL OR (amount IS NOT NULL
        AND unit IN ('g', 'kg', 'ml', 'l', 'tsp', 'tbsp', 'cup', 'piece')));
ALTER TABLE recipe_ingredient_lines ADD COLUMN note TEXT
    CHECK (note IS NULL OR length(trim(note)) > 0);
```

`down.sql` is empty (forward-only). REC-07's `down.sql` drops its table,
which breaks the forward-only rule; change 2 empties it.

SQLite accepts a column `CHECK` that names another column in `ADD COLUMN`.
A quick check with `sqlite3` refused "g" with no amount and kept the older
rows.

`recipe_models.rs`: `RecipeIngredientRow` and `NewRecipeIngredientRow` get `amount`, `unit` and
`note`. `DbRecipeIngredientAmount(RecipeIngredientAmount)` stores the decimal as text, as the
catalog's quantities do. `DbKnownUnit(KnownUnit)` stores the unit's code. A
row with a unit and no amount fails to read, as a blank name does.

`recipe_store.rs`: no new query. `load_ingredients` and `insert_ingredients` carry the
new columns.

### rust (`crates/kreatyna/src/recipes`)

`model.rs` (*sketch*):

```rust
/// An ingredient's amount: an exact decimal, more than 0 (REC-08). `try_new`
/// fails with `RecipeIngredientAmountError::GreaterViolated`.
#[nutype(validate(greater = Decimal::ZERO), derive(Debug, Clone, Copy, PartialEq, Eq, Display, TryFrom, Into))]
pub struct RecipeIngredientAmount(Decimal);

/// The units that the app knows (D-25), in picker order.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum KnownUnit { Gram, Kilogram, Millilitre, Litre, Teaspoon, Tablespoon, Cup, Piece }

/// How much of the ingredient the recipe needs. A unit without an amount
/// cannot be built.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum RecipeIngredientQuantity {
    /// "salt, to taste"
    Unmeasured,
    /// "2 eggs"
    Count(RecipeIngredientAmount),
    /// "300 g penne pasta"
    Measured { amount: RecipeIngredientAmount, unit: KnownUnit },
}

/// An ingredient's note, such as "extra virgin": trimmed and not empty.
#[nutype(sanitize(trim), validate(not_empty), derive(Debug, Clone, PartialEq, Eq, AsRef))]
pub struct RecipeIngredientNote(String);

pub struct RecipeIngredient {
    pub id: RecipeIngredientId,
    pub name: RecipeIngredientName,
    pub quantity: RecipeIngredientQuantity,
    pub note: Option<RecipeIngredientNote>,
}
```

REC-09 changes `Measured`'s unit into a `RecipeIngredientUnit` with a custom case. The
other cases stay.

A new module `crate::decimal_input` holds the catalog's text-to-decimal
rules, moved from `ingredients/draft.rs` (an existing helper, not a second
one). It checks the format, the decimal places and the range. Each caller
keeps its own "more than 0" or "not negative" rule (*sketch*):

```rust
pub(crate) enum DecimalInputError { NotANumber, TooManyDecimalPlaces, OutOfRange }

/// "1.5" or "1,5", at most 6 decimal places.
pub(crate) fn parse_decimal(input: &str) -> Result<Decimal, DecimalInputError>;
```

`recipe_ingredients.rs` (*sketch*):

```rust
pub enum RecipeIngredientsEvent {
    Add,
    SetName { id: RecipeIngredientId, name: String },
    /// The Amount field, as typed. Rust parses it only on Save.
    SetAmount { id: RecipeIngredientId, amount: String },
    /// The Unit picker. `None` is "No unit".
    SetUnit { id: RecipeIngredientId, unit: Option<KnownUnit> },
    /// The Note field, as typed.
    SetNote { id: RecipeIngredientId, note: String },
    Remove { id: RecipeIngredientId },
    Move { id: RecipeIngredientId, onto: RecipeIngredientId },
}

/// Why Save refuses an ingredient's amount.
pub enum RecipeIngredientAmountProblem {
    NotANumber,
    TooManyDecimalPlaces,
    OutOfRange,
    /// "0" or "-1".
    NotPositive,
    /// A unit is picked, and the amount is empty.
    MissingForUnit,
}

pub struct RecipeIngredientProblems {
    pub name: Option<RecipeIngredientNameError>,
    pub amount: Option<RecipeIngredientAmountProblem>,
}

pub struct RecipeIngredientView {
    pub id: RecipeIngredientId,
    /// The popover's fields, as typed.
    pub name: String,
    pub amount: String,
    pub unit: Option<KnownUnit>,
    pub note: String,
    /// What the row shows.
    pub summary: RecipeIngredientSummary,
    /// All `None` until the first Save.
    pub problems: RecipeIngredientProblems,
}

/// The row's text: "1,50 kg" + "tomatoes" + "chopped". Swift joins the parts.
pub struct RecipeIngredientSummary {
    /// The amount as typed and trimmed, and the unit's label. `None` with
    /// neither.
    pub amount: Option<String>,
    pub name: RecipeIngredientLabel,
    pub note: Option<String>,
}

pub enum RecipeIngredientLabel { Named { name: String }, Unnamed }

/// One choice of the Unit picker, after "No unit".
pub struct KnownUnitOption { pub unit: KnownUnit, pub label: String }
```

`DraftRecipeIngredient` gets `amount: String`, `unit: Option<KnownUnit>` and
`note: String`. `writes()` builds each `RecipeIngredient` or returns the
problems. `differs_from` compares the new fields too, so a changed amount
makes Cancel ask "Discard changes?".

`editor.rs`: `RecipeEditorView` gets `units: Vec<KnownUnitOption>`.

`detail.rs`: `RecipeDetailIngredient` gets `amount: Option<String>` ("300 g",
"2 cups", formatted) and `note: Option<String>`.

A widget test (*sketch*):

```rust
#[tokio::test]
async fn a_unit_without_an_amount_blocks_save_on_that_ingredient() -> anyhow::Result<()> {
    let mut f = Fixture::new_recipe("Pasta").await?;
    let flour = f.add_ingredient("flour").await?;
    f.send(Ingredients(SetUnit { id: flour, unit: Some(KnownUnit::Gram) })).await?;
    f.save_refused().await?;
    assert_eq!(f.ingredient(flour)?.problems.amount, Some(RecipeIngredientAmountProblem::MissingForUnit));
    Ok(())
}
```

### ffi (`crates/kreatyna-ffi`)

`recipes.rs` mirrors `KnownUnit`, `RecipeIngredientAmountProblem`, `RecipeIngredientProblems`,
`RecipeIngredientSummary`, `RecipeIngredientLabel`, `KnownUnitOption`, the new events, and the new
view fields. `RecipeIngredientView.problem` becomes `problems`.

### swift

- `MacRecipeIngredients.swift`: each row shows the summary as text, not a name
  field: the amount in bold, the name, and ", note" in grey
  (`recipeIngredientSummary.<n>`). The problems show in red under it
  (`recipeIngredientProblem.<n>`). A click on the summary opens the popover.
  ＋ Line opens the popover for the new line, with the cursor in Name.
- New `MacRecipeIngredientEditor.swift`: the popover. Amount
  (`recipeIngredientAmount`), a Unit `Picker` with "No unit" and Rust's
  options (`recipeIngredientUnit`), Name (`recipeIngredientName`) and Note
  (`recipeIngredientNote`), each a `SyncedTextField`, with each problem under
  its field.
- `ErrorText.swift`: the text for each `RecipeIngredientAmountProblem`: "Amount is not a
  number", "At most 6 decimal places", "Amount is too large", "Amount must be
  more than 0", "Amount needed for a unit".
- `MacRecipeDetail.swift`: each line shows its amount in a column on the
  left, then the name, then ", note" in grey.
- `SharedTests/Fixtures.swift`: lines with amounts, units and notes.
- Snapshots (`MacScreenSnapshotTests`): the row summaries, the popover
  (filled, and with problems), and the read view with amounts, in light and
  dark. REC-07's editor snapshots change: the rows show summaries.
- UI test (`MacRecipeUITests`): `testRecipeIngredientsAddDragRemoveAndSave`
  types names in the popover. A new test sets 300 g and a note, saves, and
  finds "300 g" in the read view.

### docs

- The epic page: the REC-08 card gets `plan: "tasks/REC-08.md"`.
- This plan, its design page, and the demo.

## Steps

### Change 1 — REC-08: plan (this change, no Rust)

1. Write this plan and `REC-08-design.html`. Look at the design in light and
   dark.
2. Set the REC-08 card's `plan` on the epic page and the bead's spec.
3. Run `bd export -o .beads/issues.jsonl`.

### Change 2 — REC-08: lines store an amount, a unit and a note (about 450 lines, review)

1. Add the migration and the schema columns. Empty REC-07's `down.sql`.
2. Add `RecipeIngredientAmount`, `KnownUnit`, `RecipeIngredientQuantity` and `RecipeIngredientNote`, and the new
   `RecipeIngredient` fields.
3. Add `DbRecipeIngredientAmount` and `DbKnownUnit`, and carry the columns in
   `load_ingredients` and `insert_ingredients`.
4. Add store tests: each quantity case and a note round-trip; a unit without
   an amount and a blank note fail in the database; a REC-07 line upgrades
   with no amount, no unit and no note.
5. Keep the editor writing `RecipeIngredientQuantity::Unmeasured` and no note until
   change 3.

### Change 2a — REC-08: ingredient lines become recipe ingredients (about 630 lines, review)

1. Rename the types, the modules and the files of REC-07 and change 2 to
   the `RecipeIngredient…` names (REC-08/Q3).
2. Rename the Swift accessibility ids to `recipeIngredient…`.
3. Rename the code names in this plan.

### Change 2b — REC-08: recipe ingredients in fields, variables and tests (about 700 lines, review)

1. Rename the `lines` fields, the `Lines` event variant, the variables, the
   test names, the comments and the snapshot files.

### Change 3a — REC-08: the read view shows amounts and notes (about 230 lines, review)

Change 3 was about 1,010 lines, so it is two changes.

1. Move the catalog's decimal rules to `crate::decimal_input`. The catalog
   tests do not change.
2. Give `KnownUnit` its picker order and names, and
   `RecipeIngredientQuantity` its read-view text.
3. Format the read view's amount and note in `detail.rs`.
4. Add Rust tests in `tests/recipe_detail.rs`: "2", "300 g", "1.5 kg",
   "1 cup", "2 cups", "0.5 pieces", and a note without an amount.

### Change 3b — REC-08: the editor edits amounts, units and notes (about 790 lines, review)

1. Add the new events, fields, problems, summary and unit options to
   `recipe_ingredients.rs` and the editor.
2. Add Rust tests in `tests/recipe_ingredients.rs`: each event; each amount
   problem; a unit without an amount; "2 eggs" and "salt, to taste" save;
   the summary as typed; a changed amount, unit or note makes Cancel ask.

### Change 4 — REC-08: the line popover on the Mac (about 700 lines, review of the FFI)

1. Mirror the types in the FFI.
2. Build the summary row and the popover, and the read view's amount column.
3. Update the fixtures, the snapshots and the UI tests.

### Change 5 — REC-08: the demo (about 100 lines, report)

1. Record the demo per document-apple-delivery: the popover with the unit
   menu, a row summary, "Amount needed for a unit", and the read view.

## Checks

| Command | Expected |
|---|---|
| `devenv shell -- cargo fmt --check` | No output. |
| `devenv shell -- cargo clippy --workspace --all-targets -- -D warnings` | No warnings. |
| `devenv shell -- cargo test --workspace` | All pass; the new store and editor tests among them. |
| `devenv shell -- mac-bindings debug` | Generates the bindings. |
| `devenv shell -- mac-test --ui MacRecipeUITests` | All unit and snapshot tests, and the recipe UI tests, pass. |
| `devenv shell -- ios-app-build` | Builds. |

## Stubs and exclusions

| Stub or exclusion | Replaced by |
|---|---|
| Only the 8 known units. | REC-09: custom units. |
| No catalog link, and no catalog reference units in the picker. | REC-10, REC-12. |
| Fractions such as "½" or "1/2" are not numbers. | Not planned. |
| No unit conversion and no scaling. | Not planned in the MVP (D-12 makes them possible). |
| The amount shows with a dot ("1.5"), whatever the system locale. | Not planned. |

## Validation

Not run yet.
