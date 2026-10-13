import AppKit
import SwiftUI

/// The catalog's search field, above the list as in the recipes mockup.
///
/// SwiftUI's `.searchable` cannot sit in the middle column of a split view,
/// so this wraps AppKit's `NSSearchField`: native look, a clear button, and
/// Esc to clear. Rust owns the text (`rustSearchText`).
struct MacSearchField: View {
    let search: String
    let send: (IngredientListEvent) -> Void

    @State private var text: String

    init(search: String, send: @escaping (IngredientListEvent) -> Void) {
        self.search = search
        self.send = send
        _text = State(initialValue: search)
    }

    var body: some View {
        SearchFieldRepresentable(text: $text, prompt: "Search ingredients")
            .rustSearchText(search, text: $text, send: send)
            .accessibilityIdentifier("ingredientSearch")
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
