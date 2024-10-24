use serde::{Deserialize, Serialize};

use crate::legacy_idl::{LegacyIdlEnumFields, LegacyIdlType};

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

#[derive(Debug, Serialize, Deserialize)]
pub struct IdlTypeDefinitionTy {
    pub kind: String,
    #[serde(default)]
    pub fields: Option<Vec<IdlTypeDefinitionField>>,
    #[serde(default)]
    pub variants: Option<Vec<IdlEnumVariant>>,
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
