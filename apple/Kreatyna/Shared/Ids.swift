// Rust's ids. Each one crosses the FFI as its UUID text; `uniffi.toml` in
// `crates/kreatyna-ffi` maps it to these structs. Swift compares ids and
// passes them back, but never makes or reads one.

/// A recipe's id.
public struct RecipeId: Hashable, Sendable {
    public let uuid: String

    public init(uuid: String) {
        self.uuid = uuid
    }
}

/// An ingredient's id.
public struct IngredientId: Hashable, Sendable {
    public let uuid: String

    public init(uuid: String) {
        self.uuid = uuid
    }
}

/// A step's id.
public struct StepId: Hashable, Sendable {
    public let uuid: String

    public init(uuid: String) {
        self.uuid = uuid
    }
}
