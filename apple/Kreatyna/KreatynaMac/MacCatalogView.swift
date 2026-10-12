import SwiftUI

/// The Mac window's three columns: sections, the catalog list, and a detail pane.
///
/// The sidebar has one static item until Rust owns sections (REC-01). The
/// detail pane is a placeholder until rows become selectable (MAC-04).
struct MacCatalogView: View {
    let view: IngredientCatalogView
    let send: (IngredientCatalogEvent) -> Void

    /// Presentation only: whether the sidebar is shown.
    @State private var columnVisibility: NavigationSplitViewVisibility

    init(
        view: IngredientCatalogView,
        columnVisibility: NavigationSplitViewVisibility = .all,
        send: @escaping (IngredientCatalogEvent) -> Void
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
            MacIngredientList(view: view.list) { send(view.list($0)) }
                .navigationSplitViewColumnWidth(min: 220, ideal: 280)
        } detail: {
            ContentUnavailableView {
                Text("Select an ingredient")
            }
            .accessibilityIdentifier("ingredientDetailPlaceholder")
        }
    }
}
