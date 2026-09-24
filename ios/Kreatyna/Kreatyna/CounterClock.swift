import Foundation

final class CounterClock: ClockHandler {
    nonisolated func sleepMs(ms: UInt64) async {
        do {
            try await Task.sleep(for: .milliseconds(Double(ms)))
        } catch is CancellationError {
            // The caller no longer needs this wait.
        } catch {
            assertionFailure("Unexpected clock error: \(error)")
        }
    }
}
