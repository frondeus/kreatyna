import SwiftUI

/// The popover of one recipe ingredient (REC-08/Q1): Amount, Unit, Name and
/// Note. Each edit goes to Rust as a `RecipeIngredientsEvent`; Rust keeps the
/// fields as typed and decides the problems, which show under their field.
struct MacRecipeIngredientEditor: View {
    let ingredient: RecipeIngredientView
    /// The Unit picker's choices after "No unit", from Rust.
    let units: [KnownUnitOption]
    let send: (RecipeIngredientsEvent) -> Void

    /// Presentation only: the popover opens with the cursor in Name.
    @FocusState private var nameFocused: Bool

    var body: some View {
        VStack(alignment: .leading, spacing: 10) {
            HStack(alignment: .top, spacing: 12) {
                field("Amount") {
                    SyncedTextField("Amount", value: ingredient.amount, prompt: "Amount") {
                        send(.setAmount(id: ingredient.id, amount: $0))
                    }
                    .labelsHidden()
                    .textFieldStyle(.roundedBorder)
                    .frame(width: 90)
                    .accessibilityIdentifier("recipeIngredientAmount")
                    if let problem = ingredient.problems.amount {
                        problemText(problem.message, id: "recipeIngredientAmountProblem")
                            .frame(width: 90, alignment: .leading)
                    }
                }
                field("Unit") {
                    Picker("Unit", selection: unit) {
                        Text("No unit").tag(KnownUnit?.none)
                        Divider()
                        ForEach(units, id: \.unit) { option in
                            Text(option.label).tag(KnownUnit?.some(option.unit))
                        }
                    }
                    .labelsHidden()
                    .accessibilityIdentifier("recipeIngredientUnit")
                }
            }
            field("Name") {
                SyncedTextField("Name", value: ingredient.name, prompt: "Name") {
                    send(.setName(id: ingredient.id, name: $0))
                }
                .labelsHidden()
                .textFieldStyle(.roundedBorder)
                .focused($nameFocused)
                .accessibilityIdentifier("recipeIngredientName")
                if let problem = ingredient.problems.name {
                    problemText(problem.message, id: "recipeIngredientNameProblem")
                }
            }
            field("Note") {
                SyncedTextField("Note", value: ingredient.note, prompt: "Optional") {
                    send(.setNote(id: ingredient.id, note: $0))
                }
                .labelsHidden()
                .textFieldStyle(.roundedBorder)
                .accessibilityIdentifier("recipeIngredientNote")
            }
        }
        .padding(14)
        .frame(width: 300)
        .onAppear { nameFocused = true }
    }

    /// The picker shows Rust's unit; a choice sends it.
    private var unit: Binding<KnownUnit?> {
        Binding(
            get: { ingredient.unit },
            set: { send(.setUnit(id: ingredient.id, unit: $0)) }
        )
    }

    private func field(_ label: String, @ViewBuilder content: () -> some View) -> some View {
        VStack(alignment: .leading, spacing: 4) {
            Text(label.uppercased())
                .font(.caption.weight(.semibold))
                .foregroundStyle(.secondary)
            content()
        }
    }

    private func problemText(_ message: String, id: String) -> some View {
        Text(message)
            .font(.footnote)
            .foregroundStyle(.red)
            .fixedSize(horizontal: false, vertical: true)
            .accessibilityIdentifier(id)
    }
}
