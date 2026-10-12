import SwiftUI

struct ContentView: View {
    @State private var model = AppModel()

    var body: some View {
        MacRootView(
            view: model.view,
            errorMessage: model.errorMessage,
            send: model.dispatch,
            retryInitialization: model.retryInitialization
        )
    }
}
