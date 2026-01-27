use {
    borsh_derive_internal_satellite::*,
    proc_macro::TokenStream,
    proc_macro2::{Span, TokenStream as TokenStream2},
    quote::{format_ident, quote},
    syn::{
        parse::{Parse, ParseStream},
        parse_macro_input, DeriveInput, Ident, Item, ItemEnum, Lit, Meta, NestedMeta, Token,
        TypePath,
    },
};

#[proc_macro_derive(CarbonDeserialize, attributes(carbon))]
pub fn carbon_deserialize_derive(input_token_stream: TokenStream) -> TokenStream {
    let derive_input = input_token_stream.clone();
    let input = parse_macro_input!(derive_input as DeriveInput);
    let name = &input.ident;

    let discriminator = get_discriminator(&input.attrs).unwrap_or(quote! { &[] });
    let deser = gen_borsh_deserialize(input_token_stream);

    let expanded = quote! {
        #deser

        #[automatically_derived]
        impl carbon_core::deserialize::CarbonDeserialize for #name {
            const DISCRIMINATOR: &'static [u8] = #discriminator;

            fn deserialize(data: &[u8]) -> Option<Self> {
                if data.len() < Self::DISCRIMINATOR.len() {
                    return None;
                }


                let (disc, mut rest) = data.split_at(Self::DISCRIMINATOR.len());
                if disc != Self::DISCRIMINATOR {
                    return None;
                }

                 match carbon_core::borsh::BorshDeserialize::deserialize(&mut rest) {
                    Ok(res) => {
                        if !rest.is_empty() {
                            carbon_core::log::debug!(
                                "Not all bytes were read when deserializing {}: {} bytes remaining",
                                stringify!(#name),
                                rest.len(),
                            );
                        }
                        Some(res)
                    }
                    Err(_) => None,
                }
            }
        }
    };

    TokenStream::from(expanded)
}

