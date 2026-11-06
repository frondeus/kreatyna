import SwiftUI

/// The Ingredients list in the recipe editor, under the canvas. Each row shows
/// Rust's summary, such as "**300 g** penne pasta, extra virgin" (REC-08), and
/// a click on it opens the ingredient's popover. ＋ Line, each edit in the
/// popover, the − button and each drop go to Rust as
/// `RecipeIngredientsEvent`s; Rust keeps the ingredients, their order and
/// their problems.
///
/// The rows are plain views in a `ScrollView`, not a `List` or `Form`: a drag
/// inside a grouped `Form` did not reorder on macOS (MAC-05/Q4). The handle
/// uses a `DragGesture`, as the canvas nodes do, not a system drag: Swift
/// finds the row under the pointer, and the drop sends `Move` to Rust.
struct MacRecipeIngredients: View {
    let ingredients: [RecipeIngredientView]
    let units: [KnownUnitOption]
    let send: (RecipeIngredientsEvent) -> Void

    /// Presentation only: the ingredient whose popover is open, like the
    /// keyboard focus (REC-08). ＋ Line opens the new ingredient's popover.
    @State private var open: RecipeIngredientId?
    /// Presentation only: the ingredient being dragged and the row under the
    /// pointer, which shows the drop.
    @State private var drag: RecipeIngredientDrag?
    /// Presentation only: each row's frame, to find the row under the pointer.
    @State private var rowFrames: [RecipeIngredientId: CGRect] = [:]
    /// Presentation only: the rows' height, so the list grows to 6 rows and
    /// then scrolls.
    @State private var contentHeight: CGFloat = 0

    private static let maxHeight: CGFloat = 6 * 32
    private static let space = "recipeIngredients"

    var body: some View {
        VStack(alignment: .leading, spacing: 6) {
            HStack {
                Text("Ingredients")
                    .font(.headline)
                Spacer()
                Button("＋ Line") { send(.add) }
                    .accessibilityIdentifier("addRecipeIngredient")
            }
            if !ingredients.isEmpty {
                ScrollView {
                    VStack(alignment: .leading, spacing: 2) {
                        ForEach(Array(ingredients.enumerated()), id: \.element.id) { index, ingredient in
                            row(index: index, ingredient)
                        }
                    }
                    .coordinateSpace(.named(Self.space))
                    .onGeometryChange(for: CGFloat.self) { $0.size.height } action: { contentHeight = $0 }
                }
                .frame(height: min(contentHeight, Self.maxHeight))
                .scrollBounceBehavior(.basedOnSize)
            }
        }
        .padding(.horizontal, 20)
        .padding(.vertical, 12)
        .onChange(of: ingredients.map(\.id)) { old, new in
            // ＋ Line adds one id at the bottom; a move or a remove adds none.
            if let added = new.last, !old.contains(added) { open = added }
        }
    }

