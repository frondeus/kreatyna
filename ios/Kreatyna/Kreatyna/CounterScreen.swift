import SwiftUI

struct CounterScreen: View {
    let view: CounterView
    let send: (CounterEvent) -> Void

    var body: some View {
        VStack(spacing: 16) {
            Image(systemName: "globe")
                .imageScale(.large)
                .foregroundStyle(.tint)
            switch view.status {
            case .loading:
                ProgressView()
                    .accessibilityIdentifier("counterLoading")
            case .ready:
                Text("Counter: \(view.count)")
                    .accessibilityIdentifier("counterValue")
            }
            HStack {
                Button("Decrease", systemImage: "minus") { send(.decrease) }
                    .accessibilityIdentifier("decreaseCounter")
                Button("Increase", systemImage: "plus") { send(.increase) }
                    .accessibilityIdentifier("increaseCounter")
            }
            Button("Reset", systemImage: "arrow.counterclockwise") { send(.reset) }
                .accessibilityIdentifier("resetCounter")
            HStack {
                Button("Start", systemImage: "play.fill") { send(.startBackground) }
                    .accessibilityIdentifier("startBackground")
                Button("Stop", systemImage: "stop.fill") { send(.stopBackground) }
                    .accessibilityIdentifier("stopBackground")
            }
        }
        .buttonStyle(.bordered)
        .padding()
    }
}
