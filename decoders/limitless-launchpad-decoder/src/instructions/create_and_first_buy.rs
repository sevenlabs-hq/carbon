use crate::types::CreateAndFirstBuyParams;
use borsh::{BorshDeserialize, BorshSerialize};
use carbon_core::{account_utils::next_account, deserialize::ArrangeAccounts};
use solana_instruction::AccountMeta;
use solana_pubkey::Pubkey;

pub const CREATE_AND_FIRST_BUY_DISCRIMINATOR: [u8; 8] = [64, 187, 180, 199, 216, 75, 227, 78];

#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[derive(Debug, Clone, PartialEq, Eq, BorshSerialize, BorshDeserialize)]
pub struct CreateAndFirstBuy {
    pub params: CreateAndFirstBuyParams,
}

#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct CreateAndFirstBuyInstructionAccounts {
    pub buyer: Pubkey,
    pub creator: Pubkey,
    pub config: Pubkey,
    pub mint: Pubkey,
    pub launched_coin: Pubkey,
    pub vault_token_account: Pubkey,
    pub genie_alloc_vault_authority: Pubkey,
    pub genie_alloc_vault_token_account: Pubkey,
    pub buyer_token_account: Pubkey,
    pub treasury: Pubkey,
    pub creator_fee_recipient: Pubkey,
    pub rewards_pool: Pubkey,
    pub charity_wallet: Pubkey,
    pub token_program: Pubkey,
    pub associated_token_program: Pubkey,
    pub system_program: Pubkey,
    pub rent: Pubkey,
    pub remaining: Vec<AccountMeta>,
}

impl CreateAndFirstBuy {
    pub fn decode(data: &[u8]) -> Option<Self> {
        if data.len() < 8 || data[..8] != CREATE_AND_FIRST_BUY_DISCRIMINATOR {
            return None;
        }
        let mut slice = &data[8..];
        Self::deserialize(&mut slice).ok()
    }
}

impl ArrangeAccounts for CreateAndFirstBuy {
    type ArrangedAccounts = CreateAndFirstBuyInstructionAccounts;

    fn arrange_accounts(accounts: &[AccountMeta]) -> Option<Self::ArrangedAccounts> {
        let mut iter = accounts.iter();
        Some(CreateAndFirstBuyInstructionAccounts {
            buyer: next_account(&mut iter)?,
            creator: next_account(&mut iter)?,
            config: next_account(&mut iter)?,
            mint: next_account(&mut iter)?,
            launched_coin: next_account(&mut iter)?,
            vault_token_account: next_account(&mut iter)?,
            genie_alloc_vault_authority: next_account(&mut iter)?,
            genie_alloc_vault_token_account: next_account(&mut iter)?,
            buyer_token_account: next_account(&mut iter)?,
            treasury: next_account(&mut iter)?,
            creator_fee_recipient: next_account(&mut iter)?,
            rewards_pool: next_account(&mut iter)?,
            charity_wallet: next_account(&mut iter)?,
            token_program: next_account(&mut iter)?,
            associated_token_program: next_account(&mut iter)?,
            system_program: next_account(&mut iter)?,
            rent: next_account(&mut iter)?,
            remaining: iter.as_slice().to_vec(),
        })
    }
}
