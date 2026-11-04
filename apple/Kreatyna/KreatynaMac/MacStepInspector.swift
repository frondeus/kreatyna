import SwiftUI

/// The inspector column of the recipe editor (REC-14/Q1). It shows the
/// selected step's Title, Duration, Active/Waiting, and Instructions fields
/// with their problems (REC-17), and After: the steps before it (REC-18). A
/// selected arrow shows its two steps. With nothing selected, it shows
/// "Select a step".
struct MacStepInspector: View {
    let canvas: StepCanvasView
    let send: (StepCanvasEvent) -> Void

    static let width: CGFloat = 260

    var body: some View {
        Group {
            switch canvas.nodes {
            case .unselected:
                Text("Select a step")
                    .foregroundStyle(.secondary)
                    .frame(maxWidth: .infinity, maxHeight: .infinity)
                    .accessibilityIdentifier("selectStepHint")
            case .selected(let selected, _):
                MacStepFields(step: selected, send: send)
                    // A new field for each step, so its text starts from that step.
                    .id(selected.id)
            case .connectionSelected(let connection, _):
                MacConnectionFields(connection: connection)
            }
        }
        .frame(width: Self.width)
        .frame(maxHeight: .infinity, alignment: .top)
        .background(Color(nsColor: .underPageBackgroundColor).opacity(0.4))
    }
}

private struct MacStepFields: View {
    let step: StepNodeView
    let send: (StepCanvasEvent) -> Void

    /// Presentation only: a selected step puts the cursor in its Title.
    @FocusState private var titleFocused: Bool

    var body: some View {
        VStack(alignment: .leading, spacing: 14) {
            Text("Step")
                .font(.headline)
            field("Title") {
                SyncedTextField("Title", value: step.title, prompt: "Required") {
                    send(.setTitle(id: step.id, title: $0))
                }
                .labelsHidden()
                .textFieldStyle(.roundedBorder)
                .focused($titleFocused)
                .accessibilityIdentifier("stepTitleField")
                if let problem = step.problems.title {
                    Text(problem.message)
                        .font(.footnote)
                        .foregroundStyle(.red)
                        .accessibilityIdentifier("stepTitleProblem")
                }
            }
            field("Duration") {
                MacDurationField(duration: step.duration) {
                    send(.setDuration(id: step.id, input: $0))
                }
                if let problem = step.problems.duration {
                    Text(problem.message)
                        .font(.footnote)
                        .foregroundStyle(.red)
                        .accessibilityIdentifier("stepDurationProblem")
                }
            }
            Picker("Kind", selection: kind) {
                Text("Active").tag(StepKind.active)
                Text("Waiting").tag(StepKind.waiting)
            }
            .pickerStyle(.segmented)
            .labelsHidden()
            .accessibilityIdentifier("stepKindPicker")
            field("Instructions") {
                SyncedTextField(
                    "Instructions", value: step.instructions, prompt: "Optional", axis: .vertical
                ) {
                    send(.setInstructions(id: step.id, text: $0))
                }
                .labelsHidden()
                .textFieldStyle(.roundedBorder)
                .lineLimit(3...6)
                .accessibilityIdentifier("stepInstructionsField")
            }
            field("After") { MacAfterChips(after: step.after) }
            Spacer(minLength: 0)
        }
        .padding(16)
        .onAppear { titleFocused = true }
    }

    /// The switch shows Rust's kind; a click sends the other one.
    private var kind: Binding<StepKind> {
        Binding(get: { step.kind }, set: { send(.setKind(id: step.id, kind: $0)) })
    }

    private func field(_ label: String, @ViewBuilder content: () -> some View) -> some View {
        VStack(alignment: .leading, spacing: 4) {
            Text(label.uppercased())
                .font(.caption)
                .foregroundStyle(.secondary)
            content()
        }
    }

}

/// The steps before the selected one, as read-only chips. You change them
/// on the canvas.
private struct MacAfterChips: View {
    let after: [StepAfterView]

    var body: some View {
        VStack(alignment: .leading, spacing: 4) {
            if after.isEmpty {
                Text("None")
                    .foregroundStyle(.secondary)
                    .accessibilityIdentifier("stepAfterNone")
            } else {
                // One chip a line: the inspector is narrow.
                VStack(alignment: .leading, spacing: 4) {
                    ForEach(after, id: \.id) { step in
                        Text(step.label.text)
                            .font(.callout)
                            .lineLimit(1)
                            .padding(.horizontal, 8)
                            .padding(.vertical, 2)
                            .overlay(
                                Capsule().strokeBorder(
                                    Color(nsColor: .separatorColor), style: StrokeStyle(lineWidth: 1, dash: [3, 2])
                                )
                            )
                            .accessibilityIdentifier("stepAfterChip")
                    }
                }
            }
            Text("Drawn on the canvas.")
                .font(.caption)
                .foregroundStyle(.secondary)
        }
    }
}

/// The selected arrow: its two steps, "Dice onion → Fry onion".
private struct MacConnectionFields: View {
    let connection: StepConnectionView

    var body: some View {
        VStack(alignment: .leading, spacing: 8) {
            Text("Connection")
                .font(.headline)
            Text("\(connection.from.text) → \(connection.to.text)")
                .accessibilityIdentifier("connectionSteps")
            Text("Delete it with Edit ▸ Delete Connection or the arrow's menu.")
                .font(.caption)
                .foregroundStyle(.secondary)
            Spacer(minLength: 0)
        }
        .padding(16)
        .frame(maxWidth: .infinity, alignment: .leading)
    }
}
