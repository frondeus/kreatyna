# REC-17 — Duration, active/waiting and instructions

Tracked in beads: `vibetyna-hrx.17`. Changes: five, listed in Steps.
Dependencies: REC-14 (`vibetyna-hrx.14`), closed. REC-15 and REC-16 are
closed too; this plan builds on their canvas.

## Open questions

None. You answered four questions in the REC-17 Inbox thread. They are in
Decisions.

## Decisions

| ID | Question | Answer |
|---|---|---|
| REC-17/Q1 | How do you type a duration? | After a design round ([REC-17-design.html](REC-17-design.html)): **E. Presets, then More…** Chips for common lengths and a minutes field; More… opens one field per unit: d · h · min · s. |
| REC-17/Q2 | What does the read view show for each step? | The title, the duration, "waiting", and the instructions under the step in smaller text. |
| REC-17/Q3 | Do durations need seconds? | Yes. Rust stores seconds. |
| REC-17/Q4 | Do durations need days? | Yes, up to one year (365 days), for long recipes such as a sourdough starter. |

## Decided in the plan

| Rule | Follows from |
|---|---|
| A new step is Active, with no duration and no instructions. | Q-R5: no duration until set. The mockup shows Active selected. |
| A duration is 1 second to 365 days (31,536,000 seconds), in whole seconds. | Q3, Q4. It fits in a `u32`. |
| Swift sends what you set: a cleared field, whole minutes (a chip or the minutes field), or the four unit fields. Rust adds them up and checks the range. | Rust owns the rule (AGENTS.md). Swift never adds units. |
| All unit fields at 0 means "no duration", as an empty minutes field does. | The design page: "All zero means no duration." |
| More than 365 days is a problem. It shows after the first Save, as for titles. Save refuses and selects the first step with any problem. | REC-14/Q2. The ingredient form works the same way. |
| An empty duration is not a problem until REC-22. | E-02, Q-R5. |
| Rust formats a duration with the largest units first and leaves out zero units: `45 s`, `1 min 30 s`, `1 h 30 min`, `2 d`. | The design page, section 2. One Rust function serves the node, the read view, and REC-21's total. |
| More… opens by itself when the duration has seconds or days, so the fields show the whole value. | The design page, variant E. Swift owns this presentation state. |
| The chips are 1, 2, 5, 10, 15, 20, 30, 45 min, and 1 h. | The design page, variant E. |
| Instructions are free text over several lines. Save trims them, and an empty text is "no instructions". No length limit. | D-09: "optional longer instructions". |
| A node's second line is `20 min · waiting` or `No duration · active`. A waiting node has a dashed border. | The mockup and the design page. |
| Steps saved before this slice read as Active, with no duration and no instructions. | The migration's defaults. Forward-only migrations. |

## Outcome

You open Pasta and click Edit. You click "Boil water". The inspector shows
Title, Duration, Active/Waiting, and Instructions. You click the **10 min**
chip, choose **Waiting**, and type "Salt it well." in Instructions. The node
shows `10 min · waiting` with a dashed border.

You add a step "Sear scallops". You click **More…** and set 0 d, 0 h,
1 min, 30 s. The node shows `1 min 30 s · active`. You click Save. The
read view lists "Boil water · 10 min · waiting" with "Salt it well." under
it, and "Sear scallops · 1 min 30 s".

## Terms

- **Duration:** how long a step takes, in whole seconds (Q3).
- **Active / waiting:** whether your hands are busy during the step
  (active) or free (waiting), as in "Bake 20 min" (D-09, D-30).
- **Instructions:** optional longer text for a step, such as "Medium heat,
  stir often" (D-09).
- **Chip:** a small round button with a common length, such as "10 min".
- **Migration:** a SQL file that changes the database schema. Diesel runs
  each one once, in order. This project never edits or removes an applied
  migration (forward-only).
- **nutype:** a Rust crate that builds a type with a checked constructor,
  `try_new`, so a value that breaks the rule cannot exist.

## Design

The targets are variant E of [REC-17-design.html](REC-17-design.html) and
section 2 of the [recipes mockups](../../../tasks/recipes/mockups.html).

