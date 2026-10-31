import SwiftUI

/// Renders an `AppView` value in the Mac window; it holds no app state of its own.
struct MacRootView: View {
    let view: AppView
    let errorMessage: String?
    let send: (AppEvent) -> Void
    let retryInitialization: () -> Void

    var body: some View {
        Group {
            switch view {
            case .initializing:
                ProgressView("Starting…")
                    .frame(maxWidth: .infinity, maxHeight: .infinity)
                    .accessibilityIdentifier("appInitializing")
            case .initializationFailed(let error):
                ContentUnavailableView {
                    Text(error.title)
                } description: {
                    Text(error.message)
                } actions: {
                    Button("Retry", action: retryInitialization)
                        .buttonStyle(.borderedProminent)
                        .accessibilityIdentifier("retryInitialization")
                }
                .accessibilityIdentifier("appInitializationFailed")
            case .crashed:
                ContentUnavailableView {
                    Text(CrashText.title)
                } description: {
                    Text(CrashText.message)
                } actions: {
                    Button("Quit Kreatyna") { NSApplication.shared.terminate(nil) }
                        .buttonStyle(.borderedProminent)
                        .accessibilityIdentifier("quitAfterCrash")
                }
                .accessibilityIdentifier("appCrashed")
            case .running(let app):
                // The Mac always starts with the split layout (`AppLayout.platform`).
                if let split = app.split {
                    MacMainView(app: split, send: send)
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
