use serde::{Deserialize, Serialize};

// ----------------------- Codama Types -----------------------

#[derive(Debug, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct RootNode {
    pub program: ProgramNode,
}

#[derive(Debug, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct ProgramNode {
    pub name: String,
    pub accounts: Vec<AccountNode>,
    pub instructions: Vec<InstructionNode>,
    pub defined_types: Vec<DefinedTypeNode>,
}

#[derive(Debug, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct AccountNode {
    pub name: String,
    pub data: StructTypeNode,
    pub discriminators: Vec<FieldDiscriminatorNode>,
}

#[derive(Debug, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct InstructionNode {
    pub name: String,
    pub accounts: Vec<InstructionAccountNode>,
    pub arguments: Vec<InstructionArgumentNode>,
    pub discriminators: Vec<FieldDiscriminatorNode>, // Not sure if needed yet
}

#[derive(Debug, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct DefinedTypeNode {
    pub name: String,
    pub data: TypeNode,
}

#[derive(Debug, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct StructTypeNode {
    pub fields: Vec<StructFieldTypeNode>,
}

#[derive(Debug, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct StructFieldTypeNode {
    pub name: String,
    #[serde(rename = "type")]
    pub field_type: TypeNode,
}

#[derive(Debug, Deserialize, Serialize)]
#[serde(rename_all = "camelCase", tag = "kind")]
pub enum TypeNode {
    NumberTypeNode {
        format: String,
        endian: String,
    },
    PublicKeyTypeNode,
    BooleanTypeNode {
        size: NumberTypeNode,
    },
    FixedSizeTypeNode {
        size: usize,
        r#type: Box<TypeNode>,
    },
    OptionTypeNode {
        item: Box<TypeNode>,
        prefix: NumberTypeNode,
    },
    DefinedTypeLinkNode {
        name: String,
    },
    BytesTypeNode,
    SizePrefixTypeNode {
        r#type: Box<TypeNode>,
        prefix: NumberTypeNode,
    },
    StringTypeNode {
        encoding: String,
    },
    StructTypeNode {
        fields: Vec<StructFieldTypeNode>,
    },
    EnumTypeNode {
        variants: Vec<EnumVariantTypeNode>,
    },
}

#[derive(Debug, Deserialize, Serialize)]
#[serde(rename_all = "camelCase", tag = "kind")]
pub enum ValueNode {
    BytesValueNode { data: String, encoding: String },
}

#[derive(Debug, Deserialize, Serialize)]
#[serde(rename_all = "camelCase", tag = "kind")]
pub enum EnumVariantTypeNode {
    EnumEmptyVariantTypeNode {
        name: String,
    },
    EnumStructVariantTypeNode {
        name: String,
        fields: Vec<StructFieldTypeNode>,
    },
    EnumTupleVariantTypeNode {
        name: String,
        items: Vec<TypeNode>,
    },
}

#[derive(Debug, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct NumberTypeNode {
    pub format: String,
    pub endian: String,
}

#[derive(Debug, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct FieldDiscriminatorNode {
    pub name: String,
    pub offset: usize,
}

#[derive(Debug, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct InstructionAccountNode {
    pub name: String,
    pub is_writable: bool,
    pub is_signer: bool,
}

#[derive(Debug, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct InstructionArgumentNode {
    pub name: String,
    #[serde(rename = "type")]
    pub arg_type: TypeNode,
    pub default_value: Option<DefaultValue>,
}

#[derive(Debug, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct DefaultValue {
    pub kind: ValueNode,
    pub data: Option<String>,
}

// ----------------------- Decoder Types -----------------------

#[derive(Debug)]
pub struct AccountData {
    pub struct_name: String,
    pub module_name: String,
    pub discriminator: String,
    pub fields: Vec<FieldData>,
    pub requires_imports: bool,
}

#[derive(Debug)]
pub struct InstructionData {
    pub struct_name: String,
    pub module_name: String,
    pub discriminator: String,
    pub args: Vec<ArgumentData>,
    pub accounts: Vec<AccountMetaData>,
    pub requires_imports: bool,
}

#[derive(Debug)]
pub struct TypeData {
    pub name: String,
    pub fields: Vec<FieldData>,
    pub kind: TypeKind,
    pub requires_imports: bool,
}

#[derive(Debug, PartialEq, Eq)]
pub enum TypeKind {
    Struct,
    Enum(Vec<EnumVariantData>),
}

#[derive(Debug, PartialEq, Eq)]
pub struct FieldData {
    pub name: String,
    pub rust_type: String,
    pub is_pubkey: bool,
    pub attributes: Option<String>,
}

#[derive(Debug, PartialEq, Eq)]
pub struct EnumVariantData {
    pub name: String,
    pub fields: Option<EnumVariantFields>,
}

#[derive(Debug, PartialEq, Eq)]
pub enum EnumVariantFields {
    Named(Vec<FieldData>),
    Unnamed(Vec<String>),
}

#[derive(Debug)]
pub struct ArgumentData {
    pub name: String,
    pub rust_type: String,
}

#[derive(Debug)]
pub struct AccountMetaData {
    pub name: String,
    pub is_mut: bool,
    pub is_signer: bool,
}
