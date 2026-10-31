import SwiftUI

struct ContentView: View {
    @State private var model = AppModel()

    var body: some View {
        RootView(
            view: model.view,
            errorMessage: model.errorMessage,
            send: model.dispatch,
            retryInitialization: model.retryInitialization
        )
    }
}

/// Renders an `AppView` value; it holds no app state of its own.
struct RootView: View {
    let view: AppView
    let errorMessage: String?
    let send: (AppEvent) -> Void
    let retryInitialization: () -> Void

    var body: some View {
        Group {
            switch view {
            case .initializing:
                ProgressView("Starting…")
                    .accessibilityIdentifier("appInitializing")
            case .initializationFailed(let error):
                ContentUnavailableView {
                    Label(error.title, systemImage: "exclamationmark.triangle")
                } description: {
                    Text(error.message)
                } actions: {
                    Button("Retry", action: retryInitialization)
                        .buttonStyle(.borderedProminent)
                        .accessibilityIdentifier("retryInitialization")
                }
                .accessibilityIdentifier("appInitializationFailed")
            case .running(let app):
                // iOS always starts with the stack layout (`AppLayout.platform`).
                if let catalog = app.ingredients.stack {
                    IngredientCatalogScreen(view: catalog) { send(app.ingredients($0)) }
                }
            }
        }
        .safeAreaInset(edge: .bottom) {
            if let errorMessage {
                Text(errorMessage)
                    .font(.footnote)
                    .foregroundStyle(.red)
                    .padding(8)
                    .accessibilityIdentifier("appError")
            }
        }
    }
}

#Preview {
    ContentView()
}
