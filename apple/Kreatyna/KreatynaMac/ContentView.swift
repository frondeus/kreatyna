import SwiftUI

/// Xcode's template window. MAC-01 replaces it with the ingredient catalog.
struct ContentView: View {
    var body: some View {
        VStack {
            Image(systemName: "globe")
                .imageScale(.large)
                .foregroundStyle(.tint)
            Text("Hello, world!")
                .accessibilityIdentifier("greeting")
        }
        .padding()
    }
}

#Preview {
    ContentView()
}
