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
        XCTAssertFalse(app.buttons["addIngredient"].isEnabled)
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
    func testLaunchShowsEmptyCatalog() throws {
        let app = try launch(seed: [])

        XCTAssertTrue(app.staticTexts["No ingredients yet"].waitForExistence(timeout: 5))
    }

    @MainActor
    private func launch(seed: [String]) throws -> XCUIApplication {
        // The seed is a JSON array of names (E-17).
        try launch(json: seed.isEmpty ? nil : String(decoding: JSONEncoder().encode(seed), as: UTF8.self))
    }

    @MainActor
    private func launch(json: String?) throws -> XCUIApplication {
        let app = XCUIApplication()
        app.launchEnvironment["KREATYNA_TEST_DATABASE_ID"] = UUID().uuidString
        if let json {
            app.launchEnvironment["KREATYNA_TEST_SEED"] = json
        }
        app.launch()
        return app
    }

    private func waitForNonExistence(_ element: XCUIElement, timeout: TimeInterval) -> Bool {
        let gone = XCTNSPredicateExpectation(predicate: NSPredicate(format: "exists == false"), object: element)
        return XCTWaiter().wait(for: [gone], timeout: timeout) == .completed
    }

    @MainActor
    private func row(named name: String, in app: XCUIApplication) -> XCUIElement {
        app.staticTexts.matching(identifier: "ingredientRow")
            .matching(NSPredicate(format: "value == %@ OR label == %@", name, name))
            .firstMatch
    }
}
