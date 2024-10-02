use crate::{
    idl::Idl,
    legacy_idl::{LegacyIdl, LegacyIdlType},
};
use anyhow::{Context, Result};
use std::fs::File;

pub fn legacy_read_idl(idl_path: &str) -> Result<LegacyIdl> {
    let file = File::open(idl_path).unwrap();
    let idl: Result<LegacyIdl> = serde_json::from_reader(file).context("Can't parse legacy idl");
    idl
}

pub fn read_idl(idl_path: &str) -> Result<Idl> {
    let file = File::open(idl_path).unwrap();
    let idl: Result<Idl> = serde_json::from_reader(file).context("Can't parse idl");
    idl
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
    }
}
