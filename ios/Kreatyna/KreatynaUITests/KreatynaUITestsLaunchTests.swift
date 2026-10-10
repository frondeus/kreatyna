//
//  KreatynaUITestsLaunchTests.swift
//  KreatynaUITests
//
//  Created by Wojciech  Polak on 13/09/2026.
//

import XCTest

final class KreatynaUITestsLaunchTests: XCTestCase {

    override class var runsForEachTargetApplicationUIConfiguration: Bool {
        true
    }

    override func setUpWithError() throws {
        continueAfterFailure = false
    }

    @MainActor
    func testLaunch() throws {
        let app = XCUIApplication()
        app.launchEnvironment["KREATYNA_TEST_DATABASE_ID"] = UUID().uuidString
        app.launch()

        XCTAssertTrue(app.buttons["addIngredient"].waitForExistence(timeout: 5))

        let attachment = XCTAttachment(screenshot: app.screenshot())
        attachment.name = "Launch Screen"
        attachment.lifetime = .keepAlways
        add(attachment)
    }
}
