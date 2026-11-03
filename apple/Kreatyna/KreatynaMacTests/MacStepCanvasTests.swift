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
}
