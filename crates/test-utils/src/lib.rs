use serde::Deserialize;

use solana_account::Account;
use solana_instruction::{AccountMeta, Instruction};
use solana_pubkey::Pubkey;
use solana_transaction_status::UiTransactionStatusMeta;
use std::{fs, path::Path};

pub mod base58_deserialize;
mod base64_deserialize;
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

#[derive(Debug, Deserialize)]
pub struct TestAccount {
    #[serde(with = "base64_deserialize")]
    pub data: Vec<u8>,
    pub executable: bool,
    pub lamports: u64,
    #[serde(with = "field_as_string")]
    pub owner: Pubkey,
    pub rent_epoch: u64,
}

impl From<TestAccount> for Account {
    fn from(val: TestAccount) -> Self {
        Account {
            data: val.data,
            lamports: val.lamports,
            owner: val.owner,
            executable: val.executable,
            rent_epoch: val.rent_epoch,
        }
    }
}

pub fn read_transaction_meta<P: AsRef<Path>>(
    tx_path: P,
) -> anyhow::Result<UiTransactionStatusMeta> {
    let data = fs::read(tx_path).map_err(|e| anyhow::anyhow!("Couldn't read fixture: {e}"))?;

    let tx_status_meta = serde_json::from_slice::<UiTransactionStatusMeta>(&data)
        .map_err(|e| anyhow::anyhow!("Couldn't deserialize fixture: {e}"))?;

    Ok(tx_status_meta)
}

pub fn read_instruction<P: AsRef<Path>>(ix_path: P) -> anyhow::Result<Instruction> {
    let data = fs::read(ix_path).map_err(|e| anyhow::anyhow!("Couldn't read fixture: {e}"))?;

    let ix = serde_json::from_slice::<TestInstruction>(&data)
        .map_err(|e| anyhow::anyhow!("Couldn't deserialize fixture: {e}"))?;

    Ok(ix.into())
}

pub fn read_account<P: AsRef<Path>>(acc_path: P) -> anyhow::Result<Account> {
    let data = fs::read(acc_path).map_err(|e| anyhow::anyhow!("Couldn't read fixture: {e}"))?;

    let acc = serde_json::from_slice::<TestAccount>(&data)
        .map_err(|e| anyhow::anyhow!("Couldn't deserialize fixture: {e}"))?;

    Ok(acc.into())
}
