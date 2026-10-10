import SwiftUI

struct IngredientFormScreen: View {
    let view: IngredientFormView
    let send: (IngredientFormEvent) -> Void
    let cancel: () -> Void

    var body: some View {
        let draft = view.draft
        Form {
            if let error = view.saveError {
                Section {
                    Label(error.message, systemImage: "exclamationmark.triangle")
                        .foregroundStyle(.red)
                        .accessibilityIdentifier("ingredientSaveError")
                }
            }
            Section {
                field("Name", .name, draft.name, issue: .name)
                field("Notes", .notes, draft.notes, axis: .vertical)
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
                field("Calories (kcal)", .calories, draft.calories, issue: .calories, numeric: true)
                field("Protein (g)", .protein, draft.protein, issue: .protein, numeric: true)
                field("Carbohydrates (g)", .carbohydrates, draft.carbohydrates, issue: .carbohydrates, numeric: true)
                field("Fat (g)", .fat, draft.fat, issue: .fat, numeric: true)
            }
            Section("Reference units") {
                ForEach(Array(draft.units.enumerated()), id: \.offset) { offset, unit in
                    let index = UInt64(offset)
                    VStack(alignment: .leading) {
                        field("Name", .unitName(index: index), unit.name, issue: .unitName(index: index))
                        field("Weight (g)", .unitGrams(index: index), unit.grams, issue: .unitGrams(index: index), numeric: true)
                        field("Volume (ml)", .unitMillilitres(index: index), unit.millilitres, issue: .unitMillilitres(index: index), numeric: true)
                        issueText(for: .unitMeasurements(index: index))
                    }
                }
                .onDelete { offsets in
                    // Remove from the end so earlier indices stay valid.
                    for offset in offsets.reversed() { send(.removeUnit(index: UInt64(offset))) }
                }
                .onMove { sources, destination in
                    for source in sources {
                        if let event = IngredientFormEvent.move(from: source, toOffset: destination) { send(event) }
                    }
                }
                Button("Add Unit", systemImage: "plus") { send(.addUnit) }
                    .accessibilityIdentifier("addIngredientUnit")
            }
        }
        .disabled(isSaving)
        .navigationTitle(title)
        .navigationBarTitleDisplayMode(.inline)
        .toolbar {
            ToolbarItem(placement: .cancellationAction) {
                Button("Cancel", action: cancel)
                    .accessibilityIdentifier("cancelIngredientForm")
            }
            ToolbarItem(placement: .confirmationAction) {
                if isSaving {
                    ProgressView().accessibilityIdentifier("ingredientSaving")
                } else {
                    Button("Save") { send(.save) }
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
    }

    private var title: String {
        switch view.draft.mode {
        case .create: "New Ingredient"
        case .edit: "Edit Ingredient"
        }
    }

    private var isSaving: Bool {
        if case .saving = view.status { true } else { false }
    }

    private var awaitingDiscardDecision: Bool {
        if case .confirmingDiscard(.awaitingDecision) = view.status { true } else { false }
    }

    private func field(
        _ title: String,
        _ field: IngredientDraftField,
        _ value: String,
        issue: ValidationField? = nil,
        axis: Axis = .horizontal,
        numeric: Bool = false
    ) -> some View {
        VStack(alignment: .leading, spacing: 4) {
            if axis == .vertical {
                SyncedTextField(title, value: value, axis: axis) { send(.setField(field: field, value: $0)) }
                    .accessibilityIdentifier(Self.identifier(for: field))
            } else {
                LabeledContent(title) {
                    SyncedTextField(numeric ? "Unknown" : "Required", value: value) {
                        send(.setField(field: field, value: $0))
                    }
                    .multilineTextAlignment(.trailing)
                    .keyboardType(numeric ? .decimalPad : .default)
                    .accessibilityIdentifier(Self.identifier(for: field))
                }
            }
            if let issue { issueText(for: issue) }
        }
    }

    static func identifier(for field: IngredientDraftField) -> String {
        switch field {
        case .name: "ingredientNameField"
        case .notes: "ingredientNotesField"
        case .calories: "ingredientCaloriesField"
        case .protein: "ingredientProteinField"
        case .carbohydrates: "ingredientCarbohydratesField"
        case .fat: "ingredientFatField"
        case .unitName(let index): "ingredientUnitNameField.\(index)"
        case .unitGrams(let index): "ingredientUnitGramsField.\(index)"
        case .unitMillilitres(let index): "ingredientUnitMillilitresField.\(index)"
        }
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
