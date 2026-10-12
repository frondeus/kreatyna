import Foundation

#if DEBUG
/// Ingredients that UI tests and demos ask Rust to save before the app starts (E-17).
///
/// Swift only reads the request. Rust validates and saves the names.
enum TestSeed {
    /// Names from `KREATYNA_TEST_SEED`, only for an isolated UI-test database.
    static func names(environment: [String: String] = ProcessInfo.processInfo.environment) -> [String] {
        guard environment["KREATYNA_TEST_DATABASE_ID"] != nil,
              let value = environment["KREATYNA_TEST_SEED"] else { return [] }
        return value.split(separator: ",").map(String.init)
    }
}
#endif
