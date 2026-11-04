import SwiftUI

/// The read view of one recipe in the Recipes detail pane (REC-03). Rust loads
/// the recipe by id and sends its state; Swift shows it and forwards Retry,
/// Edit, and Delete with its answer (REC-05).
struct MacRecipeDetail: View {
    let view: RecipeDetailView
    let send: (RecipeDetailEvent) -> Void

    var body: some View {
        content
            .toolbar {
                ToolbarItemGroup(placement: .primaryAction) {
                    Button("Edit", systemImage: "pencil") { send(.edit) }
                        .help("Edit")
                        .disabled(!isIdle)
                        .accessibilityIdentifier("editRecipe")
                    Button("Delete", systemImage: "trash", role: .destructive) { send(.requestDelete) }
                        .help("Delete")
                        .disabled(!isIdle)
                        .accessibilityIdentifier("deleteRecipe")
                }
            }
            // The setter does nothing: Rust's view opens and closes the dialog.
            // Esc runs the Cancel button.
            .confirmationDialog(
                DeleteText.title(title ?? ""),
                isPresented: Binding(get: { awaitingDeleteDecision }, set: { _ in }),
                titleVisibility: .visible
            ) {
                Button("Delete", role: .destructive) { send(.confirmation(event: .confirm)) }
                    .accessibilityIdentifier("confirmDelete")
                Button("Cancel", role: .cancel) { send(.confirmation(event: .cancel)) }
                    .accessibilityIdentifier("cancelDelete")
            } message: {
                Text(DeleteText.message(deleteError))
            }
            .overlay {
                if isDeleting {
                    ProgressView("Deleting…")
                        .padding()
                        .background(.regularMaterial, in: .rect(cornerRadius: 12))
                        .accessibilityIdentifier("recipeDeleting")
                }
            }
    }

    private var title: String? {
        if case .ready(let title, _, _) = view.status { title } else { nil }
    }

    private var deletion: RecipeDeletion? {
        if case .ready(_, _, let deletion) = view.status { deletion } else { nil }
    }

    /// Rust takes Edit and Delete only for a loaded recipe with no delete
    /// asked or running; the disabled buttons show it.
    private var isIdle: Bool {
        if case .idle = deletion { true } else { false }
    }

    private var awaitingDeleteDecision: Bool {
        if case .confirming(.awaitingDecision, _) = deletion { true } else { false }
    }

    private var deleteError: RecipeDeleteError? {
        if case .confirming(_, let error) = deletion { error } else { nil }
    }

    /// The pane empties right after `deleted`, so the overlay stays until then.
    private var isDeleting: Bool {
        switch deletion {
        case .deleting, .deleted: true
        case .idle, .confirming, nil: false
        }
    }

    @ViewBuilder
    private var content: some View {
        switch view.status {
        case .loading:
            ProgressView("Loading recipe…")
                .frame(maxWidth: .infinity, maxHeight: .infinity)
                .accessibilityIdentifier("recipeDetailLoading")
        case .failed(let error):
            ContentUnavailableView {
                Text("Recipe unavailable")
            } description: {
                Text(error.detailMessage)
            } actions: {
                Button("Retry") { send(.retry) }
                    .buttonStyle(.borderedProminent)
                    .accessibilityIdentifier("retryRecipeDetail")
            }
            .accessibilityIdentifier("recipeDetailFailed")
        case .missing:
            ContentUnavailableView {
                Text("This recipe no longer exists.")
            }
            .accessibilityIdentifier("recipeDetailMissing")
        case .ready(let title, let steps, _):
            Form {
                // Lines arrive with REC-07 (stages: REC-19).
                noneYet("Ingredients")
                if steps.isEmpty {
                    noneYet("Steps")
                } else {
                    // In the order added (REC-14).
                    Section("Steps") {
                        ForEach(steps, id: \.id) { step in
                            MacRecipeDetailStepRow(step: step)
                        }
                    }
                }
            }
            .formStyle(.grouped)
            // On the form only: on the whole view it would replace the title's identifier.
            .accessibilityIdentifier("recipeDetail")
            .safeAreaInset(edge: .top, spacing: 0) {
                Text(title)
                    .font(.title2.bold())
                    .frame(maxWidth: .infinity, alignment: .leading)
                    .padding(.horizontal, 20)
                    .padding(.top, 16)
                    .accessibilityIdentifier("recipeTitle")
            }
        }
    }

    private func noneYet(_ title: String) -> some View {
        Section(title) {
            Text("None yet")
                .foregroundStyle(.secondary)
        }
    }
}

/// One step of the read view: "Boil water · 10 min · waiting", with the
/// instructions under it in smaller text (REC-17/Q2).
struct MacRecipeDetailStepRow: View {
    let step: RecipeDetailStep

    var body: some View {
        VStack(alignment: .leading, spacing: 2) {
            HStack(spacing: 0) {
                Text(step.title)
                    .accessibilityIdentifier("recipeStep")
                if let details = Self.details(step) {
                    Text(" · \(details)")
                        .foregroundStyle(.secondary)
                        .accessibilityIdentifier("recipeStepDetails")
                }
            }
            if let instructions = step.instructions {
                Text(instructions)
                    .font(.callout)
                    .foregroundStyle(.secondary)
                    .accessibilityIdentifier("recipeStepInstructions")
            }
        }
    }

    /// "10 min · waiting", "1 min 30 s", "waiting", or `nil` for an active
    /// step without a duration.
    static func details(_ step: RecipeDetailStep) -> String? {
        let parts = [step.duration, step.kind == .waiting ? "waiting" : nil].compactMap { $0 }
        return parts.isEmpty ? nil : parts.joined(separator: " · ")
    }
}
