import Observation

@MainActor
@Observable
final class CounterModel {
    private(set) var value: UInt64
    private(set) var errorMessage: String?
    @ObservationIgnored private let core: CounterFfi

    func dispatch(_ event: CounterEvent) {
        do {
            try core.dispatch(event: event)
        } catch {
            errorMessage = "The counter could not accept this action."
        }
    }

    func shutdown() {
        core.shutdown()
    }

    init(clock: any ClockHandler = CounterClock()) {
        let renderer = CounterRenderer()
        let core = counter(render: renderer, clock: clock)
        self.core = core
        value = core.view()
        renderer.onRender = { [weak self] in
            guard let self else { return }
            self.value = self.core.view()
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
