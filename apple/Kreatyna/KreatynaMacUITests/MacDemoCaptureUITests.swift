import XCTest

/// Captures the Mac window screenshots for `docs/epics/mac-shell/tasks/MAC-01-demo/demo.md`.
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
