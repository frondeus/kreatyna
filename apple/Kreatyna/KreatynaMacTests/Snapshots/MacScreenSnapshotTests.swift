import AppKit
import SnapshotTesting
import SwiftUI
import Testing
@testable import Kreatyna

// Rendering conditions are fixed: a 960 × 600 point window, US English, and
// light and dark appearance. The window is never key, so controls show their
// inactive look, and its title text is transparent (MAC-12). Tests run serially (`MainActorTests`). Baselines
// were recorded on macOS 27.0.1; other versions can render slightly
// differently. Review every diff before re-recording.
extension MainActorTests {
    @MainActor
    @Suite(.snapshots(record: .missing))
    struct MacScreenSnapshotTests {

        @Test(arguments: Appearance.allCases)
        func catalogWithRows(appearance: Appearance) {
            snapshot(running(Fixtures.list()), appearance: appearance)
        }

        @Test(arguments: Appearance.allCases)
        func emptyCatalog(appearance: Appearance) {
            snapshot(running(Fixtures.list(.ready(rows: [], totalCount: 0, refresh: .idle))), appearance: appearance)
        }

        @Test(arguments: Appearance.allCases)
        func filteredCatalog(appearance: Appearance) {
            let rows = [IngredientListRow(id: Fixtures.tomatoesId, name: "Canned tomatoes")]
            snapshot(
                running(Fixtures.list(.ready(rows: rows, totalCount: 3, refresh: .idle), search: "tom")),
                appearance: appearance
            )
        }

        @Test(arguments: Appearance.allCases)
        func noMatches(appearance: Appearance) {
            snapshot(
                running(Fixtures.list(.ready(rows: [], totalCount: 3, refresh: .idle), search: "tomx")),
                appearance: appearance
            )
        }

        @Test(arguments: Appearance.allCases)
        func detailWithFullData(appearance: Appearance) {
            snapshot(running(Fixtures.list(), selected: Fixtures.garlic), appearance: appearance)
        }

        @Test(arguments: Appearance.allCases)
        func detailWithNameOnly(appearance: Appearance) {
            let rows = [IngredientListRow(id: Fixtures.basil.id, name: "Basil")] + Fixtures.rows
            snapshot(
                running(Fixtures.list(.ready(rows: rows, totalCount: 4, refresh: .idle)), selected: Fixtures.basil),
                appearance: appearance
            )
        }

        /// Rust is deleting Garlic: the overlay shows, and Edit and Delete are off (MAC-07).
        @Test(arguments: Appearance.allCases)
        func deletingIngredient(appearance: Appearance) {
            snapshot(
                running(Fixtures.list(), selected: Fixtures.garlic, deletion: .deleting),
                appearance: appearance
            )
        }

        @Test(arguments: Appearance.allCases)
        func newIngredientForm(appearance: Appearance) {
            var draft = Fixtures.blankDraft
            draft.units = [ReferenceUnitDraft(name: "", grams: "", millilitres: "")]
            snapshot(creating(Fixtures.form(draft)), appearance: appearance)
        }

        @Test(arguments: Appearance.allCases)
        func formWithIssues(appearance: Appearance) {
            var draft = Fixtures.blankDraft
            draft.calories = "abc"
            draft.units = [ReferenceUnitDraft(name: "clove", grams: "", millilitres: "")]
            let issues = [
                ValidationIssue(field: .name, reason: .required),
                ValidationIssue(field: .calories, reason: .invalidNumber),
                ValidationIssue(field: .unitMeasurements(index: 0), reason: .required),
            ]
            snapshot(creating(Fixtures.form(draft, issues: issues)), appearance: appearance)
        }

        @Test(arguments: Appearance.allCases)
        func duplicateName(appearance: Appearance) {
            var draft = Fixtures.blankDraft
            draft.name = "garlic"
            snapshot(
                creating(Fixtures.form(draft, saveError: .duplicateName)),
                appearance: appearance
            )
        }

