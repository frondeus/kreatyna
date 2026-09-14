//
//  MausritterTests.swift
//  MausritterTests
//
//  Created by Wojciech  Polak on 13/09/2026.
//

import Testing
@testable import Mausritter

struct MausritterTests {

    @MainActor
    @Test func dispatchRefreshesValueThroughRustCallback() async throws {
        let model = CounterModel()
        defer { model.shutdown() }
        try await waitForValue(42, in: model)
        model.dispatch(.increase)
        try await waitForValue(43, in: model)
        model.dispatch(.decrease)
        try await waitForValue(42, in: model)
        #expect(model.errorMessage == nil)
    }

    @MainActor
    @Test func queuedEventsPreserveOrderAndShutdownRejectsDispatch() async throws {
        let model = CounterModel()
        defer { model.shutdown() }
        try await waitForValue(42, in: model)
        for _ in 0..<42 {
            model.dispatch(.decrease)
        }
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
        let model = CounterModel()
        model.shutdown()
        #expect(model.errorMessage == nil)
        model.dispatch(.increase)
        #expect(model.errorMessage != nil)
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
        let model = CounterModel()
        defer { model.shutdown() }
        try await waitForValue(42, in: model)
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
        var model: CounterModel? = CounterModel()
        weak var reference = model
        model = nil
        #expect(reference == nil)
    }

}
