import Foundation

extension ReferenceUnit {
    /// The unit's measurements as shown in both apps, for example "5 g" or "14 g · 15 ml".
    var measurementText: String {
        [grams.map { "\($0) g" }, millilitres.map { "\($0) ml" }]
            .compactMap { $0 }
            .joined(separator: " · ")
    }
}