        /// The title and the highlighted row keep the saved name (MAC-06/Q1).
        @Test(arguments: Appearance.allCases)
        func editIngredientForm(appearance: Appearance) {
            var draft = Fixtures.garlicDraft
            draft.name = "Black Garlic"
            snapshot(editing(Fixtures.form(draft)), appearance: appearance)
        }

        /// The Mac app opens on Recipes; a new database has none (REC-01).
        @Test(arguments: Appearance.allCases)
        func emptyRecipeList(appearance: Appearance) {
            snapshot(
                MacMainView(app: Fixtures.splitApp(section: .recipes), columnVisibility: .doubleColumn, send: { _ in }),
                appearance: appearance
            )
        }

        /// "sha" in the search field: only Shakshuka is left (REC-06).
        @Test(arguments: Appearance.allCases)
        func searchedRecipeList(appearance: Appearance) {
            snapshot(
                recipeList(Fixtures.recipeSection(.ready(rows: [Fixtures.recipeRow("Shakshuka")]), search: "sha")),
                appearance: appearance
            )
        }

        /// No recipe title contains "shx" (REC-06).
        @Test(arguments: Appearance.allCases)
        func recipeListNoMatches(appearance: Appearance) {
            snapshot(
                recipeList(Fixtures.recipeSection(.noMatches, search: "shx")),
                appearance: appearance
            )
        }

        /// New Recipe: the draft row first, the Title field, an empty canvas,
        /// "Select a step" in the inspector (REC-02, REC-14/Q1).
        @Test(arguments: Appearance.allCases)
        func emptyRecipeEditor(appearance: Appearance) {
            snapshot(
                recipes(.ready(rows: [Fixtures.recipeRow("Pancakes")]), editor: ("", nil)),
                appearance: appearance
            )
        }

        /// A refused title: the draft row shows the typed title, the problem shows under the field (REC-02).
        @Test(arguments: Appearance.allCases)
        func duplicateRecipeTitle(appearance: Appearance) {
            snapshot(
                recipes(
                    .ready(rows: [Fixtures.recipeRow("Shakshuka")]),
                    editor: ("shakshuka", .saveFailed(error: .duplicateTitle))
                ),
                appearance: appearance
            )
        }

        /// Three steps on the canvas: Fry onion is selected, with the accent
        /// border and its Title in the inspector (REC-14).
        @Test(arguments: Appearance.allCases)
        func recipeCanvas(appearance: Appearance) {
            let canvas = StepNodesView.selected(
                selected: Fixtures.stepNode("Fry onion", x: 200, y: 110),
                others: [
                    Fixtures.stepNode("Dice onion", x: 20, y: 20),
                    Fixtures.stepNode("Crack eggs", x: 40, y: 210),
                ]
            )
            snapshot(
                recipes(.ready(rows: []), editor: ("Shakshuka", nil), canvas: canvas),
                appearance: appearance
            )
        }

        /// Save refused an untitled step: Rust selected it, its node has the red
        /// border, and the inspector shows the problem (REC-14/Q2).
        @Test(arguments: Appearance.allCases)
        func untitledStep(appearance: Appearance) {
            let canvas = StepNodesView.selected(
                selected: Fixtures.stepNode(
                    "", x: 200, y: 110, label: .untitled, problem: .notEmptyViolated, key: "new"
                ),
                others: [Fixtures.stepNode("Dice onion", x: 20, y: 20)]
            )
            snapshot(
                recipes(.ready(rows: []), editor: ("Shakshuka", nil), canvas: canvas),
                appearance: appearance
            )
        }

        /// An active and a waiting node, each with a duration, and one with
        /// "No duration". The waiting node has a dashed border (REC-17).
        @Test(arguments: Appearance.allCases)
        func recipeCanvasActiveAndWaiting(appearance: Appearance) {
            let canvas = StepNodesView.unselected(steps: [
                Fixtures.stepNode("Boil water", x: 20, y: 20, duration: Fixtures.duration("10 min", .minutes(minutes: 10), minutes: 10)),
                Fixtures.stepNode(
                    "Bake sauce", x: 200, y: 110, duration: Fixtures.duration("1 h 30 min", .minutes(minutes: 90), hours: 1, minutes: 30),
                    kind: .waiting
                ),
                Fixtures.stepNode("Drain", x: 40, y: 220),
            ])
            snapshot(
                recipes(.ready(rows: []), editor: ("Pasta", nil), canvas: canvas),
                appearance: appearance
            )
        }

