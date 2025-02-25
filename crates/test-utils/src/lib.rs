use serde::Deserialize;
use solana_sdk::{
    instruction::{AccountMeta, Instruction},
    pubkey::Pubkey,
};
use std::{fs, path::Path};

mod field_as_string;
mod hex_deserialize;

#[derive(Debug, Deserialize)]
pub struct TestAccountMeta {
    #[serde(with = "field_as_string")]
    pub pubkey: Pubkey,
    pub is_signer: bool,
    pub is_writable: bool,
}

impl From<TestAccountMeta> for AccountMeta {
    fn from(val: TestAccountMeta) -> Self {
        AccountMeta {
            pubkey: val.pubkey,
            is_signer: val.is_signer,
            is_writable: val.is_writable,
        }
    }
}

#[derive(Debug, Deserialize)]
pub struct TestInstruction {
    #[serde(with = "field_as_string")]
    pub program_id: Pubkey,
    pub accounts: Vec<TestAccountMeta>,
    #[serde(with = "hex_deserialize")]
    pub data: Vec<u8>,
}

impl From<TestInstruction> for Instruction {
    fn from(val: TestInstruction) -> Self {
        Instruction {
            program_id: val.program_id,
            accounts: val.accounts.into_iter().map(Into::into).collect(),
            data: val.data,
        }
    }
}

pub fn read_instruction<P: AsRef<Path>>(ix_path: P) -> anyhow::Result<Instruction> {
    let data = fs::read(ix_path).map_err(|e| anyhow::anyhow!("Couldn't read fixture: {e}"))?;

    let ix = serde_json::from_slice::<TestInstruction>(&data)
        .map_err(|e| anyhow::anyhow!("Couldn't deserialize fixture: {e}"))?;

    Ok(ix.into())
}
