import SwiftUI

/// The Ingredients section's detail column: Rust's split catalog pane (E-19).
struct MacIngredientPane: View {
    let catalog: IngredientSplitCatalogView
    let send: (IngredientSplitCatalogEvent) -> Void

    var body: some View {
        switch catalog.pane.value {
        case .empty:
            ContentUnavailableView {
                Text("Select an ingredient")
            }
            .accessibilityIdentifier("ingredientDetailPlaceholder")
        case .detail(let detail):
            MacIngredientDetail(view: detail) { send(catalog.detail($0)) }
        case .create(let form):
            // A new pane scope is new content, so its fields start fresh.
            MacIngredientForm(view: form, title: "New Ingredient") { send(catalog.form($0)) }
                .id(catalog.pane.scope)
        case .edit(let savedName, let form):
            MacIngredientForm(view: form, title: savedName) { send(catalog.form($0)) }
                .id(catalog.pane.scope)
        }
    }
}