        /// The inspector of a waiting step: the 10 min chip filled, Waiting,
        /// and instructions (REC-17).
        @Test(arguments: Appearance.allCases)
        func recipeInspectorFilled(appearance: Appearance) {
            let canvas = StepNodesView.selected(
                selected: Fixtures.stepNode(
                    "Boil water", x: 20, y: 20, duration: Fixtures.duration("10 min", .minutes(minutes: 10), minutes: 10),
                    kind: .waiting, instructions: "Salt it well."
                ),
                others: [Fixtures.stepNode("Cook pasta", x: 200, y: 110)]
            )
            snapshot(
                recipes(.ready(rows: []), editor: ("Pasta", nil), canvas: canvas),
                appearance: appearance
            )
        }

        /// A duration with seconds opens More… by itself: the unit fields show
        /// 1 min 30 s and the minutes field is empty (REC-17).
        @Test(arguments: Appearance.allCases)
        func recipeInspectorMore(appearance: Appearance) {
            let canvas = StepNodesView.selected(
                selected: Fixtures.stepNode(
                    "Sear scallops", x: 20, y: 20, duration: Fixtures.duration("1 min 30 s", .seconds, minutes: 1, seconds: 30)
                ),
                others: []
            )
            snapshot(
                recipes(.ready(rows: []), editor: ("Scallops", nil), canvas: canvas),
                appearance: appearance
            )
        }

        /// A diamond of arrows: Heat pan comes before Fry onion and Toast
        /// cumin, and both before Add tomatoes. Add tomatoes is selected, with
        /// two After chips (REC-18). The window is wider for room.
        @Test(arguments: Appearance.allCases)
        func recipeCanvasConnected(appearance: Appearance) {
            let nodes = Self.diamond(selected: "Add tomatoes")
            snapshot(
                recipes(.ready(rows: []), editor: ("Shakshuka", nil), canvas: nodes, connections: Self.diamondEdges ),
                appearance: appearance, width: Self.wideWindow
            )
        }

        /// A drag from Toast cumin's ● draws a dashed line to the pointer
        /// (REC-18). Only the canvas: the drag lives in it.
        @Test(arguments: Appearance.allCases)
        func recipeConnectionDrag(appearance: Appearance) {
            let canvas = Fixtures.canvas(
                .unselected(steps: [
                    Fixtures.stepNode("Heat pan", x: 20, y: 130),
                    Fixtures.stepNode("Toast cumin", x: 270, y: 220),
                    Fixtures.stepNode("Fry onion", x: 270, y: 40),
                ]),
                connections: [Fixtures.edge("Heat pan", "Toast cumin")]
            )
            snapshot(
                MacStepCanvas(
                    canvas: canvas,
                    connecting: ConnectionDrag(from: StepId(uuid: "step-Toast cumin"), pointer: CGPoint(x: 600, y: 160)),
                    send: { _ in }
                ),
                appearance: appearance
            )
        }

        /// A selected arrow: accent and thicker, and the inspector shows its
        /// two steps (REC-18/Q1).
        @Test(arguments: Appearance.allCases)
        func recipeConnectionSelected(appearance: Appearance) {
            let connection = StepConnectionView(
                edge: Fixtures.edge("Heat pan", "Fry onion"),
                from: .titled(title: "Heat pan"), to: .titled(title: "Fry onion")
            )
            let nodes = StepNodesView.connectionSelected(connection: connection, steps: Self.diamondSteps)
            snapshot(
                recipes(.ready(rows: []), editor: ("Shakshuka", nil), canvas: nodes, connections: Self.diamondEdges ),
                appearance: appearance, width: Self.wideWindow
            )
        }

