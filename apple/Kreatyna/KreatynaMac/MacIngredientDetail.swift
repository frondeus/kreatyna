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
        .safeAreaInset(edge: .top, spacing: 0) {
            Text(ingredient.name)
                .font(.title2.bold())
                .frame(maxWidth: .infinity, alignment: .leading)
                .padding(.horizontal, 20)
                .padding(.top, 16)
                .accessibilityIdentifier("ingredientName")
        }
        .accessibilityIdentifier("ingredientDetail")
        .toolbar {
            ToolbarItemGroup(placement: .primaryAction) {
                Button("Edit", systemImage: "pencil") { send(.edit) }
                    .help("Edit")
                    .accessibilityIdentifier("editIngredient")
                Button("Delete", systemImage: "trash") {}
                    .help("Delete")
                    .disabled(true) // MAC-07
                    .accessibilityIdentifier("deleteIngredient")
            }
        }
    }

    private func nutritionRow(_ title: String, _ value: String?, unit: String) -> some View {
        LabeledContent(title, value: value.map { "\($0) \(unit)" } ?? "—")
    }
}
