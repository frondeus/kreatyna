import SwiftUI

/// The catalog list column.
///
/// Swift keeps no selection: the highlight is Rust's `selected`, and a click,
/// an arrow key, or a ⌘-click only reports the new choice through `select`.
/// While the create form is open, Rust's draft row is first and highlighted.
struct MacIngredientList: View {
    let view: IngredientListView
    let selected: IngredientId?
    /// The typed name of the ingredient being created; empty before a name.
    let draftRow: String?
    let canCreate: Bool
    let select: (IngredientId?) -> Void
    let send: (IngredientListEvent) -> Void

    /// Presentation only: keyboard focus, so the arrow keys move the selection.
    @FocusState private var focused: Bool

    var body: some View {
        VStack(spacing: 0) {
            MacSearchField(
                search: view.search,
                prompt: "Search ingredients",
                identifier: "ingredientSearch"
            ) { send(.search(query: $0)) }
                .padding(.horizontal, 10)
                .padding(.vertical, 6)
            content
        }
        .navigationTitle("Ingredients")
        .toolbar {
            ToolbarItem(placement: .primaryAction) {
                Button("Add Ingredient", systemImage: "plus") { send(.createRequested) }
                    .keyboardShortcut("n", modifiers: .command)
                    .disabled(!canCreate)
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
            List(selection: MacListTag.selection(draftShown: draftRow != nil, selected: selected, select: select)) {
                if case .failed(let error) = refresh {
                    Section {
                        Text(error.message)
                            .foregroundStyle(.red)
                        Button("Retry") { send(.refresh) }
                            .accessibilityIdentifier("retryIngredientRefresh")
                    }
                }
                if let draftRow {
                    Text(draftRow.isEmpty ? "New Ingredient" : draftRow)
                        .italic()
                        .tag(MacListTag<IngredientId>.draft)
                        .accessibilityIdentifier("ingredientDraftRow")
                }
                ForEach(rows, id: \.id) { row in
                    Text(row.name)
                        .tag(MacListTag.item(row.id))
                        .accessibilityIdentifier("ingredientRow")
                }
            }
            .accessibilityIdentifier("ingredientList")
            .focused($focused)
            // A click selects through the binding but does not focus the list.
            // Focusing it on each selection lets ↑ and ↓ work after a click.
            // Not when the form opens (`selected` becomes nil): Name has the focus then.
            .onChange(of: selected) { if selected != nil { focused = true } }
            .overlay {
                if rows.isEmpty {
                    if totalCount > 0 {
                        ContentUnavailableView {
                            Text("No matches for “\(view.search)”")
                        }
                        .accessibilityIdentifier("ingredientListNoMatches")
                    } else if draftRow == nil {
                        // A draft row covers the empty state: something is being added.
                        ContentUnavailableView {
                            Text("No ingredients yet")
                        } description: {
                            Text("Ingredients you add appear here.")
                        }
                        .accessibilityIdentifier("ingredientListEmpty")
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
