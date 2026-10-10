import Foundation

// Rust reports typed errors; Swift owns only their user-facing text.

extension AppInitializationError {
    var message: String {
        switch self {
        case .databaseOpen: "The database could not be opened."
        case .databaseConfiguration: "The database could not be configured."
        case .databaseMigration: "The database could not be updated."
        }
    }
}

extension IngredientLoadError {
    var message: String {
        switch self {
        case .storage: "Ingredients could not be read from storage."
        case .transport: "Ingredients are temporarily unavailable."
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
