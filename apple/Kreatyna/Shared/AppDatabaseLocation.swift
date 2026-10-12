import Foundation

enum AppDatabaseLocation {
    static func path(
        fileManager: FileManager = .default,
        applicationSupportDirectory: URL? = nil,
        environment: [String: String] = ProcessInfo.processInfo.environment
    ) throws -> String {
        #if DEBUG
        // Tests never open the real database (E-18). A unit-test host keeps its
        // database in memory; UI tests use the temporary folder, which survives a
        // relaunch and which the system cleans up.
        if environment["XCTestConfigurationFilePath"] != nil {
            return ":memory:"
        }
        if let value = environment["KREATYNA_TEST_DATABASE_ID"],
           let id = UUID(uuidString: value) {
            let directory = fileManager.temporaryDirectory
                .appendingPathComponent("KreatynaUITests", isDirectory: true)
                .appendingPathComponent(id.uuidString, isDirectory: true)
            try fileManager.createDirectory(at: directory, withIntermediateDirectories: true)
            return directory.appendingPathComponent("kreatyna.sqlite").path
        }
        #endif
        let root = try applicationSupportDirectory ?? fileManager.url(
            for: .applicationSupportDirectory,
            in: .userDomainMask,
            appropriateFor: nil,
            create: true
        )
        let directory = root.appendingPathComponent("Kreatyna", isDirectory: true)
        try fileManager.createDirectory(
            at: directory,
            withIntermediateDirectories: true
        )
        return directory.appendingPathComponent("kreatyna.sqlite").path
    }
}
