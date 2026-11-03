import SwiftUI

/// The recipe editor's step canvas (REC-14). Rust keeps the steps, their
/// positions, and the selection; Swift draws a node at each position and sends
/// the clicks and Add Step.
///
/// A click on a node selects it; a click on empty canvas clears the selection.
/// The context menu has Add Step, which adds a step where the pointer was.
struct MacStepCanvas: View {
    let canvas: StepCanvasView
    let send: (StepCanvasEvent) -> Void

    /// Presentation only: the last pointer location on the canvas. SwiftUI's
    /// context menu does not give the click location, so Add Step uses it.
    @State private var pointer: CGPoint = .zero

    static let nodeSize = CGSize(width: 150, height: 54)
    /// Room right of and below the last node, so a step can be added there.
    private static let margin: CGFloat = 120

    var body: some View {
        GeometryReader { proxy in
            ScrollView([.horizontal, .vertical]) {
                ZStack(alignment: .topLeading) {
                    MacCanvasGrid()
                        .contentShape(.rect)
                        .onTapGesture { send(.clearSelection) }
                    ForEach(nodes, id: \.node.id) { item in
                        MacStepNode(node: item.node, selected: item.selected)
                            .offset(x: CGFloat(item.node.position.x), y: CGFloat(item.node.position.y))
                            .onTapGesture { send(.select(id: item.node.id)) }
                    }
                }
                .frame(width: contentSize(min: proxy.size).width, height: contentSize(min: proxy.size).height, alignment: .topLeading)
                .onContinuousHover(coordinateSpace: .local) { phase in
                    if case .active(let location) = phase { pointer = location }
                }
                .contextMenu {
                    Button("Add Step", systemImage: "plus") { send(.add(at: canvasPoint(pointer))) }
                        .accessibilityIdentifier("addStep")
                }
            }
            .accessibilityIdentifier("stepCanvas")
        }
    }

    /// Every node, the selected one last so it draws on top.
    private var nodes: [(node: StepNodeView, selected: Bool)] {
        switch canvas {
        case .unselected(let steps):
            steps.map { ($0, false) }
        case .selected(let selected, let others):
            others.map { ($0, false) } + [(selected, true)]
        }
    }

    /// The scrolled area: the visible size, or more when a node lies beyond it.
    private func contentSize(min visible: CGSize) -> CGSize {
        let right = nodes.map { CGFloat($0.node.position.x) + Self.nodeSize.width + Self.margin }.max() ?? 0
        let bottom = nodes.map { CGFloat($0.node.position.y) + Self.nodeSize.height + Self.margin }.max() ?? 0
        return CGSize(width: max(visible.width, right), height: max(visible.height, bottom))
    }

    /// Rust takes whole points from the top left corner, never negative.
    private func canvasPoint(_ location: CGPoint) -> CanvasPoint {
        CanvasPoint(x: UInt32(max(0, location.x.rounded())), y: UInt32(max(0, location.y.rounded())))
    }
}

/// One step on the canvas: the label Rust gives it, a title or "Untitled
/// step". The selected node has the accent border; a node with a problem has
/// a red border.
struct MacStepNode: View {
    let node: StepNodeView
    let selected: Bool

    var body: some View {
        let shape = RoundedRectangle(cornerRadius: 10)
        Text(text)
            .font(.body.weight(.medium))
            .foregroundStyle(untitled ? .secondary : .primary)
            .lineLimit(2)
            .padding(.horizontal, 12)
            .frame(width: MacStepCanvas.nodeSize.width, height: MacStepCanvas.nodeSize.height, alignment: .leading)
            .background(Color(nsColor: .controlBackgroundColor), in: shape)
            .overlay(shape.strokeBorder(border, lineWidth: selected || node.problem != nil ? 2 : 1))
            .shadow(color: .black.opacity(0.08), radius: 2, y: 1)
            .contentShape(shape)
            .accessibilityElement(children: .ignore)
            .accessibilityLabel(text)
            .accessibilityAddTraits(selected ? [.isButton, .isSelected] : .isButton)
            .accessibilityIdentifier("stepNode")
    }

    private var text: String {
        switch node.label {
        case .titled(let title): title
        case .untitled: "Untitled step"
        }
    }

    private var untitled: Bool {
        node.label == .untitled
    }

    private var border: Color {
        if node.problem != nil { .red } else if selected { .accentColor } else { Color(nsColor: .separatorColor) }
    }
}

/// The dotted background of the canvas, as in the mockups.
private struct MacCanvasGrid: View {
    private static let spacing: CGFloat = 18

    var body: some View {
        Canvas { context, size in
            let dot = Path(ellipseIn: CGRect(x: 0, y: 0, width: 1.5, height: 1.5))
            var y: CGFloat = Self.spacing / 2
            while y < size.height {
                var x: CGFloat = Self.spacing / 2
                while x < size.width {
                    context.fill(dot.offsetBy(dx: x, dy: y), with: .color(Color(nsColor: .separatorColor)))
                    x += Self.spacing
                }
                y += Self.spacing
            }
        }
        .background(Color(nsColor: .textBackgroundColor))
    }
}
