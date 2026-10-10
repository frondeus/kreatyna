import SwiftUI

/// A text field that shows typed text immediately and sends every edit to Rust.
///
/// Rust renders asynchronously, so a field bound directly to the view value
/// would show stale text while typing and could lose keystrokes. The local
/// text is presentation only; `TextEcho` decides when Rust's value replaces it.
struct SyncedTextField: View {
    let title: String
    let value: String
    var axis: Axis = .horizontal
    let send: (String) -> Void

    @State private var text: String
    @State private var echo = TextEcho()

    init(_ title: String, value: String, axis: Axis = .horizontal, send: @escaping (String) -> Void) {
        self.title = title
        self.value = value
        self.axis = axis
        self.send = send
        _text = State(initialValue: value)
    }

    var body: some View {
        TextField(title, text: $text, axis: axis)
            .onChange(of: text) { _, newText in
                if let sent = echo.typed(newText, displayed: value) { send(sent) }
            }
            .onChange(of: value) { _, newValue in
                if let replacement = echo.received(newValue, showing: text) { text = replacement }
            }
    }
}

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
