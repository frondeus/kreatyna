import CoreGraphics
import Testing
@testable import Kreatyna

/// Add Step sends the last pointer location as a `CanvasPoint`. The
/// conversion must give whole, non-negative points and never trap. The
/// generated `CanvasPoint` compares on the main actor.
@MainActor
struct MacStepCanvasTests {
    @Test(arguments: [
        (CGPoint(x: 120.4, y: 80.6), CanvasPoint(x: 120, y: 81)),
        (CGPoint(x: -12, y: -0.4), CanvasPoint(x: 0, y: 0)),
        (CGPoint(x: CGFloat.nan, y: 40), CanvasPoint(x: 0, y: 40)),
        (CGPoint(x: CGFloat.infinity, y: -CGFloat.infinity), CanvasPoint(x: 0, y: 0)),
        (CGPoint(x: 1e12, y: CGFloat(UInt32.max) + 0.4), CanvasPoint(x: .max, y: .max)),
    ])
    func canvasPointIsWholeAndInRange(location: CGPoint, expected: CanvasPoint) {
        #expect(MacStepCanvas.canvasPoint(location) == expected)
    }

    /// A dragged node follows the pointer, and stops at the left and the
    /// top edge (REC-15).
    @Test(arguments: [
        (CGSize(width: 200, height: 120), CGPoint(x: 240, y: 160)),
        (CGSize(width: -15.5, height: -30), CGPoint(x: 24.5, y: 10)),
        (CGSize(width: -100, height: -60), CGPoint(x: 0, y: 0)),
    ])
    func dragFollowsThePointerAndStopsAtTheEdges(translation: CGSize, expected: CGPoint) {
        #expect(MacStepCanvas.moved(CanvasPoint(x: 40, y: 40), by: translation) == expected)
    }

    /// An arrow runs from the ● on the right of `from` to just left of the
    /// ○ of `to`, both half way down the node (REC-18).
    @Test func anArrowRunsFromTheOutPortToTheInPort() {
        let arrow = MacArrow(
            start: MacStepCanvas.outPort(at: CGPoint(x: 20, y: 40)),
            end: MacStepCanvas.inPort(at: CGPoint(x: 270, y: 140)),
            selected: false
        )
        #expect(arrow.start == CGPoint(x: 170, y: 72))
        #expect(arrow.end == CGPoint(x: 264, y: 172))
        #expect(arrow.point(at: 0) == arrow.start)
        #expect(arrow.point(at: 1) == arrow.end)
    }

    /// A click within 5 points of an arrow selects it; farther away it
    /// clears the selection (REC-18/Q1).
    @Test(arguments: [
        (CGPoint(x: 217, y: 122), true),
        (CGPoint(x: 217, y: 126), true),
        (CGPoint(x: 217, y: 140), false),
        (CGPoint(x: 400, y: 300), false),
    ])
    func aClickNearAnArrowFindsIt(point: CGPoint, found: Bool) {
        let steps = [
            Fixtures.stepNode("Dice onion", x: 20, y: 40),
            Fixtures.stepNode("Fry onion", x: 270, y: 140),
        ]
        let edge = Fixtures.edge("Dice onion", "Fry onion")
        // The curve is symmetric, so its middle is half way between the ports.
        let hit = MacStepCanvas.connection(near: point, among: steps, in: [edge])
        #expect((hit == edge) == found)
    }

    /// A drop anywhere on a node connects to it; the topmost node wins
    /// where two overlap; empty canvas finds nothing (REC-18/Q3).
    @Test(arguments: [
        (CGPoint(x: 30, y: 50), "Dice onion"),
        (CGPoint(x: 200, y: 100), "Fry onion"),
        (CGPoint(x: 600, y: 400), nil),
    ] as [(CGPoint, String?)])
    func aDropFindsTheNodeUnderIt(point: CGPoint, title: String?) {
        let steps = [
            Fixtures.stepNode("Dice onion", x: 20, y: 40),
            Fixtures.stepNode("Fry onion", x: 120, y: 80),
        ]
        #expect(MacStepCanvas.node(at: point, among: steps)?.title == title)
    }
}
