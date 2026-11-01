import SwiftUI

/// The Recipes section's detail column: Rust's pane (E-19).
struct MacRecipePane: View {
    let section: RecipeSectionView
    let send: (RecipeSectionEvent) -> Void

    var body: some View {
        switch section.pane.value {
        // The read view arrives in the next change; until then it looks empty.
        case .empty, .detail:
            // A recipe opens here from REC-03.
            ContentUnavailableView {
                Text("Select a recipe")
            }
            .accessibilityIdentifier("recipeDetailPlaceholder")
        case .create(let editor, _):
            // A new pane scope is new content, so the title field starts fresh.
            MacRecipeEditor(view: editor) { send(section.editor($0)) }
                .id(section.pane.scope)
        }
    }
}
