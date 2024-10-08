use crate::{
    idl::Idl,
    legacy_idl::{LegacyIdl, LegacyIdlType},
};
use anyhow::Result;
use std::fs::File;

pub fn legacy_read_idl(idl_path: &str) -> Result<LegacyIdl> {
    let file = File::open(idl_path).unwrap();

    match serde_json::from_reader(file) {
        Ok(idl) => Ok(idl),
        Err(e) => {
            println!("Error parsing legacy IDL: {:?}", e);
            anyhow::bail!("Error parsing legacy idl: {:?}", e);
        }
    }
}

pub fn read_idl(idl_path: &str) -> Result<Idl> {
    let file = File::open(idl_path).unwrap();
    match serde_json::from_reader(file) {
        Ok(idl) => Ok(idl),
        Err(e) => {
            println!("Error parsing IDL: {:?}", e);
            anyhow::bail!("Error parsing idl: {:?}", e);
        }
    }
}

pub fn idl_type_to_rust_type(idl_type: &LegacyIdlType) -> String {
    match idl_type {
        LegacyIdlType::Primitive(s) => match s.as_str() {
            "bool" => "bool".to_string(),
            "u8" => "u8".to_string(),
            "i8" => "i8".to_string(),
            "u16" => "u16".to_string(),
            "i16" => "i16".to_string(),
            "u32" => "u32".to_string(),
            "i32" => "i32".to_string(),
            "u64" => "u64".to_string(),
            "i64" => "i64".to_string(),
            "u128" => "u128".to_string(),
            "i128" => "i128".to_string(),
            "f32" => "f32".to_string(),
            "f64" => "f64".to_string(),
            "bytes" => "Vec<u8>".to_string(),
            "string" => "String".to_string(),
            "publicKey" => "solana_sdk::pubkey::Pubkey".to_string(),
            "pubkey" => "solana_sdk::pubkey::Pubkey".to_string(),
            _ => s.clone(),
        },
        LegacyIdlType::Array { array } => {
            let (elem_type, size) = array;
            format!("[{}; {}]", idl_type_to_rust_type(elem_type), size)
        }
        LegacyIdlType::Vec { vec } => {
            format!("Vec<{}>", idl_type_to_rust_type(vec))
        }
        LegacyIdlType::Option { option } => {
            format!("Option<{}>", idl_type_to_rust_type(option))
        }
        LegacyIdlType::Defined { defined } => defined.clone(),
        LegacyIdlType::HashMap { hash_map } => {
            let (key_type, value_type) = hash_map;
            format!(
                "std::collections::HashMap<{}, {}>",
                idl_type_to_rust_type(key_type),
                idl_type_to_rust_type(value_type)
            )
        }
        LegacyIdlType::OptionPrimitive { option } => match option.as_str() {
            "bool" => "bool".to_string(),
            "u8" => "u8".to_string(),
            "i8" => "i8".to_string(),
            "u16" => "u16".to_string(),
            "i16" => "i16".to_string(),
            "u32" => "u32".to_string(),
            "i32" => "i32".to_string(),
            "u64" => "u64".to_string(),
            "i64" => "i64".to_string(),
            "u128" => "u128".to_string(),
            "i128" => "i128".to_string(),
            "f32" => "f32".to_string(),
            "f64" => "f64".to_string(),
            "bytes" => "Vec<u8>".to_string(),
            "string" => "String".to_string(),
            "publicKey" => "solana_sdk::pubkey::Pubkey".to_string(),
            "pubkey" => "solana_sdk::pubkey::Pubkey".to_string(),
            _ => option.clone(),
        },
    }
}

pub fn is_big_array(rust_type: &str) -> bool {
    if rust_type.starts_with("[") && rust_type.ends_with("]") {
        if let Some(semicolon_index) = rust_type.find(';') {
            if let Some(size_str) = rust_type[semicolon_index + 1..rust_type.len() - 1]
                .trim()
                .parse::<usize>()
                .ok()
            {
                return size_str > 32;
            }
        }
    }
    false
}
