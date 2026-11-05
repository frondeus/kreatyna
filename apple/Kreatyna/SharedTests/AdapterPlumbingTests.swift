import Testing
@testable import Kreatyna

struct EventCompositionTests {

    @Test func parentsWrapChildEventsWithDisplayedScopes() throws {
        let app = Fixtures.running(screen: .list)
        let catalog = try #require(app.stack)

        let event = AppEvent.stack(event: catalog.list(.createRequested))

        #expect(event == .stack(
            event: .list(scope: ScopeId(value: 2), event: .createRequested)
        ))
        #expect(catalog.back == .back(scope: ScopeId(value: 2)))
    }

    @Test func splitEventsUseTheListAndPaneScopes() throws {
        let app = RunningAppView.split(app: SplitAppView(
            section: .ingredients,
            recipes: Fixtures.recipeSection(),
            ingredients: Fixtures.splitCatalog(selected: Fixtures.garlic)
        ))
        let split = try #require(app.split)
        let catalog = split.ingredients

        #expect(split.ingredients(catalog.list(.select(id: Fixtures.cucumberId))) == .split(
            event: .ingredients(event: .list(scope: ScopeId(value: 3), event: .select(id: Fixtures.cucumberId)))
        ))
        #expect(split.selectSection(.recipes) == .split(event: .selectSection(section: .recipes)))
        #expect(split.recipes(split.recipes.list(.refresh)) == .split(event: .recipes(event: .list(event: .refresh))))
        #expect(split.recipes(split.recipes.list(.search(query: "sha"))) == .split(
            event: .recipes(event: .list(event: .search(query: "sha")))
        ))
        #expect(split.recipes.newRecipe == .newRecipe)
        #expect(split.recipes.editor(.save) == .editor(scope: ScopeId(value: 5), event: .save))
        #expect(catalog.detail(.edit) == .detail(scope: ScopeId(value: 4), event: .edit))
        #expect(catalog.form(.save) == .form(scope: ScopeId(value: 4), event: .save))
        #expect(catalog.clearSelection == .clearSelection(scope: ScopeId(value: 4)))
        #expect(app.stack == nil)
    }

    /// The draft row is Rust's trimmed title, and only while the create editor is open.
    @Test func recipeDraftRowFollowsThePane() {
        #expect(Fixtures.recipeSection().draftRow == nil)
        let editor = RecipeEditorView(title: " Shakshuka ", lines: [], canvas: Fixtures.canvas(.unselected(steps: [])), status: .editing(problem: nil))
        let creating = Fixtures.recipeSection(pane: .create(editor: editor, draftRow: "Shakshuka"))
        #expect(creating.draftRow == "Shakshuka")
        #expect(creating.create == .unavailable)
    }

    /// The edit editor has no draft row, and its events carry the pane scope.
    @Test func recipeEditPaneEventsCarryThePaneScope() {
        let shakshuka = Fixtures.recipeRow("Shakshuka")
        let editor = RecipeEditorView(title: "Spicy shakshuka", lines: [], canvas: Fixtures.canvas(.unselected(steps: [])), status: .editing(problem: nil))
        let editing = Fixtures.recipeSection(.ready(rows: [shakshuka]), editing: shakshuka, editor: editor)
        #expect(editing.draftRow == nil)
        #expect(editing.editor(.save) == .editor(scope: ScopeId(value: 5), event: .save))
    }

    /// A row click carries the id; ⌘-click, Retry, Edit, Delete and its answer
    /// carry the pane scope.
    @Test func recipeReadViewEventsCarryTheIdAndThePaneScope() {
        let shakshuka = Fixtures.recipeRow("Shakshuka")
        let detail = RecipeDetailView(id: shakshuka.id, status: .ready(title: "Shakshuka", lines: [], steps: [], deletion: .idle))
        let reading = Fixtures.recipeSection(pane: .detail(detail: detail))
        #expect(reading.selected == shakshuka.id)
        #expect(reading.create == .available)
        #expect(reading.draftRow == nil)
        let pasta = Fixtures.recipeRow("Pasta").id
        #expect(reading.select(pasta) == .select(id: pasta))
        #expect(reading.clearSelection == .clearSelection(scope: ScopeId(value: 5)))
        #expect(reading.detail(.retry) == .detail(scope: ScopeId(value: 5), event: .retry))
        #expect(reading.detail(.edit) == .detail(scope: ScopeId(value: 5), event: .edit))
        #expect(reading.detail(.requestDelete) == .detail(scope: ScopeId(value: 5), event: .requestDelete))
        #expect(
            reading.detail(.confirmation(event: .confirm))
                == .detail(scope: ScopeId(value: 5), event: .confirmation(event: .confirm))
        )
    }

    @Test func platformLayoutMatchesTheTestHost() {
        #if os(macOS)
        #expect(AppLayout.platform == .split)
        #else
        #expect(AppLayout.platform == .stack)
        #endif
    }

    @Test(arguments: [
        (source: 0, destination: 2, expected: (from: UInt64(0), to: UInt64(1))),
        (source: 2, destination: 0, expected: (from: UInt64(2), to: UInt64(0))),
        (source: 0, destination: 3, expected: (from: UInt64(0), to: UInt64(2))),
    ])
    func convertsSwiftUIMoveOffsetToFinalIndex(
        source: Int, destination: Int, expected: (from: UInt64, to: UInt64)
    ) {
        #expect(IngredientFormEvent.move(from: source, toOffset: destination)
            == .moveUnit(from: expected.from, to: expected.to))
    }

    @Test(arguments: [(source: 1, destination: 1), (source: 1, destination: 2)])
    func dropsMoveThatKeepsPosition(source: Int, destination: Int) {
        #expect(IngredientFormEvent.move(from: source, toOffset: destination) == nil)
    }
}

struct TextEchoTests {

    @Test func sendsEditsAndKeepsTypedTextWhileRustCatchesUp() {
        var echo = TextEcho()
        #expect(echo.typed("a", displayed: "") == "a")
        #expect(echo.typed("ab", displayed: "") == "ab")

        // A render for the first edit must not erase the second one.
        #expect(echo.received("a", showing: "ab") == nil)
        #expect(echo.received("ab", showing: "ab") == nil)
        #expect(echo.pending.isEmpty)
    }

    @Test func showsValueChangedIndependentlyByRust() {
        var echo = TextEcho()
        _ = echo.typed("garlic", displayed: "")
        #expect(echo.received("onion", showing: "garlic") == "onion")
        #expect(echo.pending.isEmpty)
    }

    @Test func doesNotEchoRustValueBack() {
        var echo = TextEcho()
        #expect(echo.typed("onion", displayed: "onion") == nil)
    }
}