| Part | What you see |
|---|---|
| Inspector, Duration | A minutes field with "min" after it and a **More…** link; under it the chips. More… shows the unit fields d · h · min · s, each with ↑ ↓ arrows, and its link becomes **Less**. |
| Inspector, kind | A two-part Active/Waiting switch under Duration. |
| Inspector, Instructions | A text field that grows to several lines. |
| Inspector, After | Stays a disabled placeholder (REC-18). |
| Node | The title, then a smaller second line: `10 min · waiting`. Waiting nodes have a dashed border. |
| Problem | A red node border, and the message under the field that has the problem. |
| Read view | Each step: "Boil water · 10 min · waiting", then its instructions in smaller, secondary text. |

## Changes by layer

### db (`crates/kreatyna-db`)

New migration `2026-11-04-120000-0000_add_step_details` (*sketch*):

```sql
-- Duration, kind, and instructions of a step (REC-17). Old steps become
-- active steps with no duration and no instructions.
ALTER TABLE recipe_steps ADD COLUMN duration_seconds BIGINT
    CHECK (duration_seconds BETWEEN 1 AND 31536000);
ALTER TABLE recipe_steps ADD COLUMN kind TEXT NOT NULL DEFAULT 'active'
    CHECK (kind IN ('active', 'waiting'));
ALTER TABLE recipe_steps ADD COLUMN instructions TEXT
    CHECK (instructions IS NULL OR length(trim(instructions)) > 0);
```

- `schema.rs`, `StepRow`, and `NewStepRow` get the three columns.
  `DbStepKind` converts `StepKind` to and from its text.
- Tests (`recipe_store/tests.rs`): a step keeps its duration, kind, and
  instructions through create, update, and get. A step from before REC-17
  reads as active with neither. A row with a kind other than the two fails
  to read.

### rust (`crates/kreatyna/src/recipes`)

New types in `model.rs` (*sketch*):

```rust
/// A step's duration in whole seconds: 1 s to 365 days (Q3, Q4).
#[nutype(
    validate(greater = 0, less_or_equal = 31_536_000),
    derive(Debug, Clone, Copy, PartialEq, Eq, Into)
)]
pub struct StepDurationS(u32);

/// Whether your hands are busy during a step (D-09, D-30).
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub enum StepKind {
    #[default]
    Active,
    Waiting,
}

/// A step's longer text: trimmed and not empty (D-09).
#[nutype(sanitize(trim), validate(not_empty), derive(Debug, Clone, PartialEq, Eq, AsRef))]
pub struct StepInstructions(String);

pub struct Step {
    pub id: StepId,
    pub title: String,
    pub position: CanvasPoint,
    pub duration: Option<StepDurationS>,
    pub kind: StepKind,
    pub instructions: Option<StepInstructions>,
}
// `StepWrite` gets the same three fields.
```

New module `duration.rs`, pure functions (*sketch*):

```rust
/// What the Duration field sends (Q1, variant E).
pub enum DurationInput {
    /// An empty minutes field, or every unit field at 0.
    Cleared,
    /// A chip, or the minutes field.
    Minutes(u32),
    /// The unit fields of More….
    Parts(DurationParts),
}

/// A duration split into units. Rust fills it for the unit fields; Swift
/// sends it back as typed. A field may hold more than its unit's range,
/// such as 75 min: Rust adds it up all the same.
pub struct DurationParts {
    pub days: u32,
    pub hours: u32,
    pub minutes: u32,
    pub seconds: u32,
}

/// Why a duration cannot be saved.
pub enum DurationError {
    /// More than 365 days.
    TooLong,
}

/// `Ok(None)` for `Cleared` and for all parts at 0. Adds with checked
/// arithmetic, so a huge field is `TooLong`, never an overflow.
pub fn duration_from(input: DurationInput) -> Result<Option<StepDurationS>, DurationError>;

/// `StepDurationS` split into days, hours, minutes, seconds.
pub fn parts_of(duration: StepDurationS) -> DurationParts;

/// "45 s", "1 min 30 s", "1 h 30 min", "2 d".
pub fn format_duration(duration: StepDurationS) -> String;
```

