import Foundation
import Observation

@MainActor
@Observable
final class CounterModel {
    private(set) var value: UInt64 = 0
    private(set) var hasRendered = false
    private(set) var errorMessage: String?
    @ObservationIgnored private let core: CounterFfi?

    func dispatch(_ event: CounterEvent) {
        guard let core else { return }
        do {
            try core.dispatch(event: event)
        } catch {
            errorMessage = "The counter could not accept this action."
        }
    }

    func shutdown() {
        core?.shutdown()
    }

    init(databasePath: String? = nil, clock: (any ClockHandler)? = nil) {
        let path: String
        do {
            path = try databasePath ?? CounterDatabaseLocation.path()
        } catch {
            core = nil
            errorMessage = "The counter's storage directory could not be created."
            return
        }
        let renderer = CounterRenderer()
        let core = counter(
            caps: CounterCapabilities(render: renderer, clock: clock ?? CounterClock()),
            databasePath: path
        )
        self.core = core
        value = core.view()
        renderer.onRender = { [weak self] in
            guard let self else { return }
            self.value = self.core?.view() ?? self.value
            self.hasRendered = true
        }
    }
}

@MainActor
final class CounterRenderer: RenderHandler {
    var onRender: (() -> Void)?

    nonisolated func render() async {
        await refresh()
    }

    private func refresh() {
        onRender?()
    }
}
