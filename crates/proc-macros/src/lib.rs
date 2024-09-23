use proc_macro::TokenStream;
use quote::quote;
use syn::{parse_macro_input, DeriveInput, Lit, Meta, NestedMeta};

#[proc_macro_derive(CarbonDeserialize, attributes(carbon))]
pub fn carbon_deserialize_derive(input: TokenStream) -> TokenStream {
    let input = parse_macro_input!(input as DeriveInput);
    let name = &input.ident;

    let discriminator =
        get_discriminator(&input.attrs).expect("Missing #[carbon(discriminator = \"...\")]");

    let expanded = quote! {
        #[automatically_derived]
        impl carbon_core::borsh::BorshDeserialize for #name {
            fn deserialize_reader<R: std::io::Read>(reader: &mut R) -> ::core::result::Result<Self, carbon_core::borsh::io::Error> {
                carbon_core::borsh::BorshDeserialize::deserialize_reader(reader)
            }
        }

        #[automatically_derived]
        impl CarbonDeserialize for #name {
            fn deserialize(data: &[u8]) -> Option<Self> {
                let discriminator: &[u8] = #discriminator;
                if data.len() < discriminator.len() {
                    return None;
                }

                let (disc, rest) = data.split_at(discriminator.len());
                if disc != discriminator {
                    return None;
                }

                carbon_core::borsh::BorshDeserialize::try_from_slice(rest).ok()
            }
        }
    };

    TokenStream::from(expanded)
}

fn get_discriminator(attrs: &[syn::Attribute]) -> Option<quote::__private::TokenStream> {
    attrs.iter().find_map(|attr| {
        if attr.path.is_ident("carbon") {
            attr.parse_meta().ok().and_then(|meta| {
                if let Meta::List(list) = meta {
                    list.nested.iter().find_map(|nested| {
                        if let NestedMeta::Meta(Meta::NameValue(nv)) = nested {
                            if nv.path.is_ident("discriminator") {
                                if let Lit::Str(lit_str) = &nv.lit {
                                    let disc_str = lit_str.value();
                                    let disc_bytes = hex::decode(disc_str.trim_start_matches("0x"))
                                        .expect("Invalid hex string");
                                    let disc_array = disc_bytes.as_slice();
                                    return Some(quote! { &[#(#disc_array),*] });
                                }
                            }
                        }
                        None
                    })
                } else {
                    None
                }
            })
        } else {
            None
        }
    })
}
