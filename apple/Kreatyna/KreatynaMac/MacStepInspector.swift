import SwiftUI

/// The inspector column of the recipe editor (REC-14/Q1). It shows the
/// selected step's Title, Duration, Active/Waiting, and Instructions fields
/// with their problems (REC-17), or "Select a step". After is a placeholder
/// until REC-18.
struct MacStepInspector: View {
    let canvas: StepCanvasView
    let send: (StepCanvasEvent) -> Void

    static let width: CGFloat = 260

    var body: some View {
        Group {
            switch canvas {
            case .unselected:
                Text("Select a step")
                    .foregroundStyle(.secondary)
                    .frame(maxWidth: .infinity, maxHeight: .infinity)
                    .accessibilityIdentifier("selectStepHint")
            case .selected(let selected, _):
                MacStepFields(step: selected, send: send)
                    // A new field for each step, so its text starts from that step.
                    .id(selected.id)
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
            // A placeholder for REC-18: it shows where the field goes.
            field("After") { placeholder("Coming soon") }
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

    private func placeholder(_ text: String) -> some View {
        TextField(text, text: .constant(""), prompt: Text(text))
            .labelsHidden()
            .textFieldStyle(.roundedBorder)
            .disabled(true)
    }
}
