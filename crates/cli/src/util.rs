use crate::idl::{Idl, IdlType};
use anyhow::Result;
use std::fs::File;

pub fn read_idl(idl_path: &str) -> Result<Idl> {
    let file = File::open(idl_path).unwrap();
    let idl: Idl = serde_json::from_reader(file).unwrap();

    Ok(idl)
}

pub fn idl_type_to_rust_type(idl_type: &IdlType) -> String {
    match idl_type {
        IdlType::Primitive(s) => match s.as_str() {
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
            _ => s.clone(),
        },
        IdlType::Array { array } => {
            let (elem_type, size) = array;
            format!("[{}; {}]", idl_type_to_rust_type(elem_type), size)
        }
        IdlType::Vec { vec } => {
            format!("Vec<{}>", idl_type_to_rust_type(vec))
        }
        IdlType::Option { option } => {
            format!("Option<{}>", idl_type_to_rust_type(option))
        }
        IdlType::Defined { defined } => defined.clone(),
    }
}