**`step_canvas.rs`:**

- `DraftStep` keeps the duration as the editor has it:

  ```rust
  enum DraftDuration {
      NotSet,
      Set(StepDurationS),
      /// What you set, kept so the fields still show it, with the problem.
      TooLong(DurationParts),
  }
  ```

  It also keeps the kind and the raw Instructions text.
- Three new events, each ignored for an unknown id and `Unchanged` for
  the same value:

  ```rust
  SetDuration { id: StepId, input: DurationInput },
  SetKind { id: StepId, kind: StepKind },
  SetInstructions { id: StepId, text: String },
  ```

- `writes()` checks the title and the duration of each step. It fails
  with the id of the first step that has any problem.
- `differs_from` compares the new fields too: the duration as seconds, and
  the instructions after the same trim as Save.

`StepNodeView` (*sketch*):

```rust
pub struct StepNodeView {
    pub id: StepId,
    pub title: String,              // raw, for the Title field
    pub label: StepLabel,
    pub position: CanvasPoint,
    pub duration: StepDurationView,
    pub kind: StepKind,
    pub instructions: String,       // raw, for the Instructions field
    /// Only after the first Save.
    pub problems: StepProblems,
}

pub enum StepDurationView {
    /// "No duration". The fields are empty.
    NotSet,
    Set {
        /// `format_duration`: what the node shows.
        text: String,
        /// For the unit fields.
        parts: DurationParts,
        /// For the minutes field and the chips: `None` when the duration
        /// has seconds, which opens More….
        whole_minutes: Option<u32>,
    },
    /// The node shows "No duration"; the unit fields show `parts`.
    TooLong { parts: DurationParts },
}

/// The problems of one step. Each field is independent of the other.
pub struct StepProblems {
    pub title: Option<StepTitleError>,
    pub duration: Option<DurationError>,
}
```

`problem: Option<StepTitleError>` becomes `problems: StepProblems`.

**`detail.rs`:** `RecipeDetailStep` gets `duration: Option<String>` (the
formatted text), `kind: StepKind`, and `instructions: Option<String>`
(Q2).

**Tests:**

| File | It checks |
|---|---|
| `tests/step_duration.rs` (new) | `duration_from` for `Cleared`, minutes, parts, all parts at 0, 75 min in the minutes part, exactly 365 d, 365 d 1 s, and fields large enough to overflow. `parts_of` and `format_duration` for 45 s, 90 s, 6 min, 1 h 30 min, 8 h, 2 d, and 365 d. |
| `tests/recipe_editor.rs` | A chip sets the duration and renders. The view for a duration with seconds (`whole_minutes` is `None`). Clearing the duration. Switching to Waiting. Instructions keep the raw text. Save writes the duration, the kind, and trimmed instructions (`None` for spaces). Save refuses more than 365 days and selects that step. Cancel asks after each kind of change. Setting 90 minutes as `Minutes(90)` and as parts 1 h 30 min on a saved 90-minute step keeps the editor unchanged. The new events are ignored for an unknown id, for the same value, while saving, and while Discard is asked. |
| `tests/recipe_detail.rs` | The read view's steps carry the formatted duration, the kind, and the instructions. |

### ffi (`crates/kreatyna-ffi`)

- `StepKind`, `DurationInput`, `DurationParts`, `StepDurationView`,
  `StepProblems`, and `DurationError` as UniFFI types.
- The three new events, and the new `StepNodeView` and `RecipeDetailStep`
  fields.
- Conversion tests in `app_types/tests.rs`.

### swift

New `MacDurationField.swift` (variant E):

- A minutes field with "min" after it. It shows `whole_minutes`. Typing
  sends `.minutes(n)`; an empty field sends `.cleared`.
- The chips: each sends `.minutes(n)`. The chip equal to `whole_minutes`
  is highlighted.
- **More…** shows four unit fields with ↑ ↓ arrows (0–365 d, 0–23 h, 0–59
  min, 0–59 s). They show `parts`, and any change sends `.parts(…)`.
- More… opens by itself when `whole_minutes` is `nil` or `parts.days > 0`.

