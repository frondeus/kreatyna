import SwiftUI

/// The inspector's Duration field (REC-17/Q1, variant E): a minutes field,
/// chips for the common lengths, and More… with a field for each unit.
///
/// Rust owns the duration: it adds the units up, checks the range, and
/// gives back what each field shows. Swift only turns typed text into a
/// `DurationInput`.
struct MacDurationField: View {
    let duration: StepDurationView
    let send: (DurationInput) -> Void

    /// Presentation only: More… opened by a click.
    @State private var moreOpened = false

    /// The chips, in minutes. 60 shows as "1 h".
    static let chips: [UInt32] = [1, 2, 5, 10, 15, 20, 30, 45, 60]

    var body: some View {
        VStack(alignment: .leading, spacing: 8) {
            HStack(spacing: 6) {
                SyncedTextField("Minutes", value: minutesText, prompt: minutesPrompt) { text in
                    if let input = Self.minutesInput(text) { send(input) }
                }
                .labelsHidden()
                .textFieldStyle(.roundedBorder)
                .frame(width: 64)
                .accessibilityIdentifier("durationMinutesField")
                Text("min").foregroundStyle(.secondary)
                Spacer(minLength: 0)
                if !Self.needsUnits(duration) {
                    Button(moreOpened ? "Less" : "More…") { moreOpened.toggle() }
                        .buttonStyle(.link)
                        .accessibilityIdentifier("durationMoreButton")
                }
            }
            chipGrid
            if moreOpened || Self.needsUnits(duration) {
                MacDurationUnits(parts: parts, send: send)
            }
        }
    }

    private var chipGrid: some View {
        LazyVGrid(columns: Array(repeating: GridItem(.flexible(), spacing: 4), count: 3), spacing: 4) {
            ForEach(Self.chips, id: \.self) { minutes in
                chip(minutes, on: wholeMinutes == minutes)
            }
        }
    }

    private func chip(_ minutes: UInt32, on: Bool) -> some View {
        Button(Self.chipLabel(minutes)) { send(.minutes(minutes: minutes)) }
            .buttonStyle(MacDurationChipStyle(on: on))
            .accessibilityAddTraits(on ? .isSelected : [])
            .accessibilityIdentifier("durationChip\(minutes)")
    }

    private var wholeMinutes: UInt32? {
        if case .set(_, _, let minutes) = duration { minutes } else { nil }
    }

    private var parts: DurationParts? {
        switch duration {
        case .notSet: nil
        case .set(_, let parts, _), .tooLong(let parts): parts
        }
    }

    private var minutesText: String {
        wholeMinutes.map(String.init) ?? ""
    }

    /// "…" when the duration cannot show in whole minutes.
    private var minutesPrompt: String {
        if case .notSet = duration { "" } else if wholeMinutes == nil { "…" } else { "" }
    }

    /// More… opens by itself when the minutes field cannot show the
    /// duration, or when it has days.
    static func needsUnits(_ duration: StepDurationView) -> Bool {
        switch duration {
        case .notSet: false
        case .set(_, let parts, let wholeMinutes): wholeMinutes == nil || parts.days > 0
        case .tooLong: true
        }
    }

    static func chipLabel(_ minutes: UInt32) -> String {
        minutes == 60 ? "1 h" : "\(minutes) min"
    }

    /// The minutes field: digits send `.minutes`, an empty field `.cleared`,
    /// and anything else nothing.
    static func minutesInput(_ text: String) -> DurationInput? {
        let trimmed = text.trimmingCharacters(in: .whitespaces)
        if trimmed.isEmpty { return .cleared }
        return wholeNumber(trimmed).map { .minutes(minutes: $0) }
    }

    /// A unit field: an empty field is 0, digits are their number, and
    /// anything else is `nil`.
    static func unitValue(_ text: String) -> UInt32? {
        let trimmed = text.trimmingCharacters(in: .whitespaces)
        return trimmed.isEmpty ? 0 : wholeNumber(trimmed)
    }

    private static func wholeNumber(_ text: String) -> UInt32? {
        guard text.allSatisfy(\.isASCII), text.allSatisfy(\.isNumber) else { return nil }
        return UInt32(text)
    }
}

/// A chip: filled with the accent color when it is the duration, in an
/// active window or not.
private struct MacDurationChipStyle: ButtonStyle {
    let on: Bool

    func makeBody(configuration: Configuration) -> some View {
        let shape = RoundedRectangle(cornerRadius: 6)
        configuration.label
            .font(.callout)
            .foregroundStyle(on ? Color.white : Color.primary)
            .frame(maxWidth: .infinity)
            .padding(.vertical, 3)
            .background(on ? Color.accentColor : Color(nsColor: .controlColor), in: shape)
            .overlay(shape.strokeBorder(Color(nsColor: .separatorColor), lineWidth: on ? 0 : 1))
            .opacity(configuration.isPressed ? 0.7 : 1)
            .contentShape(shape)
    }
}

/// The unit fields under More…: days, hours, minutes, and seconds, each with
/// ↑ ↓ arrows.
private struct MacDurationUnits: View {
    /// `nil` when there is no duration: the fields are empty.
    let parts: DurationParts?
    let send: (DurationInput) -> Void

    private enum Unit {
        case days, hours, minutes, seconds

        var label: String {
            switch self {
            case .days: "d"
            case .hours: "h"
            case .minutes: "min"
            case .seconds: "s"
            }
        }

        /// The arrows stay in the unit's range. Typed values are Rust's to check.
        var range: ClosedRange<UInt32> {
            switch self {
            case .days: 0...365
            case .hours: 0...23
            case .minutes, .seconds: 0...59
            }
        }

        var keyPath: WritableKeyPath<DurationParts, UInt32> {
            switch self {
            case .days: \.days
            case .hours: \.hours
            case .minutes: \.minutes
            case .seconds: \.seconds
            }
        }
    }

    var body: some View {
        VStack(alignment: .leading, spacing: 4) {
            Grid(alignment: .leading, horizontalSpacing: 12, verticalSpacing: 6) {
                GridRow {
                    unitField(.days)
                    unitField(.hours)
                }
                GridRow {
                    unitField(.minutes)
                    unitField(.seconds)
                }
            }
            Text("Empty fields count as 0.")
                .font(.caption)
                .foregroundStyle(.secondary)
        }
    }

    private func unitField(_ unit: Unit) -> some View {
        let current = parts ?? DurationParts(days: 0, hours: 0, minutes: 0, seconds: 0)
        let value = current[keyPath: unit.keyPath]
        return HStack(spacing: 1) {
            SyncedTextField(unit.label, value: parts == nil ? "" : String(value), prompt: "0") { text in
                guard let typed = MacDurationField.unitValue(text) else { return }
                var next = current
                next[keyPath: unit.keyPath] = typed
                send(.parts(parts: next))
            }
            .labelsHidden()
            .textFieldStyle(.roundedBorder)
            .frame(width: 44)
            .accessibilityIdentifier("durationUnit-\(unit.label)")
            Stepper(unit.label) {
                step(unit, from: current, by: 1)
            } onDecrement: {
                step(unit, from: current, by: -1)
            }
            .labelsHidden()
            Text(unit.label)
                .font(.caption)
                .foregroundStyle(.secondary)
        }
    }

    private func step(_ unit: Unit, from current: DurationParts, by delta: Int64) {
        let value = Int64(current[keyPath: unit.keyPath]) + delta
        let clamped = min(max(value, Int64(unit.range.lowerBound)), Int64(unit.range.upperBound))
        var next = current
        next[keyPath: unit.keyPath] = UInt32(clamped)
        send(.parts(parts: next))
    }
}
