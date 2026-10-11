# Recipes — discovery brief

Status: **complete, awaiting the user's acceptance** (rounds 1–4 and mockup reviews 1–4 done; no open questions). This
is not an epic. Discovery ends with this brief and the final mockups. Splitting the
work into epics happens only when the user asks for it, using
[plan-reviewable-epics](../../../.agents/skills/plan-reviewable-epics/SKILL.md).

Mockups: [mockups.html](mockups.html). Open it in a browser; it works offline.

## Vision

A cooking helper that is useful *at the stove*. A recipe is not a wall of text
but a graph of steps: some steps wait for others, and some can run alongside
each other ("preheat the oven" while "cut the vegetables"). The app knows which
ingredients each step uses and how long it takes. It can therefore say what you
can do right now, and later when to start so dinner is ready on time. Recipe
ingredients may point at the generic ingredient catalog that already exists, but
they don't have to. A recipe that asks for "Mutti tomatoes" stays exactly that
until products exist.

The first home is the **Mac**, with the laptop in the kitchen. The iPhone and
syncing between devices come later.

## Decision log

### Product direction

| # | Topic | Decision | Why / note |
|---|---|---|---|
| D-01 | Moments of use | 1. Cooking at the stove · 2. Choosing what to cook · 3. Shopping/prep | Cooking is the reason the step graph exists. |
| D-02 | Platform | **macOS first**. iPhone later. Sync later, over local-network discovery. | The Mac is where recipes are written *and* where the user cooks for now. |
| D-03 | Mac app scope | Whole app in a Mac-native layout: ingredient catalog + recipes. | One app, not a recipe-only side tool. |
| D-04 | Order | A small **Mac catalog** epic first (a new Mac shell the user creates in Xcode), then recipes. | Recipes build on a Mac shell that already shows the catalog. |
| D-05 | Getting recipes in (long term) | Paste text and import from a URL, structured by a **pluggable** backend. | Rust defines a "structure this recipe" capability; the backend (on-device, Ollama, cloud) is chosen later. |
| D-06 | MVP line | **Hand-made DAG recipe**: no import, no tokens, no cooking mode. | It's the first version the user would actually use; everything after it is roadmap. |
| D-07 | What the DAG pays for (roadmap order) | "What can I do now" → timing / critical path → combining recipes into one meal. | All wanted; delivered gradually. |
| D-28 | Mac catalog epic | **Full parity** with iOS: list, detail, create, edit, delete. | The catalog is a working part of the Mac app, not a viewer. |
| D-41 | Timeline view | The one-cook timeline from the mockups (one bar per step on a minute axis, hands-on vs. waiting) should exist in the real app. | The user liked it. Not in the MVP: parked as F-13. |
| D-38 | After the MVP | **Deliberately undecided.** The user will use the MVP, then pick the biggest pain point or missing feature as the next step. | Real use decides, not this brief. |
| D-17 | Counter | **Sunset it**: remove Counter from the app; no Mac version. Optionally move it to the `krucyfiks` repository as an example. | Recipes take its place next to the catalog; it only served as a demo. |

### Recipe and ingredients