        /// A refused connection: the line under the canvas says why (REC-18/Q2).
        @Test(arguments: Appearance.allCases)
        func recipeConnectionRefused(appearance: Appearance) {
            snapshot(
                recipes(
                    .ready(rows: []), editor: ("Shakshuka", nil), canvas: .unselected(steps: Self.diamondSteps),
                    connections: Self.diamondEdges, refusal: .loop
                ),
                appearance: appearance, width: Self.wideWindow
            )
        }

        /// Room for three columns of steps on the canvas.
        private static let wideWindow: CGFloat = 1240

        private static let diamondEdges = [
            Fixtures.edge("Heat pan", "Fry onion"),
            Fixtures.edge("Heat pan", "Toast cumin"),
            Fixtures.edge("Fry onion", "Add tomatoes"),
            Fixtures.edge("Toast cumin", "Add tomatoes"),
        ]

        private static var diamondSteps: [StepNodeView] {
            [
                Fixtures.stepNode("Heat pan", x: 20, y: 130),
                Fixtures.stepNode("Fry onion", x: 270, y: 40, after: [Fixtures.after("Heat pan")]),
                Fixtures.stepNode("Toast cumin", x: 270, y: 220, after: [Fixtures.after("Heat pan")]),
                Fixtures.stepNode(
                    "Add tomatoes", x: 520, y: 130,
                    after: [Fixtures.after("Fry onion"), Fixtures.after("Toast cumin")]
                ),
            ]
        }

        /// The diamond with the step titled `selected` selected.
        private static func diamond(selected: String) -> StepNodesView {
            let steps = diamondSteps
            guard let chosen = steps.first(where: { $0.title == selected }) else { return .unselected(steps: steps) }
            return .selected(selected: chosen, others: steps.filter { $0.title != selected })
        }

        /// Save refused a duration past 365 days: the node has the red border
        /// and "No duration", and the inspector keeps what you set with the
        /// problem under it (REC-17).
        @Test(arguments: Appearance.allCases)
        func recipeInspectorDurationProblem(appearance: Appearance) {
            let canvas = StepNodesView.selected(
                selected: Fixtures.stepNode(
                    "Feed starter", x: 20, y: 20,
                    duration: .tooLong(parts: DurationParts(days: 400, hours: 0, minutes: 0, seconds: 0)),
                    kind: .waiting, durationProblem: .tooLong
                ),
                others: []
            )
            snapshot(
                recipes(.ready(rows: []), editor: ("Sourdough", nil), canvas: canvas),
                appearance: appearance
            )
        }

        /// Shakshuka's read view: its row selected, its three steps in the order
        /// added, "None yet" ingredients, Edit and Delete enabled (REC-03 to
        /// REC-05). The rows show "1 step" and "3 steps" (REC-14).
        @Test(arguments: Appearance.allCases)
        func recipeReadView(appearance: Appearance) {
            let shakshuka = Fixtures.recipeRow("Shakshuka", stepCount: 3)
            let steps = ["Fry onion", "Dice onion", "Crack eggs"].map { Fixtures.detailStep($0) }
            let detail = RecipeDetailView(id: shakshuka.id, status: .ready(title: shakshuka.title, ingredients: [], steps: steps, deletion: .idle))
            snapshot(
                MacMainView(
                    app: Fixtures.splitApp(
                        section: .recipes,
                        recipes: Fixtures.recipeSection(
                            .ready(rows: [Fixtures.recipeRow("Pasta", stepCount: 1), shakshuka]),
                            pane: .detail(detail: detail)
                        )
                    ),
                    columnVisibility: .doubleColumn,
                    send: { _ in }
                ),
                appearance: appearance
            )
        }

