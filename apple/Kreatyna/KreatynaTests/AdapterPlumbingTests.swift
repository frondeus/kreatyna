import Testing
@testable import Kreatyna

struct EventCompositionTests {

    @Test func parentsWrapChildEventsWithDisplayedScopes() {
        let app = Fixtures.running(screen: .list)
        let catalog = app.ingredients

        let event = app.ingredients(catalog.list(.createRequested))

        #expect(event == .ingredients(
            event: .list(scope: ScopeId(value: 2), event: .createRequested)
        ))
        #expect(catalog.back == .back(scope: ScopeId(value: 2)))
    }

    @Test(arguments: [
        (source: 0, destination: 2, expected: (from: UInt64(0), to: UInt64(1))),
        (source: 2, destination: 0, expected: (from: UInt64(2), to: UInt64(0))),
        (source: 0, destination: 3, expected: (from: UInt64(0), to: UInt64(2))),
    ])
    func convertsSwiftUIMoveOffsetToFinalIndex(
        source: Int, destination: Int, expected: (from: UInt64, to: UInt64)
    ) {
        #expect(IngredientFormEvent.move(from: source, toOffset: destination)
            == .moveUnit(from: expected.from, to: expected.to))
    }

    @Test(arguments: [(source: 1, destination: 1), (source: 1, destination: 2)])
    func dropsMoveThatKeepsPosition(source: Int, destination: Int) {
        #expect(IngredientFormEvent.move(from: source, toOffset: destination) == nil)
    }
}

struct TextEchoTests {

    @Test func sendsEditsAndKeepsTypedTextWhileRustCatchesUp() {
        var echo = TextEcho()
        #expect(echo.typed("a", displayed: "") == "a")
        #expect(echo.typed("ab", displayed: "") == "ab")

        // A render for the first edit must not erase the second one.
        #expect(echo.received("a", showing: "ab") == nil)
        #expect(echo.received("ab", showing: "ab") == nil)
        #expect(echo.pending.isEmpty)
    }

    @Test func showsValueChangedIndependentlyByRust() {
        var echo = TextEcho()
        _ = echo.typed("garlic", displayed: "")
        #expect(echo.received("onion", showing: "garlic") == "onion")
        #expect(echo.pending.isEmpty)
    }

    @Test func doesNotEchoRustValueBack() {
        var echo = TextEcho()
        #expect(echo.typed("onion", displayed: "onion") == nil)
    }
}
