use proc_macro::TokenStream;
use quote::{format_ident, quote};
use syn::parse::{Parse, ParseStream};
use syn::{parse_macro_input, DeriveInput, Lit, Meta, NestedMeta};
use syn::{Ident, ItemEnum, Token, TypePath};

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

struct InstructionMacroInput {
    instructions_enum_name: Ident,
    instruction_types_enum_name: Ident,
    programs_enum_name: Ident,
    entries: Vec<InstructionEntry>,
}

struct InstructionEntry {
    program_variant: Ident,
    decoder_expr: syn::Expr,
    instruction_type: TypePath,
}

impl Parse for InstructionMacroInput {
    fn parse(input: ParseStream) -> syn::Result<Self> {
        let instructions_enum_name: Ident = input.parse()?;
        input.parse::<Token![,]>()?;
        let instruction_types_enum_name: Ident = input.parse()?;
        input.parse::<Token![,]>()?;
        let programs_enum_name: Ident = input.parse()?;
        input.parse::<Token![,]>()?;

        let mut entries = Vec::new();

        while !input.is_empty() {
            let program_variant: Ident = input.parse()?;
            input.parse::<Token![=>]>()?;
            let decoder_expr: syn::Expr = input.parse()?;
            input.parse::<Token![=>]>()?;
            let instruction_type: TypePath = input.parse()?;

            entries.push(InstructionEntry {
                program_variant,
                decoder_expr,
                instruction_type,
            });

            if input.peek(Token![,]) {
                input.parse::<Token![,]>()?;
            }
        }

        Ok(InstructionMacroInput {
            instructions_enum_name,
            instruction_types_enum_name,
            programs_enum_name,
            entries,
        })
    }
}

#[proc_macro]
pub fn instruction_decoder_collection(input: TokenStream) -> TokenStream {
    let input = parse_macro_input!(input as InstructionMacroInput);

    let instructions_enum_name = input.instructions_enum_name;
    let instruction_types_enum_name = input.instruction_types_enum_name;
    let programs_enum_name = input.programs_enum_name;
    let entries = input.entries;

    let mut instruction_variants = Vec::new();
    let mut instruction_type_variants = Vec::new();
    let mut program_variants = Vec::new();
    let mut parse_instruction_arms = Vec::new();
    let mut get_type_arms = Vec::new();

    for entry in entries {
        let program_variant = entry.program_variant;
        let decoder_expr = entry.decoder_expr;
        let instruction_type = entry.instruction_type;

        let instruction_enum_ident = &instruction_type.path.segments.last().unwrap().ident;
        let instruction_type_ident = format_ident!("{}Type", instruction_enum_ident);

        instruction_variants.push(quote! {
            #program_variant(#instruction_enum_ident)
        });
        instruction_type_variants.push(quote! {
            #program_variant(#instruction_type_ident)
        });
        program_variants.push(quote! {
            #program_variant
        });

        parse_instruction_arms.push(quote! {
            if let Some(decoded_instruction) = #decoder_expr.decode_instructions(instruction.clone()) {
                return Some(DecodedInstruction {
                    program_id: instruction.program_id,
                    data: #instructions_enum_name::#program_variant(decoded_instruction.data),
                });
            }
        });

        get_type_arms.push(quote! {
            #instructions_enum_name::#program_variant(instruction) => {
                #instruction_types_enum_name::#program_variant(instruction.get_instruction_type())
            }
        });
    }

    let expanded = quote! {
        #[derive(Debug, Clone, std::hash::Hash, serde::Serialize, PartialEq, Eq)]
        pub enum #instructions_enum_name {
            #(#instruction_variants),*
        }

        #[derive(Debug, Clone, PartialEq, Eq, serde::Serialize)]
        pub enum #instruction_types_enum_name {
            #(#instruction_type_variants),*
        }

        #[derive(Debug, Clone, PartialEq, Eq, serde::Serialize)]
        pub enum #programs_enum_name {
            #(#program_variants),*
        }

        impl InstructionDecoderCollection for #instructions_enum_name {
            type InstructionType = #instruction_types_enum_name;

            fn parse_instruction(
                instruction: solana_sdk::instruction::Instruction
            ) -> Option<DecodedInstruction<Self>> {
                #(#parse_instruction_arms)*
                None
            }

            fn get_type(&self) -> Self::InstructionType {
                match self {
                    #(#get_type_arms),*
                }
            }
        }
    };

    TokenStream::from(expanded)
}

#[proc_macro_derive(InstructionType)]
pub fn instruction_type_derive(input: TokenStream) -> TokenStream {
    let input = parse_macro_input!(input as ItemEnum);

    let enum_name = &input.ident;
    let instruction_type_name = format_ident!("{}Type", enum_name);

    let variants = input.variants.iter().map(|v| {
        let variant_ident = &v.ident;
        quote! {
            #variant_ident
        }
    });

    let instruction_type_enum = quote! {
        #[derive(Debug, Clone, PartialEq, Eq, serde::Serialize)]
        pub enum #instruction_type_name {
            #(#variants),*
        }
    };

    let get_instruction_type_arms = input.variants.iter().map(|v| {
        let variant_ident = &v.ident;
        if let syn::Fields::Unit = v.fields {
            quote! {
                Self::#variant_ident => #instruction_type_name::#variant_ident,
            }
        } else if let syn::Fields::Unnamed(_) = v.fields {
            quote! {
                Self::#variant_ident(..) => #instruction_type_name::#variant_ident,
            }
        } else if let syn::Fields::Named(_) = v.fields {
            quote! {
                Self::#variant_ident { .. } => #instruction_type_name::#variant_ident,
            }
        } else {
            quote! {}
        }
    });

    let impl_get_instruction_type = quote! {
        impl #enum_name {
            pub fn get_instruction_type(&self) -> #instruction_type_name {
                match self {
                    #(#get_instruction_type_arms)*
                }
            }
        }
    };

    let expanded = quote! {
        #instruction_type_enum

        #impl_get_instruction_type
    };

    TokenStream::from(expanded)
}
