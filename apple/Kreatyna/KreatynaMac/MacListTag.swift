import SwiftUI

/// A row's tag in a Mac list's selection: the draft row, or one item's id.
/// The draft row has no id, so a click on it cannot look like a selected item.
enum MacListTag<Id: Hashable>: Hashable {
    case draft
    case item(Id)

    /// A selection binding over Rust's state. The draft row is highlighted
    /// while it is shown; otherwise Rust's `selected` item is. A click on the
    /// draft row changes nothing, because Rust keeps it highlighted.
    static func selection(
        draftShown: Bool,
        selected: Id?,
        select: @escaping (Id?) -> Void
    ) -> Binding<MacListTag?> {
        Binding(
            get: { draftShown ? .draft : selected.map(MacListTag.item) },
            set: { tag in
                switch tag {
                case .draft: break
                case .item(let id): select(id)
                case nil: select(nil)
                }
            }
        )
    }
}
