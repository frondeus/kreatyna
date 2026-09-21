use std::collections::HashSet;

use heck::ToUpperCamelCase;
use proc_macro2::{Span, TokenStream};
use quote::{format_ident, quote, quote_spanned};
use syn::{FnArg, Ident, ItemTrait, Pat, ReturnType, TraitItem, TraitItemFn, Type, ext::IdentExt};

use crate::validation::{Options, is_unit, no_generics, owned_type, plain_attributes};

struct Method {
    method: TraitItemFn,
    variant: Ident,
    fields: Vec<Ident>,
    types: Vec<Type>,
    output: Type,
    panic_on_transport: bool,
}

fn parse(input: &mut ItemTrait) -> syn::Result<Vec<Method>> {
    no_generics(&input.generics)?;
    plain_attributes(&input.attrs)?;
    if input.unsafety.is_some() || input.auto_token.is_some() || input.restriction.is_some() {
        return Err(syn::Error::new_spanned(
            &input.ident,
            "capability requires an ordinary safe trait",
        ));
    }
    for bound in &input.supertraits {
        let valid = matches!(bound, syn::TypeParamBound::Trait(bound)
            if matches!(bound.modifier, syn::TraitBoundModifier::None) && bound.lifetimes.is_none()
            && bound.path.segments.iter().all(|segment| segment.arguments.is_empty())
            && (bound.path.is_ident("Send") || bound.path.is_ident("Sync")
                || (bound.path.segments.len() == 3
                    && (bound.path.segments[0].ident == "core" || bound.path.segments[0].ident == "std")
                    && bound.path.segments[1].ident == "marker"
                    && (bound.path.segments[2].ident == "Send" || bound.path.segments[2].ident == "Sync"))));
        if !valid {
            return Err(syn::Error::new_spanned(
                bound,
                "capability only supports Send and Sync supertraits",
            ));
        }
    }
    let mut methods = Vec::new();
    let mut variants = HashSet::new();
    for item in &mut input.items {
        let TraitItem::Fn(method) = item else {
            return Err(syn::Error::new_spanned(
                item,
                "capability only supports async methods, not associated items",
            ));
        };
        let mut panic_on_transport = false;
        let mut attributes = Vec::new();
        for attribute in &method.attrs {
            if attribute.path().is_ident("krucyfiks") {
                let mut present = false;
                attribute.parse_nested_meta(|meta| {
                    if !meta.path.is_ident("panic_on_transport") {
                        return Err(meta.error("expected `panic_on_transport`"));
                    }
                    if panic_on_transport {
                        return Err(meta.error("duplicate panic_on_transport policy"));
                    }
                    panic_on_transport = true;
                    present = true;
                    Ok(())
                })?;
                if !present {
                    return Err(syn::Error::new_spanned(
                        attribute,
                        "expected `panic_on_transport`",
                    ));
                }
            } else {
                attributes.push(attribute.clone());
            }
        }
        plain_attributes(&attributes)?;
        method.attrs = attributes;
        let signature = &method.sig;
        no_generics(&signature.generics)?;
        if signature.asyncness.is_none()
            || signature.constness.is_some()
            || signature.unsafety.is_some()
            || signature.abi.is_some()
            || signature.variadic.is_some()
        {
            return Err(syn::Error::new_spanned(
                signature,
                "capability requires safe async methods with Rust ABI",
            ));
        }
        if method.default.is_some() {
            return Err(syn::Error::new_spanned(
                method,
                "capability methods cannot have default bodies",
            ));
        }
        let valid_receiver = matches!(signature.inputs.first(), Some(FnArg::Receiver(receiver))
            if receiver.reference.as_ref().is_some_and(|(_, lifetime)| lifetime.is_none())
            && receiver.mutability.is_none() && receiver.colon_token.is_none());
        if !valid_receiver {
            return Err(syn::Error::new_spanned(
                signature,
                "capability methods require an &self receiver",
            ));
        }
        let name = signature.ident.unraw().to_string();
        if name == "mut" {
            return Err(syn::Error::new_spanned(
                &signature.ident,
                "method name conflicts with the generated resolve_mut helper",
            ));
        }
        let variant_name = name.to_upper_camel_case();
        let variant = syn::parse_str::<Ident>(&variant_name).map_err(|_| {
            syn::Error::new_spanned(
                &signature.ident,
                "method name cannot form a response variant",
            )
        })?;
        if !variants.insert(variant_name) {
            return Err(syn::Error::new_spanned(
                &signature.ident,
                "method names produce duplicate request/response variants",
            ));
        }
        let mut fields = Vec::new();
        let mut types = Vec::new();
        for argument in signature.inputs.iter().skip(1) {
            let FnArg::Typed(argument) = argument else {
                return Err(syn::Error::new_spanned(
                    argument,
                    "expected a named owned argument",
                ));
            };
            plain_attributes(&argument.attrs)?;
            let Pat::Ident(pattern) = &*argument.pat else {
                return Err(syn::Error::new_spanned(
                    &argument.pat,
                    "capability arguments require simple identifier patterns",
                ));
            };
            if pattern.by_ref.is_some() || pattern.mutability.is_some() || pattern.subpat.is_some()
            {
                return Err(syn::Error::new_spanned(
                    pattern,
                    "capability arguments require simple identifier patterns",
                ));
            }
            owned_type(&argument.ty)?;
            fields.push(pattern.ident.clone());
            types.push((*argument.ty).clone());
        }
        let output = match &signature.output {
            ReturnType::Default => syn::parse_quote!(()),
            ReturnType::Type(_, ty) => (**ty).clone(),
        };
        owned_type(&output)?;
        methods.push(Method {
            method: method.clone(),
            variant,
            fields,
            types,
            output,
            panic_on_transport,
        });
    }
    if methods.is_empty() {
        return Err(syn::Error::new_spanned(
            &input.ident,
            "capability requires at least one async method",
        ));
    }
    Ok(methods)
}

