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
            let model = AppModel(databasePath: ":memory:")
            defer { model.shutdown() }
            _ = try await waitForRunning(model)
            #expect(model.errorMessage == nil)
        }

        @Test func dispatchedEventReturnsThroughRenderCallback() async throws {
            let model = AppModel(databasePath: ":memory:")
            defer { model.shutdown() }
            _ = try await waitForRunning(model)
            model.dispatch(.selectTab(tab: .counter))
            try await waitUntil { running(model)?.tab == .counter }
            #expect(model.errorMessage == nil)
        }

        @Test func rejectedConversionIsVisible() async throws {
            let model = AppModel(databasePath: ":memory:")
            defer { model.shutdown() }
            let app = try await waitForRunning(model)
            model.dispatch(app.ingredients(app.ingredients.list(.select(id: "not-a-uuid"))))
            #expect(model.errorMessage != nil)
        }

        @Test func dispatchAfterShutdownIsVisible() async throws {
            let model = AppModel(databasePath: ":memory:")
            _ = try await waitForRunning(model)
            model.shutdown()
            model.dispatch(.selectTab(tab: .counter))
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
            var model: AppModel? = AppModel(databasePath: ":memory:")
            weak let reference = model
            _ = try await waitForRunning(try #require(model))
            model = nil
            #expect(reference == nil)
        }

        @Test func rendererDeliversBackgroundCallbackBeforeReturning() async {
            let renderer = AppRenderer()
            var calls = 0
            renderer.onRender = { calls += 1 }
            await Task.detached { await renderer.render() }.value
            #expect(calls == 1)
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
        let path = try AppDatabaseLocation.path(applicationSupportDirectory: root)
        // The file name predates the ingredient catalog; keeping it preserves data.
        #expect(path == root.appendingPathComponent("Kreatyna/counter.sqlite").path)
        #expect(try AppDatabaseLocation.path(applicationSupportDirectory: root) == path)
        var isDirectory: ObjCBool = false
        #expect(FileManager.default.fileExists(atPath: root.appendingPathComponent("Kreatyna").path, isDirectory: &isDirectory))
        #expect(isDirectory.boolValue)
    }

    @Test func propagatesDirectoryFailure() throws {
        let root = FileManager.default.temporaryDirectory.appendingPathComponent(UUID().uuidString)
        try Data().write(to: root)
        defer { try? FileManager.default.removeItem(at: root) }
        #expect(throws: (any Error).self) {
            try AppDatabaseLocation.path(applicationSupportDirectory: root)
        }
    }
}
