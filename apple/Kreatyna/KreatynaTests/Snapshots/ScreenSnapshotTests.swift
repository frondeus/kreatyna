import SnapshotTesting
import SwiftUI
import Testing
import UIKit
@testable import Kreatyna

// Rendering conditions are fixed: device size, light mode, US English, and the
// default text size. Tests run serially because each one uses the key window.
// Baselines were recorded on the iOS 26.3 simulator; other runtimes can render
// slightly differently. Review every diff before re-recording.
extension MainActorTests {
    @MainActor
    @Suite(.snapshots(record: .missing))
    struct ScreenSnapshotTests {

        // MARK: App startup

        @Test func initializing() {
            snapshot(RootView(view: .initializing, errorMessage: nil, send: { _ in }, retryInitialization: {}))
        }

        @Test func initializationFailed() {
            snapshot(RootView(
                view: .initializationFailed(error: .databaseMigration),
                errorMessage: nil, send: { _ in }, retryInitialization: {}
            ))
        }

        @Test func runningCatalog() {
            snapshot(RootView(
                view: .running(app: Fixtures.running(screen: .list)),
                errorMessage: nil, send: { _ in }, retryInitialization: {}
            ))
        }

        @Test func adapterError() {
            snapshot(RootView(
                view: .running(app: Fixtures.running(screen: .list)),
                errorMessage: "The app could not accept this action.",
                send: { _ in }, retryInitialization: {}
            ))
        }

        // MARK: Ingredient list

        @Test(arguments: [
            ("loading", IngredientListStatus.loading),
            ("failed", .failed(error: .storage)),
            ("empty", .ready(rows: [], totalCount: 0, refresh: .idle)),
            ("rows", .ready(rows: Fixtures.rows, totalCount: 3, refresh: .idle)),
            ("refreshFailed", .ready(rows: Fixtures.rows, totalCount: 3, refresh: .failed(error: .transport))),
        ])
        func list(name: String, status: IngredientListStatus) {
            snapshot(listScreen(Fixtures.list(status)), named: name)
        }

        @Test func listWithoutSearchMatches() {
            let list = Fixtures.list(.ready(rows: [], totalCount: 3, refresh: .idle), search: "basil")
            snapshot(listScreen(list))
        }

        // MARK: Ingredient detail

        @Test(arguments: [
            ("idle", IngredientDetailDeletion.idle),
            ("deleteFailed", .confirming(confirmation: .resolved(decision: .confirmed), error: .storage)),
            ("deleting", .deleting),
        ])
        func detail(name: String, deletion: IngredientDetailDeletion) {
            snapshot(NavigationStack { IngredientDetailScreen(view: Fixtures.detail(deletion), send: { _ in }) }, named: name)
        }

        // MARK: Ingredient form

        @Test func formCreateBlank() {
            snapshot(formScreen(Fixtures.form(Fixtures.blankDraft)))
        }

        @Test func formEditWithUnits() {
            snapshot(formScreen(Fixtures.form(Fixtures.garlicDraft)))
        }

        @Test func formValidationIssues() {
            var draft = Fixtures.garlicDraft
            draft.name = ""
            draft.calories = "lots"
            draft.units[0].grams = "0"
            let issues = [
                ValidationIssue(field: .name, reason: .required),
                ValidationIssue(field: .calories, reason: .invalidNumber),
                ValidationIssue(field: .unitGrams(index: 0), reason: .mustBePositive),
            ]
            snapshot(formScreen(Fixtures.form(draft, dirty: true, issues: issues)))
        }

        @Test func formDuplicateNameSaveError() {
            snapshot(formScreen(Fixtures.form(Fixtures.garlicDraft, dirty: true, saveError: .duplicateName)))
        }

        @Test func formSaving() {
            snapshot(formScreen(Fixtures.form(Fixtures.garlicDraft, dirty: true, status: .saving)))
        }

        // MARK: Support

        private func listScreen(_ view: IngredientListView) -> some View {
            NavigationStack { IngredientListScreen(view: view, send: { _ in }) }
        }

        private func formScreen(_ view: IngredientFormView) -> some View {
            NavigationStack { IngredientFormScreen(view: view, send: { _ in }, cancel: {}) }
        }

        private func snapshot(
            _ view: some View,
            named name: String? = nil,
            fileID: StaticString = #fileID,
            filePath: StaticString = #filePath,
            testName: String = #function,
            line: UInt = #line,
            column: UInt = #column
        ) {
            let controller = UIHostingController(
                rootView: view.environment(\.locale, Locale(identifier: "en_US"))
            )
            controller.overrideUserInterfaceStyle = .light
            assertSnapshot(
                of: controller,
                as: .image(
                    on: .iPhone13Pro,
                    // System bars use glass materials that render only in a window.
                    drawHierarchyInKeyWindow: true,
                    // Spinners keep animating in a window; allow only their few pixels to differ.
                    precision: 0.995,
                    perceptualPrecision: 0.98,
                    traits: UITraitCollection { traits in
                        traits.userInterfaceStyle = .light
                        traits.preferredContentSizeCategory = .large
                    }
                ),
                named: name,
                fileID: fileID,
                file: filePath,
                testName: testName,
                line: line,
                column: column
            )
        }
    }
}
