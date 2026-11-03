import SwiftUI

/// The Recipes section's detail column: Rust's pane (E-19).
struct MacRecipePane: View {
    let section: RecipeSectionView
    let send: (RecipeSectionEvent) -> Void

    var body: some View {
        switch section.pane.value {
        case .empty:
            ContentUnavailableView {
                Text("Select a recipe")
            }
            .accessibilityIdentifier("recipeDetailPlaceholder")
        case .detail(let detail):
            MacRecipeDetail(view: detail) { send(section.detail($0)) }
        case .create(let editor, _):
            // A new pane scope is new content, so the title field starts fresh.
            MacRecipeEditor(heading: "New Recipe", view: editor) { send(section.editor($0)) }
                .id(section.pane.scope)
        case .edit(let editor, let savedTitle):
            // The heading keeps the saved title until Save (MAC-06/Q1).
            MacRecipeEditor(heading: savedTitle, view: editor) { send(section.editor($0)) }
                .id(section.pane.scope)
        }
    }
}