| # | Topic | Decision | Why / note |
|---|---|---|---|
| D-10 | Ingredients ↔ steps | A recipe ingredient list. Steps link to the lines they use. | Chosen over inline-only text and over unlinked lists. |
| D-11 | Ingredients ↔ catalog | Each line has its own name ("Mutti tomatoes") and **may** link to a catalog ingredient (tomato). The link is optional. | No products/EAN yet; the link adds meaning without forcing it. |
| D-12 | Amounts | Number + unit + free note ("a pinch", "to taste"). | Enables scaling, conversion and shopping later. |
| D-14 | Recipe-level fields (MVP) | **Title only**, besides ingredients and steps. | Servings, source, notes, tags and photo are parked as F-01. |
| D-36 | Deleting a linked catalog ingredient | Allowed. Linked recipe lines **keep their text** and lose the link. The delete confirmation says how many recipes use it. | The recipe stays as written; the catalog never holds it hostage. |
| D-37 | What a saved recipe needs | A title, **at least one ingredient**, **at least one step**, and **a duration on every step**. | Guarantees the total time is always complete. |
| D-40 | Half-written recipes | **No drafts** in the MVP: a recipe is saved only when complete. | Keeps the MVP small; drafts are parked as F-12. |
| D-39 | Recipe titles | **Unique**, using the same case-insensitive name rule as catalog ingredients. | "Shakshuka (spicy)" instead of two "Shakshuka"s. |
| D-23 | Catalog suggestions | While you type a line's name, the app suggests catalog matches. | Linking should cost one click. |
| D-24 | Unlinked lines | **Gentle nudge** (mockup variant B): a suggestion chip when a match looks likely, otherwise a quiet "no link". | Linking is encouraged, never required. |
| D-25 | Units | A fixed list of known units (g, kg, ml, l, tsp, tbsp, cup, piece…), **plus** the linked catalog ingredient's reference units, **plus** any custom unit such as "clove", even on an unlinked line. | Recipes use whatever words they use; only known units convert. |

### Steps and canvas

| # | Topic | Decision | Why / note |
|---|---|---|---|
| D-08 | Editing dependencies | **Visual canvas from day one.** | The user accepted the larger UI investment; the first version may be rough. |
| D-09 | Step contents (MVP) | Short title, optional longer instructions, duration marked **active** (hands busy) or **waiting** (hands free). | The title is what a canvas node shows; active vs. waiting feeds timing later. |
| D-19 | Node positions | **Stored**: you arrange the nodes and the app keeps them. No auto-arrange in the MVP. | Auto-arrange is parked as F-02. |
| D-20 | Adding and connecting | **Right-click on empty canvas** → context menu → "Add step". Connect steps by dragging from a node's output port to another node's input port, as in node editors such as ComfyUI. | Familiar node-editor gestures. |
| D-21 | Duration (MVP) | A **single value**. | Ranges and personal timings are parked as F-06 and F-07. |
| D-30 | Mixed active/waiting work | **Two steps.** "Put the sauce in the oven" (active) and "Bake 20 min" (waiting) are separate nodes. Each step is either active or waiting. | Simpler model; the graph says exactly when your hands are free. |
| D-32 | Make-ahead prep | A step can be marked **"can be done ahead"** (prep such as dicing onions or mincing garlic). Other steps should happen when they're due. | Lets timing move prep earlier instead of treating every step as just-in-time. Not in the MVP editor (D-34). |
| D-22 | Graph ends | Several loose ends are allowed; there is no mandatory "Serve" node. | Keeps recipes as written. |

### Screens

| # | Topic | Decision | Why / note |
|---|---|---|---|
| D-15 | Recipe list row | Title, total time and step count. | Total time is defined in D-29. |
| D-33 | Total time includes prep | The list row shows the full on-the-day time, prep included (38 min, not 32). | It tells you whether you have time to cook this at all. Pantry-aware shortcuts are parked as F-10. |
| D-29 | Total time | The **one-cook estimate**: active steps never overlap (you have one pair of hands), waiting steps run in the background. 38 min in the example, versus 35 min for the longest chain and 59 min for the plain sum. | It answers "how long will this take *me*?". Consequence: the MVP already needs a small one-cook scheduler in Rust to show this number. "Two cooks" is parked as F-09. |
| D-16 | Read view | **Stages**: steps grouped by "what can start together", derived from the graph. The canvas appears only in the editor. | Reads like a normal recipe. |
| D-18 | Window model | **Edit** and **Cook** replace the detail pane; no separate windows. | Simple, one window. |

### Later rungs (decided now so the MVP doesn't block them)

