use super::{
    types::{
        AccountData, AccountMetaData, ArgumentData, EnumVariantData, EnumVariantFields,
        EnumVariantTypeNode, FieldData, InstructionData, ProgramNode, TypeData, TypeKind, TypeNode,
    },
    utils::{get_instruction_discriminator, map_type},
};
use heck::{ToSnekCase, ToUpperCamelCase};

pub fn process_codama_accounts(program: &ProgramNode) -> Vec<AccountData> {
    let mut accounts_data = Vec::new();

    for account in &program.accounts {
        let mut requires_imports = false;

        let struct_name = account.name.to_upper_camel_case();
        let module_name = account.name.to_snek_case();
        let discriminator = "".to_string(); // TODO: get discriminator as from ixs

        let mut fields = Vec::new();
        for field in &account.data.fields {
            let rust_type = map_type(&field.field_type);
            if rust_type.contains("Pubkey") {
                requires_imports = true;
            }

            let is_pubkey = rust_type == "Pubkey";
            fields.push(FieldData {
                name: field.name.to_snek_case(),
                rust_type,
                is_pubkey,
                attributes: None,
            });
        }

        accounts_data.push(AccountData {
            struct_name,
            module_name,
            discriminator,
            fields,
            requires_imports,
        });
    }

    accounts_data
}

pub fn process_codama_instructions(program: &ProgramNode) -> Vec<InstructionData> {
    let mut instructions_data = Vec::new();

    for instruction in &program.instructions {
        let mut requires_imports = false;

        let struct_name = instruction.name.to_upper_camel_case();
        let module_name = instruction.name.to_snek_case();
        let discriminator =
            get_instruction_discriminator(&instruction.arguments, &instruction.name);

        let mut args = Vec::new();
        for arg in &instruction.arguments {
            let rust_type = map_type(&arg.arg_type);
            if rust_type.contains("Pubkey") {
                requires_imports = true;
            }
            args.push(ArgumentData {
                name: arg.name.to_snek_case(),
                rust_type,
            });
        }

        let mut accounts = Vec::new();
        for account in &instruction.accounts {
            accounts.push(AccountMetaData {
                name: account.name.to_snek_case(),
                is_mut: account.is_writable,
                is_signer: account.is_signer,
            });
        }

        instructions_data.push(InstructionData {
            struct_name,
            module_name,
            discriminator,
            args,
            accounts,
            requires_imports,
        });
    }

    instructions_data
}

pub fn process_codama_defined_types(program: &ProgramNode) -> Vec<TypeData> {
    let mut types_data = Vec::new();

    for defined_type in &program.defined_types {
        let mut requires_imports = false;

        let name = defined_type.name.clone();
        let mut fields = Vec::new();
        let mut kind = TypeKind::Struct;

        match &defined_type.data {
            TypeNode::StructTypeNode {
                fields: struct_fields,
            } => {
                for field in struct_fields {
                    let rust_type = map_type(&field.field_type);
                    if rust_type.contains("Pubkey") {
                        requires_imports = true;
                    }
                    let is_pubkey = rust_type == "Pubkey";
                    fields.push(FieldData {
                        name: field.name.to_snek_case(),
                        rust_type,
                        is_pubkey,
                        attributes: None,
                    });
                }
            }
            TypeNode::EnumTypeNode { variants } => {
                kind = TypeKind::Enum(
                    variants
                        .iter()
                        .map(|variant| match variant {
                            EnumVariantTypeNode::EnumEmptyVariantTypeNode { name } => {
                                EnumVariantData {
                                    name: name.clone(),
                                    fields: None,
                                }
                            }
                            EnumVariantTypeNode::EnumStructVariantTypeNode { name, fields } => {
                                let named_fields = fields
                                    .iter()
                                    .map(|field| {
                                        let rust_type = map_type(&field.field_type);
                                        FieldData {
                                            name: field.name.to_snek_case(),
                                            rust_type: rust_type.clone(),
                                            is_pubkey: rust_type == "Pubkey",
                                            attributes: None,
                                        }
                                    })
                                    .collect();
                                EnumVariantData {
                                    name: name.clone(),
                                    fields: Some(EnumVariantFields::Named(named_fields)),
                                }
                            }
                            EnumVariantTypeNode::EnumTupleVariantTypeNode { name, items } => {
                                let unnamed_fields = items.iter().map(map_type).collect();
                                EnumVariantData {
                                    name: name.clone(),
                                    fields: Some(EnumVariantFields::Unnamed(unnamed_fields)),
                                }
                            }
                        })
                        .collect(),
                );
            }
            _ => continue, // Skip unsupported type nodes for now.
        }

        types_data.push(TypeData {
            name,
            fields,
            kind,
            requires_imports,
        });
    }

    types_data
}
