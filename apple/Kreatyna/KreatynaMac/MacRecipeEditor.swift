import SwiftUI

/// The create editor in the Recipes detail pane. It shows Rust's editor view
/// and sends the title, Save, and Cancel to Rust; Rust checks the title and
/// decides what the pane shows next.
struct MacRecipeEditor: View {
    let view: RecipeEditorView
    let send: (RecipeEditorEvent) -> Void

    /// Presentation only: the cursor starts in Title.
    @FocusState private var titleFocused: Bool

    var body: some View {
        Form {
            Section {
                VStack(alignment: .trailing, spacing: 4) {
                    // A grouped form shows the title as the label, the field on the right.
                    SyncedTextField("Title", value: view.title, prompt: "Required") { send(.setTitle(title: $0)) }
                        .multilineTextAlignment(.trailing)
                        .focused($titleFocused)
                        .accessibilityIdentifier("recipeTitleField")
                    if let problem {
                        Text(problem.message)
                            .font(.footnote)
                            .foregroundStyle(.red)
                            .accessibilityIdentifier("recipeTitleProblem")
                    }
                }
            }
            // Ingredient lines arrive with REC-07, steps with REC-14.
            comingSoon("Ingredients")
            comingSoon("Steps")
        }
        .formStyle(.grouped)
        .disabled(isSaving)
        .accessibilityIdentifier("recipeEditor")
        .safeAreaInset(edge: .top, spacing: 0) {
            Text("New Recipe")
                .font(.title2.bold())
                .frame(maxWidth: .infinity, alignment: .leading)
                .padding(.horizontal, 20)
                .padding(.top, 16)
                .accessibilityIdentifier("recipeEditorTitle")
        }
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
        .onAppear { titleFocused = true }
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

    private func comingSoon(_ title: String) -> some View {
        Section(title) {
            Text("Coming soon")
                .foregroundStyle(.secondary)
        }
    }
}
