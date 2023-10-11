use proc_macro::TokenStream;
use quote::quote;
use syn::{parse_macro_input, Data, DeriveInput, Fields};

/// Generate syntax for updating the fspec from the deku style syntax that would be found
/// in a ASTERIX category
///
/// This is mostly a `hack` in the true sense of the word. Although it works pretty well for
/// the well defined deku derives.
///
/// Input:
/// ```rust, ignore
/// use asterix::data_item::*;
/// #[deku(skip, cond = "is_fspec(DataSourceIdentifier::FRN_48, fspec, 0)")]
/// pub data_source_identifier: Option<DataSourceIdentifier>,
/// ```
///
/// General Output:
/// ```rust, ignore
/// use asterix::data_item::*;
/// if self.data_source_identifier.is_some() {
///     fspec[0] |= DataSourceIdentifier::FRN_48;
/// }
///
/// ```
///
/// There are a few parts that are pre-pended and appended to the end of the above statements, with
/// generation for the vec and cleaning up the fspec.
#[proc_macro_derive(UpdateFspec)]
#[doc(hidden)]
pub fn derive_answer_fn(input: TokenStream) -> TokenStream {
    // Parse the input tokens into a syntax tree
    let input = parse_macro_input!(input as DeriveInput);

    let name = &input.ident; // struct name

    // (self.name, fspec_num, FRN)
    let mut data_items: Vec<(String, String, String)> = vec![];

    if let Data::Struct(s) = input.data {
        if let Fields::Named(f) = s.fields {
            for field in f.named.iter() {
                let ident = field.ident.as_ref().unwrap(); // they are 'named' fields
                                                           // check if is first 'fspec' field in struct, skip
                if ident == "fspec" {
                    continue;
                }
                for attr in &field.attrs {
                    // check if doc ident, we don't need that one
                    if !attr.path().is_ident("deku") {
                        continue;
                    }
                    // ident should be 'deku' at this point
                    // pulling out the `TokenStream` from `Meta::List` and parsing
                    attr.parse_nested_meta(|meta| {
                        if meta.path.is_ident("cond") {
                            let value = meta.value()?; // this parses the `=`
                            let token: syn::LitStr = value.parse()?; // this parses `"is_fspec(...)"`
                            let fn_call = token.parse::<syn::ExprCall>().unwrap();
                            let frn = if let syn::Expr::Path(attrs) = &fn_call.args[0] {
                                format!(
                                    "{}::{}",
                                    attrs.path.segments[0].ident, attrs.path.segments[1].ident,
                                )
                            } else {
                                unreachable!()
                            };

                            let fspec_num = if let syn::Expr::Lit(lit) = &fn_call.args[2] {
                                if let syn::Lit::Int(int) = &lit.lit {
                                    int.to_string()
                                } else {
                                    unreachable!();
                                }
                            } else {
                                unreachable!();
                            };
                            data_items.push((
                                ident.to_string(),
                                fspec_num.to_string(),
                                frn.to_string(),
                            ));
                        }
                        Ok(())
                    })
                    .expect("Error parsing nested meta");
                }
            }
        }
    }

    let mut quotes = quote! {};

    for data_item in data_items {
        let ident = syn::Ident::new(&data_item.0.to_string(), proc_macro2::Span::call_site());
        let fspec_num = data_item.1.parse::<usize>().unwrap();
        let frn = data_item.2;
        let frn = syn::parse_str::<syn::Expr>(&frn).unwrap();
        quotes = quote! {
            #quotes
            if self.#ident.is_some() {
                fspec[#fspec_num] |= #frn;
            }
        }
    }

    let expanded = quote! {
        impl #name {
            pub fn update_fspec(&mut self) {
                let mut fspec = vec![0x00; 10];
                #quotes
                trim_fspec(&mut fspec);
                add_fx(&mut fspec);
                self.fspec = fspec;
            }
        }
    };
    // Hand the output tokens back to the compiler
    TokenStream::from(expanded) // als  could be 'expanded.into()'
}
