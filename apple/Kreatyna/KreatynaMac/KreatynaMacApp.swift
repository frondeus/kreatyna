import SwiftUI

@main
struct KreatynaMacApp: App {
    // The delegate owns the model: it needs the model when the app quits.
    @NSApplicationDelegateAdaptor(MacAppDelegate.self) private var delegate

    init() {
        RustLogging.start()
    }

    var body: some Scene {
        // One window: each window would start its own Rust core on the same
        // database. The app quits when this window closes.
        Window("Kreatyna", id: "main") {
            ContentView(model: delegate.model)
                .frame(minWidth: 800, minHeight: 500)
        }
        .defaultSize(width: 960, height: 600)
        .commands { MacEditCommands() }
    }
}
