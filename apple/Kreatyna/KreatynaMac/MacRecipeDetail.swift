import SwiftUI

/// The read view of one recipe in the Recipes detail pane (REC-03). Rust loads
/// the recipe by id and sends its state; Swift shows it and forwards Retry.
struct MacRecipeDetail: View {
    let view: RecipeDetailView
    let send: (RecipeDetailEvent) -> Void

    var body: some View {
        content
            .toolbar {
                ToolbarItem(placement: .primaryAction) {
                    // Editing a saved recipe arrives with REC-04.
                    Button("Edit", systemImage: "pencil") {}
                        .help("Edit")
                        .disabled(true)
                        .accessibilityIdentifier("editRecipe")
                }
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
                Text(error.message)
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
        case .ready(let title):
            Form {
                // Lines arrive with REC-07, steps with REC-14 (stages: REC-19).
                noneYet("Ingredients")
                noneYet("Steps")
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
