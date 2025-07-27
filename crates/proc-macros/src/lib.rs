//! # Carbon Proc Macros
//!
//! `carbon-proc-macros` is a collection of procedural macros designed to
//! simplify and enhance Rust-based development for Solana programs using the
//! Carbon framework. This crate provides macros for generating
//! deserialization implementations, instruction decoders, and type conversions.
//!
//! ## Overview
//!
//! The macros in this crate are intended to streamline common patterns
//! encountered when working with Carbon, particularly around deserialization,
//! instruction decoding, and structuring custom types. By leveraging
//! `carbon-proc-macros`, you can reduce the amount of manual coding and ensure
//! consistent, performant handling of Solana-specific data structures.
//!
//! ## Key Features
//!
//! - **`CarbonDeserialize`**: Automatically implement the `CarbonDeserialize`
//!   trait for structs and enums, enabling Borsh-based deserialization with
//!   optional discriminators for type validation.
//! - **`Instruction Decoder Collection`**: Create and manage complex
//!   instruction decoders for multiple Solana programs, simplifying how
//!   instructions are parsed and categorized.
//! - **`InstructionType` Derivation**: Derive `InstructionType` enums that
//!   mirror existing enum structures, providing a simplified, data-free version
//!   of each variant.
//!
//! ## Usage
//!
//! To use any of the provided macros, simply import the desired macro into your
//! Rust program and apply it to the relevant struct or enum.
//!
//! ## Notes
//!
//! - This crate relies on the `borsh` library for serialization and
//!   deserialization, so ensure the relevant dependencies are included in your
//!   project.
//! - The macros provided are optimized for use within the Carbon framework.
//!
//! ## Contribution
//!
//! Contributions are welcome! If you have ideas for improving or expanding the
//! functionality of `carbon_macros`, please consider submitting a pull request
//! or opening an issue on the project’s GitHub repository.
use {
    borsh_derive_internal::*,
    proc_macro::TokenStream,
    proc_macro2::{Span, TokenStream as TokenStream2},
    quote::{format_ident, quote},
    syn::{
        parse::{Parse, ParseStream},
        parse_macro_input, DeriveInput, Ident, Item, ItemEnum, Lit, Meta, NestedMeta, Token,
        TypePath,
    },
};

/// Automatically generates an implementation of the `CarbonDeserialize` trait.
///
/// This derive macro creates the `CarbonDeserialize` implementation for a given
/// struct or enum, enabling deserialization from a byte slice using the `borsh`
/// serialization format. If a field in the struct or enum is marked with the
/// `#[carbon(discriminator)]` attribute, the macro uses this field's value as a
/// discriminator to match and validate data during deserialization.
///
/// # Syntax
///
/// To use this macro, annotate your struct or enum with
/// `#[derive(CarbonDeserialize)]`. Optionally, use the `#[carbon(discriminator
/// = "0x...")]` attribute to specify a unique discriminator for this type. This
/// discriminator is validated at the start of the byte slice before proceeding
/// with full deserialization.
///
/// ```ignore
/// #[derive(CarbonDeserialize)]
/// #[carbon(discriminator = "0x1234")]
/// struct MyStruct {
///     id: u32,
///     data: String,
/// }
/// ```
///
/// # Example
///
/// ```ignore
/// use carbon_proc_macros::CarbonDeserialize;
///
/// #[derive(CarbonDeserialize)]
/// #[carbon(discriminator = "0x01")]
/// struct Message {
///     header: u16,
///     body: Vec<u8>,
/// }
///
/// let bytes = vec![0x01, 0x00, 0x10, 0x20, 0x30]; // Serialized data
/// let message = Message::deserialize(&bytes)
///     .expect("Failed to deserialize `Message`");
/// ```
///
/// # Parameters
///
/// - `input_token_stream`: A `TokenStream` containing the syntax tree of the
///   input type (struct or enum). The macro parses this to generate the
///   corresponding `CarbonDeserialize` implementation.
///
/// # Return
///
/// Returns a `TokenStream` representing the generated `CarbonDeserialize`
/// implementation. The function expects the target type to implement the
/// `borsh::BorshDeserialize` trait to support deserialization.
///
/// # Notes
///
/// - The `#[carbon(discriminator = "0x...")]` attribute is optional. If not
///   provided, the deserialization proceeds without a discriminator check.
/// - Ensure the discriminator matches the data's format exactly, as the
///   deserialization will return `None` if there is a mismatch.
/// - The macro will panic if the discriminator is invalid or not provided
///   correctly as a hex string when expected.
///
/// # Errors
///
/// - The macro will return `None` during deserialization if the data is shorter
///   than the discriminator or if there is a mismatch between the provided and
///   expected discriminators.
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

