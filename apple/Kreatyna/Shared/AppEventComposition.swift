import Foundation

// Parents wrap child events with the scope IDs of the view they displayed.
// Rust checks these IDs; Swift never creates or reuses them.

extension RunningAppView {
    func ingredients(_ event: IngredientCatalogEvent) -> AppEvent {
        .ingredients(event: event)
    }

    func splitIngredients(_ event: IngredientSplitCatalogEvent) -> AppEvent {
        .splitIngredients(event: event)
    }
}

extension IngredientsView {
    /// The catalog of the stack layout (iOS), or `nil` in the split layout.
    var stack: IngredientCatalogView? {
        if case .stack(let catalog) = self { catalog } else { nil }
    }

    /// The catalog of the split layout (Mac), or `nil` in the stack layout.
    var split: IngredientSplitCatalogView? {
        if case .split(let catalog) = self { catalog } else { nil }
    }
}

// Each part of the split view carries its own scope. The list keeps one for
// its whole life; the pane gets a new one for each selection.
extension IngredientSplitCatalogView {
    func list(_ event: IngredientListEvent) -> IngredientSplitCatalogEvent {
        .list(scope: list.scope, event: event)
    }

    func detail(_ event: IngredientDetailEvent) -> IngredientSplitCatalogEvent {
        .detail(scope: pane.scope, event: event)
    }

    var clearSelection: IngredientSplitCatalogEvent {
        .clearSelection(scope: pane.scope)
    }
}

extension IngredientCatalogView {
    func list(_ event: IngredientListEvent) -> IngredientCatalogEvent {
        .list(scope: scope, event: event)
    }

    func detail(_ event: IngredientDetailEvent) -> IngredientCatalogEvent {
        .detail(scope: scope, event: event)
    }

    func form(_ event: IngredientFormEvent) -> IngredientCatalogEvent {
        .form(scope: scope, event: event)
    }

    var back: IngredientCatalogEvent {
        .back(scope: scope)
    }
}

extension IngredientFormEvent {
    /// Converts SwiftUI's `onMove` destination, which counts the moved row's
    /// original position, to the final index that Rust expects.
    static func move(from source: Int, toOffset destination: Int) -> IngredientFormEvent? {
        let target = destination > source ? destination - 1 : destination
        guard source != target, source >= 0, target >= 0 else { return nil }
        return .moveUnit(from: UInt64(source), to: UInt64(target))
    }
}
