use {
    super::{
        types::{EnumVariantTypeNode, ProgramNode, SignerType, TypeNode},
        utils::{
            get_account_discriminator, get_event_discriminator, get_instruction_discriminator,
            map_type, resolve_struct_type,
        },
    },
    crate::{
        accounts::{AccountData, FieldData as AccountFieldData},
        events::EventData,
        instructions::{AccountMetaData, ArgumentData, InstructionData},
        types::{EnumVariantData, EnumVariantFields, FieldData, TypeData, TypeKind},
    },
    heck::{ToSnakeCase, ToUpperCamelCase},
    std::collections::HashSet,
};

pub fn process_codama_accounts(program: &ProgramNode) -> Vec<AccountData> {
    let mut accounts_data = Vec::new();

    for account in &program.accounts {
        let mut requires_imports = false;

        let struct_name = account.name.to_upper_camel_case();
        let module_name = account.name.to_snake_case();
        let discriminator = get_account_discriminator(account, &account.name);

        let mut fields = Vec::new();
        for field in &account.data.fields {
            if field.name == "discriminator" {
                continue;
            }
            let rust_type = map_type(&field.field_type);
            if rust_type.1 {
                requires_imports = true;
            }

            fields.push(AccountFieldData {
                name: field.name.to_snake_case(),
                rust_type: rust_type.0,
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
        let module_name = instruction.name.to_snake_case();
        let discriminator =
            get_instruction_discriminator(&instruction.arguments, &instruction.name);

        let mut args = Vec::new();
        for arg in &instruction.arguments {
            if arg.name == "discriminator" {
                continue;
            }
            let rust_type = map_type(&arg.arg_type);
            if rust_type.1 {
                requires_imports = true;
            }
            args.push(ArgumentData {
                name: arg.name.to_snake_case(),
                rust_type: rust_type.0,
            });
        }

        let mut accounts = Vec::new();
        for account in &instruction.accounts {
            accounts.push(AccountMetaData {
                name: account.name.to_snake_case(),
                is_mut: account.is_writable,
                is_signer: match account.is_signer {
                    SignerType::Boolean(is_signer) => is_signer,
                    SignerType::Either(_) => false,
                },
                is_optional: account.is_optional,
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

pub fn process_codama_defined_types(
    program: &ProgramNode,
    event_hints: &HashSet<String>,
) -> (Vec<TypeData>, Vec<EventData>) {
    let mut types_data = Vec::new();
    let mut events_data = Vec::new();

    for defined_type in &program.defined_types {
        let mut requires_imports = false;
        let name = defined_type.name.to_upper_camel_case().clone();
        let mut fields = Vec::new();
        let mut kind = TypeKind::Struct;

        match &defined_type.type_node {
            TypeNode::StructTypeNode {
                fields: struct_fields,
            } => {
                for field in struct_fields {
                    let rust_type = map_type(&field.field_type);
                    if rust_type.1 {
                        requires_imports = true;
                    }
                    let is_pubkey = rust_type.0 == "Pubkey";
                    fields.push(FieldData {
                        name: field.name.to_snake_case(),
                        rust_type: rust_type.0,
                        is_pubkey,
                        attributes: None,
                    });
                }
            }
            TypeNode::EnumTypeNode { variants, .. } => {
                kind = TypeKind::Enum(
                    variants
                        .iter()
                        .map(|variant| match variant {
                            EnumVariantTypeNode::EnumEmptyVariantTypeNode { name } => {
                                EnumVariantData {
                                    name: name.to_upper_camel_case().clone(),
                                    fields: None,
                                }
                            }
                            EnumVariantTypeNode::EnumStructVariantTypeNode {
                                name,
                                struct_field,
                            } => {
                                let resolved_struct = resolve_struct_type(struct_field);
                                let named_fields = match resolved_struct {
                                    Some(struct_node) => struct_node
                                        .fields
                                        .iter()
                                        .map(|field| {
                                            let rust_type = map_type(&field.field_type);
                                            if rust_type.1 {
                                                requires_imports = true;
                                            }
                                            FieldData {
                                                name: field.name.to_snake_case(),
                                                rust_type: rust_type.0.clone(),
                                                is_pubkey: rust_type.0 == "Pubkey",
                                                attributes: None,
                                            }
                                        })
                                        .collect(),
                                    None => {
                                        println!(
                                            "Warning: Failed to resolve struct fields for enum variant `{}`",
                                            name
                                        );
                                        Vec::new()
                                    }
                                };

                                EnumVariantData {
                                    name: name.to_upper_camel_case().clone(),
                                    fields: Some(EnumVariantFields::Named(named_fields)),
                                }
                            }
                            EnumVariantTypeNode::EnumTupleVariantTypeNode { name, tuple } => {
                                let unnamed_fields = tuple
                                    .items
                                    .iter()
                                    .map(|item| {
                                        let rust_type = map_type(item);
                                        if rust_type.1 {
                                            requires_imports = true;
                                        }
                                        rust_type.0.clone()
                                    })
                                    .collect();
                                EnumVariantData {
                                    name: name.to_upper_camel_case().clone(),
                                    fields: Some(EnumVariantFields::Unnamed(unnamed_fields)),
                                }
                            }
                        })
                        .collect(),
                );
            }
            _ => continue, // Skip unsupported type nodes for now.
        }

        let module_name = name.to_snake_case();
        let struct_name = name.to_upper_camel_case();

        if event_hints.contains(&name) {
            let discriminator = get_event_discriminator(&name);
            let args = fields
                .into_iter()
                .map(|f| crate::events::ArgumentData {
                    name: f.name,
                    rust_type: f.rust_type,
                })
                .collect();

            let event = EventData {
                struct_name,
                module_name,
                discriminator,
                args,
                requires_imports,
            };

            events_data.push(event);
        } else {
            types_data.push(TypeData {
                name,
                fields,
                kind,
                requires_imports,
            });
        }
    }

    (types_data, events_data)
}
