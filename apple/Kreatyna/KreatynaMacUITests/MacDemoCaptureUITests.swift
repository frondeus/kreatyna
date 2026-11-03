import XCTest

/// Captures the Mac window screenshots for the Mac demos, one test method per demo.
///
/// This is delivery evidence, not a behavior check. It runs only on request, through
/// `.agents/skills/document-apple-delivery/scripts/capture-demo.sh mac KreatynaMacUITests/MacDemoCaptureUITests <folder>`.
/// Screenshots are attached to the result bundle with their demo file names.
final class MacDemoCaptureUITests: MacUITestCase {

    override func setUpWithError() throws {
        try super.setUpWithError()
        try XCTSkipUnless(
            ProcessInfo.processInfo.environment["KREATYNA_CAPTURE_DEMO"] == "1",
            "Set TEST_RUNNER_KREATYNA_CAPTURE_DEMO=1 to capture the demo."
        )
    }

    /// The window uses the Mac's system appearance; snapshots cover light and dark.
    @MainActor
    func testCaptureMacWindowDemo() throws {
        var app = try launch(seed: ["Garlic", "Tomato", "Canned tomatoes", "Cucumber"])
        XCTAssertTrue(app.staticTexts["Garlic"].waitForExistence(timeout: 5))
        capture("01-catalog", app)
        app.terminate()

        app = try launch(seed: [])
        XCTAssertTrue(app.staticTexts["No ingredients yet"].waitForExistence(timeout: 5))
        capture("02-empty-catalog", app)
        app.terminate()
    }

    /// `docs/epics/mac-shell/tasks/MAC-04-demo/`: select Garlic, then Basil.
    @MainActor
    func testCaptureMAC04Demo() throws {
        let app = try launch(json: demoIngredients())

        let garlic = app.staticTexts.matching(identifier: "ingredientRow")["Garlic"]
        XCTAssertTrue(garlic.waitForExistence(timeout: 5))
        garlic.click()
        XCTAssertTrue(app.staticTexts["clove"].waitForExistence(timeout: 5))
        capture("01-garlic", app)

        app.staticTexts.matching(identifier: "ingredientRow")["Basil"].click()
        XCTAssertTrue(waitForNonExistence(app.staticTexts["clove"]))
        capture("02-basil", app)
        app.terminate()
    }

    /// `docs/epics/mac-shell/tasks/MAC-08-demo/`: open Garlic, then search.
    @MainActor
    func testCaptureMAC08Demo() throws {
        let app = try launch(json: demoIngredients())

        let garlic = app.staticTexts.matching(identifier: "ingredientRow")["Garlic"]
        XCTAssertTrue(garlic.waitForExistence(timeout: 5))
        garlic.click()
        XCTAssertTrue(app.staticTexts["clove"].waitForExistence(timeout: 5))

        app.searchFields.firstMatch.click()
        app.typeText("tom")
        XCTAssertTrue(waitForNonExistence(garlic))
        capture("01-search", app)

        app.typeText("x")
        XCTAssertTrue(app.descendants(matching: .any)["ingredientListNoMatches"].waitForExistence(timeout: 5))
        capture("02-no-matches", app)
        app.terminate()
    }

