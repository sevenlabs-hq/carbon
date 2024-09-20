use borsh::{BorshDeserialize, BorshSerialize};
use carbon_core::deserialization::InstructionAccounts;
use serde::Serialize;
use solana_sdk::{instruction::AccountMeta, pubkey::Pubkey};

use super::*;

#[derive(BorshSerialize, BorshDeserialize, Debug, Clone, PartialEq, Eq, Serialize)]
pub struct MeteoraSwapInstructionData {
    pub amount_in: u64,
    pub min_amount_out: u64,
}

#[allow(dead_code)]
// TODO: Missing HASH here in case we will do accounts as well (bcs of AccountMeta)
#[derive(Debug, Clone, PartialEq, Eq, Serialize)]
pub struct MeteoraSwapInstructionAccounts {
    pub _lb_pair: Pubkey,
    pub _bin_array_bitmap_extension: Pubkey,
    pub _reserve_x: Pubkey,
    pub _reserve_y: Pubkey,
    pub user_token_in: Pubkey,
    pub user_token_out: Pubkey,
    pub token_x_mint: Pubkey,
    pub token_y_mint: Pubkey,
    pub _oracle: Pubkey,
    pub host_fee_in: Pubkey,
    pub _token_x_program: Pubkey,
    pub _token_y_program: Pubkey,
    pub _event_authority: Pubkey,
    pub _program: Pubkey,
    pub remaining_accounts: Vec<AccountMeta>,
}

impl InstructionAccounts for MeteoraSwapInstructionAccounts {
    fn unpack(accounts: &[AccountMeta]) -> Result<Self, Error> {
        let mut accounts = accounts.to_vec();
        if accounts.len() < 14 {
            return Err(Error::MissingAccountInInstruction);
        };

        Ok(Self {
            _lb_pair: accounts.remove(0).pubkey,
            _bin_array_bitmap_extension: accounts.remove(0).pubkey,
            _reserve_x: accounts.remove(0).pubkey,
            _reserve_y: accounts.remove(0).pubkey,
            user_token_in: accounts.remove(0).pubkey,
            user_token_out: accounts.remove(0).pubkey,
            token_x_mint: accounts.remove(0).pubkey,
            token_y_mint: accounts.remove(0).pubkey,
            _oracle: accounts.remove(0).pubkey,
            host_fee_in: accounts.remove(0).pubkey,
            _token_x_program: accounts.remove(0).pubkey,
            _token_y_program: accounts.remove(0).pubkey,
            _event_authority: accounts.remove(0).pubkey,
            _program: accounts.remove(0).pubkey,
            remaining_accounts: accounts,
        })
    }
}

impl InstructionData for MeteoraSwapInstructionData {
    fn discriminator() -> Discriminator {
        Discriminator::EightBytes([0xf8, 0xc6, 0x9e, 0x91, 0xe1, 0x75, 0x87, 0xc8])
    }
}
