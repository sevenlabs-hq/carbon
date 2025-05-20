use serde::{Deserialize, Serialize};

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
}

#[derive(Debug, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct InstructionNode {
    pub name: String,
    pub accounts: Vec<InstructionAccountNode>,
    pub arguments: Vec<InstructionArgumentNode>,
}

#[derive(Debug, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct DefinedTypeNode {
    pub name: String,
    #[serde(rename = "type")]
    pub type_node: TypeNode,
}

#[derive(Debug, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct StructTypeNode {
    pub fields: Vec<StructFieldTypeNode>,
}

#[derive(Debug, Deserialize, Serialize, Clone)]
#[serde(rename_all = "camelCase")]
pub struct StructFieldTypeNode {
    pub name: String,
    #[serde(rename = "type")]
    pub field_type: TypeNode,
    pub default_value: Option<ValueNode>,
}

#[allow(clippy::enum_variant_names)]
#[derive(Debug, Deserialize, Serialize, Clone)]
#[serde(rename_all = "camelCase", tag = "kind")]
pub enum TypeNode {
    AmountTypeNode {
        decimals: usize,
        unit: Option<String>,
        number: Box<TypeNode>,
    },
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
        size: NumberTypeNode,
    },
    SolAmountTypeNode {
        number: Box<TypeNode>,
    },
    ArrayTypeNode {
        item: Box<TypeNode>,
        count: CountNode,
    },
    RemainderOptionTypeNode {
        item: Box<TypeNode>,
    },
    HiddenPrefixTypeNode {
        r#type: Box<TypeNode>,
        prefix: Vec<ConstantValueNode>,
    },
    PreOffsetTypeNode {
        offset: usize,
        strategy: String,
        #[serde(rename = "type")]
        inner_type: Box<TypeNode>,
    },
    PostOffsetTypeNode {
        offset: usize,
        strategy: String,
        #[serde(rename = "type")]
        inner_type: Box<TypeNode>,
    },
    ZeroableOptionTypeNode {
        item: Box<TypeNode>,
        zero_value: Option<ConstantValueNode>,
    },
    MapTypeNode {
        key: Box<TypeNode>,
        value: Box<TypeNode>,
        count: CountNode,
    },
}

#[allow(clippy::enum_variant_names)]
#[derive(Debug, Deserialize, Serialize, Clone)]
#[serde(rename_all = "camelCase", tag = "kind")]
pub enum CountNode {
    FixedCountNode { value: usize },
    PrefixedCountNode { prefix: Box<TypeNode> },
    RemainderCountNode,
}

#[allow(clippy::enum_variant_names)]
#[derive(Debug, Deserialize, Serialize, Clone)]
#[serde(rename_all = "camelCase", tag = "kind")]
pub enum ValueNode {
    BytesValueNode { data: String, encoding: String },
    NumberValueNode { number: u64 },
    NoneValueNode,
    IdentityValueNode,
    BooleanValueNode { boolean: bool },
}

#[allow(clippy::enum_variant_names)]
#[derive(Debug, Deserialize, Serialize, Clone)]
#[serde(rename_all = "camelCase", tag = "kind")]
pub enum EnumVariantTypeNode {
    EnumEmptyVariantTypeNode {
        name: String,
    },
    EnumStructVariantTypeNode {
        name: String,
        #[serde(rename = "struct")]
        struct_field: Box<TypeNode>,
    },
    EnumTupleVariantTypeNode {
        name: String,
        tuple: TupleTypeNode,
    },
}

#[derive(Debug, Deserialize, Serialize, Clone)]
#[serde(rename_all = "camelCase")]
pub struct NumberTypeNode {
    pub format: String,
    pub endian: String,
}

#[derive(Debug, Deserialize, Serialize, Clone)]
#[serde(rename_all = "camelCase")]
pub struct ConstantValueNode {
    pub r#type: Box<TypeNode>,
    pub value: ValueNode,
}

#[derive(Debug, Deserialize, Serialize, Clone)]
#[serde(rename_all = "camelCase")]
pub struct TupleTypeNode {
    pub items: Vec<TypeNode>,
}

#[derive(Debug, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct FieldDiscriminatorNode {
    pub name: String,
    pub offset: usize,
}

#[derive(Debug, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct FixedCountNode {
    pub value: usize,
}

#[derive(Debug, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct InstructionAccountNode {
    pub name: String,
    pub is_writable: bool,
    pub is_signer: SignerType,
    pub is_optional: bool,
}

#[derive(Debug, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct InstructionArgumentNode {
    pub name: String,
    #[serde(rename = "type")]
    pub arg_type: TypeNode,
    pub default_value: Option<ValueNode>,
}

#[derive(Debug, Deserialize, Serialize)]
#[serde(rename_all = "lowercase")]
#[serde(untagged)]
pub enum SignerType {
    Boolean(bool),
    Either(String),
}