    /// `docs/epics/mac-shell/tasks/MAC-05-demo/`: create from the toolbar.
    @MainActor
    func testCaptureMAC05Demo() throws {
        let app = try launch(json: demoIngredients())

        let rows = app.staticTexts.matching(identifier: "ingredientRow")
        let draftRow = app.staticTexts.matching(identifier: "ingredientDraftRow").firstMatch
        XCTAssertTrue(rows["Garlic"].waitForExistence(timeout: 5))
        rows["Garlic"].click()
        XCTAssertTrue(app.staticTexts["clove"].waitForExistence(timeout: 5))

        app.buttons["addIngredient"].click()
        XCTAssertTrue(draftRow.waitForExistence(timeout: 5))
        capture("01-new-form", app)

        app.typeKey("s", modifierFlags: .command)
        XCTAssertTrue(app.staticTexts["Required."].waitForExistence(timeout: 5))
        capture("02-validation", app)

        app.typeText("garlic")
        app.typeKey("s", modifierFlags: .command)
        XCTAssertTrue(app.descendants(matching: .any)["ingredientSaveError"].waitForExistence(timeout: 5))
        capture("03-duplicate-name", app)

        // Saving disables the form, which takes the focus away from Name.
        app.textFields["ingredientNameField"].click()
        app.typeKey("a", modifierFlags: .command)
        app.typeText("Paprika")
        app.typeKey("s", modifierFlags: .command)
        XCTAssertTrue(rows["Paprika"].waitForExistence(timeout: 5))
        XCTAssertTrue(app.descendants(matching: .any)["ingredientDetail"].waitForExistence(timeout: 5))
        capture("04-saved", app)

        app.buttons["addIngredient"].click()
        XCTAssertTrue(draftRow.waitForExistence(timeout: 5))
        app.typeText("Pap")
        rows["Tomato"].click()
        XCTAssertTrue(app.buttons["confirmDiscard"].waitForExistence(timeout: 5))
        capture("05-discard-question", app)
        app.buttons["confirmDiscard"].click()
        XCTAssertTrue(app.staticTexts["medium tomato"].waitForExistence(timeout: 5))

        app.searchFields.firstMatch.click()
        app.typeText("tom")
        XCTAssertTrue(waitForNonExistence(rows["Garlic"]))
        app.buttons["addIngredient"].click()
        XCTAssertTrue(draftRow.waitForExistence(timeout: 5))
        app.typeText("Cumin")
        app.typeKey("s", modifierFlags: .command)
        XCTAssertTrue(waitForNonExistence(draftRow))
        XCTAssertTrue(app.descendants(matching: .any)["ingredientDetail"].waitForExistence(timeout: 5))
        XCTAssertFalse(rows["Cumin"].exists)
        capture("06-saved-hidden-by-search", app)
        app.terminate()
    }

    /// `docs/epics/mac-shell/tasks/MAC-06-demo/`: edit from the detail.
    @MainActor
    func testCaptureMAC06Demo() throws {
        let app = try launch(json: demoIngredients())

        let rows = app.staticTexts.matching(identifier: "ingredientRow")
        XCTAssertTrue(rows["Garlic"].waitForExistence(timeout: 5))
        rows["Garlic"].click()
        XCTAssertTrue(app.staticTexts["clove"].waitForExistence(timeout: 5))

        app.buttons["editIngredient"].click()
        let name = app.textFields["ingredientNameField"]
        XCTAssertTrue(name.waitForExistence(timeout: 5))
        capture("01-edit-form", app)

        name.click()
        app.typeKey("a", modifierFlags: .command)
        app.typeText("Black Garlic")
        capture("02-typed-row-keeps-saved-name", app)

        rows["Tomato"].click()
        XCTAssertTrue(app.buttons["confirmDiscard"].waitForExistence(timeout: 5))
        capture("03-discard-question", app)
        app.buttons["cancelDiscard"].click()
        XCTAssertTrue(waitForNonExistence(app.buttons["cancelDiscard"]))

        app.typeKey("s", modifierFlags: .command)
        XCTAssertTrue(rows["Black Garlic"].waitForExistence(timeout: 5))
        XCTAssertTrue(app.descendants(matching: .any)["ingredientDetail"].waitForExistence(timeout: 5))
        capture("04-saved", app)
        app.terminate()
    }

    @MainActor
    func testCaptureMAC07Demo() throws {
        let app = try launch(json: demoIngredients())

        let rows = app.staticTexts.matching(identifier: "ingredientRow")
        XCTAssertTrue(rows["Garlic"].waitForExistence(timeout: 5))
        rows["Garlic"].click()
        XCTAssertTrue(app.staticTexts["clove"].waitForExistence(timeout: 5))

        app.buttons["deleteIngredient"].click()
        XCTAssertTrue(app.buttons["confirmDelete"].waitForExistence(timeout: 5))
        capture("01-delete-question", app)

        app.buttons["confirmDelete"].click()
        XCTAssertTrue(waitForNonExistence(rows["Garlic"]))
        XCTAssertTrue(app.descendants(matching: .any)["ingredientDetailPlaceholder"].waitForExistence(timeout: 5))
        capture("02-after-delete", app)
        app.terminate()
    }

    /// `docs/epics/recipes/tasks/REC-01-demo/`: the app opens on Recipes, then Ingredients.
    @MainActor
    func testCaptureREC01Demo() throws {
        let app = try launch(json: demoIngredients(), openIngredients: false)

        XCTAssertTrue(app.descendants(matching: .any)["recipeListEmpty"].waitForExistence(timeout: 5))
        capture("01-recipes-empty", app)

        sidebar("sidebarIngredients", in: app).click()
        XCTAssertTrue(app.staticTexts.matching(identifier: "ingredientRow")["Garlic"].waitForExistence(timeout: 5))
        capture("02-ingredients", app)
        app.terminate()
    }

