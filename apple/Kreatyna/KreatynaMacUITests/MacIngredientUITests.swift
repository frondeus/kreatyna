import XCTest

// Smoke flows over the real Rust core. They prove that the app starts Rust and
// that its views reach the window. Rust tests own the behavior rules.
/// The Ingredients section.
final class MacIngredientUITests: MacUITestCase {

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
        XCTAssertTrue(waitFor(app.staticTexts["ingredientName"], value: "Garlic"))

        row(named: "Basil", in: app).click()
        XCTAssertTrue(waitForNonExistence(app.staticTexts["clove"], timeout: 5))
        XCTAssertTrue(waitFor(app.staticTexts["ingredientName"], value: "Basil"))

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
        XCTAssertTrue(waitFor(app.staticTexts["ingredientName"], value: "Paprika"))
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
        XCTAssertTrue(app.descendants(matching: .any)["ingredientDetail"].waitForExistence(timeout: 5))
        XCTAssertTrue(waitFor(app.staticTexts["ingredientName"], value: "Black Garlic"))
        XCTAssertTrue(app.buttons["addIngredient"].isEnabled)
    }

    @MainActor
    func testContextMenuReordersUnits() throws {
        let app = try launch(seed: ["Garlic"])
        XCTAssertTrue(row(named: "Garlic", in: app).waitForExistence(timeout: 5))
        app.buttons["addIngredient"].click()
        // The draft row can show before the form does, so wait for Add Unit
        // too: a click on a missing element fails at once (vibetyna-kqh).
        XCTAssertTrue(draftRow(in: app).waitForExistence(timeout: 5))
        let addUnit = app.buttons["addIngredientUnit"]
        XCTAssertTrue(addUnit.waitForExistence(timeout: 5))

        for (index, name) in ["clove", "bulb"].enumerated() {
            addUnit.click()
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

    @MainActor
    func testLaunchShowsEmptyCatalog() throws {
        let app = try launch(seed: [])

        XCTAssertTrue(app.staticTexts["No ingredients yet"].waitForExistence(timeout: 5))
    }

    @MainActor
    private func draftRow(in app: XCUIApplication) -> XCUIElement {
        app.staticTexts.matching(identifier: "ingredientDraftRow").firstMatch
    }

    @MainActor
    private func row(named name: String, in app: XCUIApplication) -> XCUIElement {
        app.staticTexts.matching(identifier: "ingredientRow")
            .matching(NSPredicate(format: "value == %@ OR label == %@", name, name))
            .firstMatch
    }
}
