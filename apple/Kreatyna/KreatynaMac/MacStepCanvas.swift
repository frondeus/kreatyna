import SwiftUI

/// The recipe editor's step canvas (REC-14). Rust keeps the steps, their
/// positions, and the selection; Swift draws a node at each position and sends
/// the clicks and Add Step.
///
/// A click on a node selects it; a click on empty canvas clears the selection.
/// The context menu has Add Step, which adds a step where the pointer was.
/// A node's own context menu has Delete Step (REC-16).
/// A drag moves a node: Swift draws the drag, and sends one Move on the drop
/// (REC-15). Rust places and selects the step.
struct MacStepCanvas: View {
    let canvas: StepCanvasView
    let send: (StepCanvasEvent) -> Void

    /// Presentation only: the last pointer location on the canvas. SwiftUI's
    /// context menu does not give the click location, so Add Step uses it.
    @State private var pointer: CGPoint = .zero
    /// Presentation only: the node under a drag, and where Swift draws it.
    @State private var drag: NodeDrag?

    static let nodeSize = CGSize(width: 150, height: 64)
    /// Room right of and below the last node, so a step can be added there.
    private static let margin: CGFloat = 120
    /// A press must move this far before it is a drag, not a click.
    private static let dragDistance: CGFloat = 4
    private static let space = "stepCanvas"

    var body: some View {
        GeometryReader { proxy in
            ScrollView([.horizontal, .vertical]) {
                ZStack(alignment: .topLeading) {
                    MacCanvasGrid()
                        .contentShape(.rect)
                        .onTapGesture { send(.clearSelection) }
                    ForEach(nodes, id: \.node.id) { item in
                        let origin = origin(of: item.node)
                        MacStepNode(node: item.node, selected: item.selected)
                            .offset(x: origin.x, y: origin.y)
                            // The dragged node draws above the others.
                            .zIndex(drag?.id == item.node.id ? 1 : 0)
                            .gesture(dragGesture(for: item.node))
                            .onTapGesture { send(.select(id: item.node.id)) }
                            .contextMenu {
                                Button("Delete Step", systemImage: "trash", role: .destructive) {
                                    send(.delete(id: item.node.id))
                                }
                                .accessibilityIdentifier("deleteStep")
                            }
                    }
                }
                .coordinateSpace(.named(Self.space))
                .frame(width: contentSize(min: proxy.size).width, height: contentSize(min: proxy.size).height, alignment: .topLeading)
                .onContinuousHover(coordinateSpace: .local) { phase in
                    if case .active(let location) = phase { pointer = location }
                }
                .contextMenu {
                    Button("Add Step", systemImage: "plus") { send(.add(at: Self.canvasPoint(pointer))) }
                        .accessibilityIdentifier("addStep")
                }
            }
            .accessibilityIdentifier("stepCanvas")
        }
        .onChange(of: canvas) {
            // The next view has the dropped node where Rust put it.
            if case .dropped = drag { drag = nil }
        }
    }

    /// Every node, the selected one last so it draws on top.
    private var nodes: [(node: StepNodeView, selected: Bool)] {
        switch canvas.nodes {
        case .unselected(let steps):
            steps.map { ($0, false) }
        case .selected(let selected, let others):
            others.map { ($0, false) } + [(selected, true)]
        case .connectionSelected(_, let steps):
            steps.map { ($0, false) }
        }
    }

    /// Where the node draws: Rust's position, or the drag's while it runs.
    private func origin(of node: StepNodeView) -> CGPoint {
        switch drag {
        case .moving(let id, let translation) where id == node.id:
            Self.moved(node.position, by: translation)
        case .dropped(let id, let point) where id == node.id:
            CGPoint(x: CGFloat(point.x), y: CGFloat(point.y))
        default:
            CGPoint(x: CGFloat(node.position.x), y: CGFloat(node.position.y))
        }
    }

    private func dragGesture(for node: StepNodeView) -> some Gesture {
        DragGesture(minimumDistance: Self.dragDistance, coordinateSpace: .named(Self.space))
            .onChanged { value in
                drag = .moving(id: node.id, translation: value.translation)
            }
            .onEnded { value in
                let to = Self.canvasPoint(Self.moved(node.position, by: value.translation))
                // Swift draws the drop point until the next view arrives, so
                // the node does not jump back for a frame.
                drag = .dropped(id: node.id, at: to)
                send(.move(id: node.id, to: to))
            }
    }

