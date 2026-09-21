use std::collections::HashSet;

use heck::ToSnakeCase;
use proc_macro2::TokenStream;
use quote::{format_ident, quote};
use syn::{Data, DeriveInput, Fields, ext::IdentExt};

use crate::validation::{Options, no_generics, plain_attributes};

pub fn expand(input: DeriveInput) -> syn::Result<TokenStream> {
    no_generics(&input.generics)?;
    let mut options = None;
    for attribute in &input.attrs {
        if attribute.path().is_ident("krucyfiks") {
            if options.is_some() {
                return Err(syn::Error::new_spanned(
                    attribute,
                    "duplicate krucyfiks options",
                ));
            }
            options = Some(attribute.parse_args::<Options>()?);
        }
    }
    let root = options.unwrap_or_default().root;
    let Data::Enum(data) = &input.data else {
        return Err(syn::Error::new_spanned(
            &input.ident,
            "Effect requires an enum",
        ));
    };
    let name = &input.ident;
    let visibility = &input.vis;
    let mut seen = HashSet::new();
    let mut children = Vec::new();
    for variant in &data.variants {
        plain_attributes(&variant.attrs)?;
        let Fields::Unnamed(fields) = &variant.fields else {
            return Err(syn::Error::new_spanned(
                variant,
                "Effect variants must contain exactly one unnamed child field",
            ));
        };
        if fields.unnamed.len() != 1 {
            return Err(syn::Error::new_spanned(
                variant,
                "Effect variants must contain exactly one unnamed child field",
            ));
        }
        let field = &fields.unnamed[0];
        plain_attributes(&field.attrs)?;
        let suffix = variant.ident.unraw().to_string().to_snake_case();
        if !seen.insert(suffix.clone()) {
            return Err(syn::Error::new_spanned(
                &variant.ident,
                "variant names produce duplicate navigation methods",
            ));
        }
        children.push((&variant.ident, &field.ty, suffix));
    }
    let extraction = children.iter().map(|(variant, _, _)| quote! {
        Self::#variant(child) => #root::effects::Effect::extract::<T>(child).map_err(Self::#variant),
    });
    let helpers = children.iter().map(|(variant, ty, suffix)| {
        let try_into = format_ident!("try_into_{suffix}");
        let expect = format_ident!("expect_{suffix}");
        let failures = children
            .iter()
            .filter(|(other, _, _)| other != variant)
            .map(|(other, _, _)| {
                let message = format!("expected {name}::{variant}, got {name}::{other}");
                quote! { Self::#other(_) => ::core::panic!(#message), }
            });
        quote! {
            #visibility fn #try_into(self) -> ::core::result::Result<#ty, Self> {
                match self {
                    Self::#variant(child) => ::core::result::Result::Ok(child),
                    #[allow(unreachable_patterns)]
                    other => ::core::result::Result::Err(other),
                }
            }
            #[track_caller]
            #visibility fn #expect(self) -> #ty {
                match self {
                    Self::#variant(child) => child,
                    #(#failures)*
                }
            }
        }
    });
    Ok(quote! {
        impl #root::effects::Effect for #name {
            fn extract<T: 'static>(self) -> ::core::result::Result<T, Self> {
                match self { #(#extraction)* }
            }
        }
        impl #name { #(#helpers)* }
    })
}