    /// `docs/epics/recipes/tasks/REC-02-demo/`: New Recipe, a saved recipe, a refused title.
    @MainActor
    func testCaptureREC02Demo() throws {
        let app = try launch(json: nil, openIngredients: false)
        let draftRow = app.staticTexts.matching(identifier: "recipeDraftRow").firstMatch

        XCTAssertTrue(app.descendants(matching: .any)["recipeListEmpty"].waitForExistence(timeout: 5))
        app.buttons["newRecipe"].click()
        XCTAssertTrue(app.textFields["recipeTitleField"].waitForExistence(timeout: 5))
        capture("01-empty-editor", app)

        app.typeText("Shakshuka")
        app.typeKey("s", modifierFlags: .command)
        XCTAssertTrue(app.staticTexts.matching(identifier: "recipeRow")["Shakshuka"].waitForExistence(timeout: 5))
        XCTAssertTrue(waitForNonExistence(draftRow))
        capture("02-saved", app)

        app.buttons["newRecipe"].click()
        XCTAssertTrue(draftRow.waitForExistence(timeout: 5))
        app.typeText("shakshuka")
        app.typeKey("s", modifierFlags: .command)
        XCTAssertTrue(app.staticTexts["recipeTitleProblem"].waitForExistence(timeout: 5))
        capture("03-duplicate-title", app)
        app.terminate()
    }

    /// REC-03: a row click opens the read view; Save opens the new recipe;
    /// a row click with a typed title asks first.
    @MainActor
    func testCaptureREC03Demo() throws {
        let app = try launch(json: nil, openIngredients: false)
        let title = app.staticTexts["recipeTitle"]
        let rows = app.staticTexts.matching(identifier: "recipeRow")
        XCTAssertTrue(app.buttons["newRecipe"].waitForExistence(timeout: 5))
        for recipe in ["Shakshuka", "Pasta"] {
            app.typeKey("n", modifierFlags: .command)
            XCTAssertTrue(app.textFields["recipeTitleField"].waitForExistence(timeout: 5))
            app.typeText(recipe)
            app.typeKey("s", modifierFlags: .command)
            XCTAssertTrue(rows[recipe].waitForExistence(timeout: 5))
        }

        rows["Shakshuka"].click()
        XCTAssertTrue(title.waitForExistence(timeout: 5))
        XCTAssertTrue(waitFor(title, value: "Shakshuka"))
        capture("01-read-view", app)

        app.typeKey("n", modifierFlags: .command)
        XCTAssertTrue(app.textFields["recipeTitleField"].waitForExistence(timeout: 5))
        app.typeText("Omelette")
        app.typeKey("s", modifierFlags: .command)
        XCTAssertTrue(waitFor(title, value: "Omelette"))
        capture("02-after-save", app)

        app.typeKey("n", modifierFlags: .command)
        XCTAssertTrue(app.textFields["recipeTitleField"].waitForExistence(timeout: 5))
        app.typeText("Toast")
        rows["Shakshuka"].click()
        XCTAssertTrue(app.buttons["confirmDiscard"].waitForExistence(timeout: 5))
        capture("03-discard-on-click", app)
        app.terminate()
    }

