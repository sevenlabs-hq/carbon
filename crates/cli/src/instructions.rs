use crate::{
    events::EventData,
    idl::Idl,
    legacy_idl::{LegacyIdl, LegacyIdlInstructionDiscriminant},
    util::idl_type_to_rust_type,
};
use askama::Template;
use heck::{ToSnekCase, ToUpperCamelCase};
use sha2::{Digest, Sha256};

#[allow(dead_code)]
#[derive(Debug)]
pub struct InstructionData {
    pub struct_name: String,
    pub module_name: String,
    pub discriminator: String,
    pub args: Vec<ArgumentData>,
    pub accounts: Vec<AccountMetaData>,
    pub requires_imports: bool,
}

#[allow(dead_code)]
#[derive(Debug)]
pub struct ArgumentData {
    pub name: String,
    pub rust_type: String,
}

#[allow(dead_code)]
#[derive(Debug)]
pub struct AccountMetaData {
    pub name: String,
    pub is_mut: bool,
    pub is_signer: bool,
    pub is_optional: bool,
}

#[derive(Template)]
#[template(path = "instructions_struct.askama", escape = "none", ext = ".askama")]
pub struct InstructionsStructTemplate<'a> {
    pub instruction: &'a InstructionData,
}

#[derive(Template)]
#[template(path = "instructions_mod.askama", escape = "none", ext = ".askama")]
pub struct InstructionsModTemplate<'a> {
    pub instructions: &'a Vec<InstructionData>,
    pub decoder_name: String,
    pub program_instruction_enum: String,
    pub events: &'a Vec<EventData>,
}

pub fn legacy_process_instructions(idl: &LegacyIdl) -> Vec<InstructionData> {
    let mut instructions_data = Vec::new();

    for instruction in &idl.instructions {
        let mut requires_imports = false;
        let module_name = instruction.name.to_snek_case();
        let struct_name = instruction.name.to_upper_camel_case();
        let discriminator = legacy_compute_instruction_discriminator(
            &instruction.name.to_snek_case(),
            instruction.discriminant.as_ref(),
        );

        let mut args = Vec::new();
        for arg in &instruction.args {
            let rust_type = idl_type_to_rust_type(&arg.type_);
            if rust_type.1 {
                requires_imports = true;
            }
            args.push(ArgumentData {
                name: arg.name.to_snek_case(),
                rust_type: rust_type.0,
            });
        }

        let mut accounts = Vec::new();
        for account in &instruction.accounts {
            accounts.push(AccountMetaData {
                name: account.name.to_snek_case(),
                is_mut: account.is_mut,
                is_signer: account.is_signer,
                is_optional: account.is_optional.unwrap_or(false),
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

pub fn process_instructions(idl: &Idl) -> Vec<InstructionData> {
    let mut instructions_data = Vec::new();

    for instruction in &idl.instructions {
        let mut requires_imports = false;
        let module_name = instruction.name.to_snek_case();
        let struct_name = instruction.name.to_upper_camel_case();
        let discriminator = compute_instruction_discriminator(&instruction.discriminator);

        let mut args = Vec::new();
        for arg in &instruction.args {
            let rust_type = idl_type_to_rust_type(&arg.type_);
            if rust_type.1 {
                requires_imports = true;
            }
            args.push(ArgumentData {
                name: arg.name.to_snek_case(),
                rust_type: rust_type.0,
            });
        }

        let mut accounts = Vec::new();
        for account in &instruction.accounts {
            accounts.push(AccountMetaData {
                name: account.name.to_snek_case(),
                is_mut: account.writable.unwrap_or(false),
                is_signer: account.signer.unwrap_or(false),
                // TODO: Check
                is_optional: false,
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

fn legacy_compute_instruction_discriminator(
    instruction_name: &str,
    option_discriminant: Option<&LegacyIdlInstructionDiscriminant>,
) -> String {
    if let Some(discriminant) = option_discriminant {
        let disc = format!("0x{}", hex::encode(discriminant.value.to_be_bytes()));
        return disc;
    } else {
        let mut hasher = Sha256::new();
        let discriminator_input = format!("global:{}", instruction_name);
        hasher.update(discriminator_input.as_bytes());
        let hash = hasher.finalize();
        let discriminator_bytes = &hash[..8];
        format!("0x{}", hex::encode(discriminator_bytes))
    }
}

fn compute_instruction_discriminator(bytes: &[u8]) -> String {
    format!("0x{}", hex::encode(bytes))
}
