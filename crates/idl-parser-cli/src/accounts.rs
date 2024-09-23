use sha2::{Digest, Sha256};

use crate::{idl::Idl, util::idl_type_to_rust_type};

// I plan to use these for templates

#[allow(dead_code)]
#[derive(Debug)]
pub struct AccountData {
    struct_name: String,
    discriminator: String,
    fields: Vec<FieldData>,
}

#[allow(dead_code)]
#[derive(Debug)]
pub struct FieldData {
    name: String,
    rust_type: String,
}

// To get these above, I need to transform the idl accounts part, get the discrim

pub fn process_accounts(idl: &Idl) -> Vec<AccountData> {
    let mut accounts_data = Vec::new();

    for account in &idl.accounts {
        // TODO: Change these to snake case
        let struct_name = account.name.clone();
        let discriminator = compute_account_discriminator(&account.name);

        let mut fields = Vec::new();

        if let Some(ref fields_vec) = account.type_.fields {
            for field in fields_vec {
                let rust_type = idl_type_to_rust_type(&field.type_);
                fields.push(FieldData {
                    name: field.name.clone(),
                    rust_type,
                });
            }
        }

        accounts_data.push(AccountData {
            struct_name,
            discriminator,
            fields,
        });
    }

    accounts_data
}

fn compute_account_discriminator(account_name: &str) -> String {
    let mut hasher = Sha256::new();
    let discriminator_input = format!("account:{}", account_name);
    hasher.update(discriminator_input.as_bytes());
    let hash = hasher.finalize();
    let discriminator_bytes = &hash[..8];
    format!("0x{}", hex::encode(discriminator_bytes))
}
