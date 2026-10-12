import SwiftUI

@main
struct KreatynaMacApp: App {
    init() {
        RustLogging.start()
    }

    var body: some Scene {
        // One window: each window would start its own Rust core on the same
        // database. The app quits when this window closes.
        Window("Kreatyna", id: "main") {
            ContentView()
                .frame(minWidth: 800, minHeight: 500)
        }
        .defaultSize(width: 960, height: 600)
    }
}
