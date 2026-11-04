import Testing
@testable import Kreatyna

/// The Duration field turns typed text into a `DurationInput`; Rust does the
/// rest (REC-17). The generated types compare on the main actor.
@MainActor
struct MacDurationFieldTests {
    @Test(arguments: [
        ("10", DurationInput.minutes(minutes: 10)),
        (" 90 ", .minutes(minutes: 90)),
        ("0", .minutes(minutes: 0)),
        ("", .cleared),
        ("  ", .cleared),
    ])
    func theMinutesFieldSendsItsMinutes(text: String, expected: DurationInput) {
        #expect(MacDurationField.minutesInput(text) == expected)
    }

    /// Letters, signs, other digits, and numbers past `UInt32` send nothing.
    @Test(arguments: ["abc", "1.5", "-3", "١٢", "99999999999"])
    func theMinutesFieldSendsNothingForOtherText(text: String) {
        #expect(MacDurationField.minutesInput(text) == nil)
    }

    @Test(arguments: [("", UInt32?.some(0)), ("7", 7), ("75", 75), ("x", nil), ("-1", nil)])
    func aUnitFieldIsZeroWhenEmpty(text: String, expected: UInt32?) {
        #expect(MacDurationField.unitValue(text) == expected)
    }

    /// More… opens by itself when the minutes field cannot show the duration.
    @Test(arguments: [
        (StepDurationView.notSet, false),
        (Fixtures.duration("10 min", minutes: 10), false),
        (Fixtures.duration("1 min 30 s", minutes: 1, seconds: 30), true),
        (Fixtures.duration("2 d", days: 2), true),
        (.tooLong(parts: DurationParts(days: 400, hours: 0, minutes: 0, seconds: 0)), true),
    ])
    func moreOpensWhenMinutesCannotShowTheDuration(duration: StepDurationView, expected: Bool) {
        #expect(MacDurationField.needsUnits(duration) == expected)
    }

    @Test(arguments: [(UInt32(1), "1 min"), (45, "45 min"), (60, "1 h")])
    func chipsNameTheirLength(minutes: UInt32, expected: String) {
        #expect(MacDurationField.chipLabel(minutes) == expected)
    }
}
