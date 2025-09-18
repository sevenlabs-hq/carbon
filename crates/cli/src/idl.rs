use {
    crate::legacy_idl::{LegacyIdlEnumFields, LegacyIdlType},
    serde::de::{self, Deserializer},
    serde::{Deserialize, Serialize},
};

#[derive(Debug, Serialize, Deserialize)]
pub struct Idl {
    pub address: String,
    pub metadata: IdlMetadata,
    #[serde(default)]
    pub instructions: Vec<IdlInstruction>,
    #[serde(default)]
    pub accounts: Vec<IdlAccount>,
    #[serde(default)]
    pub errors: Vec<IdlError>,
    #[serde(default)]
    pub types: Vec<IdlTypeDefinition>,
    #[serde(default)]
    pub events: Vec<IdlEvent>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct IdlMetadata {
    pub name: String,
    pub version: String,
    pub spec: String,
    pub description: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct IdlInstruction {
    pub name: String,
    pub discriminator: Vec<u8>,
    #[serde(default)]
    pub docs: Option<Vec<String>>,
    #[serde(default)]
    pub accounts: Vec<IdlInstructionAccount>,
    #[serde(default)]
    pub args: Vec<IdlInstructionArg>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct IdlInstructionAccount {
    pub name: String,
    #[serde(default)]
    pub writable: Option<bool>,
    #[serde(default)]
    pub signer: Option<bool>,
    #[serde(default)]
    pub pda: Option<IdlPda>,
    #[serde(default)]
    pub address: Option<String>,
    #[serde(default)]
    pub desc: Option<String>,
    #[serde(default)]
    pub docs: Option<Vec<String>>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct IdlPda {
    pub seeds: Vec<IdlPdaSeed>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct IdlPdaSeed {
    pub kind: String,
    #[serde(default)]
    pub value: Option<Vec<u8>>,
    #[serde(default)]
    pub path: Option<String>,
    #[serde(default)]
    pub account: Option<String>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct IdlInstructionArg {
    pub name: String,
    #[serde(rename = "type")]
    pub type_: LegacyIdlType,
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(untagged)]
pub enum IdlType {
    Primitive(String),
    Option {
        option: Box<IdlType>,
    },
    OptionPrimitive {
        option: String,
    },
    Vec {
        vec: Box<IdlType>,
    },
    Array {
        array: (Box<IdlType>, usize),
    },
    Defined {
        defined: String,
    },
    DefinedWithName {
        defined: IdlDefinedType,
    },
    HashMap {
        #[serde(rename = "hashMap")]
        hash_map: (Box<LegacyIdlType>, Box<LegacyIdlType>),
    },
}

#[derive(Debug, Serialize, Deserialize)]
pub struct IdlAccount {
    pub name: String,
    pub discriminator: Vec<u8>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct IdlError {
    pub code: u32,
    pub name: String,
    pub msg: Option<String>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct IdlTypeDefinition {
    pub name: String,
    #[serde(rename = "type")]
    pub type_: IdlTypeDefinitionTy,
}

#[derive(Debug, Serialize)]
pub struct IdlTypeDefinitionTy {
    pub kind: String,
    #[serde(default)]
    pub fields: Option<Vec<IdlEnumField>>,
    #[serde(default)]
    pub variants: Option<Vec<IdlEnumVariant>>,
}

impl<'de> Deserialize<'de> for IdlTypeDefinitionTy {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        #[derive(Deserialize, Debug)]
        struct Helper {
            kind: String,
            #[serde(default)]
            fields: Option<Vec<IdlTypeDefinitionField>>,
            #[serde(default)]
            variants: Option<Vec<IdlEnumVariant>>,
        }

        fn is_primish(t: &LegacyIdlType) -> bool {
            match t {
                LegacyIdlType::Primitive(s) => matches!(
                    s.as_str(),
                    "bool"
                        | "u8"
                        | "i8"
                        | "u16"
                        | "i16"
                        | "u32"
                        | "i32"
                        | "u64"
                        | "i64"
                        | "u128"
                        | "i128"
                        | "f32"
                        | "f64"
                        | "bytes"
                        | "string"
                        | "pubkey"
                        | "publicKey"
                ),
                LegacyIdlType::OptionPrimitive { .. } => true,
                _ => false,
            }
        }

        let mut helper = Helper::deserialize(deserializer)?;

        if helper.kind == "struct" {
            println!("fields: {:#?}", helper);
            if let Some(fields) = &helper.fields {
                let mut any_named = false;
                let mut any_primish = false;

                println!("fields: {:#?}", fields);
                // for f in fields {
                //     match f {
                //         IdlTypeDefinitionField::Tuple(type_) => {
                //             if is_primish(type_) {
                //                 any_primish = true;
                //             } else {
                //                 any_named = true;
                //             }
                //         }
                //         _ => {
                //             any_named = true;
                //         }
                //     }
                // }

                if any_primish && !any_named {
                    helper.kind = "tuple_struct".to_owned();
                } else if any_named && any_primish {
                    return Err(de::Error::custom(
                        "for kind=\"struct\", fields must be either all Primitive/OptionPrimitive (→ tuple_struct) or all proper Named struct fields",
                    ));
                }
            }
        }

        Ok(IdlTypeDefinitionTy {
            kind: helper.kind,
            fields: helper.fields,
            variants: helper.variants,
        })
    }
}

#[derive(Debug, Serialize, Deserialize)]
pub struct IdlTypeDefinitionField {
    pub name: String,
    #[serde(rename = "type")]
    pub type_: LegacyIdlType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct IdlEnumVariant {
    pub name: String,
    #[serde(default)]
    pub fields: Option<LegacyIdlEnumFields>,
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(untagged)]
pub enum IdlEnumField {
    Named(IdlTypeDefinitionField),
    Tuple(LegacyIdlType),
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(untagged)]
pub enum IdlEnumFields {
    Named(Vec<IdlTypeDefinitionField>),
    Tuple(Vec<LegacyIdlType>),
}

#[derive(Debug, Serialize, Deserialize)]
pub struct IdlEvent {
    pub name: String,
    pub discriminator: Vec<u8>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct IdlEventField {
    pub name: String,
    #[serde(rename = "type")]
    pub type_: LegacyIdlType,
    pub index: bool,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct IdlDefinedType {
    pub name: String,
}
