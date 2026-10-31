import Foundation

extension ReferenceUnit {
    /// The unit's measurements as shown in both apps, for example "5 g" or "14 g · 15 ml".
    var measurementText: String {
        [grams.map { "\($0) g" }, millilitres.map { "\($0) ml" }]
            .compactMap { $0 }
            .joined(separator: " · ")
    }
}

extension IngredientDraftField {
    /// The form field's identifier for UI tests, the same on both apps.
    var accessibilityIdentifier: String {
        switch self {
        case .name: "ingredientNameField"
        case .notes: "ingredientNotesField"
        case .calories: "ingredientCaloriesField"
        case .protein: "ingredientProteinField"
        case .carbohydrates: "ingredientCarbohydratesField"
        case .fat: "ingredientFatField"
        case .unitName(let index): "ingredientUnitNameField.\(index)"
        case .unitGrams(let index): "ingredientUnitGramsField.\(index)"
        case .unitMillilitres(let index): "ingredientUnitMillilitresField.\(index)"
        }
    }
}

/// The delete dialog's words, the same on both apps. Rust decides when the
/// dialog shows and which error it shows (MAC-07/Q1).
enum DeleteText {
    static func title(_ name: String) -> String { "Delete \(name)?" }

    static func message(_ error: IngredientDeleteError?) -> String {
        error?.message ?? "This cannot be undone."
    }
}