        /// Pasta's read view with step details: the duration and "waiting"
        /// after the title, and the instructions under it (REC-17/Q2).
        @Test(arguments: Appearance.allCases)
        func recipeReadViewStepDetails(appearance: Appearance) {
            let pasta = Fixtures.recipeRow("Pasta", stepCount: 4)
            let steps = [
                Fixtures.detailStep("Boil water", duration: "10 min", kind: .waiting, instructions: "Salt it well."),
                Fixtures.detailStep("Sear scallops", duration: "1 min 30 s"),
                Fixtures.detailStep("Rest the dough", kind: .waiting),
                Fixtures.detailStep("Plate"),
            ]
            let detail = RecipeDetailView(id: pasta.id, status: .ready(title: pasta.title, ingredients: [], steps: steps, deletion: .idle))
            snapshot(
                MacMainView(
                    app: Fixtures.splitApp(
                        section: .recipes,
                        recipes: Fixtures.recipeSection(.ready(rows: [pasta]), pane: .detail(detail: detail))
                    ),
                    columnVisibility: .doubleColumn,
                    send: { _ in }
                ),
                appearance: appearance
            )
        }

        /// Pasta's read view with four recipe ingredients in list order, above
        /// the steps (REC-07): Rust's amount in a column, the name, and the
        /// note in grey (REC-08). Salt has no amount.
        @Test(arguments: Appearance.allCases)
        func recipeReadViewIngredients(appearance: Appearance) {
            let pasta = Fixtures.recipeRow("Pasta", stepCount: 1)
            let ingredients = [
                Fixtures.detailIngredient("penne pasta", amount: "300 g"),
                Fixtures.detailIngredient("olive oil", amount: "1 tbsp", note: "extra virgin"),
                Fixtures.detailIngredient("salt", note: "to taste"),
                Fixtures.detailIngredient("milk", amount: "2 cups"),
            ]
            let detail = RecipeDetailView(
                id: pasta.id,
                status: .ready(title: pasta.title, ingredients: ingredients, steps: [Fixtures.detailStep("Boil water")], deletion: .idle)
            )
            snapshot(
                MacMainView(
                    app: Fixtures.splitApp(
                        section: .recipes,
                        recipes: Fixtures.recipeSection(.ready(rows: [pasta]), pane: .detail(detail: detail))
                    ),
                    columnVisibility: .doubleColumn,
                    send: { _ in }
                ),
                appearance: appearance
            )
        }

        /// The Pasta editor with four recipe ingredients under the canvas, each
        /// with its handle, Rust's summary and the − button (REC-07, REC-08):
        /// the amount in bold, the name, and ", note" in grey.
        @Test(arguments: Appearance.allCases)
        func recipeEditorIngredients(appearance: Appearance) {
            let ingredients = [
                Fixtures.ingredient("penne pasta", amount: "300", unit: .gram, summary: "300 g"),
                Fixtures.ingredient("olive oil", amount: "1", unit: .tablespoon, summary: "1 tbsp", note: "extra virgin"),
                Fixtures.ingredient("salt", note: "to taste"),
                Fixtures.ingredient("eggs", amount: "2", summary: "2"),
            ]
            snapshot(
                recipes(
                    .ready(rows: []), editor: ("Pasta", nil),
                    canvas: .unselected(steps: [Fixtures.stepNode("Boil water", x: 20, y: 20)]), ingredients: ingredients
                ),
                appearance: appearance
            )
        }

        /// After Save, each row with a problem shows it in red under its
        /// summary: "Name required" (REC-07), and each amount problem
        /// (REC-08). The row without a name shows "No name".
        @Test(arguments: Appearance.allCases)
        func recipeEditorIngredientProblem(appearance: Appearance) {
            let ingredients = [
                Fixtures.ingredient(
                    "flour", unit: .gram, summary: "g",
                    problems: RecipeIngredientProblems(name: nil, amount: .missingForUnit)
                ),
                Fixtures.ingredient(
                    "milk", amount: "0", unit: .millilitre, summary: "0 ml",
                    problems: RecipeIngredientProblems(name: nil, amount: .notPositive)
                ),
                Fixtures.ingredient(
                    "", amount: "2", summary: "2",
                    problems: RecipeIngredientProblems(name: .notEmptyViolated, amount: nil), key: "empty"
                ),
            ]
            snapshot(
                recipes(.ready(rows: []), editor: ("Pasta", nil), ingredients: ingredients),
                appearance: appearance
            )
        }

