import XCTest

/// Captures the simulator screenshots for `docs/epics/ingredient-catalog/demo.md`.
///
/// This is delivery evidence, not a behavior check. It runs only on request, through
/// `.agents/skills/document-apple-delivery/scripts/capture-demo.sh ios KreatynaUITests/DemoCaptureUITests <folder>`.
/// Screenshots are attached to the result bundle with their demo file names.
final class DemoCaptureUITests: XCTestCase {

    override func setUpWithError() throws {
        try XCTSkipUnless(
            ProcessInfo.processInfo.environment["KREATYNA_CAPTURE_DEMO"] == "1",
            "Set TEST_RUNNER_KREATYNA_CAPTURE_DEMO=1 to capture the demo."
        )
        continueAfterFailure = false
    }

    @MainActor
    func testCaptureIngredientCatalogDemo() throws {
        let databaseId = UUID()
        var app = launch(databaseId: databaseId)
        XCTAssertTrue(app.staticTexts["No ingredients yet"].waitForExistence(timeout: 5))
        capture("01-empty-catalog", app)

        // Validation comes from Rust; Swift only shows the reasons.
        app.buttons["addIngredient"].tap()
        XCTAssertTrue(field("ingredientNameField", app).waitForExistence(timeout: 5))
        capture("02-create-form", app)
        app.buttons["saveIngredient"].tap()
        XCTAssertTrue(app.staticTexts["Required."].waitForExistence(timeout: 5))
        type("Garlic", into: field("ingredientNameField", app))
        type("1,2,3", into: field("ingredientCaloriesField", app))
        app.buttons["saveIngredient"].tap()
        XCTAssertTrue(app.staticTexts["Enter a number."].waitForExistence(timeout: 5))
        capture("03-validation", app)

        replace(field("ingredientCaloriesField", app), with: "149")
        app.buttons["addIngredientUnit"].tap()
        type("clove", into: field("ingredientUnitNameField.0", app))
        type("5", into: field("ingredientUnitGramsField.0", app))
        capture("04-filled-form", app)
        app.buttons["saveIngredient"].tap()
        XCTAssertTrue(row("Garlic", app).waitForExistence(timeout: 5))

        create("Cucumber", app)
        create("Canned tomatoes", app)
        capture("05-catalog", app)

        app.buttons["addIngredient"].tap()
        type("garlic", into: field("ingredientNameField", app))
        app.buttons["saveIngredient"].tap()
        XCTAssertTrue(app.descendants(matching: .any)["ingredientSaveError"].waitForExistence(timeout: 5))
        capture("06-duplicate-name", app)
        app.buttons["cancelIngredientForm"].tap()
        let discard = app.buttons["confirmDiscard"].firstMatch
        XCTAssertTrue(discard.waitForExistence(timeout: 5))
        capture("07-discard-confirmation", app)
        discard.tap()
        XCTAssertTrue(row("Garlic", app).waitForExistence(timeout: 5))

        let search = app.searchFields.firstMatch
        search.tap()
        search.typeText("cu")
        XCTAssertTrue(row("Cucumber", app).waitForExistence(timeout: 5))
        capture("08-search", app)
        search.typeText("zz")
        XCTAssertTrue(app.staticTexts["No Results for “cuzz”"].waitForExistence(timeout: 5))
        capture("09-search-no-matches", app)
        search.typeText(String(repeating: XCUIKeyboardKey.delete.rawValue, count: 4))
        XCTAssertTrue(row("Garlic", app).waitForExistence(timeout: 5))

        row("Garlic", app).tap()
        XCTAssertTrue(app.navigationBars["Garlic"].waitForExistence(timeout: 5))
        capture("10-detail", app)

        app.buttons["editIngredient"].tap()
        let name = field("ingredientNameField", app)
        XCTAssertTrue(name.waitForExistence(timeout: 5))
        name.coordinate(withNormalizedOffset: CGVector(dx: 0.98, dy: 0.5)).tap()
        name.typeText(" bulb")
        capture("11-edit-from-detail", app)
        app.buttons["saveIngredient"].tap()
        XCTAssertTrue(app.navigationBars["Garlic bulb"].waitForExistence(timeout: 5))
        capture("12-edited-detail", app)

        app.buttons["deleteIngredient"].tap()
        let confirm = app.buttons["confirmDelete"].firstMatch
        XCTAssertTrue(confirm.waitForExistence(timeout: 5))
        capture("13-delete-confirmation", app)
        confirm.tap()
        XCTAssertTrue(row("Cucumber", app).waitForExistence(timeout: 5))
        XCTAssertFalse(row("Garlic bulb", app).exists)
        capture("14-after-delete", app)

        app.terminate()
        app = launch(databaseId: databaseId)
        XCTAssertTrue(row("Cucumber", app).waitForExistence(timeout: 5))
        capture("15-after-relaunch", app)
    }

    // MARK: Support

    @MainActor
    private func launch(databaseId: UUID) -> XCUIApplication {
        let app = XCUIApplication()
        app.launchEnvironment["KREATYNA_TEST_DATABASE_ID"] = databaseId.uuidString
        app.launch()
        XCTAssertTrue(app.buttons["addIngredient"].waitForExistence(timeout: 5))
        return app
    }

    @MainActor
    private func create(_ name: String, _ app: XCUIApplication) {
        app.buttons["addIngredient"].tap()
        type(name, into: field("ingredientNameField", app))
        app.buttons["saveIngredient"].tap()
        XCTAssertTrue(row(name, app).waitForExistence(timeout: 5))
    }

    @MainActor
    private func field(_ identifier: String, _ app: XCUIApplication) -> XCUIElement {
        app.textFields[identifier]
    }

    @MainActor
    private func row(_ name: String, _ app: XCUIApplication) -> XCUIElement {
        app.buttons.matching(identifier: "ingredientRow")
            .matching(NSPredicate(format: "label == %@", name))
            .firstMatch
    }

    @MainActor
    private func type(_ text: String, into field: XCUIElement) {
        XCTAssertTrue(field.waitForExistence(timeout: 5))
        field.tap()
        field.typeText(text)
    }

    @MainActor
    private func replace(_ field: XCUIElement, with text: String) {
        XCTAssertTrue(field.waitForExistence(timeout: 5))
        field.coordinate(withNormalizedOffset: CGVector(dx: 0.98, dy: 0.5)).tap()
        // The field's value may be its placeholder, so delete generously.
        field.typeText(String(repeating: XCUIKeyboardKey.delete.rawValue, count: 12))
        field.typeText(text)
    }

    @MainActor
    private func capture(_ name: String, _ app: XCUIApplication) {
        let attachment = XCTAttachment(screenshot: app.screenshot())
        attachment.name = name
        attachment.lifetime = .keepAlways
        add(attachment)
    }
}
