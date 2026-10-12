import Foundation

#if DEBUG
/// Ingredients that UI tests and demos ask Rust to save before the app starts (E-17).
///
/// The seed is JSON text. Swift only reads the request. Rust parses,
/// validates, and saves the ingredients.
enum TestSeed {
    /// The text of `KREATYNA_TEST_SEED`, only for an isolated UI-test database.
    static func json(environment: [String: String] = ProcessInfo.processInfo.environment) -> String? {
        guard environment["KREATYNA_TEST_DATABASE_ID"] != nil else { return nil }
        return environment["KREATYNA_TEST_SEED"]
    }
}
#endif