fn gen_borsh_deserialize(input: TokenStream) -> TokenStream2 {
    let cratename = Ident::new("borsh", Span::call_site());

    let item: Item = syn::parse(input).expect("Failed to parse input");
    let res = match item {
        Item::Struct(item) => struct_de(&item, cratename),
        Item::Enum(item) => enum_de(&item, cratename),
        Item::Union(item) => union_de(&item, cratename),
        // Derive macros can only be defined on structs, enums, and unions.
        _ => unreachable!(),
    };

    match res {
        Ok(res) => res,
        Err(err) => err.to_compile_error(),
    }
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
    program_id_path: Option<syn::Path>,
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

            // Attempt to parse 4-part syntax: variant => PROGRAM_ID_PATH => DECODER => INSTRUCTION
            // Use a forked parser to decide without consuming on failure.
            let mut use_four_part = false;
            let fork = input.fork();
            let program_id_path_candidate: syn::Path = match fork.parse() {
                Ok(p) => p,
                Err(_) => {
                    // Cannot parse path; must be legacy 3-part
                    syn::Path {
                        leading_colon: None,
                        segments: syn::punctuated::Punctuated::new(),
                    }
                }
            };

            if !program_id_path_candidate.segments.is_empty()
                && fork.parse::<Token![=>]>().is_ok()
                && fork.parse::<syn::Expr>().is_ok()
                && fork.parse::<Token![=>]>().is_ok()
                && fork.parse::<TypePath>().is_ok()
            {
                use_four_part = true;
            }

            if use_four_part {
                let program_id_path: syn::Path = input.parse()?;
                input.parse::<Token![=>]>()?;
                let decoder_expr: syn::Expr = input.parse()?;
                input.parse::<Token![=>]>()?;
                let instruction_type: TypePath = input.parse()?;

                entries.push(InstructionEntry {
                    program_variant,
                    program_id_path: Some(program_id_path),
                    decoder_expr,
                    instruction_type,
                });
            } else {
                // Legacy 3-part syntax: variant => DECODER => INSTRUCTION
                let decoder_expr: syn::Expr = input.parse()?;
                input.parse::<Token![=>]>()?;
                let instruction_type: TypePath = input.parse()?;

                entries.push(InstructionEntry {
                    program_variant,
                    program_id_path: None,
                    decoder_expr,
                    instruction_type,
                });
            }

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

#[deprecated(
    note = "Use `instruction_decoder_collection_fast!` for faster dispatch by program_id."
)]
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

        let instruction_enum_ident = &instruction_type
            .path
            .segments
            .last()
            .expect("segment")
            .ident;
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
            if let Some(decoded_instruction) = #decoder_expr.decode_instruction(&instruction) {
                return Some(carbon_core::instruction::DecodedInstruction {
                    program_id: instruction.program_id,
                    accounts: instruction.accounts.clone(),
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
        #[derive(Debug, Clone, std::hash::Hash, serde::Serialize, serde::Deserialize, PartialEq, Eq)]
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

        impl carbon_core::collection::InstructionDecoderCollection for #instructions_enum_name {
            type InstructionType = #instruction_types_enum_name;

            fn parse_instruction(
                instruction: &solana_instruction::Instruction
            ) -> Option<carbon_core::instruction::DecodedInstruction<Self>> {
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

#[proc_macro]
pub fn instruction_decoder_collection_fast(input: TokenStream) -> TokenStream {
    let input = parse_macro_input!(input as InstructionMacroInput);

    let instructions_enum_name = input.instructions_enum_name;
    let instruction_types_enum_name = input.instruction_types_enum_name;
    let programs_enum_name = input.programs_enum_name;
    let entries = input.entries;

    let mut instruction_variants = Vec::new();
    let mut instruction_type_variants = Vec::new();
    let mut program_variants = Vec::new();
    let mut parse_instruction_match_arms = Vec::new();
    let mut fallback_decode_blocks = Vec::new();
    let mut get_type_arms = Vec::new();

    for entry in entries {
        let program_variant = entry.program_variant;
        let decoder_expr = entry.decoder_expr;
        let instruction_type = entry.instruction_type;

        let instruction_enum_ident = &instruction_type
            .path
            .segments
            .last()
            .expect("segment")
            .ident;
        let instruction_type_ident = format_ident!("{}Type", instruction_enum_ident);

        // Resolve the program id path for dispatch. Prefer explicitly provided
        // path if available; otherwise, fall back to inferring `<crate>::PROGRAM_ID`
        // from the first segment of the instruction type path for backward
        // compatibility with older 3-part syntax.
        let explicit_program_id_path = entry.program_id_path;

        instruction_variants.push(quote! {
            #program_variant(#instruction_enum_ident)
        });
        instruction_type_variants.push(quote! {
            #program_variant(#instruction_type_ident)
        });
        program_variants.push(quote! {
            #program_variant
        });

        if let Some(program_id_path) = explicit_program_id_path {
            parse_instruction_match_arms.push(quote! {
                #program_id_path => {
                    if let Some(decoded_instruction) = #decoder_expr.decode_instruction(&instruction) {
                        Some(carbon_core::instruction::DecodedInstruction {
                            program_id: instruction.program_id,
                            accounts: instruction.accounts.clone(),
                            data: #instructions_enum_name::#program_variant(decoded_instruction.data),
                        })
                    } else {
                        None
                    }
                }
            });
        } else {
            // No program id path: include in slow-path fallback.
            fallback_decode_blocks.push(quote! {
                if let Some(decoded_instruction) = #decoder_expr.decode_instruction(&instruction) {
                    return Some(carbon_core::instruction::DecodedInstruction {
                        program_id: instruction.program_id,
                        accounts: instruction.accounts.clone(),
                        data: #instructions_enum_name::#program_variant(decoded_instruction.data),
                    });
                }
            });
        }

        get_type_arms.push(quote! {
            #instructions_enum_name::#program_variant(instruction) => {
                #instruction_types_enum_name::#program_variant(instruction.get_instruction_type())
            }
        });
    }

    let expanded = quote! {
        #[derive(Debug, Clone, std::hash::Hash, serde::Serialize, serde::Deserialize, PartialEq, Eq)]
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

        impl carbon_core::collection::InstructionDecoderCollection for #instructions_enum_name {
            type InstructionType = #instruction_types_enum_name;

            fn parse_instruction(
                instruction: &solana_instruction::Instruction
            ) -> Option<carbon_core::instruction::DecodedInstruction<Self>> {
                match instruction.program_id {
                    #(#parse_instruction_match_arms),*
                    _ => {
                        #(#fallback_decode_blocks)*
                        None
                    }
                }
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
