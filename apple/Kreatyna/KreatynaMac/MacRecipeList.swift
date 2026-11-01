import SwiftUI

/// The Recipes section's list column. Rust sorts the rows by title.
/// No search (REC-06) and no selection (REC-03) yet. While the create editor
/// is open, Rust's draft row is first and highlighted (REC-02/Q1).
struct MacRecipeList: View {
    let view: RecipeListView
    /// The typed title of the recipe being created; empty before a title.
    var draftRow: String? = nil
    var canCreate = true
    let newRecipe: () -> Void
    let send: (RecipeListEvent) -> Void

    /// The draft row's tag in the list selection. It is never a recipe ID.
    private static let draftTag = "draft"

    var body: some View {
        content
            .navigationTitle("Recipes")
            .toolbar {
                ToolbarItem(placement: .primaryAction) {
                    Button("New Recipe", systemImage: "plus", action: newRecipe)
                        .keyboardShortcut("n", modifiers: .command)
                        .disabled(!canCreate)
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
            // Only the draft row can be highlighted; a click changes nothing.
            List(selection: .constant(draftRow == nil ? nil : Self.draftTag)) {
                if let draftRow {
                    Text(draftRow.isEmpty ? "New Recipe" : draftRow)
                        .italic()
                        .tag(Self.draftTag)
                        .accessibilityIdentifier("recipeDraftRow")
                }
                ForEach(rows, id: \.id) { row in
                    Text(row.title)
                        .accessibilityIdentifier("recipeRow")
                }
            }
            .accessibilityIdentifier("recipeList")
            .overlay {
                // A draft row covers the empty state: something is being added.
                if rows.isEmpty && draftRow == nil {
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
