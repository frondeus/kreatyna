import AppKit
import SwiftUI

/// The search field above a list column: the catalog's and the recipe
/// list's, as in the recipes mockup.
///
/// SwiftUI's `.searchable` cannot sit in the middle column of a split view,
/// so this wraps AppKit's `NSSearchField`: native look, a clear button, and
/// Esc to clear. Rust owns the text (`rustSearchText`).
struct MacSearchField: View {
    let search: String
    let prompt: String
    let identifier: String
    /// Sends the typed text to Rust.
    let send: (String) -> Void

    @State private var text: String

    init(search: String, prompt: String, identifier: String, send: @escaping (String) -> Void) {
        self.search = search
        self.prompt = prompt
        self.identifier = identifier
        self.send = send
        _text = State(initialValue: search)
    }

    var body: some View {
        SearchFieldRepresentable(text: $text, prompt: prompt)
            .rustSearchText(search, text: $text, send: send)
            .accessibilityIdentifier(identifier)
    }
}

private struct SearchFieldRepresentable: NSViewRepresentable {
    @Binding var text: String
    let prompt: String

    func makeNSView(context: Context) -> NSSearchField {
        let field = NSSearchField()
        field.placeholderString = prompt
        field.sendsSearchStringImmediately = true
        field.target = context.coordinator
        field.action = #selector(Coordinator.changed(_:))
        return field
    }

    func updateNSView(_ field: NSSearchField, context: Context) {
        context.coordinator.text = $text
        if field.stringValue != text { field.stringValue = text }
    }

    func makeCoordinator() -> Coordinator { Coordinator(text: $text) }

    final class Coordinator: NSObject {
        var text: Binding<String>

        init(text: Binding<String>) { self.text = text }

        /// Typing, the clear button, and Esc all send this action.
        @objc func changed(_ field: NSSearchField) {
            text.wrappedValue = field.stringValue
        }
    }
}
