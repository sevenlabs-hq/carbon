use std::collections::HashSet;

use borsh_derive_internal::*;
use proc_macro::TokenStream;
use proc_macro2::{Span, TokenStream as TokenStream2};
use quote::{format_ident, quote};
use syn::parse::{Parse, ParseStream};
use syn::spanned::Spanned;
use syn::{
    bracketed, parse_macro_input, DeriveInput, Expr, ExprLit, Item, Lit, Meta, NestedMeta, Result,
};
use syn::{Ident, ItemEnum, Token, TypePath};

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

fn gen_borsh_deserialize(input: TokenStream) -> TokenStream2 {
    let cratename = Ident::new("borsh", Span::call_site());

    let item: Item = syn::parse(input).unwrap();
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
            if let Some(decoded_instruction) = #decoder_expr.decode_instruction(instruction.clone()) {
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

// #[proc_macro]
// pub fn generate_output_struct(input: TokenStream) -> TokenStream {
//     let input = parse_macro_input!(input as GenerateOutputStructInput);

//     let struct_name = &input.struct_name;
//     let mut identifiers = Vec::new();
//     input.collect_identifiers(&mut identifiers);

//     identifiers = {
//         let mut seen = std::collections::HashSet::new();
//         identifiers
//             .into_iter()
//             .filter(|ident| seen.insert(ident.to_string()))
//             .collect()
//     };

//     let fields = identifiers.iter().map(|ident| {
//         quote! {
//             pub #ident: (AllInstructionTypes, DecodedInstruction),
//         }
//     });

//     // Generate the final struct code
//     let expanded = quote! {
//         #[derive(Debug, Clone, serde::Deserialize)]
//         pub struct #struct_name {
//             #(#fields)*
//         }
//     };

//     // Return the generated code as a TokenStream
//     TokenStream::from(expanded)
// }

// // Define the structures for parsing the macro input
// struct GenerateOutputStructInput {
//     struct_name: Ident,
//     instructions: Vec<Instruction>,
// }

// enum Instruction {
//     Any,
//     InstructionNode {
//         instruction_type: Expr,
//         name: Ident,
//         inner_instructions: Vec<Instruction>,
//     },
// }

// impl Parse for GenerateOutputStructInput {
//     fn parse(input: ParseStream) -> Result<Self> {
//         // Parse the struct name
//         let struct_name: Ident = input.parse()?;

//         // Parse the instructions
//         let instructions = parse_instructions(input)?;

//         Ok(GenerateOutputStructInput {
//             struct_name,
//             instructions,
//         })
//     }
// }

// // Recursive function to parse instructions
// fn parse_instructions(input: ParseStream) -> Result<Vec<Instruction>> {
//     let mut instructions = Vec::new();

//     while !input.is_empty() {
//         // Skip optional commas
//         while input.peek(Token![,]) {
//             input.parse::<Token![,]>()?;
//         }

//         if input.peek(Ident) {
//             let ident: Ident = input.parse()?;
//             if ident == "any" {
//                 instructions.push(Instruction::Any);
//             } else {
//                 return Err(syn::Error::new(ident.span(), "Unexpected identifier"));
//             }
//         } else if input.peek(syn::token::Bracket) {
//             let instruction = parse_instruction(input)?;
//             instructions.push(instruction);
//         } else {
//             // Consume unexpected tokens to prevent infinite loops
//             input.parse::<proc_macro2::TokenTree>()?;
//         }
//     }

//     Ok(instructions)
// }

// // Function to parse individual instruction
// fn parse_instruction(input: ParseStream) -> Result<Instruction> {
//     let content;
//     syn::bracketed!(content in input);

//     // Parse the instruction type
//     let instruction_type: Expr = content.parse()?;

//     // Skip optional commas
//     while content.peek(Token![,]) {
//         content.parse::<Token![,]>()?;
//     }

//     // Parse the instruction name (identifier or string literal)
//     let name_expr: Expr = content.parse()?;
//     let name = match name_expr {
//         Expr::Path(expr_path) => {
//             // It's an identifier
//             if expr_path.path.segments.len() == 1 {
//                 expr_path.path.segments[0].ident.clone()
//             } else {
//                 return Err(syn::Error::new(
//                     expr_path.span(),
//                     "Invalid identifier for instruction name",
//                 ));
//             }
//         }
//         Expr::Lit(ExprLit {
//             lit: Lit::Str(lit_str),
//             ..
//         }) => {
//             // It's a string literal, convert to identifier
//             let name_str = lit_str.value();
//             // Ensure the string is a valid Rust identifier
//             format_ident!("{}", name_str)
//         }
//         _ => {
//             return Err(syn::Error::new(
//                 name_expr.span(),
//                 "Instruction name must be an identifier or string literal",
//             ));
//         }
//     };

//     let mut inner_instructions = Vec::new();

//     // Skip optional commas
//     while content.peek(Token![,]) {
//         content.parse::<Token![,]>()?;
//     }

//     // Check for inner instructions
//     if !content.is_empty() {
//         if content.peek(syn::token::Bracket) || content.peek(Ident) {
//             // Parse inner instructions
//             inner_instructions = parse_instructions(&content)?;
//         } else {
//             return Err(syn::Error::new(
//                 content.span(),
//                 "Expected inner instructions or 'any'",
//             ));
//         }
//     }

//     Ok(Instruction::InstructionNode {
//         instruction_type,
//         name,
//         inner_instructions,
//     })
// }

// impl GenerateOutputStructInput {
//     fn collect_identifiers(&self, identifiers: &mut Vec<Ident>) {
//         for instruction in &self.instructions {
//             instruction.collect_identifiers(identifiers);
//         }
//     }
// }

// impl Instruction {
//     fn collect_identifiers(&self, identifiers: &mut Vec<Ident>) {
//         match self {
//             Instruction::Any => {}
//             Instruction::InstructionNode {
//                 name,
//                 inner_instructions,
//                 ..
//             } => {
//                 identifiers.push(name.clone());
//                 for instr in inner_instructions {
//                     instr.collect_identifiers(identifiers);
//                 }
//             }
//         }
//     }
// }

use syn::ExprArray;

#[proc_macro]
pub fn generate_struct(input: TokenStream) -> TokenStream {
    let input_expr = parse_macro_input!(input as ExprArray);

    let mut fields = Vec::new();
    // Iterate through each element in the array
    for name in input_expr.elems {
        if let Expr::Lit(expr_lit) = name {
            if let syn::Lit::Str(lit_str) = expr_lit.lit {
                // Convert the string literal into an identifier for the struct field
                let field_name = syn::Ident::new(&lit_str.value(), lit_str.span());
                fields.push(quote! {
                    pub #field_name: AllInstructions
                });
            }
        }
    }

    // Generate the struct with the collected fields
    let output = quote! {
        pub struct CarbonOutput {
            #(#fields,)*
        }
    };

    output.into()
}
