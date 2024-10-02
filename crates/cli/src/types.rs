use crate::{
    idl::Idl,
    legacy_idl::{LegacyIdl, LegacyIdlEnumFields},
    util::idl_type_to_rust_type,
};
use askama::Template;
use heck::ToSnakeCase;

#[allow(dead_code)]
#[derive(Debug)]
pub struct TypeData {
    pub name: String,
    pub fields: Vec<FieldData>,
    pub kind: TypeKind,
}

#[allow(dead_code)]
#[derive(Debug, PartialEq, Eq)]
pub enum TypeKind {
    Struct,
    Enum(Vec<EnumVariantData>),
}

#[allow(dead_code)]
#[derive(Debug, PartialEq, Eq)]
pub struct FieldData {
    pub name: String,
    pub rust_type: String,
    pub is_pubkey: bool,
}

#[allow(dead_code)]
#[derive(Debug, PartialEq, Eq)]
pub struct EnumVariantData {
    pub name: String,
    pub fields: Option<EnumVariantFields>,
}

#[allow(dead_code)]
#[derive(Debug, PartialEq, Eq)]
pub enum EnumVariantFields {
    Named(Vec<FieldData>),
    Unnamed(Vec<String>),
}

#[derive(Template)]
#[template(path = "types_struct.askama", escape = "none", ext = ".askama")]
pub struct TypeStructTemplate<'a> {
    pub type_data: &'a TypeData,
}

pub fn legacy_process_types(idl: &LegacyIdl) -> Vec<TypeData> {
    let mut types_data = Vec::new();

    for idl_type_def in &idl.types {
        let name = idl_type_def.name.clone();
        let mut fields = Vec::new();
        let mut kind = TypeKind::Struct;

        match idl_type_def.type_.kind.as_str() {
            "struct" => {
                if let Some(ref fields_vec) = idl_type_def.type_.fields {
                    for field in fields_vec {
                        let rust_type = idl_type_to_rust_type(&field.type_);
                        let is_pubkey = rust_type.contains("Pubkey");
                        fields.push(FieldData {
                            name: field.name.to_snake_case(),
                            rust_type,
                            is_pubkey,
                        });
                    }
                }
            }
            "enum" => {
                let mut variants = Vec::new();
                if let Some(ref variants_vec) = idl_type_def.type_.variants {
                    for variant in variants_vec {
                        let variant_name = variant.name.clone();
                        let variant_fields = if let Some(ref fields) = variant.fields {
                            match fields {
                                LegacyIdlEnumFields::Named(named_fields) => {
                                    let mut variant_field_data = Vec::new();
                                    for field in named_fields {
                                        let rust_type = idl_type_to_rust_type(&field.type_);
                                        let is_pubkey = rust_type.contains("Pubkey");
                                        variant_field_data.push(FieldData {
                                            name: field.name.to_snake_case(),
                                            rust_type,
                                            is_pubkey,
                                        });
                                    }
                                    Some(EnumVariantFields::Named(variant_field_data))
                                }
                                LegacyIdlEnumFields::Tuple(tuple_fields) => {
                                    let rust_types = tuple_fields
                                        .iter()
                                        .map(|ty| idl_type_to_rust_type(ty))
                                        .collect();
                                    Some(EnumVariantFields::Unnamed(rust_types))
                                }
                            }
                        } else {
                            None
                        };
                        variants.push(EnumVariantData {
                            name: variant_name,
                            fields: variant_fields,
                        });
                    }
                }
                kind = TypeKind::Enum(variants);
            }
            _ => {}
        }

        types_data.push(TypeData { name, fields, kind });
    }

    types_data
}

pub fn process_types(idl: &Idl) -> Vec<TypeData> {
    let mut types_data = Vec::new();

    for idl_type_def in &idl.types {
        let name = idl_type_def.name.clone();
        let mut fields = Vec::new();
        let mut kind = TypeKind::Struct;

        match idl_type_def.type_.kind.as_str() {
            "struct" => {
                if let Some(ref fields_vec) = idl_type_def.type_.fields {
                    for field in fields_vec {
                        let rust_type = idl_type_to_rust_type(&field.type_);
                        let is_pubkey = rust_type.contains("Pubkey");
                        fields.push(FieldData {
                            name: field.name.to_snake_case(),
                            rust_type,
                            is_pubkey,
                        });
                    }
                }
            }
            "enum" => {
                let mut variants = Vec::new();
                if let Some(ref variants_vec) = idl_type_def.type_.variants {
                    for variant in variants_vec {
                        let variant_name = variant.name.clone();
                        let variant_fields = if let Some(ref fields) = variant.fields {
                            match fields {
                                LegacyIdlEnumFields::Named(named_fields) => {
                                    let mut variant_field_data = Vec::new();
                                    for field in named_fields {
                                        let rust_type = idl_type_to_rust_type(&field.type_);
                                        let is_pubkey = rust_type.contains("Pubkey");
                                        variant_field_data.push(FieldData {
                                            name: field.name.to_snake_case(),
                                            rust_type,
                                            is_pubkey,
                                        });
                                    }
                                    Some(EnumVariantFields::Named(variant_field_data))
                                }
                                LegacyIdlEnumFields::Tuple(tuple_fields) => {
                                    let rust_types = tuple_fields
                                        .iter()
                                        .map(|ty| idl_type_to_rust_type(ty))
                                        .collect();
                                    Some(EnumVariantFields::Unnamed(rust_types))
                                }
                            }
                        } else {
                            None
                        };
                        variants.push(EnumVariantData {
                            name: variant_name,
                            fields: variant_fields,
                        });
                    }
                }
                kind = TypeKind::Enum(variants);
            }
            _ => {}
        }

        types_data.push(TypeData { name, fields, kind });
    }

    types_data
}
