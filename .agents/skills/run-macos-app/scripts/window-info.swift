// Prints "<width> × <height>" for each on-screen window of one process.
// Usage: swift window-info.swift <pid>
//
// It reads only the owner, layer, and bounds of each window. These need no
// Screen Recording permission (MAC-02/Q1).
import CoreGraphics

guard CommandLine.arguments.count == 2, let pid = Int32(CommandLine.arguments[1]) else {
    print("Usage: swift window-info.swift <pid>")
    exit(2)
}
let windows = CGWindowListCopyWindowInfo([.optionOnScreenOnly, .excludeDesktopElements],
                                         kCGNullWindowID) as? [[String: Any]] ?? []
for window in windows {
    guard window[kCGWindowOwnerPID as String] as? Int32 == pid,
          window[kCGWindowLayer as String] as? Int == 0,
          let bounds = window[kCGWindowBounds as String] as? [String: Double],
          let width = bounds["Width"], let height = bounds["Height"] else { continue }
    print("\(Int(width)) × \(Int(height))")
}
