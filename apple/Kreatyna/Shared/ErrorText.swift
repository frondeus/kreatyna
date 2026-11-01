import Foundation

// Rust reports typed errors; Swift owns only their user-facing text.

extension AppInitializationError {
    /// The failure screen's title: a stop while running is not a failed start.
    var title: String {
        switch self {
        case .databaseStopped: "Kreatyna stopped working"
        case .databaseOpen, .databaseConfiguration, .databaseMigration: "Kreatyna could not start"
        }
    }

    var message: String {
        switch self {
        case .databaseOpen: "The database could not be opened."
        case .databaseConfiguration: "The database could not be configured."
        case .databaseMigration: "The database could not be updated."
        case .databaseStopped: "The database stopped. Retry starts it again; unsaved changes are lost."
        }
    }
}

/// The screen after a bug stopped the Rust core (`AppView.crashed`).
enum CrashText {
    static let title = "Kreatyna stopped working"
    static let message = "Something went wrong inside Kreatyna. Quit and open it again."
}

extension IngredientLoadError {
    var message: String {
        switch self {
        case .storage: "Ingredients could not be read from storage."
        case .transport: "Ingredients are temporarily unavailable."
        }
    }
}

extension RecipeLoadError {
    var message: String {
        switch self {
        case .storage: "Recipes could not be read from storage."
        case .transport: "Recipes are temporarily unavailable."
        }
    }

    /// The read view loads one recipe, so its text is singular.
    var detailMessage: String {
        switch self {
        case .storage: "This recipe could not be read from storage."
        case .transport: "This recipe is temporarily unavailable."
        }
    }
}

extension RecipeSaveError {
    var message: String {
        switch self {
        case .duplicateTitle: "A recipe with this title already exists."
        case .storage: "The recipe could not be saved."
        case .transport: "Recipes are temporarily unavailable."
        }
    }
}

extension RecipeTitleError {
    var message: String {
        switch self {
        case .notEmptyViolated: "Enter a title."
        }
    }
}

extension RecipeEditorProblem {
    var message: String {
        switch self {
        case .invalid(let error): error.message
        case .saveFailed(let error): error.message
        }
    }
}

extension IngredientSaveError {
    var message: String {
        switch self {
        case .duplicateName: "An ingredient with this name already exists."
        case .notFound: "This ingredient no longer exists."
        case .storage: "The ingredient could not be saved to storage."
        case .transport: "The ingredient could not be saved right now."
        }
    }
}

extension IngredientDeleteError {
    var message: String {
        switch self {
        case .storage: "The ingredient could not be deleted from storage."
        case .transport: "The ingredient could not be deleted right now."
        }
    }
}

extension ValidationReason {
    var message: String {
        switch self {
        case .required: "Required."
        case .invalidNumber: "Enter a number."
        case .tooManyDecimalPlaces: "Too many decimal places."
        case .outOfRange: "This number is too large."
        case .mustBeNonNegative: "Must be zero or more."
        case .mustBePositive: "Must be more than zero."
        case .duplicateName: "This name is already used."
        }
    }
}

extension NutritionBasis {
    var label: String {
        switch self {
        case .per100Grams: "per 100 g"
        case .per100Millilitres: "per 100 ml"
        }
    }
}
