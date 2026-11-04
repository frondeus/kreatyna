---
name: write-slice-plan
description: Write the implementation plan for one epic slice at docs/epics/<epic>/tasks/<ID>.md, in technical English (ASD-STE100, explained for an 18-year-old), with self-contained open questions first, concrete Rust examples, and links to mockups. Use when a slice from an accepted epic starts; not for discovery or epic planning.
---

# Write a slice plan

One slice from [plan-reviewable-epics](../plan-reviewable-epics/SKILL.md)
gets one plan file. Write it when the slice starts, in the slice's own jj
change (see [use-of-jj-vcs](../use-of-jj-vcs/SKILL.md)). Never write plans for
later slices.

## Ground the plan

1. Read the slice bead (`bd show <slice-bead>`) and its card in
   `docs/epics/<epic>/epic.html`: outcome, stubs, acceptance, refs. Read the epic decisions (`E-xx`) and brief decisions
   (`D-xx`) it cites.
2. Read the code the slice touches. Search for every reference you must
   change (`rg`), and record the result. A plan that says "update all paths"
   without the list is incomplete.
3. Check the slice dependencies in `bd show <slice-bead>`. Each one must be
   closed.

## Structure (fixed order)

1. `# <ID> — <title>`, then a tracking line: `Tracked in beads: `<slice-bead>`.`,
   the jj change IDs (or "listed in Steps" for a series), and the dependencies. Status lives only in beads.
2. **Open questions.** Always first, so the reader sees them first. The
   user answers them in the Inbox before the plan is sent (see Review), so
   in a sent plan this section is usually "None.".
   Give each one an ID `<ID>/Qn` (for example `MAC-09/Q1`) and these parts:
   - *Today:* what exists now, with file paths, current values, and a short
     snippet.
   - *Why it is a question:* the tension or the missing decision.
   - *Options:* A, B (C). For each: what changes, what it costs, what it
     blocks later.
   - *Recommendation* and why.
   - *Effect on this plan:* which steps change for each answer.

   A question about how something looks or works on screen gets a mockup
   for each option, in the question itself (a picture choice in the Inbox
   block, or a section of the design page). Do not ask it in words only. The
   user asked for mockups in MAC-05/Q2 and REC-17/Q1 (2026-10-13,
   2026-11-04).

   Each question must be complete alone. Never write "as mentioned below" or
   "see section X"; repeat the fact. If there are no questions, write
   "None." and one sentence why.
3. **Decisions.** Answered questions, with their original IDs. Omit the
   section until the first answer.
4. **Outcome.** What the user sees after the slice, with a concrete example
   ("Clicking Garlic shows clove ≈ 5 g").
5. **Terms.** Define each term the reader may not know (Apple tooling, jj,
   UniFFI, project concepts). One or two sentences each.
6. **Design.** Link existing mockups (`docs/tasks/<topic>/mockups.html`,
   demo screenshots) that show the target state. If none shows it, create
   `docs/epics/<epic>/tasks/<ID>-design.html` (self-contained, light and dark,
   same rules as discovery mockups in
   [run-product-discovery](../run-product-discovery/SKILL.md)). If the slice
   has no UI change, say so. Open each design page and look at its
   screenshot, light and dark, before you send it.
7. **Changes by layer.** db, rust, ffi, swift, tooling, docs. For each:
   files, types, contracts, state transitions. Show examples (see below).
8. **Steps.** Numbered. One action per step. Estimate the diff size of the
   slice (compare a similar earlier slice with `jj diff -r <change> --stat`).
   If it is 1,000 lines or more, group the steps under one heading per jj
   change, each with its estimated size and whether it gets a review. Each
   change must build and pass every check alone (see "Size limit" in
   [use-of-jj-vcs](../use-of-jj-vcs/SKILL.md)).
9. **Checks.** Exact commands and the expected result of each.
10. **Stubs and exclusions.** Every stub, and the slice that replaces it.
    Everything this slice does not do.
11. **Validation.** "Not run yet." until implementation. Then: the checks
    that ran, their results, and the checks that could not run.

## Examples

- Prefer Rust. Show the real types and signatures the slice adds or changes:
  enums, events, view structs, store traits, and a sketch of one widget test.
  Do not write the full implementation.
- Check every type you show against
  [designing-types](../designing-types/SKILL.md): no `Option` or `bool` that
  stands for a mode, and no fields that can disagree.

  ```rust
  // sketch
  pub enum CatalogEvent {
      SelectIngredient { id: IngredientId },
      // ...
  }
  ```
- If the slice has no Rust, say so in one sentence. Then use the language
  the slice changes: shell, YAML, Swift, or a `jj diff --summary` listing.
- Use the project's example data: Garlic, Tomato, "clove ≈ 5 g".
- Mark each example as *real* (copied from the code) or *sketch* (proposed).

## Technical English (ASD-STE100, explained for an 18-year-old)

- Write for an 18-year-old who can program but does not know this codebase,
  Apple tooling, or jj.
- Procedures: at most 20 words per sentence. Descriptions: at most 25.
- One instruction per sentence. Use the imperative for steps ("Run",
  "Rename", "Delete").
- Use the active voice. Name who does the action (Rust, Swift, the script,
  you).
- Use one word for one meaning. Do not switch between synonyms ("folder" and
  "directory").
- Do not use idioms, slang, or "simply"/"just".
- Use lists and tables for more than two parallel items.
- Write numbers and paths exactly. Put code, paths, and commands in backticks.

## Review

Talk to the user through the Plannotator Inbox, in a thread named after the
slice (see [use-plannotator-inbox](../use-plannotator-inbox/SKILL.md)).

1. Ground the plan, then send the open questions as Inbox question blocks.
   Each block repeats the question's context, options, and recommendation.
   Stop until the user answers.
2. Record each answer: the question moves into **Decisions** (keep its ID),
   and the steps it affects change. An answer that changes the whole epic
   also becomes an `E-xx` entry on the epic page. An answer that takes your
   recommendation, or a **Looks good** label, accepts it. A partial answer
   gets a new, narrower question in the next round.
3. Write the full plan. Set the slice's `plan` to `tasks/<ID>.md` in the epic
   data object, and its bead's spec: `bd update <slice-bead> --spec-id
   docs/epics/<epic>/tasks/<ID>.md`.
4. Send an Inbox message with the plan attached. Stop until the user reviews
   it.
5. Process each annotation on the plan as in step 2. A new question goes
   back to step 1 for that question only.
6. Implement only after the user accepts the plan: close the "the user
   accepts the plan" gate, and the slice's implement step becomes ready. Then continue with step 2
   of "Deliver one slice at a time" in the epic skill.
