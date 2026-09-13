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
        model.dispatch(.increase)
        try await waitForValue(1, in: model)
        model.dispatch(.decrease)
        try await waitForValue(0, in: model)
        await model.shutdown()
        #expect(model.errorMessage == nil)
    }

    @MainActor
    @Test func queuedEventsPreserveOrderAndShutdownRejectsDispatch() async throws {
        let model = CounterModel()
        model.dispatch(.decrease)
        model.dispatch(.increase)
        try await waitForValue(1, in: model)
        await model.shutdown()
        model.dispatch(.increase)
        #expect(model.errorMessage != nil)
        #expect(model.value == 1)
    }

    @MainActor
    @Test func shutdownBeforeRunnerStartsCompletes() async {
        let model = CounterModel()
        await model.shutdown()
        #expect(model.errorMessage == nil)
    }

    @MainActor
    private func waitForValue(_ value: UInt64, in model: CounterModel) async throws {
        let clock = ContinuousClock()
        let deadline = clock.now.advanced(by: .seconds(3))
        while model.value != value && clock.now < deadline {
            try await Task.sleep(for: .milliseconds(10))
        }
        #expect(model.value == value)
    }

    @MainActor
    @Test func readsInitialValueFromRust() {
        let model = CounterModel()
        #expect(model.value == 0)
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
