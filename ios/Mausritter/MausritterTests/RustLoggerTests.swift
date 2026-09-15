import Foundation
import OSLog
import Synchronization
import Testing
@testable import Mausritter

struct RustLoggerTests {
    @Test func mapsSeverityToAppleLevels() {
        #expect(RustLogger.osLogType(for: .trace) == .debug)
        #expect(RustLogger.osLogType(for: .debug) == .debug)
        #expect(RustLogger.osLogType(for: .info) == .info)
        #expect(RustLogger.osLogType(for: .warn) == .default)
        #expect(RustLogger.osLogType(for: .error) == .error)
    }

    @Test func preservesNestedContextAndEscapesFields() {
        let record = LogRecord(
            level: .info, target: "counter", message: "Loaded 🐭",
            fields: [LogField(name: "id", value: "3"), LogField(name: "text", value: "a\n\"b\"")],
            spans: [
                LogSpan(name: "load", target: "database", fields: [LogField(name: "id", value: "1")]),
                LogSpan(name: "query", target: "database", fields: [LogField(name: "id", value: "2")]),
            ],
            file: "database.rs", line: 42
        )
        #expect(RustLogger.format(record) == "database::load{id=\"1\"} database::query{id=\"2\"} Loaded 🐭 id=\"3\" text=\"a\\n\\\"b\\\"\" (database.rs:42)")
    }

    @Test func handlesMissingMessageAndSourceLocation() {
        let record = LogRecord(level: .debug, target: "counter", message: nil,
                               fields: [LogField(name: "count", value: "7")],
                               spans: [], file: nil, line: nil)
        #expect(RustLogger.format(record) == "count=\"7\"")
        let fileOnly = LogRecord(level: .info, target: "counter", message: "hello",
                                 fields: [], spans: [], file: "counter.rs", line: nil)
        #expect(RustLogger.format(fileOnly) == "hello (counter.rs)")
    }

    @MainActor
    @Test func deliversConcurrentCallbacksBeforeReturning() async {
        let output = Mutex<[(UInt8, String, String)]>([])
        let logger = RustLogger { level, target, payload in
            output.withLock { $0.append((level.rawValue, target, payload)) }
        }
        await withTaskGroup(of: Void.self) { group in
            for index in 0..<100 {
                let record = LogRecord(level: .warn, target: "worker", message: "event \(index)",
                                       fields: [], spans: [], file: nil, line: nil)
                group.addTask {
                    logger.log(record: record)
                }
            }
        }
        let records = output.withLock { $0 }
        #expect(records.count == 100)
        #expect(Set(records.map { $0.2 }).count == 100)
        #expect(records.allSatisfy { $0.0 == OSLogType.default.rawValue && $0.1 == "worker" })
    }

    @MainActor
    @Test func startupInstallsTheRealRustSubscriberOnlyOnce() {
        RustLogging.start()
        RustLogging.start()
        #expect(throws: LoggingInitError.AlreadyInitialized) {
            try initializeLogging(handler: RustLogger(), minimumLevel: .trace)
        }
    }
}
