//
//  KreatynaUITests.swift
//  KreatynaUITests
//
//  Created by Wojciech  Polak on 13/09/2026.
//

import XCTest

// Smoke flows over the real Rust core. They prove that UI events reach Rust and
// that its views reach the screen. Rust tests own the behavior rules.
final class KreatynaUITests: XCTestCase {

    override func setUpWithError() throws {
        continueAfterFailure = false
    }

    @MainActor
    func testCreateEditAndDeleteIngredient() throws {
        let app = launch(databaseId: UUID())

        createIngredient(named: "Garlic", in: app)
        row(named: "Garlic", in: app).tap()
        XCTAssertTrue(app.navigationBars["Garlic"].waitForExistence(timeout: 5))

        app.buttons["editIngredient"].tap()
        let name = app.textFields["ingredientNameField"]
        XCTAssertTrue(name.waitForExistence(timeout: 5))
        // The field is trailing-aligned; tap its end to append.
        name.coordinate(withNormalizedOffset: CGVector(dx: 0.98, dy: 0.5)).tap()
        name.typeText(" bulb")
        app.buttons["saveIngredient"].tap()
        XCTAssertTrue(app.navigationBars["Garlic bulb"].waitForExistence(timeout: 5))

        app.buttons["deleteIngredient"].tap()
        let confirm = app.buttons["confirmDelete"].firstMatch
        XCTAssertTrue(confirm.waitForExistence(timeout: 5))
        confirm.tap()
        XCTAssertTrue(app.staticTexts["No ingredients yet"].waitForExistence(timeout: 5))
    }

    @MainActor
    func testIngredientPersistsAcrossRelaunch() throws {
        let databaseId = UUID()
        let app = launch(databaseId: databaseId)
        createIngredient(named: "Cucumber", in: app)
        app.terminate()

        let relaunched = launch(databaseId: databaseId)
        XCTAssertTrue(row(named: "Cucumber", in: relaunched).waitForExistence(timeout: 5))
    }

    @MainActor
    func testTabsKeepTheirState() throws {
        let app = launch(databaseId: UUID())
        createIngredient(named: "Basil", in: app)

        app.tabBars.buttons["Counter"].tap()
        let value = app.staticTexts["counterValue"]
        XCTAssertTrue(value.wait(for: \.label, toEqual: "Counter: 0", timeout: 5))
        app.buttons["increaseCounter"].tap()
        XCTAssertTrue(value.wait(for: \.label, toEqual: "Counter: 1", timeout: 5))

        app.tabBars.buttons["Ingredients"].tap()
        XCTAssertTrue(row(named: "Basil", in: app).waitForExistence(timeout: 5))
    }

    @MainActor
    func testLaunchPerformance() throws {
        measure(metrics: [XCTApplicationLaunchMetric()]) {
            XCUIApplication().launch()
        }
    }

    @MainActor
    private func launch(databaseId: UUID) -> XCUIApplication {
        let app = XCUIApplication()
        app.launchEnvironment["KREATYNA_TEST_DATABASE_ID"] = databaseId.uuidString
        app.launch()
        XCTAssertTrue(app.buttons["addIngredient"].waitForExistence(timeout: 5))
        return app
    }

    @MainActor
    private func createIngredient(named name: String, in app: XCUIApplication) {
        app.buttons["addIngredient"].tap()
        let field = app.textFields["ingredientNameField"]
        XCTAssertTrue(field.waitForExistence(timeout: 5))
        field.tap()
        field.typeText(name)
        app.buttons["saveIngredient"].tap()
        XCTAssertTrue(row(named: name, in: app).waitForExistence(timeout: 5))
    }

    @MainActor
    private func row(named name: String, in app: XCUIApplication) -> XCUIElement {
        app.buttons.matching(identifier: "ingredientRow")
            .matching(NSPredicate(format: "label == %@", name))
            .firstMatch
    }
}
