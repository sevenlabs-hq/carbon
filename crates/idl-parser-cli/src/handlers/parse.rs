use crate::{
    accounts::process_accounts, commands::ParseOptions, instructions::process_instructions,
    util::read_idl,
};
use anyhow::{bail, Result};

pub fn parse(options: ParseOptions) -> Result<()> {
    let idl = if let Some(idl_path) = options.idl {
        read_idl(&idl_path)?
    } else {
        bail!("No idl file provided");
    };

    let accounts_data = process_accounts(&idl);
    let instructions_data = process_instructions(&idl);

    // Output is long so either or

    // println!("accounts data: {:#?}", accounts_data);
    println!("instructions data: {:#?}", instructions_data);

    Ok(())
}
