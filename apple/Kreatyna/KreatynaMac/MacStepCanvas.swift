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
///
/// Each node has ○ on its left and ● on its right (REC-18). A drag from a ●
/// draws a dashed line to the pointer; a drop anywhere on another node sends
/// Connect, and Rust adds the arrow or refuses it. Arrows run from ● to ○
/// under the nodes. A click near an arrow selects it; its context menu has
/// Delete Connection.
struct MacStepCanvas: View {
    let canvas: StepCanvasView
    let send: (StepCanvasEvent) -> Void

    init(canvas: StepCanvasView, send: @escaping (StepCanvasEvent) -> Void) {
        self.init(canvas: canvas, connecting: nil, send: send)
    }

    /// Starts with a connection drag, so a snapshot can show the dashed line.
    init(canvas: StepCanvasView, connecting: ConnectionDrag?, send: @escaping (StepCanvasEvent) -> Void) {
        self.canvas = canvas
        self.send = send
        _connecting = State(initialValue: connecting)
    }

    /// Presentation only: the last pointer location on the canvas. SwiftUI's
    /// context menu does not give the click location, so Add Step uses it.
    @State private var pointer: CGPoint = .zero
    /// Presentation only: the node under a drag, and where Swift draws it.
    @State private var drag: NodeDrag?
    /// Presentation only: a drag from a ●, until the drop.
    @State private var connecting: ConnectionDrag?

    static let nodeSize = CGSize(width: 150, height: 64)
    /// Room right of and below the last node, so a step can be added there.
    private static let margin: CGFloat = 120
    /// A press must move this far before it is a drag, not a click.
    private static let dragDistance: CGFloat = 4
    private static let space = "stepCanvas"
    /// A click this close to an arrow selects it.
    static let arrowHitDistance: CGFloat = 5

