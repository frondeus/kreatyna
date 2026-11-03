import SwiftUI

extension View {
    /// A system search field whose text Rust owns (`IngredientListView.search`).
    func rustSearchable(
        _ search: String,
        send: @escaping (IngredientListEvent) -> Void
    ) -> some View {
        modifier(RustSearchable(search: search, send: send))
    }

    /// Keeps a search field's `text` in step with Rust's `search`.
    ///
    /// Typed text goes to Rust through `send`, for example as the list's
    /// `.search(query:)`. `TextEcho` keeps a render that is one keystroke
    /// behind from erasing what the user typed.
    func rustSearchText(
        _ search: String,
        text: Binding<String>,
        send: @escaping (String) -> Void
    ) -> some View {
        modifier(RustSearchText(search: search, text: text, send: send))
    }
}

private struct RustSearchable: ViewModifier {
    let search: String
    let send: (IngredientListEvent) -> Void

    @State private var text: String

    init(search: String, send: @escaping (IngredientListEvent) -> Void) {
        self.search = search
        self.send = send
        _text = State(initialValue: search)
    }

    func body(content: Content) -> some View {
        content
            .searchable(text: $text, prompt: "Search ingredients")
            .rustSearchText(search, text: $text) { send(.search(query: $0)) }
    }
}

private struct RustSearchText: ViewModifier {
    let search: String
    @Binding var text: String
    let send: (String) -> Void

    @State private var echo = TextEcho()

    func body(content: Content) -> some View {
        content
            .onChange(of: text) { _, text in
                if let query = echo.typed(text, displayed: search) {
                    send(query)
                }
            }
            .onChange(of: search) { _, search in
                if let text = echo.received(search, showing: text) { self.text = text }
            }
    }
}
