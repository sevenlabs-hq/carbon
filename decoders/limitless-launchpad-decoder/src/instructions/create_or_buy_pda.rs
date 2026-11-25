use crate::types::{CreateAndFirstBuyParams, PdaVanityFee};
use borsh::{BorshDeserialize, BorshSerialize};
use carbon_core::{account_utils::next_account, deserialize::ArrangeAccounts};
use solana_instruction::AccountMeta;
use solana_pubkey::Pubkey;

pub const CREATE_OR_BUY_PDA_DISCRIMINATOR: [u8; 8] = [234, 252, 139, 234, 4, 113, 139, 190];
pub const CREATE_OR_BUY_PDA_WITH_VANITY_FEE_DISCRIMINATOR: [u8; 8] =
    [67, 255, 122, 120, 237, 55, 32, 191];

#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[derive(Debug, Clone, PartialEq, Eq, BorshSerialize, BorshDeserialize)]
pub struct CreateOrBuyPda {
    pub params: CreateAndFirstBuyParams,
    pub nonce: u64,
}

#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[derive(Debug, Clone, PartialEq, Eq, BorshSerialize, BorshDeserialize)]
pub struct CreateOrBuyPdaWithVanityFee {
    pub params: CreateAndFirstBuyParams,
    pub nonce: u64,
    pub fee: PdaVanityFee,
}

#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct CreateOrBuyPdaInstructionAccounts {
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
    pub launchpad_program: Pubkey,
    pub remaining: Vec<AccountMeta>,
}

fn arrange_pda_accounts(accounts: &[AccountMeta]) -> Option<CreateOrBuyPdaInstructionAccounts> {
    let mut iter = accounts.iter();
    Some(CreateOrBuyPdaInstructionAccounts {
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
        launchpad_program: next_account(&mut iter)?,
        remaining: iter.as_slice().to_vec(),
    })
}

impl CreateOrBuyPda {
    pub fn decode(data: &[u8]) -> Option<Self> {
        if data.len() < 8 || data[..8] != CREATE_OR_BUY_PDA_DISCRIMINATOR {
            return None;
        }
        let mut slice = &data[8..];
        Self::deserialize(&mut slice).ok()
    }
}

impl ArrangeAccounts for CreateOrBuyPda {
    type ArrangedAccounts = CreateOrBuyPdaInstructionAccounts;

    fn arrange_accounts(accounts: &[AccountMeta]) -> Option<Self::ArrangedAccounts> {
        arrange_pda_accounts(accounts)
    }
}

impl CreateOrBuyPdaWithVanityFee {
    pub fn decode(data: &[u8]) -> Option<Self> {
        if data.len() < 8 || data[..8] != CREATE_OR_BUY_PDA_WITH_VANITY_FEE_DISCRIMINATOR {
            return None;
        }
        let mut slice = &data[8..];
        Self::deserialize(&mut slice).ok()
    }
}

impl ArrangeAccounts for CreateOrBuyPdaWithVanityFee {
    type ArrangedAccounts = CreateOrBuyPdaInstructionAccounts;

    fn arrange_accounts(accounts: &[AccountMeta]) -> Option<Self::ArrangedAccounts> {
        arrange_pda_accounts(accounts)
    }
}
