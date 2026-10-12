import Foundation

/// Tracks text sent to Rust that has not been rendered back yet.
struct TextEcho {
    private(set) var pending: [String] = []

    /// Returns the text to send, or nil when it already matches Rust's value.
    mutating func typed(_ text: String, displayed value: String) -> String? {
        guard text != value || !pending.isEmpty else { return nil }
        if pending.last == text { return nil }
        pending.append(text)
        return text
    }

    /// Returns replacement text, or nil when the field should keep its text.
    mutating func received(_ value: String, showing text: String) -> String? {
        if let index = pending.lastIndex(of: value) {
            // Rust caught up to an edit we sent; later edits are still in flight.
            pending.removeFirst(index + 1)
            return nil
        }
        // Rust changed the value independently, for example after a unit move.
        pending.removeAll()
        return value == text ? nil : value
    }
}
