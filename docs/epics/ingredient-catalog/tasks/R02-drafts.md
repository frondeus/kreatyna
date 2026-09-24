# R02 — Ingredient drafts and typed validation

Status: awaiting review. Change: `pwkoosll`. Dependency R01 is approved.

Implement editable ingredient and reference-unit drafts in `ingredients/draft.rs`
and re-export them from `ingredients`. Drafts contain text fields, explicit
nutrition basis, ordered units, and explicit create/edit mode. Creation generates
no ID; conversion from an ingredient preserves its ID and exact decimal text.

`TryFrom<&IngredientDraft>` returns typed create/update operations containing
private validated write data with read-only getters. Use `From`/`Into` quantities.
Validation never changes the draft. Collect typed field/reason issues in stable
name, nutrition, and unit order, with `usize` indices for unit fields.

Trim names and notes; require names and map empty notes to `None`. Empty nutrition
means unknown, zero is known, and negative nutrition is rejected. Units require
at least one strictly positive measurement; every supplied value must be valid.
Unit names are unique through `name_key`, reporting later duplicates.

Numbers accept an optional minus, ASCII integer digits, and an optional dot or
comma followed by digits. Reject other syntax, then more than six fractional
digits (including trailing zeros), then Decimal overflow, then invalid signs.
Parse with `Decimal::from_str_exact` without rounding. Negative zero is zero.
Keep existing higher-precision values in edit drafts, but require correction
before saving. Return enum issues, without UI text or catalog uniqueness checks.

Add named `test_case` tests for syntax, precision/range, optional values,
measurement combinations, duplicates, issue ordering, create/edit identity,
unchanged drafts, normalization, both bases, and exact edit conversion.
Run core tests, Clippy with warnings denied, FFI compilation, and formatting.
Report existing unrelated workspace formatting failures separately.

Update the epic/R01 approval status and mark R02 awaiting review after checks.
No store capability, SQL, widgets, navigation, Swift, or bindings belong here.

Validation: all 39 draft tests and the existing core tests pass. Clippy with
warnings denied, FFI compilation, and changed-file formatting pass. Workspace
formatting still reports pre-existing import ordering in counter tests and FFI
files; those files are unchanged by R02.
