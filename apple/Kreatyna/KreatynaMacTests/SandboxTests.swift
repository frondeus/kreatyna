import Foundation
import Testing

/// The tests run inside the app (the test host), so they see its sandbox.
/// MAC-01 keeps the Rust database in this container.
struct SandboxTests {
    @Test func appRunsInItsSandboxContainer() throws {
        let bundleID = try #require(Bundle.main.bundleIdentifier)
        #expect(bundleID == "pl.frondeus.Kreatyna")
        #expect(NSHomeDirectory().hasSuffix("/Library/Containers/\(bundleID)/Data"))
    }
}
