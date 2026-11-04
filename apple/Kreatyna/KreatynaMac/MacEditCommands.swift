import SwiftUI

extension FocusedValues {
    /// Deletes the selected step or arrow of the open recipe editor. The
    /// editor publishes it while one is selected; otherwise it is absent
    /// (REC-16, REC-18).
    @Entry var deleteSelection: DeleteSelectionAction?
}

/// Deletes what `target` names. Two actions for the same target are equal,
/// so the menu updates only when the selection changes.
struct DeleteSelectionAction: Equatable {
    enum Target: Equatable {
        case step(StepId)
        case connection(StepEdge)
    }

    let target: Target
    let perform: () -> Void

    /// The Edit menu item.
    var title: String {
        switch target {
        case .step: "Delete Step"
        case .connection: "Delete Connection"
        }
    }

    static func == (lhs: Self, rhs: Self) -> Bool { lhs.target == rhs.target }
}

/// The app's items in the Edit menu.
///
/// Delete Step (⌘⌫) deletes the selected step (REC-16/Q1), and reads
/// Delete Connection while an arrow is selected (REC-18/Q1). macOS gives a
/// menu's ⌘ keys to the menu first, so it works while the cursor is in the
/// step's Title field.
struct MacEditCommands: Commands {
    @FocusedValue(\.deleteSelection) private var deleteSelection

    var body: some Commands {
        CommandGroup(after: .pasteboard) {
            Divider()
            Button(deleteSelection?.title ?? "Delete Step", systemImage: "trash") { deleteSelection?.perform() }
                .keyboardShortcut(.delete, modifiers: .command)
                .disabled(deleteSelection == nil)
        }
    }
}
