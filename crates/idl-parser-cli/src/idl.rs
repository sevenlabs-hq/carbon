use serde::Deserialize;
use serde::Serialize;

#[derive(Debug, Serialize, Deserialize)]
pub struct Idl {
    pub version: String,
    pub name: String,
    #[serde(default)]
    pub constants: Vec<IdlConst>,
    #[serde(default)]
    pub instructions: Vec<IdlInstruction>,
    #[serde(default)]
    pub accounts: Vec<IdlAccountItem>,
    #[serde(default)]
    pub types: Vec<IdlTypeDefinition>,
    #[serde(default)]
    pub events: Vec<IdlEvent>,
    #[serde(default)]
    pub errors: Vec<IdlError>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct IdlConst {
    pub name: String,
    #[serde(rename = "type")]
    pub type_: IdlType,
    pub value: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct IdlInstruction {
    pub name: String,
    #[serde(default)]
    pub docs: Option<Vec<String>>,
    #[serde(default)]
    pub accounts: Vec<IdlInstructionAccount>,
    #[serde(default)]
    pub args: Vec<IdlInstructionArgField>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct IdlInstructionAccount {
    pub name: String,
    #[serde(rename = "isMut")]
    pub is_mut: bool,
    #[serde(rename = "isSigner")]
    pub is_signer: bool,
    #[serde(default)]
    #[serde(rename = "isOptional")]
    pub is_optional: Option<bool>,
    #[serde(default)]
    pub docs: Option<Vec<String>>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct IdlInstructionArgField {
    pub name: String,
    #[serde(rename = "type")]
    pub type_: IdlType,
    #[serde(default)]
    pub docs: Option<Vec<String>>,
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(untagged)]
pub enum IdlType {
    Primitive(String),
    Array { array: (Box<IdlType>, usize) },
    Vec { vec: Box<IdlType> },
    Option { option: Box<IdlType> },
    Defined { defined: String },
}

#[derive(Debug, Serialize, Deserialize)]
pub struct IdlAccountItem {
    pub name: String,
    #[serde(rename = "type")]
    pub type_: IdlAccountType,
    #[serde(default)]
    pub docs: Option<Vec<String>>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct IdlAccountType {
    pub kind: String,
    #[serde(default)]
    pub fields: Option<Vec<IdlTypeDefinitionField>>,
    #[serde(default)]
    pub variants: Option<Vec<IdlEnumVariant>>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct IdlTypeDefinition {
    pub name: String,
    #[serde(rename = "type")]
    pub type_: IdlTypeDefinitionTy,
    #[serde(default)]
    pub docs: Option<Vec<String>>,
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
    pub type_: IdlType,
    #[serde(default)]
    pub docs: Option<Vec<String>>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct IdlEnumVariant {
    pub name: String,
    #[serde(default)]
    pub fields: Option<IdlEnumFields>,
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(untagged)]
pub enum IdlEnumFields {
    Named(Vec<IdlTypeDefinitionField>),
    Tuple(Vec<IdlType>),
}

#[derive(Debug, Serialize, Deserialize)]
pub struct IdlEvent {
    pub name: String,
    pub fields: Vec<IdlEventField>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct IdlEventField {
    pub name: String,
    #[serde(rename = "type")]
    pub type_: IdlType,
    pub index: bool,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct IdlError {
    pub code: f64,
    pub name: String,
    pub msg: String,
}