/// Generates an implementation of the `CarbonDeserialize` trait for a given
/// type.
///
/// This procedural macro automatically derives the `CarbonDeserialize` trait
/// for structs, enums, or unions, enabling them to be deserialized using Borsh
/// serialization format. The generated implementation includes type checking
/// and allows for customized deserialization using the `#[carbon]` attribute to
/// specify a unique discriminator for the type.
///
/// # Syntax
///
/// To use this macro, annotate the target type with
/// `#[derive(CarbonDeserialize)]`. Optionally, you can specify a
/// `#[carbon(discriminator = "...")]` attribute to define a custom
/// discriminator, which will be checked during deserialization.
///
/// # Example
///
/// ```ignore
/// use carbon_proc_macros::CarbonDeserialize;
///
/// #[derive(CarbonDeserialize)]
/// #[carbon(discriminator = "0x1234")]
/// struct MyStruct {
///     id: u32,
///     name: String,
/// }
///
/// let bytes = ...; // serialized bytes
/// let my_struct = MyStruct::deserialize(&bytes)
///     .expect("Failed to deserialize `MyStruct`");
/// ```
///
/// # Parameters
///
/// - `input_token_stream`: A `TokenStream` containing the parsed syntax tree of
///   the target type definition. This input is processed to generate the
///   appropriate `CarbonDeserialize` implementation.
///
/// # Return
///
/// Returns a `TokenStream` containing the implementation of the
/// `CarbonDeserialize` trait for the given type. If successful, this enables
/// Borsh deserialization with the custom discriminator check.
///
/// # Errors
///
/// This macro will panic if the target type is not a struct, enum, or union, as
/// these are the only supported forms for `CarbonDeserialize` derivation.
/// Additionally, an invalid or missing `#[carbon]` attribute may result in a
/// deserialization failure due to discriminator mismatch.
///
/// # Notes
///
/// - Ensure the discriminator length matches the expected format in serialized
///   data; otherwise, deserialization will return `None`.
/// - This macro leverages the Borsh serialization framework and assumes that
///   the type implements `BorshDeserialize` for successful deserialization.
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

/// Extracts the discriminator value from a set of attributes.
///
/// This function searches through a list of attributes for a `carbon` attribute
/// containing a `discriminator` key in the format `carbon(discriminator =
/// "0x...")`. If found, it parses the discriminator as a hexadecimal string and
/// returns it as a byte slice within a `TokenStream`. If the
/// `carbon(discriminator = "...")` attribute is not present, the function
/// returns `None`.
///
/// # Syntax
///
/// The attribute should be specified in the format:
///
/// ```ignore
/// #[carbon(discriminator = "0x...")]
/// ```
///
/// # Example
///
/// ```ignore
/// use syn::Attribute;
///
/// // Example attribute with a discriminator
/// let attrs: Vec<Attribute> = vec![parse_quote!(#[carbon(discriminator = "0x1234")])];
/// let discriminator = get_discriminator(&attrs);
///
/// assert!(discriminator.is_some());
/// ```
///
/// # Parameters
///
/// - `attrs`: A reference to a slice of `syn::Attribute` items. These represent
///   the attributes attached to a Rust item, from which the function will
///   attempt to extract the discriminator.
///
/// # Return
///
/// Returns an `Option<TokenStream>` containing the parsed byte slice if a
/// valid `carbon(discriminator = "...")` attribute is found. If the attribute
/// is not present, or if the value is not a valid hexadecimal string, the
/// function returns `None`.
///
/// # Errors
///
/// If the `carbon(discriminator = "...")` attribute contains an invalid hex
/// string, this function will panic with an error message indicating
/// "Invalid hex string". To avoid runtime panics, ensure that the hex string
/// provided is correctly formatted.
///
/// # Notes
///
/// - The `discriminator` value must be a hexadecimal string prefixed with "0x".
/// - If the hex string is invalid, an error will be raised; consider adding
///   further error handling if required for your application.
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

