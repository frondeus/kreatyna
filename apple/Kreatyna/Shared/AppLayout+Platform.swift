import Foundation

extension AppLayout {
    /// The layout of this platform's catalog (E-19). Rust owns what each layout does.
    static var platform: AppLayout {
        #if os(macOS)
        .split
        #else
        .stack
        #endif
    }
}
