use super::types::{AccountNode, InstructionArgumentNode, RootNode, TypeNode};
use crate::handlers::codama::types::ValueNode;
use anyhow::Result;
use heck::ToUpperCamelCase;
use sha2::{Digest, Sha256};
use std::{collections::HashSet, fs::File};

pub fn map_type(type_node: &TypeNode) -> (String, bool) {
    match type_node {
        TypeNode::NumberTypeNode { format, .. } => (format!("{}", format), false),
        TypeNode::PublicKeyTypeNode => ("solana_sdk::pubkey::Pubkey".to_string(), false),
        TypeNode::BooleanTypeNode { .. } => ("bool".to_string(), false),
        TypeNode::FixedSizeTypeNode { size, r#type } => {
            let (rust_type, requres_import) = map_type(r#type);
            (format!("[{}; {}]", rust_type, size), requres_import)
        }
        TypeNode::OptionTypeNode { item, .. } => {
            let (rust_type, requres_import) = map_type(item);
            (format!("Option<{}>", rust_type), requres_import)
        }
        TypeNode::DefinedTypeLinkNode { name } => (name.to_upper_camel_case().clone(), true),
        TypeNode::BytesTypeNode => ("u8".to_string(), false),
        TypeNode::StringTypeNode { .. } => ("String".to_string(), false),
        TypeNode::SolAmountTypeNode { number } => {
            let inner = map_type(number);
            (format!("{}", inner.0), inner.1)
        }
        TypeNode::SizePrefixTypeNode { r#type, .. } => {
            let inner = map_type(r#type);
            (format!("{}", inner.0), inner.1)
        }
        TypeNode::ArrayTypeNode { item, count } => {
            let (rust_type, requires_import) = map_type(item);
            (format!("[{}; {}]", rust_type, count.value), requires_import)
        }
        _ => ("UnsupportedType".to_string(), false),
    }
}

pub fn get_instruction_discriminator(
    ix_arguments: &[InstructionArgumentNode],
    instruction_name: &str,
) -> String {
    if let Some(first_argument) = ix_arguments.get(0) {
        if first_argument.name == "discriminator" {
            if let Some(default_value) = &first_argument.default_value {
                match default_value {
                    ValueNode::BytesValueNode { data, encoding } if encoding == "base16" => {
                        return format!("0x{}", data);
                    }
                    ValueNode::NumberValueNode { number } => {
                        if let TypeNode::NumberTypeNode { format, .. } = &first_argument.arg_type {
                            let bytes = match format.as_str() {
                                "u8" => vec![*number as u8],
                                "u16" => (*number as u16).to_le_bytes().to_vec(),
                                "u32" => (*number as u32).to_le_bytes().to_vec(),
                                "u64" => number.to_le_bytes().to_vec(),
                                _ => [0u8; 8].to_vec(),
                            };

                            return format!("0x{}", hex::encode(&bytes));
                        }
                    }
                    _ => {}
                }
            }
        }
    }

    let mut hasher = Sha256::new();
    let discriminator_input = format!("global:{}", instruction_name);
    hasher.update(discriminator_input.as_bytes());
    let hash = hasher.finalize();
    let discriminator_bytes = &hash[..8];
    format!("0x{}", hex::encode(discriminator_bytes))
}

pub fn get_account_discriminator(account_node: &AccountNode, account_name: &str) -> String {
    if let Some(first_data_field) = account_node.data.fields.get(0) {
        if first_data_field.name == "discriminator" {
            if let Some(default_value) = &first_data_field.default_value {
                match default_value {
                    ValueNode::BytesValueNode { data, encoding } if encoding == "base16" => {
                        return format!("0x{}", data);
                    }
                    ValueNode::NumberValueNode { number } => {
                        if let TypeNode::NumberTypeNode { format, .. } =
                            &first_data_field.field_type
                        {
                            let bytes = match format.as_str() {
                                "u8" => vec![*number as u8],
                                "u16" => (*number as u16).to_le_bytes().to_vec(),
                                "u32" => (*number as u32).to_le_bytes().to_vec(),
                                "u64" => number.to_le_bytes().to_vec(),
                                _ => [0u8; 8].to_vec(),
                            };

                            return format!("0x{}", hex::encode(&bytes));
                        }
                    }
                    _ => {}
                }
            }
        }
    }

    let mut hasher = Sha256::new();
    let discriminator_input = format!("account:{}", account_name);
    hasher.update(discriminator_input.as_bytes());
    let hash = hasher.finalize();
    let discriminator_bytes = &hash[..8];
    format!("0x{}", hex::encode(discriminator_bytes))
}

pub fn get_event_discriminator(event_name: &str) -> String {
    let mut hasher = Sha256::new();
    let discriminator_input = format!("event:{}", event_name);
    hasher.update(discriminator_input.as_bytes());
    let hash = hasher.finalize();
    let discriminator_bytes = &hash[..8];
    format!("0xe445a52e51cb9a1d{}", hex::encode(discriminator_bytes))
}

pub fn read_codama_idl(idl_path: &str) -> Result<RootNode> {
    let file = File::open(idl_path).unwrap();
    match serde_json::from_reader(file) {
        Ok(idl) => Ok(idl),
        Err(e) => {
            println!("Error parsing Codama IDL: {:?}", e);
            anyhow::bail!("Error parsing  Codama idl: {:?}", e);
        }
    }
}

pub fn parse_event_hints(hints: Option<String>) -> HashSet<String> {
    hints
        .unwrap_or("".to_string())
        .split(',')
        .map(|s| s.trim().to_string().to_upper_camel_case())
        .filter(|s| !s.is_empty())
        .collect()
}