/// Represents the parsed input for the `instruction_decoder_collection!` macro.
///
/// The `InstructionMacroInput` struct holds the essential elements required
/// to generate instruction decoding logic within the
/// `instruction_decoder_collection!` macro. It includes the names of the enums
/// for instructions, instruction types, and programs, along with a collection
/// of `InstructionEntry` mappings that define the relationships between program
/// variants, decoder expressions, and instruction types.
///
/// # Fields
///
/// - `instructions_enum_name`: The identifier for the enum representing the
///   instructions. This enum contains the primary instruction variants to be
///   used within the macro.
/// - `instruction_types_enum_name`: The identifier for the enum representing
///   the various types of instructions. This enum categorizes instructions by
///   their specific types.
/// - `programs_enum_name`: The identifier for the enum representing the
///   programs. This enum is used to identify different programs and their
///   corresponding variants in the macro.
/// - `entries`: A vector of `InstructionEntry` items, each of which maps a
///   program variant to a decoder expression and an instruction type, defining
///   how each instruction should be processed.
///
/// # Example
///
/// ```ignore
/// use syn::Ident;
/// use syn::parse_quote;
///
/// let instructions_enum_name: Ident = parse_quote!(InstructionsEnum);
/// let instruction_types_enum_name: Ident = parse_quote!(InstructionTypesEnum);
/// let programs_enum_name: Ident = parse_quote!(ProgramsEnum);
/// let entries = vec![
///     InstructionEntry {
///         program_variant: parse_quote!(MyProgram),
///         decoder_expr: parse_quote!(my_decoder),
///         instruction_type: parse_quote!(MyInstructionType),
///     },
/// ];
///
/// let input = InstructionMacroInput {
///     instructions_enum_name,
///     instruction_types_enum_name,
///     programs_enum_name,
///     entries,
/// };
/// ```
///
/// # Usage
///
/// The `InstructionMacroInput` struct is primarily used within procedural
/// macros for parsing and storing elements required for generating complex
/// decoding logic. Each field serves a specific role in defining how
/// instructions are categorized, decoded, and mapped to programs.
///
/// # Notes
///
/// - Ensure that all identifiers correspond to valid enums in your macro
///   context, as these will be referenced directly when generating code.
/// - The `entries` vector should contain an `InstructionEntry` for each mapping
///   you wish to include. Each entry specifies a program variant and the logic
///   to decode its instructions.
struct InstructionMacroInput {
    instructions_enum_name: Ident,
    instruction_types_enum_name: Ident,
    programs_enum_name: Ident,
    entries: Vec<InstructionEntry>,
}

/// Represents a mapping between a program variant, its decoder expression, and
/// an instruction type.
///
/// The `InstructionEntry` struct is used to define individual mappings within
/// the `instruction_decoder_collection!` macro. Each entry specifies a unique
/// program variant, decoder for its instructions, and the
/// resulting instruction type. This structure enables the macro to understand
/// and process different program instructions efficiently.
///
/// # Fields
///
/// - `program_variant`: An `Ident` representing the variant of the program
///   enum. This is used to match against specific programs within the macro.
/// - `decoder_expr`: An expression (`syn::Expr`) that defines the decoding
///   logic for this program variant.
/// - `instruction_type`: A `TypePath` that specifies the type of instruction
///   resulting from the decoding process. This type should correspond to one of
///   the variants in the instruction types enum.
///
/// # Example
///
/// ```ignore
///
/// let program_variant: Ident = parse_quote!(MyProgram);
/// let decoder_expr: Expr = parse_quote!(MyDecoder);
/// let instruction_type: TypePath = parse_quote!(MyInstructionType);
///
/// let entry = InstructionEntry {
///     program_variant,
///     decoder_expr,
///     instruction_type,
/// };
/// ```
///
/// # Usage
///
/// The `InstructionEntry` struct is used as part of a vector within the
/// `InstructionMacroInput` struct. Each entry allows the macro to handle
/// multiple programs and their associated instruction types in a modular
/// and scalable manner. By specifying each program's decoding logic and
/// instruction type, the macro can dynamically adapt to different program
/// requirements.
///
/// # Notes
///
/// - Ensure that `decoder_expr` correctly implements the decoding functionality
///   expected by the associated `instruction_type`. Misalignment between the
///   decoder expression and the expected instruction type can lead to runtime
///   errors.
/// - This struct is typically not used standalone but as part of a collection
///   that defines multiple program-instruction mappings for procedural macros.
struct InstructionEntry {
    program_variant: Ident,
    decoder_expr: syn::Expr,
    instruction_type: TypePath,
}

