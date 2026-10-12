import AppKit
import SnapshotTesting
import SwiftUI
import Testing
@testable import Kreatyna

// Rendering conditions are fixed: a 960 × 600 point window, US English, and
// light and dark appearance. The window is never key, so controls show their
// inactive look. Tests run serially (`MainActorTests`). Baselines
// were recorded on macOS 27.0.1; other versions can render slightly
// differently. Review every diff before re-recording.
extension MainActorTests {
    @MainActor
    @Suite(.snapshots(record: .missing))
    struct MacScreenSnapshotTests {

        @Test(arguments: Appearance.allCases)
        func catalogWithRows(appearance: Appearance) {
            snapshot(running(Fixtures.list()), appearance: appearance)
        }

        @Test(arguments: Appearance.allCases)
        func emptyCatalog(appearance: Appearance) {
            snapshot(running(Fixtures.list(.ready(rows: [], totalCount: 0, refresh: .idle))), appearance: appearance)
        }

        @Test(arguments: Appearance.allCases)
        func startupFailure(appearance: Appearance) {
            snapshot(
                MacRootView(
                    view: .initializationFailed(error: .databaseOpen),
                    errorMessage: nil, send: { _ in }, retryInitialization: {}
                ),
                appearance: appearance
            )
        }

        // MARK: Support

        enum Appearance: String, CaseIterable {
            case light, dark

            var name: NSAppearance.Name { self == .light ? .aqua : .darkAqua }
        }

        /// The catalog without its sidebar. The sidebar material draws only
        /// through the window server, so an off-screen capture shows it blank
        /// (white in dark mode). The sidebar has one static item (REC-01).
        private func running(_ list: IngredientListView) -> some View {
            MacCatalogView(
                view: Fixtures.catalog(screen: .list, list: list),
                columnVisibility: .doubleColumn,
                send: { _ in }
            )
        }

        private func snapshot(
            _ view: some View,
            appearance: Appearance,
            fileID: StaticString = #fileID,
            filePath: StaticString = #filePath,
            testName: String = #function,
            line: UInt = #line,
            column: UInt = #column
        ) {
            // Split views and lists draw only inside a window, so the view is
            // hosted in an off-screen window. The window's frame view is
            // captured so the title bar and toolbar are included.
            let window = NSWindow(
                contentRect: CGRect(x: 0, y: 0, width: 960, height: 600),
                styleMask: [.titled, .closable, .miniaturizable, .resizable, .fullSizeContentView],
                backing: .buffered,
                defer: false
            )
            window.isReleasedWhenClosed = false
            window.appearance = NSAppearance(named: appearance.name)
            // The fixed frame keeps SwiftUI from resizing the window to its content.
            window.contentViewController = NSHostingController(
                rootView: view
                    .frame(width: 960, height: 600)
                    .environment(\.locale, Locale(identifier: "en_US"))
            )
            window.setContentSize(CGSize(width: 960, height: 600))
            window.setFrameOrigin(CGPoint(x: -10_000, y: -10_000))
            window.orderFront(nil)
            defer { window.close() }
            // Let SwiftUI lay out the columns and load the list rows.
            RunLoop.main.run(until: Date().addingTimeInterval(0.5))
            let frameView = window.contentView!.superview!
            assertSnapshot(
                of: frameView,
                as: .image(precision: 0.995, perceptualPrecision: 0.98),
                named: appearance.rawValue,
                fileID: fileID,
                file: filePath,
                testName: testName,
                line: line,
                column: column
            )
        }
    }
}
