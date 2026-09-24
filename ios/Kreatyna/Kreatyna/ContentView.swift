//
//  ContentView.swift
//  Kreatyna
//
//  Created by Wojciech  Polak on 13/09/2026.
//

import SwiftUI

struct ContentView: View {
    @State private var model = CounterModel()

    var body: some View {
        VStack {
            Image(systemName: "globe")
                .imageScale(.large)
                .foregroundStyle(.tint)
            Text("Counter: \(model.value)")
                .accessibilityIdentifier("counterValue")
            HStack {
                Button("Decrease", systemImage: "minus") {
                    model.dispatch(.decrease)
                }
                .accessibilityIdentifier("decreaseCounter")

                Button("Increase", systemImage: "plus") {
                    model.dispatch(.increase)
                }
                .accessibilityIdentifier("increaseCounter")
            }
            .buttonStyle(.bordered)
            Button("Reset", systemImage: "arrow.counterclockwise") {
                model.dispatch(.reset)
            }
            .accessibilityIdentifier("resetCounter")
            .buttonStyle(.bordered)
            HStack {
                Button("Start", systemImage: "play.fill") {
                    model.dispatch(.startBackground)
                }
                .accessibilityIdentifier("startBackground")

                Button("Stop", systemImage: "stop.fill") {
                    model.dispatch(.stopBackground)
                }
                .accessibilityIdentifier("stopBackground")
            }
            .buttonStyle(.bordered)
            if let errorMessage = model.errorMessage {
                Text(errorMessage)
                    .foregroundStyle(.red)
                    .accessibilityIdentifier("counterError")
            }
        }
        .padding()
        .onDisappear {
            model.dispatch(.stopBackground)
        }
    }
}

#Preview {
    ContentView()
}
