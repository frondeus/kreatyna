import Foundation
import OSLog

/// Rust invokes this synchronously from arbitrary worker threads.
nonisolated final class RustLogger: LoggerHandler {
    private let write: @Sendable (OSLogType, String, String) -> Void

    init(subsystem: String = "pl.frondeus.Mausritter") {
        let logger = Logger(subsystem: subsystem, category: "rust")
        write = { level, target, payload in
            // Rust's dynamic payload has no per-field privacy annotations.
            #if DEBUG
            logger.log(level: level, "[\(target, privacy: .public)] \(payload, privacy: .public)")
            #else
            logger.log(level: level, "[\(target, privacy: .public)] \(payload, privacy: .private)")
            #endif
        }
    }

    /// Injectable destination for testing delivery without querying system logs.
    init(write: @escaping @Sendable (OSLogType, String, String) -> Void) {
        self.write = write
    }

    nonisolated func log(record: LogRecord) {
        write(Self.osLogType(for: record.level), record.target, Self.format(record))
    }

    static func osLogType(for level: LogLevel) -> OSLogType {
        switch level {
        case .trace, .debug: .debug
        case .info: .info
        case .warn: .default
        case .error: .error
        }
    }

    static func format(_ record: LogRecord) -> String {
        var parts = record.spans.map { span in
            "\(span.target)::\(span.name){\(formatFields(span.fields))}"
        }
        if let message = record.message, !message.isEmpty {
            parts.append(message)
        }
        if !record.fields.isEmpty {
            parts.append(formatFields(record.fields))
        }
        if let file = record.file {
            let location = record.line.map { "\(file):\($0)" } ?? file
            parts.append("(\(location))")
        }
        return parts.joined(separator: " ")
    }

    private static func formatFields(_ fields: [LogField]) -> String {
        fields.map { "\($0.name)=\(String(reflecting: $0.value))" }.joined(separator: " ")
    }
}

@MainActor
enum RustLogging {
    private static let installation: Void = {
        #if DEBUG
        let minimumLevel = LogLevel.debug
        #else
        let minimumLevel = LogLevel.info
        #endif
        do {
            try initializeLogging(handler: RustLogger(), minimumLevel: minimumLevel)
        } catch {
            // Report directly to Apple logging to avoid re-entering the bridge.
            Logger(subsystem: "pl.frondeus.Mausritter", category: "rust")
                .error("Rust logging initialization failed: \(String(describing: error), privacy: .public)")
        }
    }()

    static func start() {
        _ = installation
    }
}
