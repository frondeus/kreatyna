import Foundation
import Observation
import OSLog

/// Holds the Rust app facade and publishes its latest view on the main actor.
@MainActor
@Observable
final class AppModel {
    private(set) var view: AppView = .initializing
    /// Swift's own errors, and events Rust could not read. Rust's own failures
    /// are part of `view` (P06). The next event Rust accepts clears it.
    private(set) var errorMessage: String?
    @ObservationIgnored private let core: AppFfi?

    init(layout: AppLayout = .platform, databasePath: String? = nil) {
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
        if let seed = TestSeed.json() {
            do {
                try seedIngredients(databasePath: path, json: seed)
            } catch {
                // The run script's user reads Rust's reason in the log.
                Logger(subsystem: "pl.frondeus.Kreatyna", category: "seed")
                    .error("Seeding failed: \(String(describing: error), privacy: .public)")
                errorMessage = "Test data could not be added."
            }
        }
        #endif
        let renderer = AppRenderer()
        let core = app(
            caps: AppCapabilities(render: renderer),
            databasePath: path,
            layout: layout
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
            errorMessage = nil
        } catch {
            report(error)
        }
    }

    func retryInitialization() {
        guard let core else { return }
        do {
            try core.retryInitialization()
            errorMessage = nil
        } catch {
            report(error)
        }
    }

    /// A stopped Rust runtime refuses events by design: the app is quitting.
    private func report(_ error: Error) {
        if case AppFfiError.Terminated = error { return }
        errorMessage = "The app could not accept this action."
    }

    func shutdown() {
        core?.shutdown()
    }

    /// Waits until Rust has finished accepted work, such as a save, or has
    /// cancelled it after its grace period (P05).
    @discardableResult
    func shutDownGracefully() async -> ShutdownResult {
        guard let core else { return .completed }
        return await core.shutDownGracefully()
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
