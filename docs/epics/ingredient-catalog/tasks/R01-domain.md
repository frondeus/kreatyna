# R01 implementation plan: ingredient domain

Tracked in beads: `vibetyna-flx.1`. Change: `tskssmvt`.

Introduce `kreatyna::ingredients` with definitions in `ingredients/model.rs` and
public re-exports in `ingredients/mod.rs`. Add `uuid` (v4), `rust_decimal`, and
`unicode-casefold` to the core crate.

- `IngredientId` privately wraps `Uuid`. `new()` generates a v4 ID; bidirectional
  `From` conversions, `FromStr`, and `Display` preserve identity. No default or nil
  sentinel creates an ingredient.
- `Grams`, `Millilitres`, and `Kilocalories` privately wrap `Decimal`. They expose
  bidirectional `From` conversions and cannot be mixed by type.
  Construction preserves exact value and scale; validity checks belong to R02.
- `NutritionBasis` distinguishes per 100 g from per 100 ml. `Nutrition` carries
  optional calories, protein, carbohydrates, and fat; `None` is unknown and
  `Some(0)` is known zero. `ReferenceUnit` carries a name and optional mass and
  volume. `Ingredient` carries its ID, name, optional notes, nutrition, and
  ordered reference units.
- `name_key` trims surrounding Unicode whitespace and applies full, non-Turkic
  Unicode case folding. It does not normalize accents or internal spacing.

Test identity generation/round trips, decimal fidelity including scale, unit
separation, optional nutrition and reference-unit shapes, and case-folding
semantics. Check `cargo test -p kreatyna`, `cargo fmt --all -- --check`,
`cargo clippy -p kreatyna --all-targets -- -D warnings`, and
`cargo check -p kreatyna-ffi`.

R01 excludes draft parsing and validation, SQL, widgets, navigation, and UniFFI.
Deliver this change for human review before starting another task.