    var body: some View {
        GeometryReader { proxy in
            ScrollView([.horizontal, .vertical]) {
                ZStack(alignment: .topLeading) {
                    MacCanvasGrid()
                        .contentShape(.rect)
                        .onTapGesture(coordinateSpace: .local) { location in
                            if let edge = Self.connection(near: location, among: steps, in: canvas.connections) {
                                send(.selectConnection(edge: edge))
                            } else {
                                send(.clearSelection)
                            }
                        }
                    MacConnections(arrows: arrows, rubberBand: rubberBand)
                        .allowsHitTesting(false)
                    ForEach(nodes, id: \.node.id) { item in
                        let origin = origin(of: item.node)
                        MacStepNode(node: item.node, selected: item.selected)
                            .overlay(alignment: .leading) {
                                MacPort(kind: .input, selected: item.selected)
                                    .offset(x: -MacPort.target / 2)
                                    .allowsHitTesting(false)
                            }
                            .overlay(alignment: .trailing) {
                                MacPort(kind: .output, selected: item.selected)
                                    .offset(x: MacPort.target / 2)
                                    .gesture(connectGesture(from: item.node))
                            }
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
                    // SwiftUI builds the menu from the last pointer location,
                    // so a right-click near an arrow offers to delete it.
                    if let edge = Self.connection(near: pointer, among: steps, in: canvas.connections) {
                        Button("Delete Connection", systemImage: "trash", role: .destructive) {
                            send(.deleteConnection(edge: edge))
                        }
                        .accessibilityIdentifier("deleteConnection")
                    } else {
                        Button("Add Step", systemImage: "plus") { send(.add(at: Self.canvasPoint(pointer))) }
                            .accessibilityIdentifier("addStep")
                    }
                }
            }
            .accessibilityIdentifier("stepCanvas")
        }
        .onChange(of: canvas) {
            // The next view has the dropped node where Rust put it.
            if case .dropped = drag { drag = nil }
        }
    }

    /// Every step, in the order added.
    private var steps: [StepNodeView] {
        switch canvas.nodes {
        case .unselected(let steps), .connectionSelected(_, let steps): steps
        case .selected(let selected, let others): others + [selected]
        }
    }

    /// The arrows as Swift draws them: from the ● to the ○, following a
    /// node while it is dragged.
    private var arrows: [MacArrow] {
        let selected: StepEdge? = if case .connectionSelected(let connection, _) = canvas.nodes { connection.edge } else { nil }
        let origins = Dictionary(steps.map { ($0.id, origin(of: $0)) }, uniquingKeysWith: { first, _ in first })
        return canvas.connections.compactMap { edge in
            guard let from = origins[edge.from], let to = origins[edge.to] else { return nil }
            return MacArrow(
                start: Self.outPort(at: from), end: Self.inPort(at: to), selected: edge == selected
            )
        }
    }

    /// The dashed line of a connection drag. Over a node, it ends at the
    /// node's ○, where the arrow will end.
    private var rubberBand: MacArrow? {
        guard let connecting, let from = steps.first(where: { $0.id == connecting.from }) else { return nil }
        let start = Self.outPort(at: origin(of: from))
        let end = Self.node(at: connecting.pointer, among: steps).map { Self.inPort(at: origin(of: $0)) }
        return MacArrow(start: start, end: end ?? connecting.pointer, selected: true)
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

    /// A drag from the ● of `node`. A drop on a node sends Connect, also on
    /// `node` itself, so Rust can say why not. A drop on empty canvas sends
    /// nothing.
    private func connectGesture(from node: StepNodeView) -> some Gesture {
        DragGesture(minimumDistance: 1, coordinateSpace: .named(Self.space))
            .onChanged { value in
                connecting = ConnectionDrag(from: node.id, pointer: value.location)
            }
            .onEnded { value in
                connecting = nil
                if let target = Self.node(at: value.location, among: steps) {
                    send(.connect(from: node.id, to: target.id))
                }
            }
    }

    /// The ● of a node whose top left corner is `origin`.
    static func outPort(at origin: CGPoint) -> CGPoint {
        CGPoint(x: origin.x + nodeSize.width, y: origin.y + nodeSize.height / 2)
    }

    /// Where an arrow ends: just left of the ○ of a node whose top left
    /// corner is `origin`, so the ○ does not hide the arrowhead.
    static func inPort(at origin: CGPoint) -> CGPoint {
        CGPoint(x: origin.x - arrowGap, y: origin.y + nodeSize.height / 2)
    }

    /// The ○'s radius and a point of space.
    private static let arrowGap: CGFloat = 6

    /// The topmost node under `point`, if any.
    static func node(at point: CGPoint, among steps: [StepNodeView]) -> StepNodeView? {
        steps.last { step in
            CGRect(origin: CGPoint(x: CGFloat(step.position.x), y: CGFloat(step.position.y)), size: nodeSize)
                .contains(point)
        }
    }

    /// The arrow within `arrowHitDistance` of `point`, if any. With two
    /// near, the closer one.
    static func connection(near point: CGPoint, among steps: [StepNodeView], in edges: [StepEdge]) -> StepEdge? {
        let positions = Dictionary(steps.map { ($0.id, $0.position) }, uniquingKeysWith: { first, _ in first })
        let distances: [(edge: StepEdge, distance: CGFloat)] = edges.compactMap { edge in
            guard let from = positions[edge.from], let to = positions[edge.to] else { return nil }
            let arrow = MacArrow(
                start: outPort(at: CGPoint(x: CGFloat(from.x), y: CGFloat(from.y))),
                end: inPort(at: CGPoint(x: CGFloat(to.x), y: CGFloat(to.y))),
                selected: false
            )
            return (edge, arrow.distance(to: point))
        }
        return distances.filter { $0.distance <= arrowHitDistance }.min { $0.distance < $1.distance }?.edge
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

/// A drag from the ● of the step `from`. Only Swift knows about it; Rust
/// gets Connect on the drop.
struct ConnectionDrag: Equatable {
    let from: StepId
    /// In canvas points.
    let pointer: CGPoint
}

/// One arrow, or the dashed line of a drag: a curve from the ● at `start`
/// to the ○ at `end`, as in the mockups.
struct MacArrow: Equatable {
    let start: CGPoint
    let end: CGPoint
    let selected: Bool

    /// The two control points pull the curve out of the ● and into the ○
    /// sideways, at least 30 points.
    private var pull: CGFloat { max(30, (end.x - start.x) / 2) }

    var path: Path {
        Path { path in
            path.move(to: start)
            path.addCurve(
                to: end,
                control1: CGPoint(x: start.x + pull, y: start.y),
                control2: CGPoint(x: end.x - pull, y: end.y)
            )
        }
    }

    /// The point at `t` in `0...1` along the curve.
    func point(at t: CGFloat) -> CGPoint {
        let c1 = CGPoint(x: start.x + pull, y: start.y)
        let c2 = CGPoint(x: end.x - pull, y: end.y)
        let u = 1 - t
        let a = u * u * u, b = 3 * u * u * t, c = 3 * u * t * t, d = t * t * t
        return CGPoint(
            x: a * start.x + b * c1.x + c * c2.x + d * end.x,
            y: a * start.y + b * c1.y + c * c2.y + d * end.y
        )
    }

    /// How far `point` is from the curve: the nearest of 64 straight pieces.
    func distance(to point: CGPoint) -> CGFloat {
        let pieces = 64
        let samples = (0...pieces).map { self.point(at: CGFloat($0) / CGFloat(pieces)) }
        return zip(samples, samples.dropFirst()).map { Self.distance(from: point, toSegment: $0, $1) }.min() ?? .infinity
    }

    private static func distance(from p: CGPoint, toSegment a: CGPoint, _ b: CGPoint) -> CGFloat {
        let dx = b.x - a.x, dy = b.y - a.y
        let length = dx * dx + dy * dy
        let t = length == 0 ? 0 : min(1, max(0, ((p.x - a.x) * dx + (p.y - a.y) * dy) / length))
        return hypot(p.x - (a.x + t * dx), p.y - (a.y + t * dy))
    }
}

/// The arrows under the nodes, with arrowheads, and the dashed line of a
/// connection drag. The selected arrow is accent and thicker.
private struct MacConnections: View {
    let arrows: [MacArrow]
    let rubberBand: MacArrow?

    var body: some View {
        Canvas { context, _ in
            for arrow in arrows {
                let color = arrow.selected ? Color.accentColor : Color.secondary
                // The line stops at the arrowhead's back, so its end does not show.
                context.stroke(arrow.path, with: .color(color), lineWidth: arrow.selected ? 3 : 1.6)
                context.fill(Self.head(at: arrow.end, length: arrow.selected ? 10 : 8), with: .color(color))
            }
            if let rubberBand {
                context.stroke(
                    rubberBand.path, with: .color(.accentColor),
                    style: StrokeStyle(lineWidth: 1.6, dash: [6, 5])
                )
                context.fill(
                    Path(ellipseIn: CGRect(x: rubberBand.end.x - 4, y: rubberBand.end.y - 4, width: 8, height: 8)),
                    with: .color(.accentColor)
                )
            }
        }
    }

    /// A triangle that points right with its tip at the ○: the curve
    /// always enters a ○ sideways.
    private static func head(at tip: CGPoint, length: CGFloat) -> Path {
        Path { path in
            path.move(to: tip)
            path.addLine(to: CGPoint(x: tip.x - length, y: tip.y - length / 2))
            path.addLine(to: CGPoint(x: tip.x - length, y: tip.y + length / 2))
            path.closeSubpath()
        }
    }
}

/// A node's ○ (where arrows end) or ● (where you drag from).
private struct MacPort: View {
    enum Kind { case input, output }

    let kind: Kind
    let selected: Bool

    static let size: CGFloat = 10
    /// The area that takes the drag, centered on the dot.
    static let target: CGFloat = size + 8

    var body: some View {
        let color: Color = selected ? .accentColor : .secondary
        Circle()
            .fill(kind == .output ? color : Color(nsColor: .controlBackgroundColor))
            .overlay(Circle().strokeBorder(color, lineWidth: 1.6))
            .frame(width: Self.size, height: Self.size)
            // A larger target than the dot, so the ● is easy to grab.
            .frame(width: Self.target, height: Self.target)
            .contentShape(Circle())
            .accessibilityElement()
            .accessibilityLabel(kind == .output ? "Connect from here" : "Arrow end")
            .accessibilityIdentifier(kind == .output ? "stepOutPort" : "stepInPort")
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
        node.label.text
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
