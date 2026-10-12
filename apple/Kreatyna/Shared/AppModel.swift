import Foundation
import Observation

/// Holds the Rust app facade and publishes its latest view on the main actor.
@MainActor
@Observable
final class AppModel {
    private(set) var view: AppView = .initializing
    private(set) var errorMessage: String?
    @ObservationIgnored private let core: AppFfi?

    init(databasePath: String? = nil) {
        // Previews and tests can create a model without running the App initializer.
        RustLogging.start()
        let path: String
        do {
            path = try databasePath ?? AppDatabaseLocation.path()
        } catch {
            core = nil
            errorMessage = "The app's storage directory could not be created."
            return
        }
        #if DEBUG
        let seed = TestSeed.names()
        if !seed.isEmpty {
            do {
                try seedIngredients(databasePath: path, names: seed)
            } catch {
                errorMessage = "Test data could not be added."
            }
        }
        #endif
        let renderer = AppRenderer()
        let core = app(
            caps: AppCapabilities(render: renderer),
            databasePath: path
        )
        self.core = core
        // Rust retains the renderer; a weak reference avoids a facade cycle.
        renderer.onRender = { [weak self] in self?.refresh() }
        refresh()
        do {
            try core.start()
        } catch {
            errorMessage = "The app could not start."
        }
    }

    func dispatch(_ event: AppEvent) {
        guard let core else { return }
        do {
            try core.dispatch(event: event)
        } catch {
            errorMessage = "The app could not accept this action."
        }
    }

    func retryInitialization() {
        guard let core else { return }
        do {
            try core.retryInitialization()
        } catch {
            errorMessage = "The app could not accept this action."
        }
    }

    func shutdown() {
        core?.shutdown()
    }

    private func refresh() {
        guard let core else { return }
        do {
            view = try core.view()
        } catch {
            errorMessage = "The app could not display its current state."
        }
    }
}

@MainActor
final class AppRenderer: RenderHandler {
    var onRender: (() -> Void)?

    nonisolated func render() async {
        await refresh()
    }

    private func refresh() {
        onRender?()
    }
}
