import AppKit
import SnapshotTesting
import SwiftUI
import Testing
@testable import Kreatyna

// Rendering conditions are fixed: a 960 × 600 point window, US English, and
// light and dark appearance. The window is never key, so controls show their
// inactive look, and its title text is transparent (MAC-12). Tests run serially (`MainActorTests`). Baselines
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
        func filteredCatalog(appearance: Appearance) {
            let rows = [IngredientListRow(id: Fixtures.tomatoesId, name: "Canned tomatoes")]
            snapshot(
                running(Fixtures.list(.ready(rows: rows, totalCount: 3, refresh: .idle), search: "tom")),
                appearance: appearance
            )
        }

        @Test(arguments: Appearance.allCases)
        func noMatches(appearance: Appearance) {
            snapshot(
                running(Fixtures.list(.ready(rows: [], totalCount: 3, refresh: .idle), search: "tomx")),
                appearance: appearance
            )
        }

        @Test(arguments: Appearance.allCases)
        func detailWithFullData(appearance: Appearance) {
            snapshot(running(Fixtures.list(), selected: Fixtures.garlic), appearance: appearance)
        }

        @Test(arguments: Appearance.allCases)
        func detailWithNameOnly(appearance: Appearance) {
            let rows = [IngredientListRow(id: Fixtures.basil.id, name: "Basil")] + Fixtures.rows
            snapshot(
                running(Fixtures.list(.ready(rows: rows, totalCount: 4, refresh: .idle)), selected: Fixtures.basil),
                appearance: appearance
            )
        }

        /// Rust is deleting Garlic: the overlay shows, and Edit and Delete are off (MAC-07).
        @Test(arguments: Appearance.allCases)
        func deletingIngredient(appearance: Appearance) {
            snapshot(
                running(Fixtures.list(), selected: Fixtures.garlic, deletion: .deleting),
                appearance: appearance
            )
        }

        @Test(arguments: Appearance.allCases)
        func newIngredientForm(appearance: Appearance) {
            var draft = Fixtures.blankDraft
            draft.units = [ReferenceUnitDraft(name: "", grams: "", millilitres: "")]
            snapshot(creating(Fixtures.form(draft, dirty: true)), appearance: appearance)
        }

        @Test(arguments: Appearance.allCases)
        func formWithIssues(appearance: Appearance) {
            var draft = Fixtures.blankDraft
            draft.calories = "abc"
            draft.units = [ReferenceUnitDraft(name: "clove", grams: "", millilitres: "")]
            let issues = [
                ValidationIssue(field: .name, reason: .required),
                ValidationIssue(field: .calories, reason: .invalidNumber),
                ValidationIssue(field: .unitMeasurements(index: 0), reason: .required),
            ]
            snapshot(creating(Fixtures.form(draft, dirty: true, issues: issues)), appearance: appearance)
        }

        @Test(arguments: Appearance.allCases)
        func duplicateName(appearance: Appearance) {
            var draft = Fixtures.blankDraft
            draft.name = "garlic"
            snapshot(
                creating(Fixtures.form(draft, dirty: true, saveError: .duplicateName)),
                appearance: appearance
            )
        }

        /// The title and the highlighted row keep the saved name (MAC-06/Q1).
        @Test(arguments: Appearance.allCases)
        func editIngredientForm(appearance: Appearance) {
            var draft = Fixtures.garlicDraft
            draft.name = "Black Garlic"
            snapshot(editing(Fixtures.form(draft, dirty: true)), appearance: appearance)
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

        /// The database stopped while the app ran (P06).
        @Test(arguments: Appearance.allCases)
        func databaseStopped(appearance: Appearance) {
            snapshot(
                MacRootView(
                    view: .initializationFailed(error: .databaseStopped),
                    errorMessage: nil, send: { _ in }, retryInitialization: {}
                ),
                appearance: appearance
            )
        }

        /// A bug stopped the Rust core (vibetyna-dm7).
        @Test(arguments: Appearance.allCases)
        func crashed(appearance: Appearance) {
            snapshot(
                MacRootView(view: .crashed, errorMessage: nil, send: { _ in }, retryInitialization: {}),
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
        private func running(
            _ list: IngredientListView,
            selected: Ingredient? = nil,
            deletion: IngredientDetailDeletion = .idle
        ) -> some View {
            MacCatalogView(
                view: Fixtures.splitCatalog(list: list, selected: selected, deletion: deletion),
                columnVisibility: .doubleColumn,
                send: { _ in }
            )
        }

        /// The catalog with the create form open (MAC-05).
        private func creating(_ form: IngredientFormView) -> some View {
            MacCatalogView(
                view: Fixtures.splitCatalog(form: form),
                columnVisibility: .doubleColumn,
                send: { _ in }
            )
        }

        /// The catalog with Garlic's edit form open (MAC-06).
        private func editing(_ form: IngredientFormView) -> some View {
            MacCatalogView(
                view: Fixtures.splitCatalog(editing: Fixtures.garlic, form: form),
                columnVisibility: .doubleColumn,
                send: { _ in }
            )
        }

        /// A window that never becomes key or main, so every snapshot shows the
        /// inactive look. Whether a normal window is key depends on what else
        /// is active on the Mac, which changed the title color between runs.
        private final class InactiveWindow: NSWindow {
            override var canBecomeKey: Bool { false }
            override var canBecomeMain: Bool { false }
        }

        /// MAC-12: after a long UI test run on an idle Mac, macOS draws the
        /// toolbar title darker, and no window property shows why. The title's
        /// text becomes transparent but keeps its space, so the toolbar items
        /// stay where the app shows them.
        private func hideTitleText(in view: NSView, title: String) {
            // An empty title would match the empty search field.
            if !title.isEmpty, let field = view as? NSTextField, field.stringValue == title {
                field.alphaValue = 0
            }
            for subview in view.subviews {
                hideTitleText(in: subview, title: title)
            }
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
            let window = InactiveWindow(
                contentRect: CGRect(x: 0, y: 0, width: 960, height: 600),
                styleMask: [.titled, .closable, .miniaturizable, .resizable, .fullSizeContentView],
                backing: .buffered,
                defer: false
            )
            window.isReleasedWhenClosed = false
            window.appearance = NSAppearance(named: appearance.name)
            // The search field's text follows the app's appearance, not the
            // window's, so the app takes the snapshot's appearance too.
            let appAppearance = NSApp.appearance
            NSApp.appearance = window.appearance
            defer { NSApp.appearance = appAppearance }
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
            // An off-screen window gives its first key view (the search
            // field) focus; the launched app focuses nothing.
            window.makeFirstResponder(nil)
            RunLoop.main.run(until: Date().addingTimeInterval(0.1))
            let frameView = window.contentView!.superview!
            hideTitleText(in: frameView, title: window.title)
            assertSnapshot(
                of: frameView,
                // Every pixel is compared; 0.99 allows only anti-aliasing noise.
                // The old 0.995/0.98 let a whole toolbar search field pass (MAC-08).
                as: .image(precision: 1, perceptualPrecision: 0.99),
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
