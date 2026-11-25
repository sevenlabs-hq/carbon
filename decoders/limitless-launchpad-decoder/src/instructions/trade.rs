use crate::types::{BuyParams, SellParams};
use borsh::{BorshDeserialize, BorshSerialize};
use carbon_core::{account_utils::next_account, deserialize::ArrangeAccounts};
use solana_instruction::AccountMeta;
use solana_pubkey::Pubkey;

pub const BUY_DISCRIMINATOR: [u8; 8] = [102, 6, 61, 18, 1, 218, 235, 234];
pub const SELL_DISCRIMINATOR: [u8; 8] = [51, 230, 133, 164, 1, 127, 131, 173];

#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[derive(Debug, Clone, Copy, PartialEq, Eq, BorshSerialize, BorshDeserialize)]
pub struct Buy {
    pub params: BuyParams,
}

#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct BuyInstructionAccounts {
    pub buyer: Pubkey,
    pub config: Pubkey,
    pub launched_coin: Pubkey,
    pub mint: Pubkey,
    pub vault_token_account: Pubkey,
    pub buyer_token_account: Pubkey,
    pub treasury: Pubkey,
    pub creator_fee_recipient: Pubkey,
    pub rewards_pool: Pubkey,
    pub charity_wallet: Pubkey,
    pub token_program: Pubkey,
    pub associated_token_program: Pubkey,
    pub system_program: Pubkey,
    pub remaining: Vec<AccountMeta>,
}

impl Buy {
    pub fn decode(data: &[u8]) -> Option<Self> {
        if data.len() < 8 || data[..8] != BUY_DISCRIMINATOR {
            return None;
        }
        let mut slice = &data[8..];
        Self::deserialize(&mut slice).ok()
    }
}

impl ArrangeAccounts for Buy {
    type ArrangedAccounts = BuyInstructionAccounts;

    fn arrange_accounts(accounts: &[AccountMeta]) -> Option<Self::ArrangedAccounts> {
        let mut iter = accounts.iter();
        Some(BuyInstructionAccounts {
            buyer: next_account(&mut iter)?,
            config: next_account(&mut iter)?,
            launched_coin: next_account(&mut iter)?,
            mint: next_account(&mut iter)?,
            vault_token_account: next_account(&mut iter)?,
            buyer_token_account: next_account(&mut iter)?,
            treasury: next_account(&mut iter)?,
            creator_fee_recipient: next_account(&mut iter)?,
            rewards_pool: next_account(&mut iter)?,
            charity_wallet: next_account(&mut iter)?,
            token_program: next_account(&mut iter)?,
            associated_token_program: next_account(&mut iter)?,
            system_program: next_account(&mut iter)?,
            remaining: iter.as_slice().to_vec(),
        })
    }
}

#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[derive(Debug, Clone, Copy, PartialEq, Eq, BorshSerialize, BorshDeserialize)]
pub struct Sell {
    pub params: SellParams,
}

#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct SellInstructionAccounts {
    pub seller: Pubkey,
    pub config: Pubkey,
    pub launched_coin: Pubkey,
    pub mint: Pubkey,
    pub vault_token_account: Pubkey,
    pub seller_token_account: Pubkey,
    pub treasury: Pubkey,
    pub creator_fee_recipient: Pubkey,
    pub rewards_pool: Pubkey,
    pub charity_wallet: Pubkey,
    pub token_program: Pubkey,
    pub associated_token_program: Pubkey,
    pub system_program: Pubkey,
    pub remaining: Vec<AccountMeta>,
}

impl Sell {
    pub fn decode(data: &[u8]) -> Option<Self> {
        if data.len() < 8 || data[..8] != SELL_DISCRIMINATOR {
            return None;
        }
        let mut slice = &data[8..];
        Self::deserialize(&mut slice).ok()
    }
}

impl ArrangeAccounts for Sell {
    type ArrangedAccounts = SellInstructionAccounts;

    fn arrange_accounts(accounts: &[AccountMeta]) -> Option<Self::ArrangedAccounts> {
        let mut iter = accounts.iter();
        Some(SellInstructionAccounts {
            seller: next_account(&mut iter)?,
            config: next_account(&mut iter)?,
            launched_coin: next_account(&mut iter)?,
            mint: next_account(&mut iter)?,
            vault_token_account: next_account(&mut iter)?,
            seller_token_account: next_account(&mut iter)?,
            treasury: next_account(&mut iter)?,
            creator_fee_recipient: next_account(&mut iter)?,
            rewards_pool: next_account(&mut iter)?,
            charity_wallet: next_account(&mut iter)?,
            token_program: next_account(&mut iter)?,
            associated_token_program: next_account(&mut iter)?,
            system_program: next_account(&mut iter)?,
            remaining: iter.as_slice().to_vec(),
        })
    }
}
