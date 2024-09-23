use std::fs::File;

use anyhow::Result;

use crate::idl::{Idl, IdlType};

pub fn read_idl(idl_path: &str) -> Result<Idl> {
    let file = File::open(idl_path).unwrap();
    let idl: Idl = serde_json::from_reader(file).unwrap();

    Ok(idl)
}

pub fn idl_type_to_rust_type(idl_type: &IdlType) -> String {
    match idl_type {
        // The rest are going to be the same I think
        IdlType::Primitive(s) => match s.as_str() {
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
