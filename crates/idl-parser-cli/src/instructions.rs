use crate::{idl::Idl, util::idl_type_to_rust_type};
use sha2::{Digest, Sha256};

#[allow(dead_code)]
#[derive(Debug)]
pub struct InstructionData {
    struct_name: String,
    discriminator: String,
    args: Vec<ArgumentData>,
    accounts: Vec<AccountMetaData>,
}

#[allow(dead_code)]
#[derive(Debug)]
pub struct ArgumentData {
    name: String,
    rust_type: String,
}

#[allow(dead_code)]
#[derive(Debug)]
pub struct AccountMetaData {
    name: String,
    is_mut: bool,
    is_signer: bool,
    is_optional: bool,
}

// I need these above for templates, below is fn for transforming

pub fn process_instructions(idl: &Idl) -> Vec<InstructionData> {
    let mut instructions_data = Vec::new();

    for instruction in &idl.instructions {
        // TODO: Change these to snake case
        let struct_name = instruction.name.clone();
        let discriminator = compute_instruction_discriminator(&instruction.name);

        let mut args = Vec::new();
        for arg in &instruction.args {
            let rust_type = idl_type_to_rust_type(&arg.type_);
            args.push(ArgumentData {
                name: arg.name.clone(),
                rust_type,
            });
        }

        let mut accounts = Vec::new();
        for account in &instruction.accounts {
            accounts.push(AccountMetaData {
                name: account.name.clone(),
                is_mut: account.is_mut,
                is_signer: account.is_signer,
                is_optional: account.is_optional.unwrap_or(false),
            });
        }

        instructions_data.push(InstructionData {
            struct_name,
            discriminator,
            args,
            accounts,
        });
    }

    instructions_data
}

fn compute_instruction_discriminator(instruction_name: &str) -> String {
    let mut hasher = Sha256::new();
    let discriminator_input = format!("global:{}", instruction_name);
    hasher.update(discriminator_input.as_bytes());
    let hash = hasher.finalize();
    let discriminator_bytes = &hash[..8]; // First 8 bytes
    format!("0x{}", hex::encode(discriminator_bytes))
}