        /// Seven ingredients: the list shows 6 rows and scrolls, so the canvas keeps
        /// its height (REC-07).
        @Test(arguments: Appearance.allCases)
        func recipeEditorManyIngredients(appearance: Appearance) {
            let names = ["penne pasta", "olive oil", "basil", "garlic", "salt", "pepper", "parmesan"]
            snapshot(
                recipes(.ready(rows: []), editor: ("Pasta", nil), ingredients: names.map { Fixtures.ingredient($0) }),
                appearance: appearance
            )
        }

        /// The popover of olive oil: Amount, Unit, Name and Note as typed
        /// (REC-08/Q1). The Unit picker shows Rust's unit.
        @Test(arguments: Appearance.allCases)
        func recipeIngredientPopover(appearance: Appearance) {
            let oil = Fixtures.ingredient("olive oil", amount: "1", unit: .tablespoon, summary: "1 tbsp", note: "extra virgin")
            snapshot(popover(oil), appearance: appearance, width: 320, height: 240)
        }

        /// The popover after Save, with a unit and no amount: the problem
        /// shows under Amount (REC-08).
        @Test(arguments: Appearance.allCases)
        func recipeIngredientPopoverProblem(appearance: Appearance) {
            let flour = Fixtures.ingredient(
                "flour", unit: .gram, summary: "g",
                problems: RecipeIngredientProblems(name: nil, amount: .missingForUnit)
            )
            snapshot(popover(flour), appearance: appearance, width: 320, height: 240)
        }

        /// Rust is deleting Shakshuka: the overlay shows, Edit and Delete are off,
        /// and New Recipe is off (REC-05).
        @Test(arguments: Appearance.allCases)
        func deletingRecipe(appearance: Appearance) {
            let shakshuka = Fixtures.recipeRow("Shakshuka")
            let detail = RecipeDetailView(
                id: shakshuka.id, status: .ready(title: shakshuka.title, ingredients: [], steps: [], deletion: .deleting)
            )
            snapshot(
                MacMainView(
                    app: Fixtures.splitApp(
                        section: .recipes,
                        recipes: Fixtures.recipeSection(
                            .ready(rows: [Fixtures.recipeRow("Pasta"), shakshuka]),
                            pane: .detail(detail: detail)
                        )
                    ),
                    columnVisibility: .doubleColumn,
                    send: { _ in }
                ),
                appearance: appearance
            )
        }

        /// Shakshuka's edit editor with a changed title: the heading and the row
        /// keep the saved title until Save, and the row stays selected (REC-04).
        /// The canvas opens with the saved steps and no selection (REC-14).
        @Test(arguments: Appearance.allCases)
        func editRecipeEditor(appearance: Appearance) {
            let shakshuka = Fixtures.recipeRow("Shakshuka", stepCount: 2)
            let steps = [
                Fixtures.stepNode("Dice onion", x: 20, y: 20),
                Fixtures.stepNode("Fry onion", x: 200, y: 110),
            ]
            let editor = RecipeEditorView(
                title: "Spicy shakshuka", ingredients: [], units: Fixtures.units, canvas: Fixtures.canvas(.unselected(steps: steps)), status: .editing(problem: nil)
            )
            snapshot(
                MacMainView(
                    app: Fixtures.splitApp(
                        section: .recipes,
                        recipes: Fixtures.recipeSection(
                            .ready(rows: [Fixtures.recipeRow("Pasta"), shakshuka]),
                            editing: shakshuka,
                            editor: editor
                        )
                    ),
                    columnVisibility: .doubleColumn,
                    send: { _ in }
                ),
                appearance: appearance
            )
        }

        /// Shakshuka's read view after a failed load: the singular storage text and Retry.
        @Test(arguments: Appearance.allCases)
        func recipeReadViewFailed(appearance: Appearance) {
            let shakshuka = Fixtures.recipeRow("Shakshuka")
            let detail = RecipeDetailView(id: shakshuka.id, status: .failed(error: .storage))
            snapshot(
                MacMainView(
                    app: Fixtures.splitApp(
                        section: .recipes,
                        recipes: Fixtures.recipeSection(
                            .ready(rows: [Fixtures.recipeRow("Pasta"), shakshuka]),
                            pane: .detail(detail: detail)
                        )
                    ),
                    columnVisibility: .doubleColumn,
                    send: { _ in }
                ),
                appearance: appearance
            )
        }

