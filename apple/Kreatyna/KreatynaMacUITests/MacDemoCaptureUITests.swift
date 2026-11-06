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

    /// REC-14: steps on the canvas. The context menu adds a third step; Save
    /// refuses it untitled and selects it; with a title, the read view lists
    /// the steps and the row counts them.
    @MainActor
    func testCaptureREC14Demo() throws {
        let app = try launch(json: nil, openIngredients: false)
        XCTAssertTrue(app.buttons["newRecipe"].waitForExistence(timeout: 5))
        app.typeKey("n", modifierFlags: .command)
        XCTAssertTrue(app.textFields["recipeTitleField"].waitForExistence(timeout: 5))
        app.typeText("Shakshuka")

        let canvas = app.scrollViews["stepCanvas"]
        XCTAssertTrue(canvas.waitForExistence(timeout: 5))
        let add = app.menuItems["Add Step"]
        for (title, offset) in [("Dice onion", CGVector(dx: 0.08, dy: 0.08)), ("Fry onion", CGVector(dx: 0.35, dy: 0.3))] {
            canvas.coordinate(withNormalizedOffset: offset).rightClick()
            XCTAssertTrue(add.waitForExistence(timeout: 5))
            add.click()
            XCTAssertTrue(app.textFields["stepTitleField"].waitForExistence(timeout: 5))
            app.typeText(title)
        }

        canvas.coordinate(withNormalizedOffset: CGVector(dx: 0.12, dy: 0.55)).rightClick()
        XCTAssertTrue(add.waitForExistence(timeout: 5))
        captureScreen("01-context-menu", around: app.windows.firstMatch)
        add.click()

        app.typeKey("s", modifierFlags: .command)
        XCTAssertTrue(waitFor(app.staticTexts["stepTitleProblem"], value: "A step needs a title."))
        capture("02-inspector", app)

        app.typeText("Crack eggs")
        app.typeKey("s", modifierFlags: .command)
        XCTAssertTrue(waitFor(app.staticTexts["recipeTitle"], value: "Shakshuka"))
        XCTAssertTrue(waitFor(app.staticTexts["recipeStepCount"], value: "3 steps"))
        capture("03-read-view", app)
        app.terminate()
    }

    /// REC-15: a drag moves "Dice onion" below "Fry onion" and selects it.
    /// After Save, another recipe, and Edit again, it is still there.
    @MainActor
    func testCaptureREC15Demo() throws {
        let app = try launch(json: nil, openIngredients: false)
        let field = app.textFields["recipeTitleField"]
        XCTAssertTrue(app.buttons["newRecipe"].waitForExistence(timeout: 5))
        app.typeKey("n", modifierFlags: .command)
        XCTAssertTrue(field.waitForExistence(timeout: 5))
        app.typeText("Pasta")
        app.typeKey("s", modifierFlags: .command)
        XCTAssertTrue(waitFor(app.staticTexts["recipeTitle"], value: "Pasta"))

        app.typeKey("n", modifierFlags: .command)
        XCTAssertTrue(field.waitForExistence(timeout: 5))
        app.typeText("Shakshuka")
        let canvas = app.scrollViews["stepCanvas"]
        XCTAssertTrue(canvas.waitForExistence(timeout: 5))
        let add = app.menuItems["Add Step"]
        for (title, offset) in [("Dice onion", CGVector(dx: 0.08, dy: 0.08)), ("Fry onion", CGVector(dx: 0.35, dy: 0.3))] {
            canvas.coordinate(withNormalizedOffset: offset).rightClick()
            XCTAssertTrue(add.waitForExistence(timeout: 5))
            add.click()
            XCTAssertTrue(app.textFields["stepTitleField"].waitForExistence(timeout: 5))
            app.typeText(title)
        }
        app.typeKey("s", modifierFlags: .command)
        XCTAssertTrue(waitFor(app.staticTexts["recipeStepCount"], value: "2 steps"))

        app.buttons["editRecipe"].click()
        XCTAssertTrue(app.staticTexts["selectStepHint"].waitForExistence(timeout: 5))
        capture("01-before-drag", app)

        let nodes = app.descendants(matching: .any).matching(identifier: "stepNode")
        let onion = nodes.matching(NSPredicate(format: "label == 'Dice onion'")).firstMatch
        let start = onion.coordinate(withNormalizedOffset: CGVector(dx: 0.5, dy: 0.5))
        start.click(forDuration: 0.2, thenDragTo: start.withOffset(CGVector(dx: 30, dy: 200)))
        XCTAssertTrue(waitFor(app.textFields["stepTitleField"], value: "Dice onion"))
        capture("02-after-drop", app)

        app.typeKey("s", modifierFlags: .command)
        XCTAssertTrue(waitFor(app.staticTexts["recipeStepCount"], value: "2 steps"))
        let rows = app.staticTexts.matching(identifier: "recipeRow")
        rows["Pasta"].click()
        XCTAssertTrue(waitFor(app.staticTexts["recipeTitle"], value: "Pasta"))
        rows["Shakshuka"].click()
        XCTAssertTrue(waitFor(app.staticTexts["recipeTitle"], value: "Shakshuka"))
        app.buttons["editRecipe"].click()
        XCTAssertTrue(app.staticTexts["selectStepHint"].waitForExistence(timeout: 5))
        capture("03-reopened", app)
        app.terminate()
    }

    /// REC-16: the Edit menu with Delete Step (⌘⌫) while "Dice onion" is selected, the node
    /// context menu on "Fry onion", and the canvas after both deletes.
    @MainActor
    func testCaptureREC16Demo() throws {
        let app = try launch(json: nil, openIngredients: false)
        XCTAssertTrue(app.buttons["newRecipe"].waitForExistence(timeout: 5))
        app.typeKey("n", modifierFlags: .command)
        XCTAssertTrue(app.textFields["recipeTitleField"].waitForExistence(timeout: 5))
        app.typeText("Shakshuka")
        let canvas = app.scrollViews["stepCanvas"]
        XCTAssertTrue(canvas.waitForExistence(timeout: 5))
        let add = app.menuItems["Add Step"]
        let steps = [("Dice onion", CGVector(dx: 0.08, dy: 0.08)), ("Fry onion", CGVector(dx: 0.35, dy: 0.3)), ("Crack eggs", CGVector(dx: 0.12, dy: 0.55))]
        for (title, offset) in steps {
            canvas.coordinate(withNormalizedOffset: offset).rightClick()
            XCTAssertTrue(add.waitForExistence(timeout: 5))
            add.click()
            XCTAssertTrue(app.textFields["stepTitleField"].waitForExistence(timeout: 5))
            app.typeText(title)
        }
        app.typeKey("s", modifierFlags: .command)
        XCTAssertTrue(waitFor(app.staticTexts["recipeStepCount"], value: "3 steps"))
        app.buttons["editRecipe"].click()
        XCTAssertTrue(app.staticTexts["selectStepHint"].waitForExistence(timeout: 5))

        let nodes = app.descendants(matching: .any).matching(identifier: "stepNode")
        nodes.matching(NSPredicate(format: "label == 'Dice onion'")).firstMatch.click()
        XCTAssertTrue(waitFor(app.textFields["stepTitleField"], value: "Dice onion"))
        let edit = app.menuBars.menuBarItems["Edit"]
        edit.click()
        XCTAssertTrue(app.menuBars.menuItems["Delete Step"].waitForExistence(timeout: 5))
        // The menu only: the menu bar and the desktop around it stay out.
        captureScreen("01-edit-menu", around: edit.menus.firstMatch)
        app.menuBars.menuItems["Delete Step"].click()
        XCTAssertTrue(app.staticTexts["selectStepHint"].waitForExistence(timeout: 5))

        nodes.matching(NSPredicate(format: "label == 'Fry onion'")).firstMatch.rightClick()
        let delete = app.menuItems.matching(identifier: "deleteStep").firstMatch
        XCTAssertTrue(delete.waitForExistence(timeout: 5))
        captureScreen("02-context-menu", around: app.windows.firstMatch)
        delete.click()
        XCTAssertTrue(waitForNonExistence(nodes.matching(NSPredicate(format: "label == 'Fry onion'")).firstMatch))
        capture("03-after-delete", app)
        app.terminate()
    }

    /// REC-17: the inspector with the 10 min chip, Waiting, and instructions; More… with
    /// 1 min 30 s; and the read view with the step details.
    @MainActor
    func testCaptureREC17Demo() throws {
        let app = try launch(json: nil, openIngredients: false)
        XCTAssertTrue(app.buttons["newRecipe"].waitForExistence(timeout: 5))
        app.typeKey("n", modifierFlags: .command)
        XCTAssertTrue(app.textFields["recipeTitleField"].waitForExistence(timeout: 5))
        app.typeText("Pasta")
        let canvas = app.scrollViews["stepCanvas"]
        XCTAssertTrue(canvas.waitForExistence(timeout: 5))
        let add = app.menuItems["Add Step"]
        let nodes = app.descendants(matching: .any).matching(identifier: "stepNode")

        canvas.coordinate(withNormalizedOffset: CGVector(dx: 0.08, dy: 0.08)).rightClick()
        XCTAssertTrue(add.waitForExistence(timeout: 5))
        add.click()
        XCTAssertTrue(app.textFields["stepTitleField"].waitForExistence(timeout: 5))
        app.typeText("Boil water")
        app.buttons["durationChip10"].click()
        app.radioButtons["Waiting"].click()
        let instructions = app.descendants(matching: .any).matching(identifier: "stepInstructionsField").firstMatch
        XCTAssertTrue(instructions.waitForExistence(timeout: 5))
        instructions.click()
        app.typeText("Salt it well.")
        let boil = nodes.matching(NSPredicate(format: "label == 'Boil water'")).firstMatch
        XCTAssertTrue(waitFor(boil, value: "10 min · waiting"))
        capture("01-inspector", app)

        canvas.coordinate(withNormalizedOffset: CGVector(dx: 0.12, dy: 0.42)).rightClick()
        XCTAssertTrue(add.waitForExistence(timeout: 5))
        add.click()
        XCTAssertTrue(app.textFields["stepTitleField"].waitForExistence(timeout: 5))
        app.typeText("Sear scallops")
        // A link-style button: macOS may list it as a link.
        app.descendants(matching: .any).matching(identifier: "durationMoreButton").firstMatch.click()
        let unitMinutes = app.textFields["durationUnit-min"]
        XCTAssertTrue(unitMinutes.waitForExistence(timeout: 5))
        unitMinutes.click()
        app.typeText("1")
        app.textFields["durationUnit-s"].click()
        app.typeText("30")
        let sear = nodes.matching(NSPredicate(format: "label == 'Sear scallops'")).firstMatch
        XCTAssertTrue(waitFor(sear, value: "1 min 30 s · active"))
        capture("02-more", app)

        app.typeKey("s", modifierFlags: .command)
        XCTAssertTrue(waitFor(app.staticTexts["recipeStepCount"], value: "2 steps"))
        XCTAssertTrue(app.staticTexts["recipeStepInstructions"].waitForExistence(timeout: 5))
        capture("03-read-view", app)
        app.terminate()
    }

    /// REC-18: three steps connected into a chain with Fry onion's After;
    /// the loop refused; and an arrow's context menu.
    @MainActor
    func testCaptureREC18Demo() throws {
        let app = try launch(json: nil, openIngredients: false)
        XCTAssertTrue(app.buttons["newRecipe"].waitForExistence(timeout: 5))
        app.typeKey("n", modifierFlags: .command)
        XCTAssertTrue(app.textFields["recipeTitleField"].waitForExistence(timeout: 5))
        app.typeText("Shakshuka")
        // The sidebar hides, so the canvas has room for two columns of steps.
        let sidebarToggle = app.windows.firstMatch.buttons
            .matching(NSPredicate(format: "label CONTAINS[c] 'sidebar'")).firstMatch
        XCTAssertTrue(sidebarToggle.waitForExistence(timeout: 5))
        sidebarToggle.click()
        let canvas = app.scrollViews["stepCanvas"]
        XCTAssertTrue(canvas.waitForExistence(timeout: 5))
        let add = app.menuItems["Add Step"]
        let nodes = app.descendants(matching: .any).matching(identifier: "stepNode")
        let node = { (title: String) in nodes.matching(NSPredicate(format: "label == %@", title)).firstMatch }
        for (title, offset) in [
            ("Dice onion", CGVector(dx: 0.04, dy: 0.06)),
            ("Mince garlic", CGVector(dx: 0.04, dy: 0.5)),
            ("Fry onion", CGVector(dx: 0.56, dy: 0.28)),
        ] {
            canvas.coordinate(withNormalizedOffset: offset).rightClick()
            XCTAssertTrue(add.waitForExistence(timeout: 5))
            add.click()
            XCTAssertTrue(app.textFields["stepTitleField"].waitForExistence(timeout: 5))
            app.typeText(title)
        }
        connect(node("Dice onion"), to: node("Fry onion"))
        connect(node("Mince garlic"), to: node("Fry onion"))
        node("Fry onion").click()
        let chips = app.staticTexts.matching(identifier: "stepAfterChip")
        XCTAssertTrue(chips.element(boundBy: 1).waitForExistence(timeout: 5))
        capture("01-connected", app)

        connect(node("Fry onion"), to: node("Dice onion"))
        XCTAssertTrue(app.staticTexts["connectRefusal"].waitForExistence(timeout: 5))
        capture("02-loop-refused", app)

        arrowMiddle(from: node("Mince garlic"), to: node("Fry onion")).rightClick()
        let delete = app.menuItems.matching(identifier: "deleteConnection").firstMatch
        XCTAssertTrue(delete.waitForExistence(timeout: 5))
        // The window and its menu only: the desktop around it stays out.
        captureScreen("03-connection-menu", around: app.windows.firstMatch)
        app.typeKey(.escape, modifierFlags: [])
        app.terminate()
    }

    /// REC-07: three recipe ingredients; basil dragged to the top; an empty
    /// line refused by Save; the read view; and Edit again, with the saved lines.
    @MainActor
    func testCaptureREC07Demo() throws {
        let app = try launch(json: nil, openIngredients: false)
        XCTAssertTrue(app.buttons["newRecipe"].waitForExistence(timeout: 5))
        app.typeKey("n", modifierFlags: .command)
        XCTAssertTrue(app.textFields["recipeTitleField"].waitForExistence(timeout: 5))
        app.typeText("Pasta")
        let canvas = app.scrollViews["stepCanvas"]
        XCTAssertTrue(canvas.waitForExistence(timeout: 5))
        canvas.coordinate(withNormalizedOffset: CGVector(dx: 0.08, dy: 0.08)).rightClick()
        let add = app.menuItems["Add Step"]
        XCTAssertTrue(add.waitForExistence(timeout: 5))
        add.click()
        XCTAssertTrue(app.textFields["stepTitleField"].waitForExistence(timeout: 5))
        app.typeText("Boil water")

        let name = { (index: Int) in app.textFields["recipeIngredientName.\(index)"] }
        for (index, line) in ["penne pasta", "olive oil", "basil"].enumerated() {
            app.buttons["addRecipeIngredient"].click()
            XCTAssertTrue(name(index).waitForExistence(timeout: 5))
            app.typeText(line)
            XCTAssertTrue(waitFor(name(index), value: line))
        }
        capture("01-lines", app)

        let handle = app.descendants(matching: .any)["recipeIngredientHandle.2"]
        handle.coordinate(withNormalizedOffset: CGVector(dx: 0.5, dy: 0.5))
            .click(forDuration: 0.2, thenDragTo: name(0).coordinate(withNormalizedOffset: CGVector(dx: 0.5, dy: 0.5)))
        XCTAssertTrue(waitFor(name(0), value: "basil"))
        capture("02-after-drag", app)

        app.buttons["addRecipeIngredient"].click()
        XCTAssertTrue(name(3).waitForExistence(timeout: 5))
        app.typeKey("s", modifierFlags: .command)
        XCTAssertTrue(app.staticTexts["recipeIngredientProblem.3"].waitForExistence(timeout: 5))
        capture("03-name-required", app)

        app.buttons["removeRecipeIngredient.3"].click()
        XCTAssertTrue(waitForNonExistence(name(3)))
        app.typeKey("s", modifierFlags: .command)
        let lines = app.staticTexts.matching(identifier: "recipeIngredient")
        XCTAssertTrue(waitFor(lines.element(boundBy: 0), value: "basil"))
        capture("04-read-view", app)

        app.buttons["editRecipe"].click()
        XCTAssertTrue(waitFor(name(0), value: "basil"))
        capture("05-edit-again", app)
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

    /// A screenshot of the screen cut to `element`'s frame. A window
    /// screenshot leaves out an open menu, which is a window of its own.
    @MainActor
    private func captureScreen(_ name: String, around element: XCUIElement) {
        let shot = XCUIScreen.main.screenshot()
        let frame = element.frame
        guard let image = shot.image.cgImage(forProposedRect: nil, context: nil, hints: nil) else {
            return XCTFail("The screenshot has no bitmap.")
        }
        // The element's frame is in screen points; the bitmap is in pixels.
        guard let screen = NSScreen.main?.frame, screen.width > 0 else {
            return XCTFail("There is no main screen.")
        }
        let scale = CGFloat(image.width) / screen.width
        let pixels = CGRect(
            x: frame.minX * scale, y: frame.minY * scale,
            width: frame.width * scale, height: frame.height * scale
        )
        guard let cropped = image.cropping(to: pixels) else {
            return XCTFail("The window lies outside the screenshot.")
        }
        let attachment = XCTAttachment(image: NSImage(cgImage: cropped, size: frame.size))
        attachment.name = name
        attachment.lifetime = .keepAlways
        add(attachment)
    }
}
