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
    func testLaunchShowsEmptyCatalog() throws {
        let app = try launch(seed: [])

        XCTAssertTrue(app.staticTexts["No ingredients yet"].waitForExistence(timeout: 5))
    }

    @MainActor
    private func launch(seed: [String]) throws -> XCUIApplication {
        let app = XCUIApplication()
        app.launchEnvironment["KREATYNA_TEST_DATABASE_ID"] = UUID().uuidString
        if !seed.isEmpty {
            // The seed is a JSON array of names (E-17).
            let json = try JSONEncoder().encode(seed)
            app.launchEnvironment["KREATYNA_TEST_SEED"] = String(decoding: json, as: UTF8.self)
        }
        app.launch()
        return app
    }

    @MainActor
    private func row(named name: String, in app: XCUIApplication) -> XCUIElement {
        app.staticTexts.matching(identifier: "ingredientRow")
            .matching(NSPredicate(format: "value == %@ OR label == %@", name, name))
            .firstMatch
    }
}
