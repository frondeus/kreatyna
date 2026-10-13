import SwiftUI

/// The create form in the detail pane. It shows Rust's form view and sends
/// every edit, Save, and Cancel to Rust; Rust validates and decides what the
/// pane shows next.
struct MacIngredientForm: View {
    let view: IngredientFormView
    let send: (IngredientFormEvent) -> Void

    /// Presentation only: the field with the cursor. It starts in Name.
    @FocusState private var focusedField: IngredientDraftField?

    var body: some View {
        let draft = view.draft
        Form {
            Section {
                field("Name", .name, draft.name, prompt: "Required", issue: .name)
                field("Notes", .notes, draft.notes, prompt: "Notes")
            }
            Section("Nutrition") {
                Picker("Basis", selection: Binding(
                    get: { draft.basis },
                    set: { send(.setBasis(basis: $0)) }
                )) {
                    Text(NutritionBasis.per100Grams.label).tag(NutritionBasis.per100Grams)
                    Text(NutritionBasis.per100Millilitres.label).tag(NutritionBasis.per100Millilitres)
                }
                .accessibilityIdentifier("ingredientBasisPicker")
                field("Calories (kcal)", .calories, draft.calories, prompt: "Unknown", issue: .calories)
                field("Protein (g)", .protein, draft.protein, prompt: "Unknown", issue: .protein)
                field("Carbohydrates (g)", .carbohydrates, draft.carbohydrates, prompt: "Unknown", issue: .carbohydrates)
                field("Fat (g)", .fat, draft.fat, prompt: "Unknown", issue: .fat)
            }
            Section("Reference units") {
                ForEach(Array(draft.units.enumerated()), id: \.offset) { offset, unit in
                    unitRow(index: UInt64(offset), count: draft.units.count, unit)
                }
                Button("Add Unit", systemImage: "plus") { send(.addUnit) }
                    .buttonStyle(.borderless)
                    .accessibilityIdentifier("addIngredientUnit")
            }
        }
        .formStyle(.grouped)
        .disabled(isSaving)
        // On the form only: on the whole pane it would replace the title's
        // and the save error's identifiers.
        .accessibilityIdentifier("ingredientForm")
        .safeAreaInset(edge: .top, spacing: 0) {
            // The save error stays outside the form. As a form row, it moved
            // the rows below it when it went away, and Name lost the cursor.
            VStack(alignment: .leading, spacing: 8) {
                Text("New Ingredient")
                    .font(.title2.bold())
                    .accessibilityIdentifier("ingredientFormTitle")
                if let error = view.saveError {
                    Label(error.message, systemImage: "exclamationmark.triangle")
                        .foregroundStyle(.red)
                        .accessibilityIdentifier("ingredientSaveError")
                }
            }
            .frame(maxWidth: .infinity, alignment: .leading)
            .padding(.horizontal, 20)
            .padding(.top, 16)
        }
        .toolbar {
            // One group, like Edit and Delete in the detail.
            ToolbarItemGroup(placement: .primaryAction) {
                Button("Cancel", systemImage: "xmark") { send(.requestClose) }
                    .help("Cancel")
                    .keyboardShortcut(.cancelAction)
                    .accessibilityIdentifier("cancelIngredientForm")
                if isSaving {
                    ProgressView()
                        .controlSize(.small)
                        .accessibilityIdentifier("ingredientSaving")
                } else {
                    Button("Save", systemImage: "checkmark") { send(.save) }
                        .help("Save")
                        .keyboardShortcut("s", modifiers: .command)
                        .accessibilityIdentifier("saveIngredient")
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
        .onAppear { focusedField = .name }
    }

    private var isSaving: Bool {
        if case .saving = view.status { true } else { false }
    }

    private var awaitingDiscardDecision: Bool {
        if case .confirmingDiscard(.awaitingDecision) = view.status { true } else { false }
    }

    /// One reference unit: its three fields and a remove button. Its context
    /// menu moves it (MAC-05/Q4: drag inside a grouped form does not reorder on macOS).
    private func unitRow(index: UInt64, count: Int, _ unit: ReferenceUnitDraft) -> some View {
        VStack(alignment: .leading, spacing: 4) {
            HStack(spacing: 8) {
                Group {
                    textField("Name", .unitName(index: index), unit.name, prompt: "Name")
                    textField("Weight (g)", .unitGrams(index: index), unit.grams, prompt: "Weight (g)")
                        .frame(width: 100)
                    textField("Volume (ml)", .unitMillilitres(index: index), unit.millilitres, prompt: "Volume (ml)")
                        .frame(width: 100)
                }
                .labelsHidden()
                .textFieldStyle(.roundedBorder)
                Button("Remove Unit", systemImage: "minus.circle.fill") { send(.removeUnit(index: index)) }
                    .labelStyle(.iconOnly)
                    .buttonStyle(.borderless)
                    .foregroundStyle(.red)
                    .accessibilityIdentifier("removeIngredientUnit.\(index)")
            }
            issueText(for: .unitName(index: index))
            issueText(for: .unitGrams(index: index))
            issueText(for: .unitMillilitres(index: index))
            issueText(for: .unitMeasurements(index: index))
        }
        .padding(.leading, 8)
        .contentShape(Rectangle())
        .contextMenu {
            Button("Move Up") { send(.moveUnit(from: index, to: index - 1)) }
                .disabled(index == 0)
            Button("Move Down") { send(.moveUnit(from: index, to: index + 1)) }
                .disabled(index + 1 >= UInt64(count))
            Divider()
            Button("Remove") { send(.removeUnit(index: index)) }
        }
        .accessibilityElement(children: .contain)
        .accessibilityIdentifier("ingredientUnitRow.\(index)")
    }

    private func field(
        _ title: String,
        _ field: IngredientDraftField,
        _ value: String,
        prompt: String,
        issue: ValidationField? = nil
    ) -> some View {
        VStack(alignment: .trailing, spacing: 4) {
            // A grouped form shows the title as the label, the field on the right.
            textField(title, field, value, prompt: prompt)
                .multilineTextAlignment(.trailing)
            if let issue { issueText(for: issue) }
        }
    }

    private func textField(
        _ title: String,
        _ field: IngredientDraftField,
        _ value: String,
        prompt: String
    ) -> some View {
        SyncedTextField(title, value: value, prompt: prompt) { send(.setField(field: field, value: $0)) }
            .focused($focusedField, equals: field)
            .accessibilityIdentifier(field.accessibilityIdentifier)
    }

    @ViewBuilder
    private func issueText(for field: ValidationField) -> some View {
        ForEach(Array(view.issues.filter { $0.field == field }.enumerated()), id: \.offset) { _, issue in
            Text(issue.reason.message)
                .font(.footnote)
                .foregroundStyle(.red)
        }
    }
}
