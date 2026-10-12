import SwiftUI

/// The catalog list column.
///
/// Swift keeps no selection: the highlight is Rust's `selected`, and a click,
/// an arrow key, or a ⌘-click only reports the new choice through `select`.
struct MacIngredientList: View {
    let view: IngredientListView
    let selected: String?
    let select: (String?) -> Void
    let send: (IngredientListEvent) -> Void

    /// Presentation only: keyboard focus, so the arrow keys move the selection.
    @FocusState private var focused: Bool

    var body: some View {
        content
            .navigationTitle("Ingredients")
            .toolbar {
                ToolbarItem(placement: .primaryAction) {
                    Button("Add Ingredient", systemImage: "plus") {}
                        .disabled(true) // MAC-05
                        .accessibilityIdentifier("addIngredient")
                }
            }
    }

    @ViewBuilder
    private var content: some View {
        switch view.status {
        case .loading:
            ProgressView("Loading ingredients…")
                .frame(maxWidth: .infinity, maxHeight: .infinity)
                .accessibilityIdentifier("ingredientListLoading")
        case .failed(let error):
            ContentUnavailableView {
                Text("Ingredients unavailable")
            } description: {
                Text(error.message)
            } actions: {
                Button("Retry") { send(.refresh) }
                    .buttonStyle(.borderedProminent)
                    .accessibilityIdentifier("retryIngredientList")
            }
            .accessibilityIdentifier("ingredientListFailed")
        case .ready(let rows, let totalCount, let refresh):
            List(selection: Binding(get: { selected }, set: select)) {
                if case .failed(let error) = refresh {
                    Section {
                        Text(error.message)
                            .foregroundStyle(.red)
                        Button("Retry") { send(.refresh) }
                            .accessibilityIdentifier("retryIngredientRefresh")
                    }
                }
                ForEach(rows, id: \.id) { row in
                    Text(row.name)
                        .tag(row.id)
                        .accessibilityIdentifier("ingredientRow")
                }
            }
            .accessibilityIdentifier("ingredientList")
            .focused($focused)
            // A click selects through the binding but does not focus the list.
            // Focusing it on each selection lets ↑ and ↓ work after a click.
            .onChange(of: selected) { focused = true }
            .overlay {
                if rows.isEmpty {
                    if totalCount == 0 {
                        ContentUnavailableView {
                            Text("No ingredients yet")
                        } description: {
                            Text("Ingredients you add appear here.")
                        }
                        .accessibilityIdentifier("ingredientListEmpty")
                    } else {
                        ContentUnavailableView {
                            Text("No matches for “\(view.search)”")
                        }
                        .accessibilityIdentifier("ingredientListNoMatches")
                    }
                }
            }
            .overlay(alignment: .top) {
                if case .loading = refresh {
                    ProgressView().padding()
                }
            }
        }
    }
}