pub fn expand(mut input: ItemTrait, options: Options) -> syn::Result<TokenStream> {
    let methods = parse(&mut input)?;
    let root = &options.root;
    let name = &input.ident;
    let visibility = &input.vis;
    let mutable = format_ident!("{}Mut", name.unraw());
    let resolver = format_ident!("Resolve{}", name.unraw());
    let request = format_ident!("{}Request", name.unraw());
    let response = format_ident!("{}Response", name.unraw());
    let client = format_ident!("{}Client", name.unraw());
    // Shared receivers must be callable from Send commands. Mutable backends
    // and consumed resolver values only need Send.
    input.supertraits = syn::parse_quote!(::core::marker::Send + ::core::marker::Sync);

    let mut requests = Vec::new();
    let mut responses = Vec::new();
    let mut mutable_methods = Vec::new();
    let mut client_methods = Vec::new();
    let mut resolver_methods = Vec::new();
    let mut envelope_methods = Vec::new();
    let mut recursive_methods = Vec::new();
    let mut dispatch = Vec::new();
    let mut debug_requests = Vec::new();
    let mut debug_responses = Vec::new();

    for model in methods {
        let Method {
            method,
            variant,
            fields,
            types,
            output,
            panic_on_transport,
        } = model;
        let ident = &method.sig.ident;
        let resolve = format_ident!("resolve_{}", ident.unraw());
        let unit = is_unit(&output);
        let locals: Vec<_> = (0..fields.len())
            .map(|index| format_ident!("__krucyfiks_arg_{index}", span = Span::mixed_site()))
            .collect();
        let request_pattern = if fields.is_empty() {
            requests.push(quote! { #variant });
            quote! { #request::#variant }
        } else {
            requests.push(quote! { #variant { #(#fields: #types),* } });
            quote! { #request::#variant { #(#fields: #locals),* } }
        };
        let response_pattern = if unit {
            responses.push(quote! { #variant });
            quote! { #response::#variant }
        } else {
            responses.push(quote! { #variant(#output) });
            quote! { #response::#variant(__krucyfiks_result) }
        };
        let response_value = if unit {
            quote!(())
        } else {
            quote!(__krucyfiks_result)
        };
        let request_label = format!("{request}::{variant}");
        let response_label = format!("{response}::{variant}");
        let request_debug_pattern = if fields.is_empty() {
            quote!(Self::#variant)
        } else {
            quote!(Self::#variant { .. })
        };
        let response_debug_pattern = if unit {
            quote!(Self::#variant)
        } else {
            quote!(Self::#variant(..))
        };
        debug_requests.push(quote!(#request_debug_pattern => formatter.write_str(#request_label),));
        debug_responses
            .push(quote!(#response_debug_pattern => formatter.write_str(#response_label),));

        let mut mutable_method = method.clone();
        if let Some(FnArg::Receiver(receiver)) = mutable_method.sig.inputs.first_mut() {
            receiver.mutability = Some(Default::default());
            *receiver.ty = syn::parse_quote!(&mut Self);
        }
        mutable_methods.push(mutable_method);
        let mut client_signature = method.sig.clone();
        for (argument, local) in client_signature.inputs.iter_mut().skip(1).zip(&locals) {
            if let FnArg::Typed(argument) = argument {
                *argument.pat = syn::parse_quote!(#local);
            }
        }
        let label = format!("{name}::{ident} transport failed: {{error}}");
        let finish = |expression: TokenStream| {
            if panic_on_transport {
                quote_spanned!(ident.span()=> #expression.unwrap_or_else(|error| ::core::panic!(#label)))
            } else {
                // Leave Result aliases and From checks to Rust. In particular,
                // never inspect or split the declared return type.
                quote_spanned!(ident.span()=> #expression?)
            }
        };
        let send = finish(quote! {
            self.tx.send_async(__krucyfiks_envelope).await
                .map_err(|_| #root::effects::TransportError::RequestChannelClosed)
        });
        let receive = finish(quote! {
            __krucyfiks_receiver.recv_async().await
                .map_err(|_| #root::effects::TransportError::ResponseChannelClosed)
        });
        let mismatch = if panic_on_transport {
            quote! {
                { let error = #root::effects::TransportError::UnexpectedResponse; ::core::panic!(#label) }
            }
        } else {
            quote_spanned!(ident.span()=> ::core::result::Result::Err(#root::effects::TransportError::UnexpectedResponse)?)
        };
        client_methods.push(quote! {
            #client_signature {
                let (__krucyfiks_envelope, __krucyfiks_receiver) =
                    #root::effects::Envelope::new(#request_pattern, self.envelope_size);
                #send;
                match #receive {
                    #response_pattern => #response_value,
                    #[allow(unreachable_patterns)]
                    _ => #mismatch,
                }
            }
        });
        let callback_signature = quote! {
            async fn #resolve<__KrucyfiksHandle, __KrucyfiksFuture>(self, handle: __KrucyfiksHandle)
                -> ::core::result::Result<(), Self::ResolveError>
            where
                __KrucyfiksHandle: ::core::ops::FnOnce(#(#types),*) -> __KrucyfiksFuture + ::core::marker::Send,
                __KrucyfiksFuture: ::core::future::Future<Output = #output> + ::core::marker::Send
        };
        resolver_methods.push(quote! { #callback_signature; });
        let callback = if unit {
            quote! { handle(#(#locals),*).await; }
        } else {
            quote! { let __krucyfiks_result = handle(#(#locals),*).await; }
        };
        envelope_methods.push(quote! {
            #callback_signature {
                let (__krucyfiks_request, __krucyfiks_responder) = self.into_parts();
                match __krucyfiks_request {
                    #request_pattern => {
                        #callback
                        __krucyfiks_responder.answer_async(#response_pattern).await
                    }
                    #[allow(unreachable_patterns)]
                    _ => ::core::result::Result::Err(#root::effects::EnvelopeError::Mismatch),
                }
            }
        });
        recursive_methods.push(quote! {
            #callback_signature {
                let envelope = #root::effects::Effect::extract::<#root::effects::Envelope<#request, #response>>(self.into_inner())
                    .map_err(|_| #root::effects::EnvelopeError::Mismatch)?;
                #resolver::#resolve(envelope, handle).await
            }
        });
        let operation = if unit {
            quote! { #mutable::#ident(backend, #(#locals),*).await; }
        } else {
            quote! { let __krucyfiks_result = #mutable::#ident(backend, #(#locals),*).await; }
        };
        dispatch.push(quote! { #request_pattern => { #operation #response_pattern } });
    }

    Ok(quote! {
        #[#root::__private::async_trait]
        #input

        #[#root::__private::async_trait]
        #visibility trait #mutable: ::core::marker::Send { #(#mutable_methods)* }

        #visibility enum #request { #(#requests),* }
        #visibility enum #response { #(#responses),* }

        impl ::core::fmt::Debug for #request {
            fn fmt(&self, formatter: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
                match self { #(#debug_requests)* }
            }
        }
        impl ::core::fmt::Debug for #response {
            fn fmt(&self, formatter: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
                match self { #(#debug_responses)* }
            }
        }

        impl #request {
            #visibility async fn dispatch_mut<__KrucyfiksBackend>(self, backend: &mut __KrucyfiksBackend) -> #response
            where __KrucyfiksBackend: #mutable + ?::core::marker::Sized,
            {
                match self { #(#dispatch),* }
            }
        }

        #visibility struct #client {
            tx: #root::effects::RequestOutbox<#request, #response>,
            envelope_size: usize,
        }
        impl #client {
            /// Creates a client with explicit request queue and per-request response capacities.
            #visibility fn new(request_capacity: usize, envelope_size: usize) -> (Self, #root::effects::RequestInbox<#request, #response>) {
                let (tx, rx) = #root::effects::request_channel(request_capacity);
                (Self { tx, envelope_size }, rx)
            }
        }
        #[#root::__private::async_trait]
        impl #name for #client { #(#client_methods)* }

        #[#root::__private::async_trait]
        #visibility trait #resolver: ::core::marker::Send {
            type ResolveError;
            async fn resolve_mut<__KrucyfiksBackend>(self, backend: &mut __KrucyfiksBackend)
                -> ::core::result::Result<(), Self::ResolveError>
            where __KrucyfiksBackend: #mutable + ?::core::marker::Sized;
            #(#resolver_methods)*
        }

        #[#root::__private::async_trait]
        impl #resolver for #root::effects::Envelope<#request, #response> {
            type ResolveError = #root::effects::EnvelopeError<#response>;
            async fn resolve_mut<__KrucyfiksBackend>(self, backend: &mut __KrucyfiksBackend)
                -> ::core::result::Result<(), Self::ResolveError>
            where __KrucyfiksBackend: #mutable + ?::core::marker::Sized,
            {
                let (request, responder) = self.into_parts();
                responder.answer_async(request.dispatch_mut(backend).await).await
            }
            #(#envelope_methods)*
        }

        #[#root::__private::async_trait]
        impl<__KrucyfiksTree> #resolver for #root::effects::AnyRoute<__KrucyfiksTree>
        where __KrucyfiksTree: #root::effects::Effect + ::core::marker::Send,
        {
            type ResolveError = #root::effects::EnvelopeError<#response>;
            async fn resolve_mut<__KrucyfiksBackend>(self, backend: &mut __KrucyfiksBackend)
                -> ::core::result::Result<(), Self::ResolveError>
            where __KrucyfiksBackend: #mutable + ?::core::marker::Sized,
            {
                let envelope = #root::effects::Effect::extract::<#root::effects::Envelope<#request, #response>>(self.into_inner())
                    .map_err(|_| #root::effects::EnvelopeError::Mismatch)?;
                #resolver::resolve_mut(envelope, backend).await
            }
            #(#recursive_methods)*
        }
    })
}
