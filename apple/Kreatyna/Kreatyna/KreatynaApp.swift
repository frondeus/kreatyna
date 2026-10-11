//
//  KreatynaApp.swift
//  Kreatyna
//
//  Created by Wojciech  Polak on 13/09/2026.
//

import SwiftUI

@main
struct KreatynaApp: App {
    init() {
        RustLogging.start()
    }

    var body: some Scene {
        WindowGroup {
            ContentView()
        }
    }
}
