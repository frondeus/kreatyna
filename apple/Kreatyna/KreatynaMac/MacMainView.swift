import SwiftUI

/// The Mac window's three columns: the sections, the selected section's list,
/// and its detail pane.
///
/// Rust owns the selected section and both sections' state (REC-01). Swift
/// renders the section that Rust selected; switching keeps the other one.
struct MacMainView: View {
    let app: SplitAppView
    let send: (AppEvent) -> Void

    /// Presentation only: whether the sidebar is shown.
    @State private var columnVisibility: NavigationSplitViewVisibility

    init(
        app: SplitAppView,
        columnVisibility: NavigationSplitViewVisibility = .all,
        send: @escaping (AppEvent) -> Void
    ) {
        self.app = app
        self.send = send
        _columnVisibility = State(initialValue: columnVisibility)
    }

    var body: some View {
        NavigationSplitView(columnVisibility: $columnVisibility) {
            MacSidebar(section: app.section) { send(app.selectSection($0)) }
                .navigationSplitViewColumnWidth(min: 160, ideal: 200)
        } content: {
            switch app.section {
            case .recipes:
                let section = app.recipes
                MacRecipeList(
                    view: section.list,
                    selected: section.selected,
                    draftRow: section.draftRow,
                    create: section.create,
                    newRecipe: { send(app.recipes(section.newRecipe)) },
                    select: { id in
                        send(app.recipes(id.map { section.select($0) } ?? section.clearSelection))
                    },
                    send: { send(app.recipes(section.list($0))) }
                )
                .navigationSplitViewColumnWidth(min: 220, ideal: 280)
            case .ingredients:
                let catalog = app.ingredients
                MacIngredientList(
                    view: catalog.list.value,
                    selected: catalog.selected,
                    draftRow: catalog.draftRow,
                    create: catalog.create,
                    select: { id in
                        send(app.ingredients(id.map { catalog.list(.select(id: $0)) } ?? catalog.clearSelection))
                    },
                    send: { send(app.ingredients(catalog.list($0))) }
                )
                .navigationSplitViewColumnWidth(min: 220, ideal: 280)
            }
        } detail: {
            switch app.section {
            case .recipes:
                MacRecipePane(section: app.recipes) { send(app.recipes($0)) }
            case .ingredients:
                MacIngredientPane(catalog: app.ingredients) { send(app.ingredients($0)) }
            }
        }
    }
}
