import XCTest

/// The Recipes section (REC-01 onward).
final class MacRecipeUITests: MacUITestCase {

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