        /// The sidebar alone, with Recipes selected (REC-01). Inside the split
        /// view, the off-screen capture draws the sidebar blank.
        @Test(arguments: Appearance.allCases)
        func sidebar(appearance: Appearance) {
            snapshot(MacSidebar(section: .recipes, select: { _ in }), appearance: appearance)
        }

        @Test(arguments: Appearance.allCases)
        func startupFailure(appearance: Appearance) {
            snapshot(
                MacRootView(
                    view: .initializationFailed(error: .databaseOpen),
                    errorMessage: nil, send: { _ in }, retryInitialization: {}
                ),
                appearance: appearance
            )
        }

        /// The database stopped while the app ran (P06).
        @Test(arguments: Appearance.allCases)
        func databaseStopped(appearance: Appearance) {
            snapshot(
                MacRootView(
                    view: .initializationFailed(error: .databaseStopped),
                    errorMessage: nil, send: { _ in }, retryInitialization: {}
                ),
                appearance: appearance
            )
        }

        /// A bug stopped the Rust core (vibetyna-dm7).
        @Test(arguments: Appearance.allCases)
        func crashed(appearance: Appearance) {
            snapshot(
                MacRootView(view: .crashed, errorMessage: nil, send: { _ in }, retryInitialization: {}),
                appearance: appearance
            )
        }

        // MARK: Support

        enum Appearance: String, CaseIterable {
            case light, dark

            var name: NSAppearance.Name { self == .light ? .aqua : .darkAqua }
        }

        /// The Ingredients section without the sidebar. The sidebar material
        /// draws only through the window server, so an off-screen capture shows
        /// it blank (white in dark mode). `sidebar` renders it alone.
        private func running(
            _ list: IngredientListView,
            selected: Ingredient? = nil,
            deletion: IngredientDetailDeletion = .idle
        ) -> some View {
            ingredients(Fixtures.splitCatalog(list: list, selected: selected, deletion: deletion))
        }

        /// The catalog with the create form open (MAC-05).
        private func creating(_ form: IngredientFormView) -> some View {
            ingredients(Fixtures.splitCatalog(form: form))
        }

        /// The catalog with Garlic's edit form open (MAC-06).
        private func editing(_ form: IngredientFormView) -> some View {
            ingredients(Fixtures.splitCatalog(editing: Fixtures.garlic, form: form))
        }

        /// The Recipes section with the create editor open.
        private func recipes(
            _ status: RecipeListStatus,
            editor: (title: String, problem: RecipeEditorProblem?),
            canvas: StepNodesView = .unselected(steps: []),
            connections: [StepEdge] = [],
            refusal: ConnectRefusal? = nil,
            ingredients: [RecipeIngredientView] = [],
        ) -> some View {
            let canvas = StepCanvasView(
                nodes: canvas, connections: connections, refusal: refusal, durationChips: Fixtures.durationChips
            )
            let view = RecipeEditorView(
                title: editor.title, ingredients: ingredients, units: Fixtures.units, canvas: canvas, status: .editing(problem: editor.problem)
            )
            let pane = RecipePaneView.create(
                editor: view, draftRow: editor.title.trimmingCharacters(in: .whitespaces)
            )
            return MacMainView(
                app: Fixtures.splitApp(section: .recipes, recipes: Fixtures.recipeSection(status, pane: pane)),
                columnVisibility: .doubleColumn,
                send: { _ in }
            )
        }

        /// One ingredient's popover content, as the window's popover shows it.
        private func popover(_ ingredient: RecipeIngredientView) -> some View {
            MacRecipeIngredientEditor(ingredient: ingredient, units: Fixtures.units, send: { _ in })
                .frame(maxWidth: .infinity, maxHeight: .infinity, alignment: .topLeading)
                .background(.background)
        }

