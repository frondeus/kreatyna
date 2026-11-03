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
            MacRecipeEditor(view: editor) { send(section.editor($0)) }
                .id(section.pane.scope)
        case .edit:
            // REC-04 change 6 shows the editor here. Edit is still disabled.
            EmptyView()
        }
    }
}
