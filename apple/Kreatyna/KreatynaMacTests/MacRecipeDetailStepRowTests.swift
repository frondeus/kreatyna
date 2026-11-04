import Testing
@testable import Kreatyna

/// The read view writes the duration and "waiting" after a step's title
/// (REC-17/Q2). The generated types compare on the main actor.
@MainActor
struct MacRecipeDetailStepRowTests {
    @Test(arguments: [
        (Fixtures.detailStep("Boil water", duration: "10 min", kind: .waiting), "10 min · waiting"),
        (Fixtures.detailStep("Sear scallops", duration: "1 min 30 s"), "1 min 30 s"),
        (Fixtures.detailStep("Rest", kind: .waiting), "waiting"),
        (Fixtures.detailStep("Plate"), nil),
    ])
    func detailsShowTheDurationAndWaiting(step: RecipeDetailStep, expected: String?) {
        #expect(MacRecipeDetailStepRow.details(step) == expected)
    }
}
