import SwiftUI

struct IngredientListScreen: View {
    let view: IngredientListView
    let send: (IngredientListEvent) -> Void

    var body: some View {
        content
            .navigationTitle("Ingredients")
            .rustSearchable(view.search, send: send)
            .toolbar {
                ToolbarItem(placement: .primaryAction) {
                    Button("Add Ingredient", systemImage: "plus") { send(.createRequested) }
                        .accessibilityIdentifier("addIngredient")
                }
            }
    }

    @ViewBuilder
    private var content: some View {
        switch view.status {
        case .loading:
            ProgressView("Loading ingredients…")
                .accessibilityIdentifier("ingredientListLoading")
        case .failed(let error):
            ContentUnavailableView {
                Label("Ingredients unavailable", systemImage: "exclamationmark.triangle")
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
                        Label(error.message, systemImage: "exclamationmark.triangle")
                            .foregroundStyle(.red)
                        Button("Retry") { send(.refresh) }
                            .accessibilityIdentifier("retryIngredientRefresh")
                    }
                }
                ForEach(rows, id: \.id) { row in
                    Button { send(.select(id: row.id)) } label: {
                        Text(row.name)
                            .frame(maxWidth: .infinity, alignment: .leading)
                            .contentShape(.rect)
                    }
                    .buttonStyle(.plain)
                    .accessibilityIdentifier("ingredientRow")
                    .swipeActions(edge: .leading) {
                        Button("Edit", systemImage: "pencil") { send(.edit(id: row.id)) }
                            .tint(.blue)
                    }
                }
            }
            .refreshable { send(.refresh) }
            .overlay {
                if rows.isEmpty {
                    if totalCount == 0 {
                        ContentUnavailableView(
                            "No ingredients yet",
                            systemImage: "carrot",
                            description: Text("Tap + to add your first ingredient.")
                        )
                        .accessibilityIdentifier("ingredientListEmpty")
                    } else {
                        ContentUnavailableView.search(text: view.search)
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
