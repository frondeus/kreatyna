import SwiftUI

/// The sections column. Swift keeps no selection: the highlight is Rust's
/// `section`, and a click only reports the new choice through `select`.
struct MacSidebar: View {
    let section: AppSection
    let select: (AppSection) -> Void

    var body: some View {
        List(selection: Binding(get: { section }, set: { if let new = $0 { select(new) } })) {
            ForEach(AppSection.sidebarOrder, id: \.self) { item in
                Label(item.title, systemImage: item.systemImage)
                    .tag(item)
                    .accessibilityIdentifier(item.accessibilityID)
            }
        }
    }
}