/// Parses input for the `instruction_decoder_collection!` macro.
///
/// This implementation of the `Parse` trait is responsible for parsing the
/// input provided to the `instruction_decoder_collection!` macro. It expects a
/// comma-separated sequence of identifiers followed by a series of
/// `InstructionEntry` items, which define mappings between program variants,
/// decoder expressions, and instruction types. These entries are collected into
/// an `InstructionMacroInput` struct, which can then be used to generate
/// instruction decoding logic.
///
/// # Syntax
///
/// The input format for the macro should follow this structure:
///
/// ```ignore
/// instruction_decoder_collection!(
///     InstructionsEnum, InstructionTypesEnum, ProgramsEnum,
///     ProgramVariant => decoder_expr => InstructionType,
///     ProgramVariant => decoder_expr => InstructionType,
///     ...
/// );
/// ```
///
/// - `InstructionsEnum`: Identifier for the enum representing instruction names
///   with data.
/// - `InstructionTypesEnum`: Identifier for the enum representing types of
///   instructions.
/// - `ProgramsEnum`: Identifier for the enum representing program types.
/// - Each `InstructionEntry` consists of a program variant, a decoder
///   expression, and an instruction type, separated by `=>` and followed by a
///   comma.
///
/// # Example
///
/// ```ignore
///
/// let input = parse_quote! {
///     MyInstructionsEnum, MyInstructionTypesEnum, MyProgramsEnum,
///     MyProgram => my_decoder => MyInstruction,
///     AnotherProgram => another_decoder => AnotherInstruction,
/// };
///
/// let parsed_input: InstructionMacroInput = syn::parse2(input)
///     .expect("Failed to parse macro input");
/// ```
///
/// # Parameters
///
/// - `input`: A `ParseStream` representing the macro input, expected to
///   contain:
///   - An enum name for instructions
///   - An enum name for instruction types
///   - An enum name for program types
///   - A series of `InstructionEntry` mappings for program variants and
///     instructions.
///
/// # Return
///
/// Returns a `syn::Result<Self>`, which will be an `InstructionMacroInput`
/// containing the parsed components if successful. On failure, returns a
/// `syn::Error` indicating the specific parsing issue.
///
/// # Notes
///
/// - The macro requires the input format to be strictly adhered to, with commas
///   separating the enum identifiers and each `InstructionEntry` mapping.
///   Ensure that all mappings include `=>` separators between program variants,
///   decoder expressions, and instruction types.
/// - This parsing process is typically used within a procedural macro and may
///   be subject to Rust's macro hygiene and parsing rules.
///
/// # Errors
///
/// An error will be returned if:
/// - An identifier or component is missing or improperly formatted
/// - The input stream does not conform to the expected comma-separated format
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

