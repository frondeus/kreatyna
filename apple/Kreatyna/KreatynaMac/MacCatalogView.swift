import SwiftUI

/// The Mac window's three columns: sections, the catalog list, and a detail pane.
///
/// The sidebar has one static item until Rust owns sections (REC-01). Rust's
/// split catalog decides the selection and the pane (E-19).
struct MacCatalogView: View {
    let view: IngredientSplitCatalogView
    let send: (IngredientSplitCatalogEvent) -> Void

    /// Presentation only: whether the sidebar is shown.
    @State private var columnVisibility: NavigationSplitViewVisibility

    init(
        view: IngredientSplitCatalogView,
        columnVisibility: NavigationSplitViewVisibility = .all,
        send: @escaping (IngredientSplitCatalogEvent) -> Void
    ) {
        self.view = view
        self.send = send
        _columnVisibility = State(initialValue: columnVisibility)
    }

    var body: some View {
        NavigationSplitView(columnVisibility: $columnVisibility) {
            List(selection: .constant("ingredients")) {
                Text("Ingredients")
                    .tag("ingredients")
                    .accessibilityIdentifier("sidebarIngredients")
            }
            .navigationSplitViewColumnWidth(min: 160, ideal: 200)
        } content: {
            MacIngredientList(
                view: view.list.value,
                selected: view.selected,
                draftRow: view.draftRow,
                canCreate: view.canCreate,
                select: { id in send(id.map { view.list(.select(id: $0)) } ?? view.clearSelection) },
                send: { send(view.list($0)) }
            )
            .navigationSplitViewColumnWidth(min: 220, ideal: 280)
        } detail: {
            switch view.pane.value {
            case .empty:
                ContentUnavailableView {
                    Text("Select an ingredient")
                }
                .accessibilityIdentifier("ingredientDetailPlaceholder")
            case .detail(let detail):
                MacIngredientDetail(view: detail) { send(view.detail($0)) }
            case .create(let form):
                // A new pane scope is new content, so its fields start fresh.
                MacIngredientForm(view: form, title: "New Ingredient") { send(view.form($0)) }
                    .id(view.pane.scope)
            case .edit(let savedName, let form):
                MacIngredientForm(view: form, title: savedName) { send(view.form($0)) }
                    .id(view.pane.scope)
            }
        }
    }
}