    /// `position` plus `translation`. A drag stops at the left and the top
    /// edge, as the drop does in `canvasPoint`.
    static func moved(_ position: CanvasPoint, by translation: CGSize) -> CGPoint {
        CGPoint(
            x: max(0, CGFloat(position.x) + translation.width),
            y: max(0, CGFloat(position.y) + translation.height)
        )
    }

    /// The scrolled area: the visible size, or more when a node lies beyond it.
    private func contentSize(min visible: CGSize) -> CGSize {
        let right = nodes.map { CGFloat($0.node.position.x) + Self.nodeSize.width + Self.margin }.max() ?? 0
        let bottom = nodes.map { CGFloat($0.node.position.y) + Self.nodeSize.height + Self.margin }.max() ?? 0
        return CGSize(width: max(visible.width, right), height: max(visible.height, bottom))
    }

    /// Rust takes whole points from the top left corner, never negative.
    static func canvasPoint(_ location: CGPoint) -> CanvasPoint {
        CanvasPoint(x: coordinate(location.x), y: coordinate(location.y))
    }

    /// Rounds to a whole point in `0...UInt32.max`. A value that is not
    /// finite gives 0, so a bad pointer location cannot stop the app.
    private static func coordinate(_ value: CGFloat) -> UInt32 {
        guard value.isFinite else { return 0 }
        let rounded = value.rounded()
        if rounded <= 0 { return 0 }
        if rounded >= CGFloat(UInt32.max) { return .max }
        return UInt32(rounded)
    }
}

/// A drag on a step node. Only Swift knows about it; Rust gets the drop.
private enum NodeDrag: Equatable {
    /// The pointer is down: the node follows it by `translation`.
    case moving(id: StepId, translation: CGSize)
    /// Swift sent Move; the node stays at the drop point until the next view.
    case dropped(id: StepId, at: CanvasPoint)

    var id: StepId {
        switch self {
        case .moving(let id, _), .dropped(let id, _): id
        }
    }
}

/// One step on the canvas: the label Rust gives it, a title or "Untitled
/// step", and under it the duration and the kind (REC-17). A waiting step
/// has a dashed border. The selected node has the accent border; a node
/// with a problem has a red border.
struct MacStepNode: View {
    let node: StepNodeView
    let selected: Bool

    var body: some View {
        let shape = RoundedRectangle(cornerRadius: 10)
        VStack(alignment: .leading, spacing: 2) {
            Text(text)
                .font(.body.weight(.medium))
                .foregroundStyle(untitled ? .secondary : .primary)
                .lineLimit(2)
            Text(details)
                .font(.caption)
                .foregroundStyle(.secondary)
                .lineLimit(1)
        }
        .padding(.horizontal, 12)
        .frame(width: MacStepCanvas.nodeSize.width, height: MacStepCanvas.nodeSize.height, alignment: .leading)
        .background(background, in: shape)
        .overlay(shape.strokeBorder(border, style: borderStyle))
        .shadow(color: .black.opacity(0.08), radius: 2, y: 1)
        .contentShape(shape)
        .accessibilityElement(children: .ignore)
        .accessibilityLabel(text)
        .accessibilityValue(details)
            .accessibilityAddTraits(selected ? [.isButton, .isSelected] : .isButton)
            .accessibilityIdentifier("stepNode")
    }

    private var text: String {
        switch node.label {
        case .titled(let title): title
        case .untitled: "Untitled step"
        }
    }

    /// "10 min · waiting". A duration that is too long shows "No duration",
    /// as Rust has no duration to save.
    private var details: String {
        let duration = if case .set(let text, _, _) = node.duration { text } else { "No duration" }
        return "\(duration) · \(waiting ? "waiting" : "active")"
    }

    private var untitled: Bool {
        node.label == .untitled
    }

    private var waiting: Bool {
        node.kind == .waiting
    }

    private var problem: Bool {
        node.problems.title != nil || node.problems.duration != nil
    }

    private var background: Color {
        waiting ? Color(nsColor: .underPageBackgroundColor) : Color(nsColor: .controlBackgroundColor)
    }

    private var border: Color {
        if problem { .red } else if selected { .accentColor } else { Color(nsColor: .separatorColor) }
    }

    private var borderStyle: StrokeStyle {
        StrokeStyle(lineWidth: selected || problem ? 2 : 1, dash: waiting ? [5, 3] : [])
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
