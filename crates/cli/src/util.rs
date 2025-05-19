use {
    crate::{
        idl::Idl,
        legacy_idl::{LegacyIdl, LegacyIdlType},
    },
    anyhow::Result,
    std::fs::File,
};

pub fn legacy_read_idl(idl_path: &str) -> Result<LegacyIdl> {
    let file = File::open(idl_path).expect("Failed to open file");

    match serde_json::from_reader(file) {
        Ok(idl) => Ok(idl),
        Err(e) => {
            println!("Error parsing legacy IDL: {:?}", e);
            anyhow::bail!("Error parsing legacy idl: {:?}", e);
        }
    }
}

pub fn read_idl(idl_path: &str) -> Result<Idl> {
    let file = File::open(idl_path).expect("Failed to open file");
    match serde_json::from_reader(file) {
        Ok(idl) => Ok(idl),
        Err(e) => {
            println!("Error parsing IDL: {:?}", e);
            anyhow::bail!("Error parsing idl: {:?}", e);
        }
    }
}

pub fn idl_type_to_rust_type(idl_type: &LegacyIdlType) -> (String, bool) {
    match idl_type {
        LegacyIdlType::Primitive(s) => match s.as_str() {
            "bool" => ("bool".to_string(), false),
            "u8" => ("u8".to_string(), false),
            "i8" => ("i8".to_string(), false),
            "u16" => ("u16".to_string(), false),
            "i16" => ("i16".to_string(), false),
            "u32" => ("u32".to_string(), false),
            "i32" => ("i32".to_string(), false),
            "u64" => ("u64".to_string(), false),
            "i64" => ("i64".to_string(), false),
            "u128" => ("u128".to_string(), false),
            "i128" => ("i128".to_string(), false),
            "f32" => ("f32".to_string(), false),
            "f64" => ("f64".to_string(), false),
            "bytes" => ("Vec<u8>".to_string(), false),
            "string" => ("String".to_string(), false),
            "publicKey" => ("solana_pubkey::Pubkey".to_string(), false),
            "pubkey" => ("solana_pubkey::Pubkey".to_string(), false),
            _ => (s.clone(), true),
        },
        LegacyIdlType::Array { array } => {
            let (elem_type, size) = array;
            let rust_type = idl_type_to_rust_type(elem_type);
            (format!("[{}; {}]", rust_type.0, size), rust_type.1)
        }
        LegacyIdlType::Vec { vec } => {
            let rust_type = idl_type_to_rust_type(vec);
            (format!("Vec<{}>", rust_type.0), rust_type.1)
        }
        LegacyIdlType::Tuple { tuple } => {
            let rust_types = tuple.iter().map(idl_type_to_rust_type).collect::<Vec<_>>();
            (
                format!(
                    "({})",
                    rust_types
                        .iter()
                        .map(|t| t.0.clone())
                        .collect::<Vec<_>>()
                        .join(", ")
                ),
                rust_types.iter().any(|t| t.1),
            )
        }
        LegacyIdlType::Option { option } => {
            let rust_type = idl_type_to_rust_type(option);

            (format!("Option<{}>", rust_type.0), rust_type.1)
        }
        LegacyIdlType::Defined { defined } => (defined.clone(), true),
        LegacyIdlType::DefinedWithName { defined } => (defined.name.clone(), true),
        LegacyIdlType::HashMap { hash_map } => {
            let (key_type, value_type) = hash_map;
            let rust_key_type = idl_type_to_rust_type(key_type);
            let rust_value_type = idl_type_to_rust_type(value_type);
            (
                format!(
                    "std::collections::HashMap<{}, {}>",
                    rust_key_type.0, rust_value_type.0
                ),
                rust_key_type.1 || rust_value_type.1,
            )
        }
        LegacyIdlType::OptionPrimitive { option } => match option.as_str() {
            "bool" => ("bool".to_string(), false),
            "u8" => ("u8".to_string(), false),
            "i8" => ("i8".to_string(), false),
            "u16" => ("u16".to_string(), false),
            "i16" => ("i16".to_string(), false),
            "u32" => ("u32".to_string(), false),
            "i32" => ("i32".to_string(), false),
            "u64" => ("u64".to_string(), false),
            "i64" => ("i64".to_string(), false),
            "u128" => ("u128".to_string(), false),
            "i128" => ("i128".to_string(), false),
            "f32" => ("f32".to_string(), false),
            "f64" => ("f64".to_string(), false),
            "bytes" => ("Vec<u8>".to_string(), false),
            "string" => ("String".to_string(), false),
            "publicKey" => ("solana_pubkey::Pubkey".to_string(), false),
            "pubkey" => ("solana_pubkey::Pubkey".to_string(), false),
            _ => (option.clone(), true),
        },
    }
}

pub fn is_big_array(rust_type: &str) -> bool {
    if rust_type.starts_with("[") && rust_type.ends_with("]") {
        if let Some(semicolon_index) = rust_type.find(';') {
            if let Ok(size_str) = rust_type[semicolon_index + 1..rust_type.len() - 1]
                .trim()
                .parse::<usize>()
            {
                return size_str > 32;
            }
        }
    }
    false
}
