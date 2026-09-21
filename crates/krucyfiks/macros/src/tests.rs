use quote::quote;

use crate::{capability, composition, validation::Options};

#[test]
fn green_path_expansion_matches_reviewed_rust() {
    use std::{
        fs,
        io::Write,
        path::Path,
        process::{Command, Stdio},
    };

    // This same input is compiled normally by the consumer crate. Its expanded
    // snapshot is also compiled there and runs the very same behavioral tests.
    let input = include_str!("../../macro-tests/fixtures/green_path.rs");
    let file = syn::parse_file(input).unwrap();
    let mut expanded = proc_macro2::TokenStream::new();
    for item in file.items {
        match item {
            syn::Item::Trait(mut item) => {
                let index = item
                    .attrs
                    .iter()
                    .position(|attribute| {
                        attribute
                            .path()
                            .segments
                            .last()
                            .is_some_and(|segment| segment.ident == "capability")
                    })
                    .expect("fixture trait must use the capability macro");
                let attribute = item.attrs.remove(index);
                expanded.extend(capability::expand(item, attribute.parse_args().unwrap()).unwrap());
            }
            syn::Item::Enum(mut item) => {
                let implementation =
                    composition::expand(syn::parse2(quote!(#item)).unwrap()).unwrap();
                // The fixture's only derive is Effect. Keep the declaration
                // but remove attributes consumed by that derive.
                item.attrs.retain(|attribute| {
                    !attribute.path().is_ident("derive") && !attribute.path().is_ident("krucyfiks")
                });
                expanded.extend(quote!(#item #implementation));
            }
            other => expanded.extend(quote!(#other)),
        }
    }
    let path = Path::new(env!("CARGO_MANIFEST_DIR")).join("tests/expanded/green_path.rs");

    fn format_rust(source: &str) -> String {
        let mut formatter = Command::new("rustfmt")
            .args(["--edition", "2024", "--emit", "stdout"])
            .stdin(Stdio::piped())
            .stdout(Stdio::piped())
            .spawn()
            .expect("rustfmt is needed to check expansion snapshots");
        formatter
            .stdin
            .take()
            .unwrap()
            .write_all(source.as_bytes())
            .unwrap();
        let output = formatter.wait_with_output().unwrap();
        assert!(output.status.success(), "rustfmt failed");
        String::from_utf8(output.stdout).unwrap()
    }

    let actual = format_rust(&expanded.to_string());
    if std::env::var("UPDATE_EXPANSIONS").as_deref() == Ok("1") {
        fs::create_dir_all(path.parent().unwrap()).unwrap();
        fs::write(&path, &actual).unwrap();
    }

    // Format both sides with the same toolchain. Whitespace changes between
    // rustfmt versions must not look like changes to the generated API.
    let expected = format_rust(&fs::read_to_string(&path).expect("missing expansion snapshot"));
    assert_eq!(
        actual, expected,
        "macro expansion changed; regenerate with UPDATE_EXPANSIONS=1 and review the Rust diff"
    );
}

#[test]
fn rejects_unsupported_capability_contracts() {
    let cases = [
        (
            quote!(
                trait Bad<T> {
                    async fn get(&self);
                }
            ),
            "generics",
        ),
        (
            quote!(
                trait Bad {
                    type Error;
                }
            ),
            "associated items",
        ),
        (
            quote!(
                trait Bad {
                    const SIZE: usize;
                }
            ),
            "associated items",
        ),
        (
            quote!(
                trait Bad {
                    fn get(&self);
                }
            ),
            "safe async",
        ),
        (
            quote!(
                trait Bad {
                    async fn get(&mut self);
                }
            ),
            "&self",
        ),
        (
            quote!(
                trait Bad {
                    async fn get(self);
                }
            ),
            "&self",
        ),
        (
            quote!(
                trait Bad {
                    async fn get();
                }
            ),
            "&self",
        ),
        (
            quote!(
                trait Bad {
                    async fn get<T>(&self);
                }
            ),
            "generics",
        ),
        (
            quote!(
                trait Bad {
                    async fn get(&self)
                    where
                        Self: Sized;
                }
            ),
            "where clauses",
        ),
        (
            quote!(
                trait Bad {
                    async fn get(&self) {}
                }
            ),
            "default bodies",
        ),
        (
            quote!(
                trait Bad {
                    async fn get(&self, value: Vec<&str>);
                }
            ),
            "owned types",
        ),
        (
            quote!(
                trait Bad {
                    async fn get(&self) -> &str;
                }
            ),
            "owned types",
        ),
        (
            quote!(
                trait Bad {
                    async fn get(&self) -> Self;
                }
            ),
            "refer to Self",
        ),
        (
            quote!(
                trait Bad {
                    async fn get(&self, value: impl Send);
                }
            ),
            "owned types",
        ),
        (
            quote!(
                trait Bad {
                    async fn get(&self, (x, y): (u8, u8));
                }
            ),
            "identifier patterns",
        ),
        (
            quote!(
                trait Bad {
                    #[cfg(unix)]
                    async fn get(&self);
                }
            ),
            "unsupported krucyfiks attribute",
        ),
        (
            quote!(
                trait Bad {
                    #[krucyfiks(panic)]
                    async fn get(&self);
                }
            ),
            "panic_on_transport",
        ),
        (
            quote!(
                trait Bad {
                    #[krucyfiks(panic_on_transport, panic_on_transport)]
                    async fn get(&self);
                }
            ),
            "duplicate",
        ),
        (
            quote!(
                trait Bad {
                    async fn get_value(&self);
                    async fn get__value(&self);
                }
            ),
            "duplicate request",
        ),
        (
            quote!(
                trait Bad {
                    async fn r#mut(&self);
                }
            ),
            "resolve_mut",
        ),
        (
            quote!(
                trait Bad: Clone {
                    async fn get(&self);
                }
            ),
            "supertraits",
        ),
        (
            quote!(
                trait Bad {}
            ),
            "at least one",
        ),
    ];
    for (input, expected) in cases {
        let input = syn::parse2(input).unwrap();
        let error = capability::expand(input, Options::default()).unwrap_err();
        assert!(
            error.to_string().contains(expected),
            "expected {expected:?}, got {error}"
        );
    }
}

#[test]
fn rejects_unsupported_compositions() {
    let cases = [
        (
            quote!(
                struct Bad;
            ),
            "requires an enum",
        ),
        (
            quote!(
                enum Bad<T> {
                    Value(T),
                }
            ),
            "generics",
        ),
        (
            quote!(
                enum Bad {
                    Unit,
                }
            ),
            "one unnamed",
        ),
        (
            quote!(
                enum Bad {
                    Named { value: () },
                }
            ),
            "one unnamed",
        ),
        (
            quote!(
                enum Bad {
                    Pair((), ()),
                }
            ),
            "one unnamed",
        ),
        (
            quote!(
                enum Bad {
                    #[cfg(unix)]
                    Value(()),
                }
            ),
            "unsupported krucyfiks attribute",
        ),
        (
            quote!(
                enum Bad {
                    HTTP(()),
                    Http(()),
                }
            ),
            "duplicate navigation",
        ),
    ];
    for (input, expected) in cases {
        let error = composition::expand(syn::parse2(input).unwrap()).unwrap_err();
        assert!(
            error.to_string().contains(expected),
            "expected {expected:?}, got {error}"
        );
    }
}

#[test]
fn unit_returns_produce_unit_response_variants_and_aliases_stay_intact() {
    let tokens = capability::expand(
        syn::parse_quote! {
            trait Example {
                #[krucyfiks(panic_on_transport)]
                async fn implicit(&self);
                #[krucyfiks(panic_on_transport)]
                async fn explicit(&self) -> ();
                async fn aliased(&self) -> Reply;
            }
        },
        Options::default(),
    )
    .unwrap();
    let file = syn::parse2::<syn::File>(tokens).unwrap();
    let response = file
        .items
        .iter()
        .find_map(|item| match item {
            syn::Item::Enum(item) if item.ident == "ExampleResponse" => Some(item),
            _ => None,
        })
        .unwrap();
    assert!(matches!(response.variants[0].fields, syn::Fields::Unit));
    assert!(matches!(response.variants[1].fields, syn::Fields::Unit));
    let field = response.variants[2].fields.iter().next().unwrap();
    let ty = &field.ty;
    assert_eq!(quote!(#ty).to_string(), "Reply");
}

#[test]
fn rejects_invalid_crate_options() {
    for options in [
        quote!(unknown),
        quote!(crate = "::framework", crate = "::other"),
        quote!(crate = 5),
        quote!(crate = "not a path"),
        quote!(crate = "Vec<u8>"),
    ] {
        assert!(syn::parse2::<Options>(options).is_err());
    }
}
