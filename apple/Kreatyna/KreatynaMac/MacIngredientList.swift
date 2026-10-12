import SwiftUI

/// The catalog list column. Rows are text only until they become selectable (MAC-04).
struct MacIngredientList: View {
    let view: IngredientListView
    let send: (IngredientListEvent) -> Void

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
            List {
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
                        .accessibilityIdentifier("ingredientRow")
                }
            }
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
