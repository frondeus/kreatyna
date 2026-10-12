//
//  KreatynaTests.swift
//  KreatynaTests
//
//  Created by Wojciech  Polak on 13/09/2026.
//

import Foundation
import Testing
@testable import Kreatyna

/// Runs main-actor suites one test at a time. Snapshot rendering uses the key
/// window, and the real-core tests need the main actor free for render callbacks.
@Suite(.serialized) struct MainActorTests {}

// These tests check the Swift adapter around the real Rust core. Rust tests own
// the app's behavior; a test here sends at most one event to prove a round trip.
extension MainActorTests {
    @MainActor
    struct AppModelTests {

        @Test func startsAndRendersRunningApp() async throws {
            let model = AppModel(layout: .stack, databasePath: ":memory:")
            defer { model.shutdown() }
            _ = try await waitForRunning(model)
            #expect(model.errorMessage == nil)
        }

        @Test func dispatchedEventReturnsThroughRenderCallback() async throws {
            let model = AppModel(layout: .stack, databasePath: ":memory:")
            defer { model.shutdown() }
            let app = try await waitForRunning(model)
            model.dispatch(app.ingredients(stack(app).list(.search(query: "gar"))))
            try await waitUntil { running(model)?.ingredients.stack?.list.search == "gar" }
            #expect(model.errorMessage == nil)
        }

        @Test func rejectedConversionIsVisible() async throws {
            let model = AppModel(layout: .stack, databasePath: ":memory:")
            defer { model.shutdown() }
            let app = try await waitForRunning(model)
            model.dispatch(app.ingredients(stack(app).list(.select(id: "not-a-uuid"))))
            #expect(model.errorMessage != nil)
        }

        @Test func dispatchAfterShutdownIsVisible() async throws {
            let model = AppModel(layout: .stack, databasePath: ":memory:")
            let app = try await waitForRunning(model)
            model.shutdown()
            model.dispatch(app.ingredients(stack(app).list(.search(query: "gar"))))
            #expect(model.errorMessage != nil)
        }

        @Test func startupFailureRendersAndRetryIsForwarded() async throws {
            let root = FileManager.default.temporaryDirectory.appendingPathComponent(UUID().uuidString)
            defer { try? FileManager.default.removeItem(at: root) }
            // SQLite cannot create a file in a missing directory.
            let path = root.appendingPathComponent("app.sqlite").path
            let model = AppModel(databasePath: path)
            defer { model.shutdown() }
            try await waitUntil {
                if case .initializationFailed = model.view { true } else { false }
            }

            try FileManager.default.createDirectory(at: root, withIntermediateDirectories: true)
            model.retryInitialization()
            _ = try await waitForRunning(model)
            #expect(model.errorMessage == nil)
        }

