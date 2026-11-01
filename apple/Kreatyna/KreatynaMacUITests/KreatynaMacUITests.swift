import XCTest

// Smoke flows over the real Rust core. They prove that the app starts Rust and
// that its views reach the window. Rust tests own the behavior rules.
final class KreatynaMacUITests: XCTestCase {

    override func setUpWithError() throws {
        continueAfterFailure = false
    }

    @MainActor
    func testLaunchShowsSeededCatalog() throws {
        let app = try launch(seed: ["Garlic", "Tomato"])

        XCTAssertTrue(row(named: "Garlic", in: app).waitForExistence(timeout: 5))
        XCTAssertTrue(row(named: "Tomato", in: app).exists)
        XCTAssertTrue(app.buttons["addIngredient"].isEnabled)
    }

    @MainActor
    func testSelectingRowsShowsTheirDetail() throws {
        let app = try launch(json: #"""
            [{"name": "Garlic", "units": [{"name": "clove", "grams": 5}]}, "Basil"]
            """#)

        XCTAssertTrue(row(named: "Garlic", in: app).waitForExistence(timeout: 5))
        XCTAssertTrue(app.descendants(matching: .any)["ingredientDetailPlaceholder"].exists)

        row(named: "Garlic", in: app).click()
        XCTAssertTrue(app.staticTexts["clove"].waitForExistence(timeout: 5))
        XCTAssertTrue(app.staticTexts["5 g"].exists)

        row(named: "Basil", in: app).click()
        XCTAssertTrue(waitForNonExistence(app.staticTexts["clove"], timeout: 5))
        XCTAssertTrue(app.staticTexts["Basil"].firstMatch.exists)

        // ↓ moves the selection like a click: Basil → Garlic.
        app.typeKey(.downArrow, modifierFlags: [])
        XCTAssertTrue(app.staticTexts["clove"].waitForExistence(timeout: 5))

        // ⌘-click on the selected row clears the selection.
        XCUIElement.perform(withKeyModifiers: .command) {
            row(named: "Garlic", in: app).click()
        }
        XCTAssertTrue(waitForNonExistence(app.staticTexts["clove"], timeout: 5))
        XCTAssertTrue(app.descendants(matching: .any)["ingredientDetailPlaceholder"].exists)
    }

    @MainActor
    func testSearchNarrowsTheList() throws {
        let app = try launch(json: #"""
            ["Tomato", "Canned tomatoes", {"name": "Garlic", "units": [{"name": "clove", "grams": 5}]}]
            """#)
        XCTAssertTrue(row(named: "Garlic", in: app).waitForExistence(timeout: 5))
        row(named: "Garlic", in: app).click()
        XCTAssertTrue(app.staticTexts["clove"].waitForExistence(timeout: 5))

        // The search field above the list has no ⌘F (MAC-08), so the test clicks it.
        app.searchFields.firstMatch.click()
        app.typeText("tom")
        XCTAssertTrue(waitForNonExistence(row(named: "Garlic", in: app), timeout: 5))
        XCTAssertTrue(row(named: "Tomato", in: app).exists)
        XCTAssertTrue(row(named: "Canned tomatoes", in: app).exists)
        // Garlic's detail stays open (MAC-08/Q1).
        XCTAssertTrue(app.staticTexts["clove"].exists)

        app.typeText("x")
        XCTAssertTrue(app.descendants(matching: .any)["ingredientListNoMatches"].waitForExistence(timeout: 5))

        // Esc clears the field, and every row comes back.
        app.typeKey(.escape, modifierFlags: [])
        XCTAssertTrue(row(named: "Garlic", in: app).waitForExistence(timeout: 5))
        XCTAssertTrue(row(named: "Tomato", in: app).exists)
    }

    @MainActor
    func testCreatedIngredientSurvivesRelaunch() throws {
        let database = UUID().uuidString
        var app = try launch(json: #"["Garlic", "Tomato"]"#, database: database)
        XCTAssertTrue(row(named: "Garlic", in: app).waitForExistence(timeout: 5))

        app.buttons["addIngredient"].click()
        XCTAssertTrue(draftRow(in: app).waitForExistence(timeout: 5))
        XCTAssertTrue(waitFor(draftRow(in: app), value: "New Ingredient"))
        XCTAssertFalse(app.buttons["addIngredient"].isEnabled)

        // Name has the focus, so typing goes there; the draft row follows.
        app.typeText("garlic")
        XCTAssertTrue(waitFor(draftRow(in: app), value: "garlic"))
        app.typeKey("s", modifierFlags: .command)
        XCTAssertTrue(app.descendants(matching: .any)["ingredientSaveError"].waitForExistence(timeout: 5))

        // Typing after the error keeps the cursor in Name; the first key
        // clears the error (it once moved the rows and dropped the cursor).
        app.textFields["ingredientNameField"].click()
        app.typeKey("a", modifierFlags: .command)
        app.typeText("Paprika")
        XCTAssertTrue(waitFor(draftRow(in: app), value: "Paprika"))
        XCTAssertFalse(app.descendants(matching: .any)["ingredientSaveError"].exists)
        app.typeKey("s", modifierFlags: .command)

        XCTAssertTrue(row(named: "Paprika", in: app).waitForExistence(timeout: 5))
        XCTAssertTrue(waitForNonExistence(draftRow(in: app), timeout: 5))
        XCTAssertTrue(app.descendants(matching: .any)["ingredientDetail"].exists)
        XCTAssertTrue(app.buttons["addIngredient"].isEnabled)

        app.terminate()
        app = try launch(json: nil, database: database)
        XCTAssertTrue(row(named: "Paprika", in: app).waitForExistence(timeout: 5))
    }

    @MainActor
    func testDirtyFormAsksBeforeOpeningARow() throws {
        let app = try launch(json: #"""
            [{"name": "Garlic", "units": [{"name": "clove", "grams": 5}]},
             {"name": "Tomato", "units": [{"name": "slice", "grams": 20}]}]
            """#)
        XCTAssertTrue(row(named: "Garlic", in: app).waitForExistence(timeout: 5))
        row(named: "Garlic", in: app).click()
        XCTAssertTrue(app.staticTexts["clove"].waitForExistence(timeout: 5))

        // Esc on a clean form brings Garlic back.
        app.buttons["addIngredient"].click()
        XCTAssertTrue(draftRow(in: app).waitForExistence(timeout: 5))
        app.typeKey(.escape, modifierFlags: [])
        XCTAssertTrue(app.staticTexts["clove"].waitForExistence(timeout: 5))
        XCTAssertFalse(draftRow(in: app).exists)

        // A row click on a dirty form asks first (E-10).
        app.buttons["addIngredient"].click()
        XCTAssertTrue(draftRow(in: app).waitForExistence(timeout: 5))
        app.typeText("Pap")
        XCTAssertTrue(waitFor(draftRow(in: app), value: "Pap"))
        row(named: "Tomato", in: app).click()
        XCTAssertTrue(app.buttons["cancelDiscard"].waitForExistence(timeout: 5))

        app.buttons["cancelDiscard"].click()
        XCTAssertTrue(waitForNonExistence(app.buttons["cancelDiscard"], timeout: 5))
        XCTAssertEqual(app.textFields["ingredientNameField"].value as? String, "Pap")
        XCTAssertFalse(app.staticTexts["slice"].exists)

        row(named: "Tomato", in: app).click()
        XCTAssertTrue(app.buttons["confirmDiscard"].waitForExistence(timeout: 5))
        app.buttons["confirmDiscard"].click()
        XCTAssertTrue(app.staticTexts["slice"].waitForExistence(timeout: 5))
        XCTAssertFalse(draftRow(in: app).exists)
    }

    @MainActor
    func testEditedNameAppearsInTheList() throws {
        let app = try launch(seed: ["Garlic", "Tomato"])
        XCTAssertTrue(row(named: "Garlic", in: app).waitForExistence(timeout: 5))
        row(named: "Garlic", in: app).click()
        XCTAssertTrue(app.buttons["editIngredient"].waitForExistence(timeout: 5))

        app.buttons["editIngredient"].click()
        let name = app.textFields["ingredientNameField"]
        XCTAssertTrue(name.waitForExistence(timeout: 5))
        XCTAssertEqual(name.value as? String, "Garlic")
        XCTAssertFalse(app.buttons["addIngredient"].isEnabled)

        // The row keeps the saved name until Save (MAC-06/Q1).
        name.click()
        app.typeKey("a", modifierFlags: .command)
        app.typeText("Black Garlic")
        XCTAssertTrue(row(named: "Garlic", in: app).exists)
        app.typeKey("s", modifierFlags: .command)

        XCTAssertTrue(row(named: "Black Garlic", in: app).waitForExistence(timeout: 5))
        XCTAssertTrue(waitForNonExistence(row(named: "Garlic", in: app), timeout: 5))
        // The detail's title, apart from the row.
        XCTAssertTrue(app.descendants(matching: .any)["ingredientDetail"].waitForExistence(timeout: 5))
        let title = app.staticTexts
            .matching(NSPredicate(format: "identifier != %@ AND (value == %@ OR label == %@)",
                                  "ingredientRow", "Black Garlic", "Black Garlic"))
            .firstMatch
        XCTAssertTrue(title.exists)
        XCTAssertTrue(app.buttons["addIngredient"].isEnabled)
    }

    @MainActor
    func testContextMenuReordersUnits() throws {
        let app = try launch(seed: ["Garlic"])
        XCTAssertTrue(row(named: "Garlic", in: app).waitForExistence(timeout: 5))
        app.buttons["addIngredient"].click()
        XCTAssertTrue(draftRow(in: app).waitForExistence(timeout: 5))

        for (index, name) in ["clove", "bulb"].enumerated() {
            app.buttons["addIngredientUnit"].click()
            let field = app.textFields["ingredientUnitNameField.\(index)"]
            XCTAssertTrue(field.waitForExistence(timeout: 5))
            field.click()
            app.typeText(name)
        }

        // Drag does not reorder inside a grouped form on macOS (MAC-05/Q4 fallback).
        app.descendants(matching: .any)["ingredientUnitRow.1"]
            .coordinate(withNormalizedOffset: CGVector(dx: 0.005, dy: 0.5))
            .rightClick()
        XCTAssertTrue(app.menuItems["Move Up"].waitForExistence(timeout: 5))
        app.menuItems["Move Up"].click()
        XCTAssertTrue(waitFor(app.textFields["ingredientUnitNameField.0"], value: "bulb"))
        XCTAssertEqual(app.textFields["ingredientUnitNameField.1"].value as? String, "clove")

        app.buttons["removeIngredientUnit.0"].click()
        XCTAssertTrue(waitFor(app.textFields["ingredientUnitNameField.0"], value: "clove"))
        XCTAssertFalse(app.textFields["ingredientUnitNameField.1"].exists)
    }

    @MainActor
    func testDeletedIngredientStaysGoneAfterRelaunch() throws {
        let database = UUID().uuidString
        var app = try launch(json: #"["Garlic", "Tomato"]"#, database: database)
        XCTAssertTrue(row(named: "Garlic", in: app).waitForExistence(timeout: 5))
        row(named: "Garlic", in: app).click()
        XCTAssertTrue(app.descendants(matching: .any)["ingredientDetail"].waitForExistence(timeout: 5))

        // Cancel keeps Garlic open.
        app.buttons["deleteIngredient"].click()
        XCTAssertTrue(app.buttons["cancelDelete"].waitForExistence(timeout: 5))
        app.buttons["cancelDelete"].click()
        XCTAssertTrue(waitForNonExistence(app.buttons["cancelDelete"], timeout: 5))
        XCTAssertTrue(app.descendants(matching: .any)["ingredientDetail"].exists)
        XCTAssertTrue(row(named: "Garlic", in: app).exists)

        app.buttons["deleteIngredient"].click()
        XCTAssertTrue(app.buttons["confirmDelete"].waitForExistence(timeout: 5))
        app.buttons["confirmDelete"].click()
        XCTAssertTrue(waitForNonExistence(row(named: "Garlic", in: app), timeout: 5))
        XCTAssertTrue(app.descendants(matching: .any)["ingredientDetailPlaceholder"].waitForExistence(timeout: 5))
        XCTAssertTrue(row(named: "Tomato", in: app).exists)

        app.terminate()
        app = try launch(json: nil, database: database)
        XCTAssertTrue(row(named: "Tomato", in: app).waitForExistence(timeout: 5))
        XCTAssertFalse(row(named: "Garlic", in: app).exists)
    }

    /// The app opens on Recipes. Switching sections keeps each section's state (REC-01).
    @MainActor
    func testSidebarSwitchesSections() throws {
        let app = try launch(seed: ["Garlic", "Tomato"], openIngredients: false)

        XCTAssertTrue(app.descendants(matching: .any)["recipeListEmpty"].waitForExistence(timeout: 5))
        XCTAssertTrue(app.buttons["newRecipe"].isEnabled)

        sidebar("sidebarIngredients", in: app).click()
        let garlic = row(named: "Garlic", in: app)
        XCTAssertTrue(garlic.waitForExistence(timeout: 5))
        garlic.click()
        XCTAssertTrue(app.buttons["editIngredient"].waitForExistence(timeout: 5))

        sidebar("sidebarRecipes", in: app).click()
        XCTAssertTrue(app.descendants(matching: .any)["recipeDetailPlaceholder"].waitForExistence(timeout: 5))
        XCTAssertFalse(app.buttons["editIngredient"].exists)

        sidebar("sidebarIngredients", in: app).click()
        // Rust kept Garlic open while Recipes showed.
        XCTAssertTrue(app.buttons["editIngredient"].waitForExistence(timeout: 5))
    }

    /// New Recipe, a title, Save: the row shows and stays after a relaunch (REC-02).
    @MainActor
    func testCreatedRecipeSurvivesRelaunch() throws {
        let database = UUID().uuidString
        var app = try launch(json: nil, database: database, openIngredients: false)
        XCTAssertTrue(app.descendants(matching: .any)["recipeListEmpty"].waitForExistence(timeout: 5))

        app.buttons["newRecipe"].click()
        XCTAssertTrue(recipeDraftRow(in: app).waitForExistence(timeout: 5))
        XCTAssertTrue(waitFor(recipeDraftRow(in: app), value: "New Recipe"))
        XCTAssertFalse(app.buttons["newRecipe"].isEnabled)

        // Title has the focus, so typing goes there; the draft row follows.
        app.typeText("Shakshuka")
        XCTAssertTrue(waitFor(recipeDraftRow(in: app), value: "Shakshuka"))
        app.typeKey("s", modifierFlags: .command)

        XCTAssertTrue(recipeRow(named: "Shakshuka", in: app).waitForExistence(timeout: 5))
        XCTAssertTrue(waitForNonExistence(recipeDraftRow(in: app), timeout: 5))
        XCTAssertTrue(app.descendants(matching: .any)["recipeDetailPlaceholder"].exists)
        XCTAssertTrue(app.buttons["newRecipe"].isEnabled)

        app.terminate()
        app = try launch(json: nil, database: database, openIngredients: false)
        XCTAssertTrue(recipeRow(named: "Shakshuka", in: app).waitForExistence(timeout: 5))
    }

    /// A title that differs only in case is refused; Cancel then asks first (REC-02).
    @MainActor
    func testDuplicateRecipeTitleIsRefused() throws {
        let app = try launch(json: nil, openIngredients: false)
        XCTAssertTrue(app.buttons["newRecipe"].waitForExistence(timeout: 5))

        app.typeKey("n", modifierFlags: .command)
        XCTAssertTrue(recipeDraftRow(in: app).waitForExistence(timeout: 5))
        app.typeText("Shakshuka")
        app.typeKey("s", modifierFlags: .command)
        XCTAssertTrue(recipeRow(named: "Shakshuka", in: app).waitForExistence(timeout: 5))

        app.typeKey("n", modifierFlags: .command)
        XCTAssertTrue(recipeDraftRow(in: app).waitForExistence(timeout: 5))
        app.typeText("shakshuka")
        app.typeKey("s", modifierFlags: .command)
        let problem = app.staticTexts["recipeTitleProblem"]
        XCTAssertTrue(problem.waitForExistence(timeout: 5))
        XCTAssertTrue(waitFor(problem, value: "A recipe with this title already exists."))

        app.typeKey(.escape, modifierFlags: [])
        let discard = app.buttons["confirmDiscard"]
        XCTAssertTrue(discard.waitForExistence(timeout: 5))
        discard.click()
        XCTAssertTrue(app.descendants(matching: .any)["recipeDetailPlaceholder"].waitForExistence(timeout: 5))
        XCTAssertEqual(app.staticTexts.matching(identifier: "recipeRow").count, 1)
    }

    @MainActor
    func testLaunchShowsEmptyCatalog() throws {
        let app = try launch(seed: [])

        XCTAssertTrue(app.staticTexts["No ingredients yet"].waitForExistence(timeout: 5))
    }

    @MainActor
    private func launch(seed: [String], openIngredients: Bool = true) throws -> XCUIApplication {
        // The seed is a JSON array of names (E-17).
        try launch(
            json: seed.isEmpty ? nil : String(decoding: JSONEncoder().encode(seed), as: UTF8.self),
            openIngredients: openIngredients
        )
    }

    /// The app opens on Recipes (REC-01/Q1). The ingredient tests click
    /// Ingredients first, unless `openIngredients` is false.
    @MainActor
    private func launch(
        json: String?,
        database: String = UUID().uuidString,
        openIngredients: Bool = true
    ) throws -> XCUIApplication {
        let app = XCUIApplication()
        app.launchEnvironment["KREATYNA_TEST_DATABASE_ID"] = database
        if let json {
            app.launchEnvironment["KREATYNA_TEST_SEED"] = json
        }
        app.launch()
        if openIngredients {
            let item = sidebar("sidebarIngredients", in: app)
            XCTAssertTrue(item.waitForExistence(timeout: 5))
            item.click()
        }
        return app
    }

    @MainActor
    private func sidebar(_ identifier: String, in app: XCUIApplication) -> XCUIElement {
        app.descendants(matching: .any).matching(identifier: identifier).firstMatch
    }

    private func waitForNonExistence(_ element: XCUIElement, timeout: TimeInterval) -> Bool {
        let gone = XCTNSPredicateExpectation(predicate: NSPredicate(format: "exists == false"), object: element)
        return XCTWaiter().wait(for: [gone], timeout: timeout) == .completed
    }

    private func waitFor(_ element: XCUIElement, value: String, timeout: TimeInterval = 5) -> Bool {
        let match = XCTNSPredicateExpectation(predicate: NSPredicate(format: "value == %@", value), object: element)
        return XCTWaiter().wait(for: [match], timeout: timeout) == .completed
    }

    @MainActor
    private func draftRow(in app: XCUIApplication) -> XCUIElement {
        app.staticTexts.matching(identifier: "ingredientDraftRow").firstMatch
    }

    @MainActor
    private func recipeDraftRow(in app: XCUIApplication) -> XCUIElement {
        app.staticTexts.matching(identifier: "recipeDraftRow").firstMatch
    }

    @MainActor
    private func recipeRow(named title: String, in app: XCUIApplication) -> XCUIElement {
        app.staticTexts.matching(identifier: "recipeRow")
            .matching(NSPredicate(format: "value == %@ OR label == %@", title, title))
            .firstMatch
    }

    @MainActor
    private func row(named name: String, in app: XCUIApplication) -> XCUIElement {
        app.staticTexts.matching(identifier: "ingredientRow")
            .matching(NSPredicate(format: "value == %@ OR label == %@", name, name))
            .firstMatch
    }
}
