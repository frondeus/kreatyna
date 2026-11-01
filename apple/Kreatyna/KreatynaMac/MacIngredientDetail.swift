import SwiftUI

/// The detail pane for one ingredient. It shows Rust's detail view as a grouped form.
struct MacIngredientDetail: View {
    let view: IngredientDetailView
    let send: (IngredientDetailEvent) -> Void

    var body: some View {
        let ingredient = view.ingredient
        Form {
            if let notes = ingredient.notes {
                Section("Notes") {
                    Text(notes)
                        .accessibilityIdentifier("ingredientNotes")
                }
            }
            Section("Nutrition \(ingredient.nutrition.basis.label)") {
                nutritionRow("Calories", ingredient.nutrition.calories, unit: "kcal")
                nutritionRow("Protein", ingredient.nutrition.protein, unit: "g")
                nutritionRow("Carbohydrates", ingredient.nutrition.carbohydrates, unit: "g")
                nutritionRow("Fat", ingredient.nutrition.fat, unit: "g")
            }
            if !ingredient.units.isEmpty {
                Section("Reference units") {
                    ForEach(Array(ingredient.units.enumerated()), id: \.offset) { _, unit in
                        LabeledContent(unit.name, value: unit.measurementText)
                    }
                }
                .accessibilityIdentifier("ingredientUnits")
            }
        }
        .formStyle(.grouped)
        // On the form only: on the whole view it would replace the name's identifier.
        .accessibilityIdentifier("ingredientDetail")
        .safeAreaInset(edge: .top, spacing: 0) {
            Text(ingredient.name)
                .font(.title2.bold())
                .frame(maxWidth: .infinity, alignment: .leading)
                .padding(.horizontal, 20)
                .padding(.top, 16)
                .accessibilityIdentifier("ingredientName")
        }
        .overlay {
            if case .deleting = view.deletion {
                ProgressView("Deleting…")
                    .padding()
                    .background(.regularMaterial, in: .rect(cornerRadius: 12))
                    .accessibilityIdentifier("ingredientDeleting")
            }
        }
        .toolbar {
            ToolbarItemGroup(placement: .primaryAction) {
                Button("Edit", systemImage: "pencil") { send(.edit) }
                    .help("Edit")
                    .disabled(!isIdle)
                    .accessibilityIdentifier("editIngredient")
                Button("Delete", systemImage: "trash", role: .destructive) { send(.requestDelete) }
                    .help("Delete")
                    .disabled(!isIdle)
                    .accessibilityIdentifier("deleteIngredient")
            }
        }
        // The setter does nothing: Rust's view opens and closes the dialog.
        // Esc runs the Cancel button.
        .confirmationDialog(
            DeleteText.title(ingredient.name),
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
    }

    /// Rust ignores Edit and Delete during a delete; the disabled buttons show it.
    private var isIdle: Bool {
        if case .idle = view.deletion { true } else { false }
    }

    private var awaitingDeleteDecision: Bool {
        if case .confirming(.awaitingDecision, _) = view.deletion { true } else { false }
    }

    private var deleteError: IngredientDeleteError? {
        if case .confirming(_, let error) = view.deletion { error } else { nil }
    }

    private func nutritionRow(_ title: String, _ value: String?, unit: String) -> some View {
        LabeledContent(title, value: value.map { "\($0) \(unit)" } ?? "—")
    }
}
