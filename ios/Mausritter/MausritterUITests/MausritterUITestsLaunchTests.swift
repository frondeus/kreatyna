//
//  MausritterUITestsLaunchTests.swift
//  MausritterUITests
//
//  Created by Wojciech  Polak on 13/09/2026.
//

import XCTest

final class MausritterUITestsLaunchTests: XCTestCase {

    override class var runsForEachTargetApplicationUIConfiguration: Bool {
        true
    }

    override func setUpWithError() throws {
        continueAfterFailure = false
    }

    @MainActor
    func testLaunch() throws {
        let app = XCUIApplication()
        app.launchEnvironment["MAUSRITTER_TEST_DATABASE_ID"] = UUID().uuidString
        app.launch()

        let value = app.staticTexts["counterValue"]
        XCTAssertTrue(value.waitForExistence(timeout: 5))
        XCTAssertTrue(value.wait(for: \.label, toEqual: "Counter: 0", timeout: 5))

        let attachment = XCTAttachment(screenshot: app.screenshot())
        attachment.name = "Launch Screen"
        attachment.lifetime = .keepAlways
        add(attachment)
    }
}
