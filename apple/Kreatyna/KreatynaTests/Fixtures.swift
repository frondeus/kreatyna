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

    static func running(
        tab: AppTab,
        screen: IngredientCatalogScreenView,
        counter: CounterView = CounterView(count: 7, status: .ready)
    ) -> RunningAppView {
        RunningAppView(
            tab: tab, scope: ScopeId(value: 1),
            ingredients: catalog(screen: screen), counter: counter
        )
    }
}
