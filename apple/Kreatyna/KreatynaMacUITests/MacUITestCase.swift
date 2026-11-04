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

    func waitForNonExistence(_ element: XCUIElement, timeout: TimeInterval = 5) -> Bool {
        let gone = XCTNSPredicateExpectation(predicate: NSPredicate(format: "exists == false"), object: element)
        return XCTWaiter().wait(for: [gone], timeout: timeout) == .completed
    }

    func waitFor(_ element: XCUIElement, value: String, timeout: TimeInterval = 5) -> Bool {
        let match = XCTNSPredicateExpectation(predicate: NSPredicate(format: "value == %@", value), object: element)
        return XCTWaiter().wait(for: [match], timeout: timeout) == .completed
    }

    /// Drags from the ● on the right edge of the node `from` and drops on
    /// the middle of the node `to` (REC-18).
    @MainActor
    func connect(_ from: XCUIElement, to: XCUIElement) {
        let port = from.coordinate(withNormalizedOffset: CGVector(dx: 1, dy: 0.5)).withOffset(CGVector(dx: 2, dy: 0))
        port.click(forDuration: 0.3, thenDragTo: to.coordinate(withNormalizedOffset: CGVector(dx: 0.5, dy: 0.5)))
    }

    /// The middle of the arrow from the node `from` to the node `to`. The
    /// curve is symmetric, so its middle is half way between its ends: the
    /// ● of `from`, and 6 points left of the ○ of `to`.
    @MainActor
    func arrowMiddle(from: XCUIElement, to: XCUIElement) -> XCUICoordinate {
        let start = from.coordinate(withNormalizedOffset: CGVector(dx: 1, dy: 0.5))
        let offset = CGVector(
            dx: (to.frame.minX - 6 - from.frame.maxX) / 2,
            dy: (to.frame.midY - from.frame.midY) / 2
        )
        return start.withOffset(offset)
    }
}
