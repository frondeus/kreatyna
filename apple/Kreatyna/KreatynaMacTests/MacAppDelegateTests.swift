import Testing
@testable import Kreatyna

/// `applicationShouldTerminate` hands its reply to `quit`. These tests call
/// `quit` directly: a real reply would end the test host.
@MainActor
struct MacAppDelegateTests {
    @Test func quitAllowsTerminationAfterRustStops() async throws {
        let model = AppModel(layout: .split, databasePath: ":memory:")
        let delegate = MacAppDelegate(model: model)
        var replies: [Bool] = []

        delegate.quit { replies.append($0) }
        let deadline = ContinuousClock.now + .seconds(5)
        while replies.isEmpty, ContinuousClock.now < deadline {
            try await Task.sleep(for: .milliseconds(10))
        }

        #expect(replies == [true])
        // Rust has stopped: a second graceful shutdown has nothing to wait for.
        #expect(await model.shutDownGracefully() == .completed)
    }
}
