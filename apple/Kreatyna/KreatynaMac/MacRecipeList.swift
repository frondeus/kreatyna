import SwiftUI

/// The Recipes section's list column. Rust sorts the rows by title.
/// No search (REC-06) and no selection (REC-03) yet.
struct MacRecipeList: View {
    let view: RecipeListView
    let send: (RecipeListEvent) -> Void

    var body: some View {
        content
            .navigationTitle("Recipes")
            .toolbar {
                ToolbarItem(placement: .primaryAction) {
                    // Creating a recipe arrives with REC-02.
                    Button("New Recipe", systemImage: "plus") {}
                        .disabled(true)
                        .accessibilityIdentifier("newRecipe")
                }
            }
    }

    @ViewBuilder
    private var content: some View {
        switch view.status {
        case .loading:
            ProgressView("Loading recipes…")
                .frame(maxWidth: .infinity, maxHeight: .infinity)
                .accessibilityIdentifier("recipeListLoading")
        case .failed(let error):
            ContentUnavailableView {
                Text("Recipes unavailable")
            } description: {
                Text(error.message)
            } actions: {
                Button("Retry") { send(.refresh) }
                    .buttonStyle(.borderedProminent)
                    .accessibilityIdentifier("retryRecipeList")
            }
            .accessibilityIdentifier("recipeListFailed")
        case .ready(let rows):
            List(rows, id: \.id) { row in
                Text(row.title)
                    .accessibilityIdentifier("recipeRow")
            }
            .accessibilityIdentifier("recipeList")
            .overlay {
                if rows.isEmpty {
                    ContentUnavailableView {
                        Text("No recipes yet")
                    } description: {
                        Text("Recipes you add appear here.")
                    }
                    .accessibilityIdentifier("recipeListEmpty")
                }
            }
        }
    }
}
