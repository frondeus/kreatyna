import Foundation

// Parents wrap child events with the scope IDs of the view they displayed.
// Rust checks these IDs; Swift never creates or reuses them.

extension RunningAppView {
    func ingredients(_ event: IngredientCatalogEvent) -> AppEvent {
        .ingredients(scope: scope, event: event)
    }

    func counter(_ event: CounterEvent) -> AppEvent {
        .counter(scope: scope, event: event)
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
