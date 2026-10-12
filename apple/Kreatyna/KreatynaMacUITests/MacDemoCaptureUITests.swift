import XCTest

/// Captures the Mac window screenshots for the Mac demos, one test method per demo.
///
/// This is delivery evidence, not a behavior check. It runs only on request, through
/// `.agents/skills/document-apple-delivery/scripts/capture-demo.sh mac KreatynaMacUITests/MacDemoCaptureUITests <folder>`.
/// Screenshots are attached to the result bundle with their demo file names.
final class MacDemoCaptureUITests: XCTestCase {

    override func setUpWithError() throws {
        try XCTSkipUnless(
            ProcessInfo.processInfo.environment["KREATYNA_CAPTURE_DEMO"] == "1",
            "Set TEST_RUNNER_KREATYNA_CAPTURE_DEMO=1 to capture the demo."
        )
        continueAfterFailure = false
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
        let fixture = URL(fileURLWithPath: #filePath)
            .deletingLastPathComponent().deletingLastPathComponent()
            .appendingPathComponent("Fixtures/demo-ingredients.json")
        let app = XCUIApplication()
        app.launchEnvironment["KREATYNA_TEST_DATABASE_ID"] = UUID().uuidString
        app.launchEnvironment["KREATYNA_TEST_SEED"] = try String(contentsOf: fixture, encoding: .utf8)
        app.launch()

        let garlic = app.staticTexts.matching(identifier: "ingredientRow")["Garlic"]
        XCTAssertTrue(garlic.waitForExistence(timeout: 5))
        garlic.click()
        XCTAssertTrue(app.staticTexts["clove"].waitForExistence(timeout: 5))
        capture("01-garlic", app)

        app.staticTexts.matching(identifier: "ingredientRow")["Basil"].click()
        let clove = XCTNSPredicateExpectation(predicate: NSPredicate(format: "exists == false"), object: app.staticTexts["clove"])
        XCTAssertEqual(XCTWaiter().wait(for: [clove], timeout: 5), .completed)
        capture("02-basil", app)
        app.terminate()
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
    private func capture(_ name: String, _ app: XCUIApplication) {
        let attachment = XCTAttachment(screenshot: app.windows.firstMatch.screenshot())
        attachment.name = name
        attachment.lifetime = .keepAlways
        add(attachment)
    }
}
