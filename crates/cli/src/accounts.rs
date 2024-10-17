use crate::idl::Idl;
use crate::{legacy_idl::LegacyIdl, util::idl_type_to_rust_type};
use askama::Template;
use heck::ToSnekCase;
use heck::ToUpperCamelCase;
use sha2::{Digest, Sha256};

#[allow(dead_code)]
#[derive(Debug)]
pub struct AccountData {
    pub struct_name: String,
    pub module_name: String,
    pub discriminator: String,
    pub fields: Vec<FieldData>,
    pub requires_imports: bool,
}

#[allow(dead_code)]
#[derive(Debug)]
pub struct FieldData {
    pub name: String,
    pub rust_type: String,
}

#[derive(Template)]
#[template(path = "accounts_struct.askama", escape = "none", ext = ".askama")]
pub struct AccountsStructTemplate<'a> {
    pub account: &'a AccountData,
}

#[derive(Template)]
#[template(path = "accounts_mod.askama", escape = "none", ext = ".askama")]
pub struct AccountsModTemplate<'a> {
    pub accounts: &'a Vec<AccountData>,
    pub decoder_name: String,
    pub program_struct_name: String,
}

pub fn legacy_process_accounts(idl: &LegacyIdl) -> Vec<AccountData> {
    let mut accounts_data = Vec::new();

    for account in &idl.accounts {
        let mut requires_imports = false;
        let module_name = account.name.to_snek_case();
        let struct_name = account.name.to_upper_camel_case();
        // TODO: Might be a problem
        let discriminator =
            legacy_compute_account_discriminator(&account.name.to_upper_camel_case());

        let mut fields = Vec::new();

        if let Some(ref fields_vec) = account.type_.fields {
            for field in fields_vec {
                let rust_type = idl_type_to_rust_type(&field.type_);
                if rust_type.1 {
                    requires_imports = true;
                }
                fields.push(FieldData {
                    name: field.name.to_snek_case(),
                    rust_type: rust_type.0,
                });
            }
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

pub fn process_accounts(idl: &Idl) -> Vec<AccountData> {
    let mut accounts_data = Vec::new();
    let requires_imports = false;

    for account in &idl.accounts {
        let module_name = account.name.to_snek_case();
        let struct_name = account.name.to_upper_camel_case();
        let discriminator = compute_account_discriminator(&account.discriminator);

        let fields = Vec::new();

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

fn legacy_compute_account_discriminator(account_name: &str) -> String {
    let mut hasher = Sha256::new();
    let discriminator_input = format!("account:{}", account_name);
    hasher.update(discriminator_input.as_bytes());
    let hash = hasher.finalize();
    let discriminator_bytes = &hash[..8];
    format!("0x{}", hex::encode(discriminator_bytes))
}

fn compute_account_discriminator(bytes: &[u8]) -> String {
    format!("0x{}", hex::encode(bytes))
}