    /// REC-04: Edit opens the editor with the saved title; Cancel after a
    /// change asks first; Save shows the new title in the read view and the list.
    @MainActor
    func testCaptureREC04Demo() throws {
        let app = try launch(json: nil, openIngredients: false)
        let title = app.staticTexts["recipeTitle"]
        let field = app.textFields["recipeTitleField"]
        let rows = app.staticTexts.matching(identifier: "recipeRow")
        XCTAssertTrue(app.buttons["newRecipe"].waitForExistence(timeout: 5))
        for recipe in ["Pasta", "Shakshuka"] {
            app.typeKey("n", modifierFlags: .command)
            XCTAssertTrue(field.waitForExistence(timeout: 5))
            app.typeText(recipe)
            app.typeKey("s", modifierFlags: .command)
            XCTAssertTrue(rows[recipe].waitForExistence(timeout: 5))
        }
        XCTAssertTrue(waitFor(title, value: "Shakshuka"))

        app.buttons["editRecipe"].click()
        XCTAssertTrue(field.waitForExistence(timeout: 5))
        capture("01-edit-recipe", app)

        field.click()
        app.typeKey("a", modifierFlags: .command)
        app.typeText("Spicy Shakshuka")
        app.typeKey(.escape, modifierFlags: [])
        XCTAssertTrue(app.buttons["confirmDiscard"].waitForExistence(timeout: 5))
        capture("02-discard-question", app)
        app.buttons["cancelDiscard"].click()
        XCTAssertTrue(waitForNonExistence(app.buttons["cancelDiscard"]))

        app.typeKey("s", modifierFlags: .command)
        XCTAssertTrue(rows["Spicy Shakshuka"].waitForExistence(timeout: 5))
        XCTAssertTrue(waitFor(title, value: "Spicy Shakshuka"))
        capture("03-after-rename", app)
        app.terminate()
    }

    /// REC-05: Delete asks "Delete Shakshuka?"; confirming empties the pane
    /// and removes the row.
    @MainActor
    func testCaptureREC05Demo() throws {
        let app = try launch(json: nil, openIngredients: false)
        let field = app.textFields["recipeTitleField"]
        let rows = app.staticTexts.matching(identifier: "recipeRow")
        XCTAssertTrue(app.buttons["newRecipe"].waitForExistence(timeout: 5))
        for recipe in ["Pasta", "Shakshuka"] {
            app.typeKey("n", modifierFlags: .command)
            XCTAssertTrue(field.waitForExistence(timeout: 5))
            app.typeText(recipe)
            app.typeKey("s", modifierFlags: .command)
            XCTAssertTrue(rows[recipe].waitForExistence(timeout: 5))
        }
        XCTAssertTrue(waitFor(app.staticTexts["recipeTitle"], value: "Shakshuka"))

        app.buttons["deleteRecipe"].click()
        XCTAssertTrue(app.buttons["confirmDelete"].waitForExistence(timeout: 5))
        capture("01-delete-question", app)

        app.buttons["confirmDelete"].click()
        XCTAssertTrue(app.descendants(matching: .any)["recipeDetailPlaceholder"].waitForExistence(timeout: 5))
        XCTAssertTrue(waitForNonExistence(rows["Shakshuka"]))
        capture("02-after-delete", app)
        app.terminate()
    }

    /// REC-06: Pasta is open; "sha" leaves only Shakshuka, "shax" matches nothing.
    @MainActor
    func testCaptureREC06Demo() throws {
        let app = try launch(json: nil, openIngredients: false)
        let field = app.textFields["recipeTitleField"]
        let rows = app.staticTexts.matching(identifier: "recipeRow")
        XCTAssertTrue(app.buttons["newRecipe"].waitForExistence(timeout: 5))
        for recipe in ["Shakshuka", "Roasted tomato pasta"] {
            app.typeKey("n", modifierFlags: .command)
            XCTAssertTrue(field.waitForExistence(timeout: 5))
            app.typeText(recipe)
            app.typeKey("s", modifierFlags: .command)
            XCTAssertTrue(rows[recipe].waitForExistence(timeout: 5))
        }
        XCTAssertTrue(waitFor(app.staticTexts["recipeTitle"], value: "Roasted tomato pasta"))

        app.searchFields["recipeSearch"].click()
        app.typeText("sha")
        XCTAssertTrue(waitForNonExistence(rows["Roasted tomato pasta"]))
        capture("01-search-sha", app)

        app.typeText("x")
        XCTAssertTrue(app.descendants(matching: .any)["recipeListNoMatches"].waitForExistence(timeout: 5))
        capture("02-no-matches", app)
        app.terminate()
    }

    /// The ingredient list the demos start from (Garlic, Tomato, Basil, …).
    private func demoIngredients() throws -> String {
        let fixture = URL(fileURLWithPath: #filePath)
            .deletingLastPathComponent().deletingLastPathComponent()
            .appendingPathComponent("Fixtures/demo-ingredients.json")
        return try String(contentsOf: fixture, encoding: .utf8)
    }

    @MainActor
    private func capture(_ name: String, _ app: XCUIApplication) {
        let attachment = XCTAttachment(screenshot: app.windows.firstMatch.screenshot())
        attachment.name = name
        attachment.lifetime = .keepAlways
        add(attachment)
    }
}
