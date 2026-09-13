//
//  ContentView.swift
//  Mausritter
//
//  Created by Wojciech  Polak on 13/09/2026.
//

import SwiftUI

struct ContentView: View {
    var body: some View {
        VStack {
            Image(systemName: "globe")
                .imageScale(.large)
                .foregroundStyle(.tint)
            Text("Rust says: \(addWithOne(left: 2, right: 3))")
        }
        .padding()
    }
}

#Preview {
    ContentView()
}
