use syn::{
    Attribute, Generics, LitStr, Path, Token, Type,
    parse::{Parse, ParseStream},
    spanned::Spanned,
    visit::{self, Visit},
};

pub struct Options {
    pub root: Path,
}

impl Default for Options {
    fn default() -> Self {
        Self {
            root: syn::parse_quote!(::krucyfiks),
        }
    }
}

impl Parse for Options {
    fn parse(input: ParseStream<'_>) -> syn::Result<Self> {
        let mut options = Self::default();
        let mut seen = false;
        while !input.is_empty() {
            if !input.peek(Token![crate]) {
                return Err(input.error("expected `crate = \"path\"`"));
            }
            let key: Token![crate] = input.parse()?;
            if seen {
                return Err(syn::Error::new(
                    key.span,
                    "duplicate krucyfiks `crate` option",
                ));
            }
            seen = true;
            input.parse::<Token![=]>()?;
            let path: LitStr = input.parse()?;
            options.root = path.parse().map_err(|error| {
                syn::Error::new(
                    path.span(),
                    format!("invalid krucyfiks crate path: {error}"),
                )
            })?;
            if options
                .root
                .segments
                .iter()
                .any(|segment| !segment.arguments.is_empty())
            {
                return Err(syn::Error::new(
                    path.span(),
                    "crate path cannot have generic arguments",
                ));
            }
            if input.is_empty() {
                break;
            }
            input.parse::<Token![,]>()?;
        }
        Ok(options)
    }
}

pub fn no_generics(generics: &Generics) -> syn::Result<()> {
    if !generics.params.is_empty() || generics.where_clause.is_some() {
        return Err(syn::Error::new(
            generics.span(),
            "krucyfiks macros do not yet support generics or where clauses",
        ));
    }
    Ok(())
}

pub fn plain_attributes(attributes: &[Attribute]) -> syn::Result<()> {
    for attribute in attributes {
        if !["doc", "allow", "warn", "deny", "forbid", "deprecated"]
            .iter()
            .any(|name| attribute.path().is_ident(name))
        {
            return Err(syn::Error::new_spanned(
                attribute,
                "unsupported krucyfiks attribute; only documentation and lint attributes are supported here (async-trait is applied automatically)",
            ));
        }
    }
    Ok(())
}

pub fn owned_type(ty: &Type) -> syn::Result<()> {
    struct Validator {
        error: Option<syn::Error>,
    }
    impl<'ast> Visit<'ast> for Validator {
        fn visit_type(&mut self, ty: &'ast Type) {
            if self.error.is_some() {
                return;
            }
            if matches!(
                ty,
                Type::Reference(_)
                    | Type::ImplTrait(_)
                    | Type::Infer(_)
                    | Type::Macro(_)
                    | Type::Never(_)
            ) {
                self.error = Some(syn::Error::new_spanned(
                    ty,
                    "capability messages require concrete owned types; references, impl Trait, inferred types, type macros, and never types are unsupported",
                ));
            } else {
                visit::visit_type(self, ty);
            }
        }
        fn visit_lifetime(&mut self, lifetime: &'ast syn::Lifetime) {
            if lifetime.ident != "static" && self.error.is_none() {
                self.error = Some(syn::Error::new_spanned(
                    lifetime,
                    "capability message lifetimes must be 'static",
                ));
            }
        }
        fn visit_path_segment(&mut self, segment: &'ast syn::PathSegment) {
            if segment.ident == "Self" && self.error.is_none() {
                self.error = Some(syn::Error::new_spanned(
                    segment,
                    "capability messages cannot refer to Self; use a concrete owned type",
                ));
            } else {
                visit::visit_path_segment(self, segment);
            }
        }
    }
    let mut validator = Validator { error: None };
    validator.visit_type(ty);
    validator.error.map_or(Ok(()), Err)
}

pub fn is_unit(ty: &Type) -> bool {
    match ty {
        Type::Tuple(tuple) => tuple.elems.is_empty(),
        Type::Paren(paren) => is_unit(&paren.elem),
        Type::Group(group) => is_unit(&group.elem),
        _ => false,
    }
}
