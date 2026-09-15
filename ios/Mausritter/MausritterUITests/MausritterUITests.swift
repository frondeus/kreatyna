//
//  MausritterUITests.swift
//  MausritterUITests
//
//  Created by Wojciech  Polak on 13/09/2026.
//

import XCTest

final class MausritterUITests: XCTestCase {

    override func setUpWithError() throws {
        // Put setup code here. This method is called before the invocation of each test method in the class.

        // In UI tests it is usually best to stop immediately when a failure occurs.
        continueAfterFailure = false

        // In UI tests it’s important to set the initial state - such as interface orientation - required for your tests before they run. The setUp method is a good place to do this.
    }

    override func tearDownWithError() throws {
        // Put teardown code here. This method is called after the invocation of each test method in the class.
    }

    @MainActor
    func testDisplaysInitialRustCounter() throws {
        // UI tests must launch the application that they test.
        let app = XCUIApplication()
        app.launchEnvironment["MAUSRITTER_TEST_DATABASE_ID"] = UUID().uuidString
        app.launch()

        let value = app.staticTexts["counterValue"]
        XCTAssertTrue(value.waitForExistence(timeout: 5))
        XCTAssertTrue(value.wait(for: \.label, toEqual: "Counter: 0", timeout: 5))
    }

    @MainActor
    func testCounterButtons() throws {
        let app = XCUIApplication()
        app.launchEnvironment["MAUSRITTER_TEST_DATABASE_ID"] = UUID().uuidString
        app.launch()
        let value = app.staticTexts["counterValue"]
        XCTAssertTrue(value.waitForExistence(timeout: 5))
        XCTAssertTrue(value.wait(for: \.label, toEqual: "Counter: 0", timeout: 5))

        app.buttons["increaseCounter"].tap()
        XCTAssertTrue(value.wait(for: \.label, toEqual: "Counter: 1", timeout: 5))
        app.buttons["decreaseCounter"].tap()
        XCTAssertTrue(value.wait(for: \.label, toEqual: "Counter: 0", timeout: 5))
    }

    @MainActor
    func testBackgroundCounterCanStopAndRestart() throws {
        let app = XCUIApplication()
        app.launchEnvironment["MAUSRITTER_TEST_DATABASE_ID"] = UUID().uuidString
        app.launch()
        let value = app.staticTexts["counterValue"]
        XCTAssertTrue(value.waitForExistence(timeout: 5))
        XCTAssertTrue(value.wait(for: \.label, toEqual: "Counter: 0", timeout: 5))
        app.buttons["startBackground"].tap()
        XCTAssertTrue(value.wait(for: \.label, toEqual: "Counter: 1", timeout: 5))
        app.buttons["stopBackground"].tap()
        let stoppedValue = value.label
        let changes = XCTNSPredicateExpectation(
            predicate: NSPredicate(format: "label != %@", stoppedValue),
            object: value
        )
        changes.isInverted = true
        wait(for: [changes], timeout: 2)
        app.buttons["startBackground"].tap()
        let resumes = XCTNSPredicateExpectation(
            predicate: NSPredicate(format: "label != %@", stoppedValue),
            object: value
        )
        wait(for: [resumes], timeout: 5)
        app.buttons["stopBackground"].tap()
    }

    @MainActor
    func testLaunchPerformance() throws {
        // This measures how long it takes to launch your application.
        measure(metrics: [XCTApplicationLaunchMetric()]) {
            XCUIApplication().launch()
        }
    }
}