`MacStepInspector.swift`:

- Duration (the new field), then the Active/Waiting switch (a segmented
  `Picker`), then Instructions (a `SyncedTextField` with `axis: .vertical`
  and 3 to 6 lines).
- Each field shows its own problem under it.

`MacStepCanvas.swift`:

- `MacStepNode` gets the second line, and a dashed border for a waiting
  step. The node height grows from 54 to 64 points.

`MacRecipeDetail.swift`:

- Each step shows "title · duration · waiting" and the instructions under
  it.

`ErrorText.swift`: `DurationError.tooLong` reads "A duration can be at most
365 days."

**Unit tests** (`MacDurationFieldTests`): the minutes field text sends
`.minutes`, `.cleared`, and nothing for letters.

**Snapshots** (`MacScreenSnapshotTests`, light and dark):

- New `recipeCanvasActiveAndWaiting`: an active and a waiting node, each
  with a duration, and one with "No duration".
- New `recipeInspectorFilled`: a chip highlighted, Waiting, and
  instructions.
- New `recipeInspectorMore`: More… open with 1 min 30 s.
- New `recipeInspectorDurationProblem`: the problem after Save.
- New `recipeReadViewStepDetails`: the read view with durations and
  instructions.
- Changed: every snapshot that shows a node or the inspector, because of
  the second line and the new fields. Each one is checked by eye.

**UI test** `testStepDetailsSurviveReopen` (`MacRecipeUITests`): create
Pasta with "Boil water"; click the 10 min chip, choose Waiting, and type
"Salt it well."; save. The read view shows "10 min" and the instructions.
Click Edit and select the step: the inspector shows the same three values.

**Demo** `testCaptureREC17Demo`: `01-inspector.png` (chip, Waiting,
instructions), `02-more.png` (the unit fields with 1 min 30 s),
`03-read-view.png`.

### docs

The Validation section of this plan, after the last change.

## Steps

### Change 1 — REC-17: plan (this change)

1. Write this plan and the design page. Set `plan: "tasks/REC-17.md"` on the
   epic card, and the bead's spec.
2. Send the questions and the design page to the REC-17 thread. Record the
   answers.

### Change 2 — REC-17: steps store duration, kind and instructions (about 450 lines, review)

1. Add `StepDurationS`, `StepKind`, `StepInstructions`, and `duration.rs`
   with its tests.
2. Add the three fields to `Step` and `StepWrite`. Update every place that
   builds one (list below).
3. Add the migration, the schema, the rows, and the store tests.
4. The editor keeps the loaded values and writes them back unchanged.
5. Run checks 1 to 3 and check 7. Send the guided review.

The files that build a `Step` or a `StepWrite` (`rg "StepWrite \{|Step \{"`):
`kreatyna/src/recipes/{model,step_canvas,detail}.rs`,
`kreatyna/tests/{recipe_editor,recipe_detail,recipe_section}.rs`,
`kreatyna-db/src/recipe_models.rs`, `kreatyna-db/src/recipe_store/tests.rs`,
`kreatyna-ffi/src/app_types/{recipes,tests}.rs`.

### Change 3 — REC-17: the editor edits duration, kind and instructions (about 550 lines, review)

1. Add the three events, `DraftDuration`, the node view fields, and
   `StepProblems`.
2. Check the duration in `writes()`.
3. Write the editor tests.
4. Add the FFI types, events, and tests. Keep Swift building: show
   `problems.title` where `problem` was.
5. Run `mac-bindings debug`, checks 1 to 3, and check 7. Send the guided
   review.

### Change 4 — REC-17: the inspector and the nodes on the Mac (about 500 lines, report)

1. Add `MacDurationField` and its unit tests.
2. Add the Duration, Active/Waiting, and Instructions fields to the
   inspector.
3. Add the node's second line and the dashed border.
4. Add the error message.
5. Record the new and changed snapshots, and check each one by eye.
6. Run checks 4, 5 and 7. Send a short report with the snapshots.

### Change 5 — REC-17: the read view shows step details (about 250 lines, review)

