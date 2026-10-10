import SwiftUI

/// Projects the Rust ingredient route into native navigation.
///
/// The list is the root, a detail is pushed, and forms are sheets. Binding
/// setters only send `Back`; Rust decides whether and where to go.
struct IngredientCatalogScreen: View {
    let view: IngredientCatalogView
    let send: (IngredientCatalogEvent) -> Void

    var body: some View {
        NavigationStack {
            IngredientListScreen(view: view.list) { send(view.list($0)) }
                .navigationDestination(isPresented: Binding(
                    get: { detail != nil },
                    set: { if !$0 { send(view.back) } }
                )) {
                    if let detail {
                        IngredientDetailScreen(view: detail) { send(view.detail($0)) }
                    }
                }
        }
        .sheet(isPresented: Binding(
            get: { form != nil },
            set: { if !$0 { send(view.back) } }
        )) {
            if let form {
                NavigationStack {
                    IngredientFormScreen(
                        view: form,
                        send: { send(view.form($0)) },
                        cancel: { send(view.back) }
                    )
                }
                .interactiveDismissDisabled()
            }
        }
    }

    private var detail: IngredientDetailView? {
        switch view.screen {
        case .detail(let detail), .editFromDetail(let detail, _): detail
        case .list, .create, .editFromList: nil
        }
    }

    private var form: IngredientFormView? {
        switch view.screen {
        case .create(let form), .editFromList(let form), .editFromDetail(_, let form): form
        case .list, .detail: nil
        }
    }
}
