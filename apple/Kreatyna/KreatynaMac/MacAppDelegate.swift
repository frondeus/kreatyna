import AppKit

/// Owns the app model, so quitting can wait until Rust has finished accepted
/// work, such as a save (P05).
@MainActor
final class MacAppDelegate: NSObject, NSApplicationDelegate {
    let model: AppModel

    override convenience init() {
        self.init(model: AppModel())
    }

    init(model: AppModel) {
        self.model = model
        super.init()
    }

    func applicationShouldTerminate(_ sender: NSApplication) -> NSApplication.TerminateReply {
        quit { sender.reply(toApplicationShouldTerminate: $0) }
        return .terminateLater
    }

    /// Replies after Rust stops. Rust cancels work that outlasts its grace
    /// period, so the app always quits.
    func quit(reply: @escaping @MainActor (Bool) -> Void) {
        Task {
            await model.shutDownGracefully()
            reply(true)
        }
    }
}
