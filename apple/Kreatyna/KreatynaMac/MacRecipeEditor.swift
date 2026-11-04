import SwiftUI

/// The recipe editor in the Recipes detail pane, for a new or a saved recipe.
/// It shows Rust's editor view and sends the title, the step canvas events,
/// Save, and Cancel to Rust; Rust checks the titles and decides what the pane
/// shows next.
///
/// The layout is REC-14/Q1: the Title field, the step canvas, and
/// "Ingredients: Coming soon" on the left; the step inspector on the right.
struct MacRecipeEditor: View {
    /// "New Recipe", or the saved title while it is edited.
    let heading: String
    let view: RecipeEditorView
    let send: (RecipeEditorEvent) -> Void

    /// Presentation only: the cursor starts in Title.
    @FocusState private var titleFocused: Bool

    var body: some View {
        HStack(spacing: 0) {
            VStack(alignment: .leading, spacing: 0) {
                Text(heading)
                    .font(.title2.bold())
                    .padding(.horizontal, 20)
                    .padding(.top, 16)
                    .accessibilityIdentifier("recipeEditorTitle")
                titleField
                    .padding(.horizontal, 20)
                    .padding(.vertical, 12)
                Divider()
                MacStepCanvas(canvas: view.canvas) { send(.canvas(event: $0)) }
                if let refusal = view.canvas.refusal {
                    // Until the next change (REC-18/Q2).
                    Text(refusal.message)
                        .font(.footnote)
                        .foregroundStyle(.red)
                        .padding(.horizontal, 20)
                        .padding(.vertical, 6)
                        .accessibilityIdentifier("connectRefusal")
                }
                Divider()
                // Ingredient lines arrive with REC-07.
                HStack {
                    Text("Ingredients")
                        .font(.headline)
                    Text("Coming soon")
                        .foregroundStyle(.secondary)
                }
                .padding(.horizontal, 20)
                .padding(.vertical, 12)
            }
            Divider()
            MacStepInspector(canvas: view.canvas) { send(.canvas(event: $0)) }
        }
        .disabled(isSaving)
        .toolbar {
            ToolbarItemGroup(placement: .primaryAction) {
                Button("Cancel", systemImage: "xmark") { send(.requestClose) }
                    .help("Cancel")
                    .keyboardShortcut(.cancelAction)
                    .accessibilityIdentifier("cancelRecipeEditor")
                if isSaving {
                    ProgressView()
                        .controlSize(.small)
                        .accessibilityIdentifier("recipeSaving")
                } else {
                    Button("Save", systemImage: "checkmark") { send(.save) }
                        .help("Save")
                        .keyboardShortcut("s", modifiers: .command)
                        .accessibilityIdentifier("saveRecipe")
                }
            }
        }
        .confirmationDialog(
            "Discard changes?",
            isPresented: Binding(get: { awaitingDiscardDecision }, set: { _ in }),
            titleVisibility: .visible
        ) {
            Button("Discard Changes", role: .destructive) { send(.confirmation(event: .confirm)) }
                .accessibilityIdentifier("confirmDiscard")
            Button("Keep Editing", role: .cancel) { send(.confirmation(event: .cancel)) }
                .accessibilityIdentifier("cancelDiscard")
        }
        .focusedSceneValue(\.deleteSelection, deleteSelection)
        .onAppear { titleFocused = true }
    }

    /// Edit ▸ Delete Step or Delete Connection for the selection; absent
    /// with no selection.
    private var deleteSelection: DeleteSelectionAction? {
        switch view.canvas.nodes {
        case .unselected:
            return nil
        case .selected(let selected, _):
            let id = selected.id
            return DeleteSelectionAction(target: .step(id)) { send(.canvas(event: .delete(id: id))) }
        case .connectionSelected(let connection, _):
            let edge = connection.edge
            return DeleteSelectionAction(target: .connection(edge)) {
                send(.canvas(event: .deleteConnection(edge: edge)))
            }
        }
    }

    private var titleField: some View {
        VStack(alignment: .leading, spacing: 4) {
            HStack {
                Text("Title")
                SyncedTextField("Title", value: view.title, prompt: "Required") { send(.setTitle(title: $0)) }
                    .labelsHidden()
                    .textFieldStyle(.roundedBorder)
                    .focused($titleFocused)
                    .accessibilityIdentifier("recipeTitleField")
            }
            if let problem {
                Text(problem.message)
                    .font(.footnote)
                    .foregroundStyle(.red)
                    .accessibilityIdentifier("recipeTitleProblem")
            }
        }
    }

    private var problem: RecipeEditorProblem? {
        if case .editing(let problem) = view.status { problem } else { nil }
    }

    private var isSaving: Bool {
        if case .saving = view.status { true } else { false }
    }

    private var awaitingDiscardDecision: Bool {
        if case .confirmingDiscard(.awaitingDecision) = view.status { true } else { false }
    }
}
