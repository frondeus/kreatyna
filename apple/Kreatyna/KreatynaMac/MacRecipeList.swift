import SwiftUI

/// The Recipes section's list column. Rust sorts the rows by title. No
/// search yet (REC-06).
///
/// Swift keeps no selection: the highlight is Rust's `selected`, and a click,
/// an arrow key, or a ⌘-click only reports the new choice through `select`.
/// While the create editor is open, Rust's draft row is first and highlighted
/// (REC-02/Q1).
struct MacRecipeList: View {
    let view: RecipeListView
    let selected: RecipeId?
    /// The typed title of the recipe being created; empty before a title.
    let draftRow: String?
    let canCreate: Bool
    let newRecipe: () -> Void
    let select: (RecipeId?) -> Void
    let send: (RecipeListEvent) -> Void

    /// The draft row's tag in the list selection. It is never a recipe ID.
    private static let draftTag = "draft"

    /// Presentation only: keyboard focus, so the arrow keys move the selection.
    @FocusState private var focused: Bool

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

    /// A click on the draft row changes nothing; Rust keeps it highlighted.
    private func choose(_ id: RecipeId?) {
        if id != Self.draftTag { select(id) }
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
            List(selection: Binding(get: { draftRow == nil ? selected : Self.draftTag }, set: choose)) {
                if let draftRow {
                    Text(draftRow.isEmpty ? "New Recipe" : draftRow)
                        .italic()
                        .tag(Self.draftTag)
                        .accessibilityIdentifier("recipeDraftRow")
                }
                ForEach(rows, id: \.id) { row in
                    Text(row.title)
                        .tag(row.id)
                        .accessibilityIdentifier("recipeRow")
                }
            }
            .accessibilityIdentifier("recipeList")
            .focused($focused)
            // A click selects through the binding but does not focus the list.
            // Focusing it on each selection lets ↑ and ↓ work after a click.
            // Not when the editor opens (`selected` becomes nil): Title has the focus then.
            .onChange(of: selected) { if selected != nil { focused = true } }
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
