//
//  MausritterApp.swift
//  Mausritter
//
//  Created by Wojciech  Polak on 13/09/2026.
//

import SwiftUI

@main
struct MausritterApp: App {
    init() {
        RustLogging.start()
    }

    var body: some Scene {
        WindowGroup {
            ContentView()
        }
    }
}
