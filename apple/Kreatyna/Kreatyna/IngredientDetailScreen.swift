import SwiftUI

struct IngredientDetailScreen: View {
    let view: IngredientDetailView
    let send: (IngredientDetailEvent) -> Void

    var body: some View {
        let ingredient = view.ingredient
        List {
            if let notes = ingredient.notes {
                Section("Notes") {
                    Text(notes)
                        .accessibilityIdentifier("ingredientNotes")
                }
            }
            Section("Nutrition \(ingredient.nutrition.basis.label)") {
                NutritionRow(title: "Calories", value: ingredient.nutrition.calories, unit: "kcal")
                NutritionRow(title: "Protein", value: ingredient.nutrition.protein, unit: "g")
                NutritionRow(title: "Carbohydrates", value: ingredient.nutrition.carbohydrates, unit: "g")
                NutritionRow(title: "Fat", value: ingredient.nutrition.fat, unit: "g")
            }
            if !ingredient.units.isEmpty {
                Section("Reference units") {
                    ForEach(Array(ingredient.units.enumerated()), id: \.offset) { _, unit in
                        LabeledContent(unit.name, value: unit.measurementText)
                    }
                }
            }
            if let error = deleteError {
                Section {
                    Label(error.message, systemImage: "exclamationmark.triangle")
                        .foregroundStyle(.red)
                        .accessibilityIdentifier("ingredientDeleteError")
                }
            }
        }
        .navigationTitle(ingredient.name)
        .accessibilityIdentifier("ingredientDetail")
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
                Button("Edit") { send(.edit) }
                    .accessibilityIdentifier("editIngredient")
                Button("Delete", systemImage: "trash", role: .destructive) { send(.requestDelete) }
                    .accessibilityIdentifier("deleteIngredient")
            }
        }
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

    private var awaitingDeleteDecision: Bool {
        if case .confirming(.awaitingDecision, _) = view.deletion { true } else { false }
    }

    private var deleteError: IngredientDeleteError? {
        if case .confirming(_, let error) = view.deletion { error } else { nil }
    }
}

private struct NutritionRow: View {
    let title: String
    let value: String?
    let unit: String

    var body: some View {
        LabeledContent(title, value: value.map { "\($0) \(unit)" } ?? "—")
    }
}