        @Test func rustCallbackDoesNotRetainModel() async throws {
            var model: AppModel? = AppModel(layout: .stack, databasePath: ":memory:")
            weak let reference = model
            _ = try await waitForRunning(try #require(model))
            model = nil
            #expect(reference == nil)
        }

        @Test func seededIngredientAppearsInList() async throws {
            let root = FileManager.default.temporaryDirectory.appendingPathComponent(UUID().uuidString)
            try FileManager.default.createDirectory(at: root, withIntermediateDirectories: true)
            defer { try? FileManager.default.removeItem(at: root) }
            let path = root.appendingPathComponent("app.sqlite").path
            try seedIngredients(databasePath: path, json: #"["Garlic"]"#)

            let model = AppModel(layout: .stack, databasePath: path)
            defer { model.shutdown() }
            try await waitUntil {
                guard case .ready(let rows, _, _) = running(model)?.ingredients.stack?.list.status else { return false }
                return rows.map(\.name) == ["Garlic"]
            }
            #expect(model.errorMessage == nil)
        }

        @Test func rendererDeliversBackgroundCallbackBeforeReturning() async {
            let renderer = AppRenderer()
            var calls = 0
            renderer.onRender = { calls += 1 }
            await Task.detached { await renderer.render() }.value
            #expect(calls == 1)
        }

        @Test func splitLayoutEventReturnsThroughRenderCallback() async throws {
            let model = AppModel(layout: .split, databasePath: ":memory:")
            defer { model.shutdown() }
            let app = try await waitForRunning(model)
            let catalog = try #require(app.ingredients.split)
            model.dispatch(app.splitIngredients(catalog.list(.search(query: "gar"))))
            try await waitUntil { running(model)?.ingredients.split?.list.value.search == "gar" }
            #expect(model.errorMessage == nil)
        }

        private func stack(_ app: RunningAppView) -> IngredientCatalogView {
            app.ingredients.stack!
        }

        private func running(_ model: AppModel) -> RunningAppView? {
            if case .running(let app) = model.view { app } else { nil }
        }

        private func waitForRunning(_ model: AppModel) async throws -> RunningAppView {
            try await waitUntil { running(model) != nil }
            return try #require(running(model))
        }

        private func waitUntil(_ condition: () -> Bool) async throws {
            let clock = ContinuousClock()
            let deadline = clock.now.advanced(by: .seconds(3))
            while !condition() && clock.now < deadline {
                try await Task.sleep(for: .milliseconds(10))
            }
            try #require(condition())
        }
    }
}

struct AppDatabaseLocationTests {

    @Test func createsStableDirectory() throws {
        let root = FileManager.default.temporaryDirectory.appendingPathComponent(UUID().uuidString)
        defer { try? FileManager.default.removeItem(at: root) }
        let path = try AppDatabaseLocation.path(applicationSupportDirectory: root, environment: [:])
        #expect(path == root.appendingPathComponent("Kreatyna/kreatyna.sqlite").path)
        #expect(try AppDatabaseLocation.path(applicationSupportDirectory: root, environment: [:]) == path)
        var isDirectory: ObjCBool = false
        #expect(FileManager.default.fileExists(atPath: root.appendingPathComponent("Kreatyna").path, isDirectory: &isDirectory))
        #expect(isDirectory.boolValue)
    }

    @Test func unitTestHostUsesMemoryDatabase() throws {
        let path = try AppDatabaseLocation.path(environment: ["XCTestConfigurationFilePath": "/test.xctestconfiguration"])
        #expect(path == ":memory:")
    }

    @Test func uiTestDatabaseIsInTemporaryFolder() throws {
        let id = UUID()
        let path = try AppDatabaseLocation.path(environment: ["KREATYNA_TEST_DATABASE_ID": id.uuidString])
        let directory = FileManager.default.temporaryDirectory.appendingPathComponent("KreatynaUITests/\(id.uuidString)")
        defer { try? FileManager.default.removeItem(at: directory) }
        #expect(path == directory.appendingPathComponent("kreatyna.sqlite").path)
        #expect(FileManager.default.fileExists(atPath: directory.path))
    }

    @Test func propagatesDirectoryFailure() throws {
        let root = FileManager.default.temporaryDirectory.appendingPathComponent(UUID().uuidString)
        try Data().write(to: root)
        defer { try? FileManager.default.removeItem(at: root) }
        #expect(throws: (any Error).self) {
            try AppDatabaseLocation.path(applicationSupportDirectory: root, environment: [:])
        }
    }
}

struct TestSeedTests {

    @Test func passesJSONTextForIsolatedDatabase() {
        let json = #"["Garlic", {"name": "Tomato", "calories": 18}]"#
        let environment = ["KREATYNA_TEST_DATABASE_ID": UUID().uuidString, "KREATYNA_TEST_SEED": json]
        #expect(TestSeed.json(environment: environment) == json)
    }

    @Test func ignoresSeedWithoutIsolatedDatabase() {
        #expect(TestSeed.json(environment: ["KREATYNA_TEST_SEED": #"["Garlic"]"#]) == nil)
    }

    @Test func noSeedWithoutSeedVariable() {
        #expect(TestSeed.json(environment: ["KREATYNA_TEST_DATABASE_ID": UUID().uuidString]) == nil)
    }
}
