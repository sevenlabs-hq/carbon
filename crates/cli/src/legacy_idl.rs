use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct LegacyIdl {
    pub version: String,
    pub name: String,
    #[serde(default)]
    pub constants: Vec<LegacyIdlConst>,
    #[serde(default)]
    pub instructions: Vec<LegacyIdlInstruction>,
    #[serde(default)]
    pub accounts: Vec<LegacyIdlAccountItem>,
    #[serde(default)]
    pub types: Vec<LegacyIdlTypeDefinition>,
    #[serde(default)]
    pub events: Vec<LegacyIdlEvent>,
    #[serde(default)]
    pub errors: Vec<LegacyIdlError>,
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct LegacyIdlConst {
    pub name: String,
    #[serde(rename = "type")]
    pub type_: LegacyIdlType,
    pub value: String,
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct LegacyIdlInstruction {
    pub name: String,
    pub discriminant: Option<LegacyIdlInstructionDiscriminant>,
    #[serde(default)]
    pub docs: Option<Vec<String>>,
    #[serde(default)]
    pub accounts: Vec<LegacyIdlInstructionAccount>,
    #[serde(default)]
    pub args: Vec<LegacyIdlInstructionArgField>,
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct LegacyIdlInstructionDiscriminant {
    #[serde(rename = "type")]
    pub type_: String,
    pub value: u8,
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct LegacyIdlInstructionAccount {
    pub name: String,
    #[serde(default)]
    pub is_mut: bool,
    #[serde(default)]
    pub is_signer: bool,
    #[serde(default, alias = "optional", alias = "IsOptional")]
    pub is_optional: Option<bool>,
    #[serde(default)]
    pub docs: Option<Vec<String>>,
    #[serde(default)]
    pub desc: Option<String>,
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct LegacyIdlInstructionArgField {
    pub name: String,
    #[serde(rename = "type")]
    pub type_: LegacyIdlType,
    #[serde(default)]
    pub docs: Option<Vec<String>>,
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(untagged)]
pub enum LegacyIdlType {
    Primitive(String),
    Array {
        array: (Box<LegacyIdlType>, usize),
    },
    Vec {
        vec: Box<LegacyIdlType>,
    },
    Tuple {
        tuple: Vec<LegacyIdlType>,
    },
    Option {
        option: Box<LegacyIdlType>,
    },
    OptionPrimitive {
        option: String,
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
#[serde(rename_all = "camelCase")]
pub struct LegacyIdlAccountItem {
    pub name: String,
    #[serde(rename = "type")]
    pub type_: LegacyIdlAccountType,
    #[serde(default)]
    pub docs: Option<Vec<String>>,
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct LegacyIdlAccountType {
    pub kind: String,
    #[serde(default)]
    pub fields: Option<Vec<LegacyIdlTypeDefinitionField>>,
    #[serde(default)]
    pub variants: Option<Vec<LegacyIdlEnumVariant>>,
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct LegacyIdlTypeDefinition {
    pub name: String,
    #[serde(rename = "type")]
    pub type_: LegacyIdlTypeDefinitionTy,
    #[serde(default)]
    pub docs: Option<Vec<String>>,
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct LegacyIdlTypeDefinitionTy {
    pub kind: String,
    #[serde(default)]
    pub fields: Option<Vec<LegacyIdlTypeDefinitionField>>,
    #[serde(default)]
    pub variants: Option<Vec<LegacyIdlEnumVariant>>,
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct LegacyIdlTypeDefinitionField {
    pub name: String,
    #[serde(rename = "type")]
    pub type_: LegacyIdlType,
    #[serde(default)]
    pub docs: Option<Vec<String>>,
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct LegacyIdlEnumVariant {
    pub name: String,
    #[serde(default)]
    pub fields: Option<LegacyIdlEnumFields>,
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(untagged)]
pub enum LegacyIdlEnumFields {
    Named(Vec<LegacyIdlTypeDefinitionField>),
    Tuple(Vec<LegacyIdlType>),
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct LegacyIdlEvent {
    pub name: String,
    pub fields: Vec<LegacyIdlEventField>,
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct LegacyIdlEventField {
    pub name: String,
    #[serde(rename = "type")]
    pub type_: LegacyIdlType,
    pub index: bool,
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct LegacyIdlError {
    pub code: f64,
    pub name: String,
    pub msg: Option<String>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct IdlDefinedType {
    pub name: String,
}
