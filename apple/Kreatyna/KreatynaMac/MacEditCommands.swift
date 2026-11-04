import SwiftUI

extension FocusedValues {
    /// Deletes the selected step of the open recipe editor. The editor
    /// publishes it while a step is selected; otherwise it is absent (REC-16).
    @Entry var deleteStep: DeleteStepAction?
}

/// Deletes the step `id`. Two actions for the same step are equal, so the
/// menu updates only when the selection changes.
struct DeleteStepAction: Equatable {
    let id: StepId
    let perform: () -> Void

    static func == (lhs: Self, rhs: Self) -> Bool { lhs.id == rhs.id }
}

/// The app's items in the Edit menu.
///
/// Delete Step (⌘⌫) deletes the selected step (REC-16/Q1). macOS gives a
/// menu's ⌘ keys to the menu first, so it works while the cursor is in the
/// step's Title field.
struct MacEditCommands: Commands {
    @FocusedValue(\.deleteStep) private var deleteStep

    var body: some Commands {
        CommandGroup(after: .pasteboard) {
            Divider()
            Button("Delete Step", systemImage: "trash") { deleteStep?.perform() }
                .keyboardShortcut(.delete, modifiers: .command)
                .disabled(deleteStep == nil)
        }
    }
}