    private func row(index: Int, _ ingredient: RecipeIngredientView) -> some View {
        HStack(alignment: .firstTextBaseline, spacing: 8) {
            Text("≡")
                .font(.system(size: 15))
                .foregroundStyle(.secondary)
                .frame(width: 18)
                .contentShape(Rectangle())
                .help("Drag to reorder")
                .gesture(reorder(ingredient.id))
                .accessibilityLabel("Reorder")
                .accessibilityIdentifier("recipeIngredientHandle.\(index)")
            VStack(alignment: .leading, spacing: 2) {
                Button { open = ingredient.id } label: {
                    MacRecipeIngredientSummary(summary: ingredient.summary)
                        .frame(maxWidth: .infinity, alignment: .leading)
                        .contentShape(Rectangle())
                }
                .buttonStyle(.plain)
                .accessibilityIdentifier("recipeIngredientSummary.\(index)")
                .popover(isPresented: isOpen(ingredient.id), arrowEdge: .bottom) {
                    MacRecipeIngredientEditor(ingredient: ingredient, units: units, send: send)
                }
                ForEach(messages(ingredient.problems), id: \.self) { message in
                    Text(message)
                        .font(.footnote)
                        .foregroundStyle(.red)
                        .accessibilityIdentifier("recipeIngredientProblem.\(index)")
                }
            }
            Button("Remove Line", systemImage: "minus.circle.fill") { send(.remove(id: ingredient.id)) }
                .labelStyle(.iconOnly)
                .buttonStyle(.borderless)
                .foregroundStyle(.red)
                .accessibilityIdentifier("removeRecipeIngredient.\(index)")
        }
        .padding(.vertical, 4)
        .padding(.horizontal, 4)
        .background(
            RoundedRectangle(cornerRadius: 6)
                .fill(drag?.target == ingredient.id ? Color.accentColor.opacity(0.15) : .clear)
        )
        .opacity(drag?.id == ingredient.id ? 0.5 : 1)
        .onGeometryChange(for: CGRect.self) { $0.frame(in: .named(Self.space)) } action: { rowFrames[ingredient.id] = $0 }
        .accessibilityElement(children: .contain)
        .accessibilityIdentifier("recipeIngredientRow.\(index)")
    }

    /// Whether the popover of `id` is open. Closing it, by a click outside
    /// or Escape, forgets which one was open.
    private func isOpen(_ id: RecipeIngredientId) -> Binding<Bool> {
        Binding(
            get: { open == id },
            set: { if !$0, open == id { open = nil } }
        )
    }

    /// The text of each problem, name first, as the popover orders its fields.
    private func messages(_ problems: RecipeIngredientProblems) -> [String] {
        [problems.name?.message, problems.amount?.message].compactMap { $0 }
    }

    /// A drag of the ingredient `id` by its handle. The drop on a row sends `Move`;
    /// Rust decides what it does, also for a drop on the ingredient itself.
    private func reorder(_ id: RecipeIngredientId) -> some Gesture {
        DragGesture(minimumDistance: 3, coordinateSpace: .named(Self.space))
            .onChanged { value in
                let target = rowFrames.first { $0.value.contains(value.location) }?.key
                drag = RecipeIngredientDrag(id: id, target: target)
            }
            .onEnded { _ in
                if let target = drag?.target { send(.move(id: id, onto: target)) }
                drag = nil
            }
    }
}

/// A row's summary as Rust gives it: the amount in bold, the name, and
/// ", note" in grey. An unnamed ingredient shows "No name" in grey.
struct MacRecipeIngredientSummary: View {
    let summary: RecipeIngredientSummary

    var body: some View {
        Text("\(amount)\(name)\(note)")
            .lineLimit(1)
            .accessibilityLabel(Self.plain(summary))
    }

    /// The summary as one plain line, "300 g penne pasta, extra virgin", for
    /// VoiceOver and the UI tests.
    static func plain(_ summary: RecipeIngredientSummary) -> String {
        let name = switch summary.name {
        case .named(let name): name
        case .unnamed: "No name"
        }
        let amount = summary.amount.map { "\($0) " } ?? ""
        let note = summary.note.map { ", \($0)" } ?? ""
        return amount + name + note
    }

    private var amount: Text {
        summary.amount.map { Text(verbatim: "\($0) ").bold() } ?? Text(verbatim: "")
    }

    private var name: Text {
        switch summary.name {
        case .named(let name): Text(verbatim: name)
        case .unnamed: Text("No name").italic().foregroundStyle(.secondary)
        }
    }

    private var note: Text {
        summary.note.map { Text(verbatim: ", \($0)").foregroundStyle(.secondary) } ?? Text(verbatim: "")
    }
}

/// A handle drag in progress: the dragged ingredient, and the row under the
/// pointer, if any.
struct RecipeIngredientDrag: Equatable {
    let id: RecipeIngredientId
    let target: RecipeIngredientId?
}
