import XCTest

/// The window shell: the sidebar and switching sections.
final class MacShellUITests: MacUITestCase {

    /// The app opens on Recipes. Switching sections keeps each section's state (REC-01).
    @MainActor
    func testSidebarSwitchesSections() throws {
        let app = try launch(seed: ["Garlic", "Tomato"], openIngredients: false)

        XCTAssertTrue(app.descendants(matching: .any)["recipeListEmpty"].waitForExistence(timeout: 5))
        XCTAssertTrue(app.buttons["newRecipe"].isEnabled)

        sidebar("sidebarIngredients", in: app).click()
        let garlic = app.staticTexts.matching(identifier: "ingredientRow")["Garlic"]
        XCTAssertTrue(garlic.waitForExistence(timeout: 5))
        garlic.click()
        XCTAssertTrue(app.buttons["editIngredient"].waitForExistence(timeout: 5))

        sidebar("sidebarRecipes", in: app).click()
        XCTAssertTrue(app.descendants(matching: .any)["recipeDetailPlaceholder"].waitForExistence(timeout: 5))
        XCTAssertFalse(app.buttons["editIngredient"].exists)

        sidebar("sidebarIngredients", in: app).click()
        // Rust kept Garlic open while Recipes showed.
        XCTAssertTrue(app.buttons["editIngredient"].waitForExistence(timeout: 5))
    }
}
