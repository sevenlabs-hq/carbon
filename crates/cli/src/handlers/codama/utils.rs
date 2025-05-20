use {
    super::types::{
        AccountNode, CountNode, InstructionArgumentNode, RootNode, StructTypeNode, TypeNode,
    },
    crate::handlers::codama::types::ValueNode,
    anyhow::Result,
    heck::ToUpperCamelCase,
    sha2::{Digest, Sha256},
    std::{collections::HashSet, fs::File},
};

pub fn map_type(type_node: &TypeNode) -> (String, bool) {
    match type_node {
        TypeNode::AmountTypeNode {
            decimals,
            unit,
            number,
        } => {
            let (rust_type, requires_import) = map_type(number);
            let unit_info = match unit {
                Some(u) => format!(" (unit: {})", u),
                None => "".to_string(),
            };

            println!(
                "Info: Mapping `AmountTypeNode` with {} decimals{} -> {}",
                decimals, unit_info, rust_type
            );

            (rust_type, requires_import)
        }

        TypeNode::NumberTypeNode { format, .. } => (format.to_string(), false),
        TypeNode::PublicKeyTypeNode => ("solana_pubkey::Pubkey".to_string(), false),
        TypeNode::BooleanTypeNode { .. } => ("bool".to_string(), false),
        TypeNode::FixedSizeTypeNode { size, r#type } => {
            let (rust_type, requires_import) = map_type(r#type);
            (format!("[{}; {}]", rust_type, size), requires_import)
        }
        TypeNode::OptionTypeNode { item, .. } => {
            let (rust_type, requires_import) = map_type(item);
            (format!("Option<{}>", rust_type), requires_import)
        }
        TypeNode::DefinedTypeLinkNode { name } => (name.to_upper_camel_case().clone(), true),
        TypeNode::BytesTypeNode => ("u8".to_string(), false),
        TypeNode::StringTypeNode { .. } => ("String".to_string(), false),
        TypeNode::SolAmountTypeNode { number } => {
            let inner = map_type(number);
            (inner.0.to_string(), inner.1)
        }
        TypeNode::SizePrefixTypeNode { r#type, .. } => {
            let inner = map_type(r#type);
            (inner.0.to_string(), inner.1)
        }
        TypeNode::ArrayTypeNode { item, count } => {
            let (rust_type, requires_import) = map_type(item);

            match count {
                CountNode::FixedCountNode { value } => {
                    (format!("[{}; {}]", rust_type, value), requires_import)
                }
                CountNode::PrefixedCountNode { .. } | CountNode::RemainderCountNode => {
                    (format!("Vec<{}>", rust_type), requires_import)
                }
            }
        }
        TypeNode::RemainderOptionTypeNode { item } => {
            let (rust_type, requires_import) = map_type(item);
            (format!("Option<Vec<{}>>", rust_type), requires_import)
        }
        TypeNode::HiddenPrefixTypeNode { r#type, .. } => {
            let (rust_type, requires_import) = map_type(r#type);
            (rust_type.to_string(), requires_import)
        }
        TypeNode::PreOffsetTypeNode {
            offset,
            strategy,
            inner_type,
        } => {
            let (rust_type, requires_import) = map_type(inner_type);
            println!(
                "Warning: PreOffsetTypeNode detected (offset: {}, strategy: {}). Inner type: {}",
                offset, strategy, rust_type
            );
            (rust_type, requires_import)
        }
        TypeNode::PostOffsetTypeNode {
            offset,
            strategy,
            inner_type,
        } => {
            let (rust_type, requires_import) = map_type(inner_type);
            println!(
                "Warning: PostOffsetTypeNode detected (offset: {}, strategy: {}). Inner type: {}",
                offset, strategy, rust_type
            );
            (rust_type, requires_import)
        }
        TypeNode::ZeroableOptionTypeNode { item, zero_value } => {
            let (rust_type, requires_import) = map_type(item);
            if zero_value.is_some() {
                println!(
                    "Warning: `ZeroableOptionTypeNode` with `zero_value` detected. Custom deserialization logic may be required."
                );
            }
            (format!("Option<{}>", rust_type), requires_import)
        }
        TypeNode::MapTypeNode { key, value, count } => {
            let (key_type, key_requires_import) = map_type(key);
            let (value_type, value_requires_import) = map_type(value);
            let requires_import = key_requires_import || value_requires_import;

            let rust_type = match count {
                CountNode::FixedCountNode { value } => {
                    format!("[({}, {}); {}]", key_type, value_type, value)
                }
                CountNode::PrefixedCountNode { .. } | CountNode::RemainderCountNode => {
                    format!("Vec<({}, {})>", key_type, value_type)
                }
            };

            (rust_type, requires_import)
        }
        _ => ("UnsupportedType".to_string(), false),
    }
}

pub fn get_instruction_discriminator(
    ix_arguments: &[InstructionArgumentNode],
    instruction_name: &str,
) -> String {
    if let Some(first_argument) = ix_arguments.first() {
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
    if let Some(first_data_field) = account_node.data.fields.first() {
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
    let file = File::open(idl_path).expect("Failed to open file");
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

pub fn resolve_struct_type(type_node: &TypeNode) -> Option<StructTypeNode> {
    match type_node {
        TypeNode::StructTypeNode { fields } => Some(StructTypeNode {
            fields: fields.clone(),
        }),
        TypeNode::SizePrefixTypeNode { r#type, .. } => resolve_struct_type(r#type),
        TypeNode::FixedSizeTypeNode { r#type, .. } => resolve_struct_type(r#type),
        TypeNode::PreOffsetTypeNode { inner_type, .. } => resolve_struct_type(inner_type),
        TypeNode::PostOffsetTypeNode { inner_type, .. } => resolve_struct_type(inner_type),
        _ => None,
    }
}
