use proc_macro::TokenStream;
use syn::{DeriveInput, ItemTrait, parse_macro_input};

mod capability;
mod composition;
mod validation;

/// Generate a channel client, request/response enums, mutable backend trait,
/// request dispatcher, and envelope/recursive resolvers from an async trait.
///
/// Supports concrete owned arguments and return types, `&self` receivers, and
/// optional `Send + Sync` supertraits. The macro supplies async-trait and the
/// shared-receiver bounds. Generics, associated items, borrowed payloads, default
/// bodies, conditional methods, and other supertraits are rejected.
///
/// Transport errors propagate through `?`, without inspecting `Result` syntax.
/// Use `#[krucyfiks(panic_on_transport)]` on a method to explicitly panic instead.
/// Implicit unit and explicit `-> ()` returns produce unit response variants.
/// Generated Debug implementations name variants without requiring Debug payloads.
///
/// Renamed dependencies can use `#[capability(crate = "::my_framework")]`.
#[proc_macro_attribute]
pub fn capability(options: TokenStream, item: TokenStream) -> TokenStream {
    let options = parse_macro_input!(options as validation::Options);
    let input = parse_macro_input!(item as ItemTrait);
    capability::expand(input, options)
        .unwrap_or_else(syn::Error::into_compile_error)
        .into()
}

/// Generate consuming leaf extraction and exact branch navigation for a
/// non-generic enum whose variants each have one unnamed field.
///
/// Generates `try_into_*` and `expect_*`, without Debug/Send/Sync bounds.
/// Configure renamed dependencies with `#[krucyfiks(crate = "::my_framework")]`.
#[proc_macro_derive(Effect, attributes(krucyfiks))]
pub fn effect(item: TokenStream) -> TokenStream {
    let input = parse_macro_input!(item as DeriveInput);
    composition::expand(input)
        .unwrap_or_else(syn::Error::into_compile_error)
        .into()
}

#[cfg(test)]
mod tests;
