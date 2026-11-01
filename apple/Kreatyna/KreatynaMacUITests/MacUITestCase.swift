import XCTest

/// Shared launch and wait helpers for the Mac UI tests. Each section has its
/// own test class, so `mac-test --ui <Class>` runs only the touched section's
/// flows (the full suite runs before each cooldown).
class MacUITestCase: XCTestCase {

    override func setUpWithError() throws {
        continueAfterFailure = false
    }

    @MainActor
    func launch(seed: [String], openIngredients: Bool = true) throws -> XCUIApplication {
        // The seed is a JSON array of names (E-17).
        try launch(
            json: seed.isEmpty ? nil : String(decoding: JSONEncoder().encode(seed), as: UTF8.self),
            openIngredients: openIngredients
        )
    }

    /// The app opens on Recipes (REC-01/Q1). The ingredient tests click
    /// Ingredients first, unless `openIngredients` is false.
    @MainActor
    func launch(
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
    func sidebar(_ identifier: String, in app: XCUIApplication) -> XCUIElement {
        app.descendants(matching: .any).matching(identifier: identifier).firstMatch
    }

    func waitForNonExistence(_ element: XCUIElement, timeout: TimeInterval) -> Bool {
        let gone = XCTNSPredicateExpectation(predicate: NSPredicate(format: "exists == false"), object: element)
        return XCTWaiter().wait(for: [gone], timeout: timeout) == .completed
    }

    func waitFor(_ element: XCUIElement, value: String, timeout: TimeInterval = 5) -> Bool {
        let match = XCTNSPredicateExpectation(predicate: NSPredicate(format: "value == %@", value), object: element)
        return XCTWaiter().wait(for: [match], timeout: timeout) == .completed
    }
}
