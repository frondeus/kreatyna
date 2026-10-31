/// The sidebar's words and icons. Rust names the sections; Swift writes how
/// they look and their order (REC-01).
extension AppSection {
    /// The sidebar order: Recipes, then Ingredients.
    static let sidebarOrder: [AppSection] = [.recipes, .ingredients]

    var title: String {
        switch self {
        case .recipes: "Recipes"
        case .ingredients: "Ingredients"
        }
    }

    var systemImage: String {
        switch self {
        case .recipes: "book"
        case .ingredients: "carrot"
        }
    }

    var accessibilityID: String {
        switch self {
        case .recipes: "sidebarRecipes"
        case .ingredients: "sidebarIngredients"
        }
    }
}
