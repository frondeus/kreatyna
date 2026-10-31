import Foundation
@testable import Kreatyna

/// Hand-built Rust view values. Tests and snapshots use them without the core.
enum Fixtures {
    static let garlicId = "6f1c1a3e-2b8f-4d9a-9a47-3c1e6b0d5a11"
    static let tomatoesId = "0b6e3c55-91d2-4f7e-8a1b-77a2c4d9e0f3"
    static let cucumberId = "c9d2e7a4-58b1-4c3f-b6e0-1d4a8f2b9c76"

    static let rows = [
        IngredientListRow(id: tomatoesId, name: "Canned tomatoes"),
        IngredientListRow(id: cucumberId, name: "Cucumber"),
        IngredientListRow(id: garlicId, name: "Garlic"),
    ]

    static func list(
        _ status: IngredientListStatus = .ready(rows: rows, totalCount: 3, refresh: .idle),
        search: String = ""
    ) -> IngredientListView {
        IngredientListView(search: search, status: status)
    }

    static let garlic = Ingredient(
        id: garlicId,
        name: "Garlic",
        notes: "Store in a cool, dry place.",
        nutrition: Nutrition(
            basis: .per100Grams, calories: "149", protein: "6.36", carbohydrates: "33.06", fat: nil
        ),
        units: [
            ReferenceUnit(name: "clove", grams: "5", millilitres: nil),
            ReferenceUnit(name: "teaspoon, minced", grams: "2.8", millilitres: "5"),
        ]
    )

    static func detail(_ deletion: IngredientDetailDeletion = .idle) -> IngredientDetailView {
        IngredientDetailView(ingredient: garlic, deletion: deletion)
    }

    static let blankDraft = IngredientDraft(
        mode: .create, name: "", notes: "", basis: .per100Grams,
        calories: "", protein: "", carbohydrates: "", fat: "", units: []
    )

    static let garlicDraft = IngredientDraft(
        mode: .edit(id: garlicId), name: "Garlic", notes: "Store in a cool, dry place.",
        basis: .per100Grams, calories: "149", protein: "6.36", carbohydrates: "33.06", fat: "",
        units: [
            ReferenceUnitDraft(name: "clove", grams: "5", millilitres: ""),
            ReferenceUnitDraft(name: "teaspoon, minced", grams: "2.8", millilitres: "5"),
        ]
    )

    static func form(
        _ draft: IngredientDraft,
        dirty: Bool = false,
        issues: [ValidationIssue] = [],
        saveError: IngredientSaveError? = nil,
        status: IngredientFormStatus = .editing
    ) -> IngredientFormView {
        IngredientFormView(draft: draft, dirty: dirty, issues: issues, saveError: saveError, status: status)
    }

    static func catalog(
        screen: IngredientCatalogScreenView,
        navigation: IngredientNavigation = .list,
        list: IngredientListView = list()
    ) -> IngredientCatalogView {
        IngredientCatalogView(navigation: navigation, list: list, scope: ScopeId(value: 2), screen: screen)
    }

    static func running(screen: IngredientCatalogScreenView) -> RunningAppView {
        RunningAppView(ingredients: .stack(catalog: catalog(screen: screen)))
    }

    /// An ingredient with a name only.
    static let basil = Ingredient(
        id: "3a7d0f52-6c1e-4b8a-9d2f-5e4c3b2a1908",
        name: "Basil",
        notes: nil,
        nutrition: Nutrition(basis: .per100Grams, calories: nil, protein: nil, carbohydrates: nil, fat: nil),
        units: []
    )

    /// The Mac catalog. With a `form`, the pane is the create form and the
    /// list shows the draft row, as Rust reports them (MAC-05).
    static func splitCatalog(
        list: IngredientListView = list(),
        selected: Ingredient? = nil,
        form: IngredientFormView? = nil
    ) -> IngredientSplitCatalogView {
        let pane: IngredientSplitPaneView = if let form {
            .create(form: form)
        } else {
            selected.map { .detail(detail: IngredientDetailView(ingredient: $0, deletion: .idle)) } ?? .empty
        }
        return IngredientSplitCatalogView(
            list: ScopedIngredientListView(scope: ScopeId(value: 3), value: list),
            selected: form == nil ? selected?.id : nil,
            pane: ScopedIngredientSplitPaneView(scope: ScopeId(value: 4), value: pane),
            canCreate: form == nil,
            draftRow: form.map { $0.draft.name.trimmingCharacters(in: .whitespaces) }
        )
    }

    /// The Mac catalog with the edit form open: the saved row stays
    /// highlighted, and there is no draft row (MAC-06/Q1).
    static func splitCatalog(editing saved: Ingredient, form: IngredientFormView) -> IngredientSplitCatalogView {
        IngredientSplitCatalogView(
            list: ScopedIngredientListView(scope: ScopeId(value: 3), value: list()),
            selected: saved.id,
            pane: ScopedIngredientSplitPaneView(
                scope: ScopeId(value: 5), value: .edit(savedName: saved.name, form: form)
            ),
            canCreate: false,
            draftRow: nil
        )
    }
}
