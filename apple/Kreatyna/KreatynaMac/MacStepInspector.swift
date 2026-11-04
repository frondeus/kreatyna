import SwiftUI

/// The inspector column of the recipe editor (REC-14/Q1). It shows the
/// selected step's Title field and its problem, or "Select a step". Duration,
/// Instructions, and After are placeholders until REC-17 and REC-18.
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
            // Placeholders for later slices: they show where the fields go.
            field("Duration") { placeholder("Coming soon") }
            field("Instructions") { placeholder("Coming soon") }
            field("After") { placeholder("Coming soon") }
            Spacer(minLength: 0)
        }
        .padding(16)
        .onAppear { titleFocused = true }
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