| # | Topic | Decision | Why / note |
|---|---|---|---|
| D-13 | Quantities inside step text | Inline tokens such as `{temp:…}` and `{dur:…}`, shown in the user's unit system. Durations become tappable timers. | Stored canonically (metric) and converted for display. |
| D-26 | Unit preference | **Per dimension**: temperature, weight and volume are chosen separately. | Mixed habits (°C oven, cups for flour). |
| D-27 | Cooking mode (first version) | One recipe at a time; **several timers** at once; <kbd>Space</kbd> marks the focused step done; progress survives quitting the app. | |
| D-34 | When make-ahead arrives | With the **timing** rung, not in the MVP editor. | The MVP editor is already complex enough. |
| D-35 | Make-ahead shape | A **yes/no** flag when it arrives. | "How far ahead" is parked as F-11. |
| D-42 | Two kinds of timers | The timer bar shows **running** timers (a step in progress) and **start** timers (a countdown to when a waiting step should begin, e.g. "Start Cook pasta in 17:00"), styled differently. | The start hint becomes something you can glance at, not just a line on a card. |
| D-43 | Text size | Standard macOS zoom: <kbd>⌘+</kbd> / <kbd>⌘−</kbd> and View-menu items. No text-size button in the window. | Native behaviour, less UI. |
| D-31 | Cooking mode needs timing | Cooking mode ships **with timing**, never as a plain "available now" list. A step that would finish too early (like cooking pasta) waits in "Coming up" with a hint: "start in ~17 min". | Starting steps the moment they're unblocked ruins dishes. |

## Roadmap

Each step should be useful on its own and buildable in thin vertical slices
(database → Rust → Swift).

**Planned, in order:**

1. **Mac catalog shell**: the existing ingredient catalog in a Mac window, with
   full create/edit/delete (D-28); Counter removed (D-17).
2. **Hand-made DAG recipe (MVP)**: recipe list with one-cook total time, read
   view as stages, ingredient lines with optional catalog link, steps on an
   editable canvas, step ↔ ingredient links, durations.

**Candidates after the MVP, unordered (D-38).** The next one is picked after
using the MVP:

- **Timing**: when each step should start, the yes/no make-ahead flag (D-34,
  D-35), "start at 18:20 to eat at 19:00".
- **Cooking mode**: "now / coming up / done", ticking steps off, timers, start
  hints. Needs timing first (D-31).
- **Inline tokens + unit setting**: temperatures and durations inside step text,
  per-dimension display (D-13, D-26).
- **Paste / URL import**: pluggable structuring backend; the user corrects the
  proposal in the editor (D-05).
- **Combining recipes**: interleave main + side into one cooking plan.
- **iPhone shell** for recipes, then **LAN sync** between Mac and iPhone (D-02).
- Any parked feature below.

## Open questions

None. All questions Q-01 … Q-26 are answered in the decision log or parked below.

## Parked for later

Ideas the user wants, but not in the MVP. Each can be picked up in any later rung.

| # | Feature | From |
|---|---|---|
| F-01 | Recipe-level fields: servings, source, notes, tags, photo | Q-01 |
| F-02 | Auto-arrange for the canvas | Q-10 |
| F-03 | A step uses only part of an ingredient line ("half the butter now, half later") | Q-07 |
| F-04 | Ingredient groups ("For the sauce", "For the dough") | Q-08 |
| F-05 | Create a catalog ingredient straight from a recipe line | Q-05 |
| F-06 | Approximate duration ranges ("20–25 min") | Q-12 |
| F-07 | Personal timings and cooking notes ("with *my* oven it's done in 18 min") | Q-12 |
| F-08 | Move the Counter example into the `krucyfiks` repository | Q-16 |
| F-09 | "Two cooks" mode: schedule for more than one pair of hands (cooking together) | Q-19 |
| F-10 | Pantry-aware total time: leftover diced onion in the fridge skips that step | Q-24 |
| F-11 | "How far ahead" a prep step can be done (same day, overnight, minutes before) | Q-23 |
| F-12 | Drafts: keep a half-written recipe out of the list until it's complete | Q-25 |
| F-13 | Timeline view of the one-cook schedule on the recipe (as in mockup section 4) | Q-26 |

## Non-goals for now

- Products / EAN barcodes, stores, prices.
- Shopping lists, pantry, meal planning.
- iPhone recipe screens and device sync.
- Picking the AI/structuring backend.
- Scaling servings and nutrition totals (made *possible* by D-12; not built yet).
