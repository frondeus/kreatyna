import XCTest

/// The Recipes section (REC-01 onward).
final class MacRecipeUITests: MacUITestCase {

    /// New Recipe, a title, Save: the row shows, its read view opens (REC-03),
    /// and the row stays after a relaunch (REC-02).
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
        XCTAssertTrue(waitForTitle(in: app, "Shakshuka"))
        XCTAssertTrue(app.buttons["newRecipe"].isEnabled)

        app.terminate()
        app = try launch(json: nil, database: database, openIngredients: false)
        XCTAssertTrue(recipeRow(named: "Shakshuka", in: app).waitForExistence(timeout: 5))
    }

    /// A title that differs only in case is refused; Cancel then asks first,
    /// and Discard goes back to the recipe that was open (REC-02, REC-03).
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
        XCTAssertTrue(waitForTitle(in: app, "Shakshuka"))
        XCTAssertEqual(app.staticTexts.matching(identifier: "recipeRow").count, 1)
    }

    /// A row click opens that recipe's read view and highlights the row (REC-03).
    @MainActor
    func testOpenRecipeShowsItsReadView() throws {
        let app = try launch(json: nil, openIngredients: false)
        XCTAssertTrue(app.buttons["newRecipe"].waitForExistence(timeout: 5))
        create("Shakshuka", in: app)
        create("Pasta", in: app)
        XCTAssertTrue(waitForTitle(in: app, "Pasta"))

        recipeRow(named: "Shakshuka", in: app).click()
        XCTAssertTrue(waitForTitle(in: app, "Shakshuka"))
        XCTAssertEqual(app.staticTexts.matching(NSPredicate(format: "value == 'None yet'")).count, 2)
        XCTAssertTrue(app.buttons["editRecipe"].isEnabled)
    }

    /// Delete, Cancel: the recipe stays. Delete, Delete: the pane empties,
    /// the row goes, and it stays gone after a relaunch (REC-05).
    @MainActor
    func testDeletedRecipeStaysGoneAfterRelaunch() throws {
        let database = UUID().uuidString
        var app = try launch(json: nil, database: database, openIngredients: false)
        XCTAssertTrue(app.buttons["newRecipe"].waitForExistence(timeout: 5))
        create("Shakshuka", in: app)

        app.buttons["deleteRecipe"].click()
        XCTAssertTrue(app.buttons["cancelDelete"].waitForExistence(timeout: 5))
        app.buttons["cancelDelete"].click()
        XCTAssertTrue(waitForNonExistence(app.buttons["cancelDelete"], timeout: 5))
        XCTAssertTrue(waitForTitle(in: app, "Shakshuka"))

        app.buttons["deleteRecipe"].click()
        XCTAssertTrue(app.buttons["confirmDelete"].waitForExistence(timeout: 5))
        app.buttons["confirmDelete"].click()
        XCTAssertTrue(app.descendants(matching: .any)["recipeDetailPlaceholder"].waitForExistence(timeout: 5))
        XCTAssertTrue(app.descendants(matching: .any)["recipeListEmpty"].waitForExistence(timeout: 5))

        app.terminate()
        app = try launch(json: nil, database: database, openIngredients: false)
        XCTAssertTrue(app.descendants(matching: .any)["recipeListEmpty"].waitForExistence(timeout: 5))
    }

    /// "sha" leaves only Shakshuka, and the open Pasta stays in the pane;
    /// "shax" shows "No matches"; Esc brings both rows back (REC-06).
    @MainActor
    func testSearchNarrowsRecipes() throws {
        let app = try launch(json: nil, openIngredients: false)
        XCTAssertTrue(app.buttons["newRecipe"].waitForExistence(timeout: 5))
        create("Shakshuka", in: app)
        create("Pasta", in: app)

        // The search field has no ⌘F (MAC-08), so the test clicks it.
        app.searchFields["recipeSearch"].click()
        app.typeText("sha")
        XCTAssertTrue(waitForNonExistence(recipeRow(named: "Pasta", in: app), timeout: 5))
        XCTAssertTrue(recipeRow(named: "Shakshuka", in: app).exists)
        XCTAssertTrue(waitForTitle(in: app, "Pasta"))

        app.typeText("x")
        XCTAssertTrue(app.descendants(matching: .any)["recipeListNoMatches"].waitForExistence(timeout: 5))

        app.typeKey(.escape, modifierFlags: [])
        XCTAssertTrue(recipeRow(named: "Pasta", in: app).waitForExistence(timeout: 5))
        XCTAssertTrue(recipeRow(named: "Shakshuka", in: app).exists)
    }

    /// Edit, a new title, ⌘S: the read view and the row show it (REC-04).
    /// The heading and the row keep the saved title until Save (MAC-06/Q1).
    @MainActor
    func testRenameRecipe() throws {
        let app = try launch(json: nil, openIngredients: false)
        XCTAssertTrue(app.buttons["newRecipe"].waitForExistence(timeout: 5))
        create("Shakshuka", in: app)

        retitle(to: "Spicy Shakshuka", in: app)
        XCTAssertTrue(waitFor(app.staticTexts["recipeEditorTitle"], value: "Shakshuka"))
        XCTAssertTrue(recipeRow(named: "Shakshuka", in: app).exists)
        XCTAssertFalse(app.buttons["newRecipe"].isEnabled)
        app.typeKey("s", modifierFlags: .command)

        XCTAssertTrue(waitForTitle(in: app, "Spicy Shakshuka"))
        XCTAssertTrue(recipeRow(named: "Spicy Shakshuka", in: app).waitForExistence(timeout: 5))
        XCTAssertEqual(app.staticTexts.matching(identifier: "recipeRow").count, 1)
        XCTAssertTrue(app.buttons["newRecipe"].isEnabled)
    }

    /// Edit, a changed title, Esc: "Discard changes?"; Discard goes back to
    /// the read view with the saved title (REC-04).
    @MainActor
    func testCancelEditAsksToDiscard() throws {
        let app = try launch(json: nil, openIngredients: false)
        XCTAssertTrue(app.buttons["newRecipe"].waitForExistence(timeout: 5))
        create("Shakshuka", in: app)

        retitle(to: "Toast", in: app)
        app.typeKey(.escape, modifierFlags: [])
        let discard = app.buttons["confirmDiscard"]
        XCTAssertTrue(discard.waitForExistence(timeout: 5))
        discard.click()

        XCTAssertTrue(waitForTitle(in: app, "Shakshuka"))
        XCTAssertTrue(recipeRow(named: "Shakshuka", in: app).exists)
    }

    /// A row click while a typed title is open asks "Discard changes?";
    /// Discard opens the clicked recipe, not the one from before (REC-03/Q2).
    @MainActor
    func testRowClickAsksToDiscardTypedTitle() throws {
        let app = try launch(json: nil, openIngredients: false)
        XCTAssertTrue(app.buttons["newRecipe"].waitForExistence(timeout: 5))
        create("Shakshuka", in: app)
        create("Pasta", in: app)

        app.typeKey("n", modifierFlags: .command)
        XCTAssertTrue(recipeDraftRow(in: app).waitForExistence(timeout: 5))
        app.typeText("Toast")
        XCTAssertTrue(waitFor(recipeDraftRow(in: app), value: "Toast"))
        recipeRow(named: "Shakshuka", in: app).click()
        let discard = app.buttons["confirmDiscard"]
        XCTAssertTrue(discard.waitForExistence(timeout: 5))
        discard.click()
        XCTAssertTrue(waitForTitle(in: app, "Shakshuka"))
        XCTAssertTrue(waitForNonExistence(recipeDraftRow(in: app), timeout: 5))
    }

    /// Right-click ▸ Add Step adds an untitled step and selects it. Save
    /// refuses it and shows the problem (REC-14/Q2). With a title, Save
    /// keeps it: the read view lists it and the row says "1 step" (REC-14).
    @MainActor
    func testAddStepAndSave() throws {
        let app = try launch(json: nil, openIngredients: false)
        XCTAssertTrue(app.buttons["newRecipe"].waitForExistence(timeout: 5))
        app.typeKey("n", modifierFlags: .command)
        XCTAssertTrue(recipeDraftRow(in: app).waitForExistence(timeout: 5))
        app.typeText("Shakshuka")

        addStep(at: CGVector(dx: 0.2, dy: 0.2), in: app)
        let node = app.descendants(matching: .any).matching(identifier: "stepNode").firstMatch
        XCTAssertTrue(node.waitForExistence(timeout: 5))
        XCTAssertTrue(app.textFields["stepTitleField"].waitForExistence(timeout: 5))

        app.typeKey("s", modifierFlags: .command)
        let problem = app.staticTexts["stepTitleProblem"]
        XCTAssertTrue(problem.waitForExistence(timeout: 5))
        XCTAssertTrue(waitFor(problem, value: "A step needs a title."))

        // The new step's Title has the focus.
        app.typeText("Dice onion")
        XCTAssertTrue(waitForNonExistence(problem, timeout: 5))
        app.typeKey("s", modifierFlags: .command)

        XCTAssertTrue(waitForTitle(in: app, "Shakshuka"))
        XCTAssertTrue(waitFor(app.staticTexts["recipeStep"], value: "Dice onion"))
        XCTAssertTrue(waitFor(app.staticTexts["recipeStepCount"], value: "1 step"))
    }

    /// Edit on a recipe with steps opens the canvas with them and no
    /// selection. A click on a node selects it and the inspector shows its
    /// Title; a click on empty canvas clears it. A renamed step is saved
    /// (REC-14).
    @MainActor
    func testSelectStepsOfSavedRecipe() throws {
        let app = try launch(json: nil, openIngredients: false)
        XCTAssertTrue(app.buttons["newRecipe"].waitForExistence(timeout: 5))
        app.typeKey("n", modifierFlags: .command)
        XCTAssertTrue(recipeDraftRow(in: app).waitForExistence(timeout: 5))
        app.typeText("Shakshuka")
        addStep(at: CGVector(dx: 0.1, dy: 0.1), in: app)
        XCTAssertTrue(app.textFields["stepTitleField"].waitForExistence(timeout: 5))
        app.typeText("Dice onion")
        addStep(at: CGVector(dx: 0.5, dy: 0.5), in: app)
        _ = stepNode("Untitled step", in: app)
        app.typeText("Fry onion")
        app.typeKey("s", modifierFlags: .command)
        XCTAssertTrue(waitForTitle(in: app, "Shakshuka"))
        XCTAssertTrue(waitFor(app.staticTexts["recipeStepCount"], value: "2 steps"))

        app.buttons["editRecipe"].click()
        let hint = app.staticTexts["selectStepHint"]
        XCTAssertTrue(hint.waitForExistence(timeout: 5))
        XCTAssertEqual(stepNodes(in: app).count, 2)

        let title = app.textFields["stepTitleField"]
        stepNode("Dice onion", in: app).click()
        XCTAssertTrue(waitFor(title, value: "Dice onion"))
        stepNode("Fry onion", in: app).click()
        XCTAssertTrue(waitFor(title, value: "Fry onion"))

        app.scrollViews["stepCanvas"].coordinate(withNormalizedOffset: CGVector(dx: 0.05, dy: 0.9)).click()
        XCTAssertTrue(hint.waitForExistence(timeout: 5))
        XCTAssertTrue(waitForNonExistence(title, timeout: 5))

        stepNode("Fry onion", in: app).click()
        XCTAssertTrue(waitFor(title, value: "Fry onion"))
        app.typeKey("a", modifierFlags: .command)
        app.typeText("Fry onion and peppers")
        app.typeKey("s", modifierFlags: .command)
        XCTAssertTrue(waitForTitle(in: app, "Shakshuka"))
        let saved = app.staticTexts.matching(identifier: "recipeStep")
        XCTAssertTrue(waitFor(saved.element(boundBy: 0), value: "Dice onion"))
        XCTAssertTrue(waitFor(saved.element(boundBy: 1), value: "Fry onion and peppers"))
    }

    @MainActor
    private func stepNodes(in app: XCUIApplication) -> XCUIElementQuery {
        app.descendants(matching: .any).matching(identifier: "stepNode")
    }

    /// The node that shows `label`.
    @MainActor
    private func stepNode(_ label: String, in app: XCUIApplication) -> XCUIElement {
        let node = stepNodes(in: app).matching(NSPredicate(format: "label == %@", label)).firstMatch
        XCTAssertTrue(node.waitForExistence(timeout: 5))
        return node
    }

    /// Right-clicks the step canvas at `offset` (0 to 1 of its size) and picks Add Step.
    @MainActor
    private func addStep(at offset: CGVector, in app: XCUIApplication) {
        let canvas = app.scrollViews["stepCanvas"]
        XCTAssertTrue(canvas.waitForExistence(timeout: 5))
        canvas.coordinate(withNormalizedOffset: offset).rightClick()
        let add = app.menuItems["Add Step"]
        XCTAssertTrue(add.waitForExistence(timeout: 5))
        add.click()
    }

    /// ⌘N, `title`, ⌘S; waits until the saved recipe's read view shows.
    @MainActor
    private func create(_ title: String, in app: XCUIApplication) {
        app.typeKey("n", modifierFlags: .command)
        XCTAssertTrue(recipeDraftRow(in: app).waitForExistence(timeout: 5))
        app.typeText(title)
        app.typeKey("s", modifierFlags: .command)
        XCTAssertTrue(waitForTitle(in: app, title))
    }

    /// Edit on the open recipe, then replaces the whole title with `title`.
    @MainActor
    private func retitle(to title: String, in app: XCUIApplication) {
        app.buttons["editRecipe"].click()
        let field = app.textFields["recipeTitleField"]
        XCTAssertTrue(field.waitForExistence(timeout: 5))
        field.click()
        app.typeKey("a", modifierFlags: .command)
        app.typeText(title)
    }

    /// Waits until the read view shows `title`.
    @MainActor
    private func waitForTitle(in app: XCUIApplication, _ title: String) -> Bool {
        let shown = XCTNSPredicateExpectation(
            predicate: NSPredicate(format: "value == %@", title), object: app.staticTexts["recipeTitle"]
        )
        return XCTWaiter().wait(for: [shown], timeout: 5) == .completed
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
}
