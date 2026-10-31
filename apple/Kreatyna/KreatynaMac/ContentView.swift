import SwiftUI

struct ContentView: View {
    let model: AppModel

    var body: some View {
        MacRootView(
            view: model.view,
            errorMessage: model.errorMessage,
            send: model.dispatch,
            retryInitialization: model.retryInitialization
        )
    }
}
