import SwiftUI

/// The Ingredients list in the recipe editor (REC-07), under the canvas.
/// ＋ Line, each name, the − button and each drop go to Rust as
/// `IngredientLinesEvent`s; Rust keeps the lines, their order and their
/// problems.
///
/// The rows are plain views in a `ScrollView`, not a `List` or `Form`: a drag
/// inside a grouped `Form` did not reorder on macOS (MAC-05/Q4). The handle
/// uses a `DragGesture`, as the canvas nodes do, not a system drag: Swift
/// finds the row under the pointer, and the drop sends `Move` to Rust.
struct MacIngredientLines: View {
    let lines: [IngredientLineView]
    let send: (IngredientLinesEvent) -> Void

    /// Presentation only: a new line's name field gets the cursor.
    @FocusState private var focused: IngredientLineId?
    /// Presentation only: the line being dragged and the row under the
    /// pointer, which shows the drop.
    @State private var drag: LineDrag?
    /// Presentation only: each row's frame, to find the row under the pointer.
    @State private var rowFrames: [IngredientLineId: CGRect] = [:]
    /// Presentation only: the rows' height, so the list grows to 6 rows and
    /// then scrolls.
    @State private var contentHeight: CGFloat = 0

    private static let maxHeight: CGFloat = 6 * 32
    private static let space = "ingredientLines"

    var body: some View {
        VStack(alignment: .leading, spacing: 6) {
            HStack {
                Text("Ingredients")
                    .font(.headline)
                Spacer()
                Button("＋ Line") { send(.add) }
                    .accessibilityIdentifier("addIngredientLine")
            }
            if !lines.isEmpty {
                ScrollView {
                    VStack(alignment: .leading, spacing: 2) {
                        ForEach(Array(lines.enumerated()), id: \.element.id) { index, line in
                            row(index: index, line)
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
        .onChange(of: lines.map(\.id)) { old, new in
            // ＋ Line adds one id at the bottom; a move or a remove adds none.
            if let added = new.last, !old.contains(added) { focused = added }
        }
    }

    private func row(index: Int, _ line: IngredientLineView) -> some View {
        HStack(alignment: .firstTextBaseline, spacing: 8) {
            Text("≡")
                .font(.system(size: 15))
                .foregroundStyle(.secondary)
                .frame(width: 18)
                .contentShape(Rectangle())
                .help("Drag to reorder")
                .gesture(reorder(line.id))
                .accessibilityLabel("Reorder")
                .accessibilityIdentifier("ingredientLineHandle.\(index)")
            VStack(alignment: .leading, spacing: 2) {
                SyncedTextField("Name", value: line.name, prompt: "Name") { send(.setName(id: line.id, name: $0)) }
                    .labelsHidden()
                    .textFieldStyle(.roundedBorder)
                    .focused($focused, equals: line.id)
                    .accessibilityIdentifier("ingredientLineName.\(index)")
                if let problem = line.problem {
                    Text(problem.message)
                        .font(.footnote)
                        .foregroundStyle(.red)
                        .accessibilityIdentifier("ingredientLineProblem.\(index)")
                }
            }
            Button("Remove Line", systemImage: "minus.circle.fill") { send(.remove(id: line.id)) }
                .labelStyle(.iconOnly)
                .buttonStyle(.borderless)
                .foregroundStyle(.red)
                .accessibilityIdentifier("removeIngredientLine.\(index)")
        }
        .padding(.vertical, 2)
        .padding(.horizontal, 4)
        .background(
            RoundedRectangle(cornerRadius: 6)
                .fill(drag?.target == line.id ? Color.accentColor.opacity(0.15) : .clear)
        )
        .opacity(drag?.id == line.id ? 0.5 : 1)
        .onGeometryChange(for: CGRect.self) { $0.frame(in: .named(Self.space)) } action: { rowFrames[line.id] = $0 }
        .accessibilityElement(children: .contain)
        .accessibilityIdentifier("ingredientLineRow.\(index)")
    }

    /// A drag of the line `id` by its handle. The drop on a row sends `Move`;
    /// Rust decides what it does, also for a drop on the line itself.
    private func reorder(_ id: IngredientLineId) -> some Gesture {
        DragGesture(minimumDistance: 3, coordinateSpace: .named(Self.space))
            .onChanged { value in
                let target = rowFrames.first { $0.value.contains(value.location) }?.key
                drag = LineDrag(id: id, target: target)
            }
            .onEnded { _ in
                if let target = drag?.target { send(.move(id: id, onto: target)) }
                drag = nil
            }
    }
}

/// A handle drag in progress: the dragged line, and the row under the
/// pointer, if any.
struct LineDrag: Equatable {
    let id: IngredientLineId
    let target: IngredientLineId?
}
