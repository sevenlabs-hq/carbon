use sha2::{Digest, Sha256};

use crate::handlers::codama::types::ValueNode;

use super::types::{FieldDiscriminatorNode, InstructionArgumentNode, TypeNode};

pub fn map_type(type_node: &TypeNode) -> String {
    match type_node {
        TypeNode::NumberTypeNode { format, .. } => format!("{}", format),
        TypeNode::PublicKeyTypeNode => "Pubkey".to_string(),
        TypeNode::BooleanTypeNode { .. } => "bool".to_string(),
        TypeNode::FixedSizeTypeNode { size, r#type } => format!("[{}; {}]", map_type(r#type), size),
        TypeNode::OptionTypeNode { item, .. } => format!("Option<{}>", map_type(item)),
        TypeNode::DefinedTypeLinkNode { name } => name.clone(),
        TypeNode::BytesTypeNode => "Vec<u8>".to_string(),
        TypeNode::StringTypeNode { .. } => "String".to_string(),
        _ => "UnsupportedType".to_string(),
    }
}

pub fn get_instruction_discriminator(
    ix_arguments: &[InstructionArgumentNode],
    instruction_name: &str,
) -> String {
    if let Some(first_argument) = ix_arguments.get(0) {
        if first_argument.name == "discriminator" {
            if let Some(default_value) = &first_argument.default_value {
                let ValueNode::BytesValueNode { data, encoding } = &default_value.kind;
                if encoding == "base16" {
                    return format!("0x{}", data);
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