        /// The Recipes section with an empty pane.
        private func recipeList(_ section: RecipeSectionView) -> some View {
            MacMainView(
                app: Fixtures.splitApp(section: .recipes, recipes: section),
                columnVisibility: .doubleColumn,
                send: { _ in }
            )
        }

        private func ingredients(_ catalog: IngredientSplitCatalogView) -> some View {
            MacMainView(
                app: Fixtures.splitApp(section: .ingredients, ingredients: catalog),
                columnVisibility: .doubleColumn,
                send: { _ in }
            )
        }

        /// A window that never becomes key or main, so every snapshot shows the
        /// inactive look. Whether a normal window is key depends on what else
        /// is active on the Mac, which changed the title color between runs.
        private final class InactiveWindow: NSWindow {
            override var canBecomeKey: Bool { false }
            override var canBecomeMain: Bool { false }
        }

        /// MAC-12: after a long UI test run on an idle Mac, macOS draws the
        /// toolbar title darker, and no window property shows why. The title's
        /// text becomes transparent but keeps its space, so the toolbar items
        /// stay where the app shows them.
        private func hideTitleText(in view: NSView, title: String) {
            // An empty title would match the empty search field.
            if !title.isEmpty, let field = view as? NSTextField, field.stringValue == title {
                field.alphaValue = 0
            }
            for subview in view.subviews {
                hideTitleText(in: subview, title: title)
            }
        }

        /// Stops every spinning progress indicator ("Deleting…"), so the
        /// snapshot does not depend on the frame the animation reached.
        private func stopProgressIndicators(in view: NSView) {
            (view as? NSProgressIndicator)?.stopAnimation(nil)
            for subview in view.subviews {
                stopProgressIndicators(in: subview)
            }
        }

        private func snapshot(
            _ view: some View,
            appearance: Appearance,
            width: CGFloat = 960,
            height: CGFloat = 600,
            fileID: StaticString = #fileID,
            filePath: StaticString = #filePath,
            testName: String = #function,
            line: UInt = #line,
            column: UInt = #column
        ) {
            // Split views and lists draw only inside a window, so the view is
            // hosted in an off-screen window. The window's frame view is
            // captured so the title bar and toolbar are included.
            let window = InactiveWindow(
                contentRect: CGRect(x: 0, y: 0, width: width, height: height),
                styleMask: [.titled, .closable, .miniaturizable, .resizable, .fullSizeContentView],
                backing: .buffered,
                defer: false
            )
            window.isReleasedWhenClosed = false
            window.appearance = NSAppearance(named: appearance.name)
            // The search field's text follows the app's appearance, not the
            // window's, so the app takes the snapshot's appearance too.
            let appAppearance = NSApp.appearance
            NSApp.appearance = window.appearance
            defer { NSApp.appearance = appAppearance }
            // The fixed frame keeps SwiftUI from resizing the window to its content.
            window.contentViewController = NSHostingController(
                rootView: view
                    .frame(width: width, height: height)
                    .environment(\.locale, Locale(identifier: "en_US"))
            )
            window.setContentSize(CGSize(width: width, height: height))
            window.setFrameOrigin(CGPoint(x: -10_000, y: -10_000))
            window.orderFront(nil)
            defer { window.close() }
            // Let SwiftUI lay out the columns and load the list rows.
            RunLoop.main.run(until: Date().addingTimeInterval(0.5))
            // An off-screen window gives its first key view (the search
            // field) focus; the launched app focuses nothing.
            window.makeFirstResponder(nil)
            RunLoop.main.run(until: Date().addingTimeInterval(0.1))
            let frameView = window.contentView!.superview!
            hideTitleText(in: frameView, title: window.title)
            stopProgressIndicators(in: frameView)
            assertSnapshot(
                of: frameView,
                // Every pixel is compared; 0.99 allows only anti-aliasing noise.
                // The old 0.995/0.98 let a whole toolbar search field pass (MAC-08).
                as: .image(precision: 1, perceptualPrecision: 0.99),
                named: appearance.rawValue,
                fileID: fileID,
                file: filePath,
                testName: testName,
                line: line,
                column: column
            )
        }
    }
}
