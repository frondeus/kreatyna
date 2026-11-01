import Foundation

// Parents wrap child events with the scope IDs of the view they displayed.
// Rust checks these IDs; Swift never creates or reuses them.

extension RunningAppView {
    /// The catalog of the stack layout (iOS), or `nil` in the split layout.
    var stack: IngredientCatalogView? {
        if case .stack(let catalog) = self { catalog } else { nil }
    }

    /// The sections of the split layout (Mac), or `nil` in the stack layout.
    var split: SplitAppView? {
        if case .split(let app) = self { app } else { nil }
    }
}

// The split layout's events: the sidebar selection and each section's events.
extension SplitAppView {
    func selectSection(_ section: AppSection) -> AppEvent {
        .split(event: .selectSection(section: section))
    }

    func recipes(_ event: RecipeSectionEvent) -> AppEvent {
        .split(event: .recipes(event: event))
    }

    func ingredients(_ event: IngredientSplitCatalogEvent) -> AppEvent {
        .split(event: .ingredients(event: event))
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

    func form(_ event: IngredientFormEvent) -> IngredientSplitCatalogEvent {
        .form(scope: pane.scope, event: event)
    }

    var clearSelection: IngredientSplitCatalogEvent {
        .clearSelection(scope: pane.scope)
    }
}

// The recipe list lives as long as the section and needs no scope. Editor
// events carry the pane's scope, which is new for each editor.
extension RecipeSectionView {
    func list(_ event: RecipeListEvent) -> RecipeSectionEvent {
        .list(event: event)
    }

    var newRecipe: RecipeSectionEvent {
        .newRecipe
    }

    func editor(_ event: RecipeEditorEvent) -> RecipeSectionEvent {
        .editor(scope: pane.scope, event: event)
    }

    /// The typed title for the list's draft row, or nil when no editor is open.
    var draftRow: String? {
        if case .create(_, let draftRow) = pane.value { draftRow } else { nil }
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