1. Add the `RecipeDetailStep` fields, the FFI fields, and the tests.
2. Show them in `MacRecipeDetail`. Record `recipeReadViewStepDetails`.
3. Add the UI test and the demo method.
4. Run checks 1 to 7. Capture the demo (check 6).
5. Fill in Validation in this plan.
6. Send the guided review, the demo, and the report.

## Checks

Test scope: Rust in full for each change. On the Mac, all unit and snapshot
tests, and the UI tests of the Recipes section only (`--ui MacRecipeUITests`).

| # | Command | Expected result |
|---|---|---|
| 1 | `cargo test --workspace` | All pass. |
| 2 | `cargo clippy --workspace --all-targets -- -D warnings` | No warnings. |
| 3 | `cargo fmt --check` | No diff. |
| 4 | `devenv shell -- mac-test --ui MacRecipeUITests` | All pass. Only the image changes listed above. |
| 5 | `devenv shell -- ios-app-build` | Builds. |
| 6 | `capture-demo.sh mac …/testCaptureREC17Demo docs/epics/recipes/tasks/REC-17-demo` | `01-inspector.png`, `02-more.png`, `03-read-view.png`. |
| 7 | `jj diff -r @ --stat '~.beads/issues.jsonl'` | Under 1,000 insertions plus deletions, for each change. |

## Stubs and exclusions

| Stub | Replaced by |
|---|---|
| A step without a duration saves. | REC-22 |
| The inspector's After field is a disabled placeholder. | REC-18 |
| The list row shows no total time. | REC-21 |
| The read view lists steps in the order added, not in stages. | REC-19 |

- No duration ranges (F-06) and no personal timings (F-07).
- No typed text such as "1 h 30 min" (variant B was not chosen).
- No weeks, months, or years as units: a sourdough starter is "365 d".
- No tokens or timers in instructions.
- No recipe screens on iOS (E-04).

## Validation

| # | Change | Result |
|---|---|---|
| 1 | 2 to 5 | `cargo test --workspace`: 817 tests pass after change 5. |
| 2 | 2 to 5 | `cargo clippy --workspace --all-targets -- -D warnings`: no warnings. |
| 3 | 2 to 5 | `cargo fmt --check`: no diff. |
| 4 | 3 | `mac-test --ui none`: passes. No image changes. |
| 4 | 4, 5 | `mac-test --ui MacRecipeUITests`: unit and snapshot tests pass, and 13 UI tests pass, `testStepDetailsSurviveReopen` among them. Image changes: the five new snapshots, and `editRecipeEditor`, `recipeCanvas`, `untitledStep` (taller nodes, real fields). Each one was checked by eye. |
| 5 | 4, 5 | `ios-app-build`: builds. |
| 6 | 5 | `capture-demo.sh`: `01-inspector.png`, `02-more.png`, `03-read-view.png` in `REC-17-demo/`. |
| 7 | 2 to 5 | Change 2: 534 insertions and 22 deletions. Change 3: 788 and 86. Change 4: 445 and 24. Change 5: under 300, with this section. |

Findings during the work:

- Review of change 2: `StepDuration` became `StepDurationS`, so the name
  says seconds. `DbDuration` became `DbDurationS`.
- Change 3 was 874 lines, not about 550. The editor tests grew: every new
  event has its unknown-id, same-value, saving, and Discard cases.
- The same duration sent another way (20 min as 1,200 s) changes nothing,
  so the editor does not render. A test that expected a render was wrong.
- `logging … threshold::error` failed once in change 3 and passed in three
  reruns. It is filed as `vibetyna-ns6a`.
- The snapshot window is never active, so a `.borderedProminent` chip
  looked like the others. The chips have their own style.
- The four unit fields did not fit in one row of the 260-point inspector.
  They sit two per row.
- The node's accessibility label stays the title; the duration and kind
  are its value. A label with both broke the UI tests that find nodes by
  title.
- macOS does not report the selected segment of the Active/Waiting switch
  as selected. The UI test reads the kind from the node's value instead.
- The More… button has the link style, and the UI test finds it by
  identifier on any element type.

All checks ran. No check was skipped.
