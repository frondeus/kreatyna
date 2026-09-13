import Observation

@MainActor
@Observable
final class CounterModel {
    private(set) var value: UInt64
    private(set) var errorMessage: String?
    @ObservationIgnored private let core: CounterFfi
    @ObservationIgnored private var runtimeTask: Task<Void, Never>?

    deinit {
        core.shutdown()
        runtimeTask?.cancel()
    }

    func dispatch(_ event: CounterEvent) {
        do {
            try core.dispatch(event: event)
        } catch {
            errorMessage = "The counter could not accept this action."
        }
    }

    func shutdown() async {
        core.shutdown()
        runtimeTask?.cancel()
        await runtimeTask?.value
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
        runtimeTask = Task { [weak self, core] in
            guard !Task.isCancelled else { return }
            do {
                try await core.run()
            } catch {
                guard !Task.isCancelled else { return }
                self?.errorMessage = "The counter stopped unexpectedly."
            }
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
