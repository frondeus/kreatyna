//
//  KreatynaTests.swift
//  KreatynaTests
//
//  Created by Wojciech  Polak on 13/09/2026.
//

import Foundation
import Testing
@testable import Kreatyna

struct KreatynaTests {

    @MainActor
    @Test func dispatchRefreshesValueThroughRustCallback() async throws {
        let model = CounterModel(databasePath: ":memory:")
        defer { model.shutdown() }
        try await waitForInitialRender(in: model)
        model.dispatch(.increase)
        try await waitForValue(1, in: model)
        model.dispatch(.decrease)
        try await waitForValue(0, in: model)
        model.dispatch(.increase)
        try await waitForValue(1, in: model)
        model.dispatch(.reset)
        try await waitForValue(0, in: model)
        #expect(model.errorMessage == nil)
    }

    @MainActor
    @Test func queuedEventsPreserveOrderAndShutdownRejectsDispatch() async throws {
        let model = CounterModel(databasePath: ":memory:")
        defer { model.shutdown() }
        try await waitForInitialRender(in: model)
        try await waitForValue(0, in: model)
        model.dispatch(.decrease)
        model.dispatch(.increase)
        try await waitForValue(1, in: model)
        model.shutdown()
        model.dispatch(.increase)
        #expect(model.errorMessage != nil)
        #expect(model.value == 1)
    }

    @MainActor
    @Test func immediateShutdownRejectsDispatch() {
        let model = CounterModel(databasePath: ":memory:")
        model.shutdown()
        #expect(model.errorMessage == nil)
        model.dispatch(.increase)
        #expect(model.errorMessage != nil)
    }

    @MainActor
    private func waitForInitialRender(in model: CounterModel) async throws {
        let clock = ContinuousClock()
        let deadline = clock.now.advanced(by: .seconds(3))
        while !model.hasRendered && clock.now < deadline {
            try await Task.sleep(for: .milliseconds(10))
        }
        try #require(model.hasRendered)
    }

    @MainActor
    @Test func databaseLocationCreatesStableDirectory() throws {
        let root = FileManager.default.temporaryDirectory.appendingPathComponent(UUID().uuidString)
        defer { try? FileManager.default.removeItem(at: root) }
        let path = try CounterDatabaseLocation.path(applicationSupportDirectory: root)
        #expect(path == root.appendingPathComponent("Kreatyna/counter.sqlite").path)
        #expect(try CounterDatabaseLocation.path(applicationSupportDirectory: root) == path)
        var isDirectory: ObjCBool = false
        #expect(FileManager.default.fileExists(atPath: root.appendingPathComponent("Kreatyna").path, isDirectory: &isDirectory))
        #expect(isDirectory.boolValue)
    }

    @MainActor
    @Test func databaseLocationPropagatesDirectoryFailure() throws {
        let root = FileManager.default.temporaryDirectory.appendingPathComponent(UUID().uuidString)
        try Data().write(to: root)
        defer { try? FileManager.default.removeItem(at: root) }
        #expect(throws: (any Error).self) {
            try CounterDatabaseLocation.path(applicationSupportDirectory: root)
        }
    }

    @MainActor
    @Test func fileDatabasePersistsAcrossCoreInstances() async throws {
        let root = FileManager.default.temporaryDirectory.appendingPathComponent(UUID().uuidString)
        defer { try? FileManager.default.removeItem(at: root) }
        let path = try CounterDatabaseLocation.path(applicationSupportDirectory: root)
        let writer = CounterModel(databasePath: path)
        defer { writer.shutdown() }
        try await waitForInitialRender(in: writer)
        writer.dispatch(.increase)
        try await waitForValue(1, in: writer)

        // Rendering precedes commit; observe persistence through a fresh Rust reader.
        let clock = ContinuousClock()
        let deadline = clock.now.advanced(by: .seconds(3))
        var stored: UInt64 = 0
        repeat {
            let reader = CounterModel(databasePath: path)
            try await waitForInitialRender(in: reader)
            stored = reader.value
            reader.shutdown()
            if stored == 1 { break }
            try await Task.sleep(for: .milliseconds(10))
        } while clock.now < deadline
        #expect(stored == 1)
    }

    @MainActor
    private func waitForValue(_ value: UInt64, in model: CounterModel) async throws {
        let clock = ContinuousClock()
        let deadline = clock.now.advanced(by: .seconds(3))
        while model.value != value && clock.now < deadline {
            try await Task.sleep(for: .milliseconds(10))
        }
        try #require(model.value == value)
    }

    @MainActor
    @Test func loadsInitialValueWithoutSwiftRunner() async throws {
        let model = CounterModel(databasePath: ":memory:")
        defer { model.shutdown() }
        try await waitForInitialRender(in: model)
        #expect(model.errorMessage == nil)
    }

    @MainActor
    @Test func rendererDeliversBackgroundCallbackBeforeReturning() async {
        let renderer = CounterRenderer()
        var calls = 0
        renderer.onRender = { calls += 1 }
        await Task.detached { await renderer.render() }.value
        #expect(calls == 1)
    }

    @MainActor
    @Test func rustCallbackDoesNotRetainModel() {
        var model: CounterModel? = CounterModel(databasePath: ":memory:")
        weak var reference = model
        model = nil
        #expect(reference == nil)
    }

}
