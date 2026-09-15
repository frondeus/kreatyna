import Foundation

enum CounterDatabaseLocation {
    static func path(
        fileManager: FileManager = .default,
        applicationSupportDirectory: URL? = nil
    ) throws -> String {
        let root = try applicationSupportDirectory ?? fileManager.url(
            for: .applicationSupportDirectory,
            in: .userDomainMask,
            appropriateFor: nil,
            create: true
        )
        var directory = root.appendingPathComponent("Mausritter", isDirectory: true)
        #if DEBUG
        if let value = ProcessInfo.processInfo.environment["MAUSRITTER_TEST_DATABASE_ID"],
           let id = UUID(uuidString: value) {
            directory = directory
                .appendingPathComponent("UITests", isDirectory: true)
                .appendingPathComponent(id.uuidString, isDirectory: true)
        }
        #endif
        try fileManager.createDirectory(
            at: directory,
            withIntermediateDirectories: true
        )
        return directory.appendingPathComponent("counter.sqlite").path
    }
}