/// Generates a collection of instruction decoders and associated enums.
///
/// This macro creates a set of enums and implementations to handle decoding
/// of instructions for multiple Solana programs. It generates:
/// 1. An enum for all instructions
/// 2. An enum for all instruction types
/// 3. An enum for all programs
/// 4. An implementation of InstructionDecoderCollection trait
///
/// # Syntax
///
/// The macro takes the following arguments:
/// 1. Name for the all-encompassing instructions enum
/// 2. Name for the all-encompassing instruction types enum
/// 3. Name for the programs enum
/// 4. One or more entries, each consisting of:
///    - Program variant name
///    - Decoder expression
///    - Instruction enum for the program
///
/// # Example
///
/// ```ignore
/// instruction_decoder_collection!(
///     AllInstructions, AllInstructionTypes, AllPrograms,
///     JupSwap => JupiterDecoder => JupiterInstruction,
///     MeteoraSwap => MeteoraDecoder => MeteoraInstruction
/// );
/// ```
///
///
/// This example will generate:
/// - AllInstructions enum with variants JupSwap(JupiterInstruction) and
///   MeteoraSwap(MeteoraInstruction)
/// - AllInstructionTypes enum with variants JupSwap(JupiterInstructionType) and
///   MeteoraSwap(MeteoraInstructionType)
/// - AllPrograms enum with variants JupSwap and MeteoraSwap
/// - An implementation of InstructionDecoderCollection for AllInstructions
///
/// # Generated Code
///
/// The macro generates the following:
/// 1. An enum AllInstructions containing variants for each program's
///    instructions
/// 2. An enum AllInstructionTypes containing variants for each program's
///    instruction types
/// 3. An enum AllPrograms listing all programs
/// 4. An implementation of InstructionDecoderCollection for AllInstructions,
///    including:
///    - parse_instruction method to decode instructions
///    - get_type method to retrieve the instruction type
///
/// # Note
///
/// Ensure that all necessary types (e.g., DecodedInstruction,
/// InstructionDecoderCollection) are in scope where this macro is used.
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

/// Derives a corresponding `InstructionType` enum for a given enum.
///
/// This procedural macro generates an `InstructionType` enum that mirrors the
/// variants of the specified input enum. The `InstructionType` enum contains
/// only the variant names, without any associated data. This is particularly
/// useful for implementations that require a simplified representation of
/// instruction types, such as in `InstructionDecoderCollection`.
///
/// # Syntax
///
/// To use this macro, annotate your enum with `#[derive(InstructionType)]`.
/// This will automatically create an `InstructionType` enum with the same
/// variant names as your original enum, suffixed with `Type`. Additionally,
/// a `get_instruction_type` method will be implemented on the original enum,
/// returning the corresponding `InstructionType` variant for each instance.
///
/// ```ignore
/// #[derive(InstructionType)]
/// enum MyEnum {
///     VariantOne,
///     VariantTwo(u32),
///     VariantThree { data: String },
/// }
/// ```
///
/// The derived `InstructionType` enum will look like:
///
/// ```rust
/// #[derive(Debug, Clone, PartialEq, Eq, serde::Serialize)]
/// pub enum MyEnumType {
///     VariantOne,
///     VariantTwo,
///     VariantThree,
/// }
/// ```
///
/// # Example
///
/// ```rust
/// use carbon_proc_macros::InstructionType;
///
/// #[derive(InstructionType)]
/// enum Instructions {
///     NoData,
///     WithData(u64),
///     NamedData { field: String },
/// }
///
/// let inst = Instructions::WithData(42);
/// let inst_type = inst.get_instruction_type();
///
/// assert_eq!(inst_type, InstructionsType::WithData);
/// ```
///
/// # Parameters
///
/// - `input`: A `TokenStream` representing the input enum, which is parsed to
///   extract variant names and generate the `InstructionType` enum. Each
///   variant is processed without any associated data.
///
/// # Return
///
/// Returns a `TokenStream` containing the expanded code for the generated
/// `InstructionType` enum and the implementation of the `get_instruction_type`
/// method on the original enum.
///
/// # Notes
///
/// - This macro will only derive an `InstructionType` enum for the input enum.
///   It does not modify or remove any data associated with the original enum
///   variants.
/// - The generated `InstructionType` enum derives `Debug`, `Clone`,
///   `PartialEq`, `Eq`, and `serde::Serialize`, making it suitable for use in
///   serialization contexts as well as comparison and debugging.
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
