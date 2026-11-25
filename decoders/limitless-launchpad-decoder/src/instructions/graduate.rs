use crate::types::GraduateMeteoraParams;
use borsh::{BorshDeserialize, BorshSerialize};
use carbon_core::{account_utils::next_account, deserialize::ArrangeAccounts};
use solana_instruction::AccountMeta;
use solana_pubkey::Pubkey;

pub const GRADUATE_DISCRIMINATOR: [u8; 8] = [45, 235, 225, 181, 17, 218, 64, 130];
pub const PREPARE_METEORA_GRADUATION_DISCRIMINATOR: [u8; 8] = [17, 37, 32, 121, 54, 150, 173, 255];
pub const GRADUATE_METEORA_DISCRIMINATOR: [u8; 8] = [59, 106, 120, 130, 43, 42, 139, 176];
pub const CLAIM_METEORA_FEES_DISCRIMINATOR: [u8; 8] = [21, 207, 178, 64, 130, 211, 31, 100];

#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[derive(Debug, Clone, Copy, PartialEq, Eq, BorshSerialize, BorshDeserialize)]
pub struct Graduate {}

#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct GraduateInstructionAccounts {
    pub payer: Pubkey,
    pub config: Pubkey,
    pub launched_coin: Pubkey,
    pub mint: Pubkey,
    pub vault_token_account: Pubkey,
    pub payer_token_ata: Pubkey,
    pub wsol_mint: Pubkey,
    pub payer_wsol_ata: Pubkey,
    pub raydium_program: Pubkey,
    pub raydium_amm_config: Pubkey,
    pub raydium_authority: Pubkey,
    pub raydium_pool_state: Pubkey,
    pub raydium_token_0_mint: Pubkey,
    pub raydium_token_1_mint: Pubkey,
    pub raydium_lp_mint: Pubkey,
    pub remaining: Vec<AccountMeta>,
}

impl Graduate {
    pub fn decode(data: &[u8]) -> Option<Self> {
        if data.len() < 8 || data[..8] != GRADUATE_DISCRIMINATOR {
            return None;
        }
        Some(Self {})
    }
}

impl ArrangeAccounts for Graduate {
    type ArrangedAccounts = GraduateInstructionAccounts;

    fn arrange_accounts(accounts: &[AccountMeta]) -> Option<Self::ArrangedAccounts> {
        let mut iter = accounts.iter();
        Some(GraduateInstructionAccounts {
            payer: next_account(&mut iter)?,
            config: next_account(&mut iter)?,
            launched_coin: next_account(&mut iter)?,
            mint: next_account(&mut iter)?,
            vault_token_account: next_account(&mut iter)?,
            payer_token_ata: next_account(&mut iter)?,
            wsol_mint: next_account(&mut iter)?,
            payer_wsol_ata: next_account(&mut iter)?,
            raydium_program: next_account(&mut iter)?,
            raydium_amm_config: next_account(&mut iter)?,
            raydium_authority: next_account(&mut iter)?,
            raydium_pool_state: next_account(&mut iter)?,
            raydium_token_0_mint: next_account(&mut iter)?,
            raydium_token_1_mint: next_account(&mut iter)?,
            raydium_lp_mint: next_account(&mut iter)?,
            remaining: iter.as_slice().to_vec(),
        })
    }
}

#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct MeteoraGraduationInstructionAccounts {
    pub payer: Pubkey,
    pub config: Pubkey,
    pub launched_coin: Pubkey,
    pub mint: Pubkey,
    pub creator: Pubkey,
    pub treasury: Pubkey,
    pub graduation: Pubkey,
    pub funding: Pubkey,
    pub vault_token_account: Pubkey,
    pub payer_token_ata: Pubkey,
    pub wsol_mint: Pubkey,
    pub payer_wsol_ata: Pubkey,
    pub meteora_program: Pubkey,
    pub pool_authority: Pubkey,
    pub meteora_config: Pubkey,
    pub pool: Pubkey,
    pub token_a_vault: Pubkey,
    pub token_b_vault: Pubkey,
    pub creator_position_mint: Pubkey,
    pub creator_position: Pubkey,
    pub creator_position_nft: Pubkey,
    pub partner_position_mint: Pubkey,
    pub partner_position: Pubkey,
    pub partner_position_nft: Pubkey,
    pub event_authority: Pubkey,
    pub token_program: Pubkey,
    pub token_2022_program: Pubkey,
    pub associated_token_program: Pubkey,
    pub system_program: Pubkey,
    pub remaining: Vec<AccountMeta>,
}

fn arrange_meteora_graduation_accounts(
    accounts: &[AccountMeta],
) -> Option<MeteoraGraduationInstructionAccounts> {
    let mut iter = accounts.iter();
    Some(MeteoraGraduationInstructionAccounts {
        payer: next_account(&mut iter)?,
        config: next_account(&mut iter)?,
        launched_coin: next_account(&mut iter)?,
        mint: next_account(&mut iter)?,
        creator: next_account(&mut iter)?,
        treasury: next_account(&mut iter)?,
        graduation: next_account(&mut iter)?,
        funding: next_account(&mut iter)?,
        vault_token_account: next_account(&mut iter)?,
        payer_token_ata: next_account(&mut iter)?,
        wsol_mint: next_account(&mut iter)?,
        payer_wsol_ata: next_account(&mut iter)?,
        meteora_program: next_account(&mut iter)?,
        pool_authority: next_account(&mut iter)?,
        meteora_config: next_account(&mut iter)?,
        pool: next_account(&mut iter)?,
        token_a_vault: next_account(&mut iter)?,
        token_b_vault: next_account(&mut iter)?,
        creator_position_mint: next_account(&mut iter)?,
        creator_position: next_account(&mut iter)?,
        creator_position_nft: next_account(&mut iter)?,
        partner_position_mint: next_account(&mut iter)?,
        partner_position: next_account(&mut iter)?,
        partner_position_nft: next_account(&mut iter)?,
        event_authority: next_account(&mut iter)?,
        token_program: next_account(&mut iter)?,
        token_2022_program: next_account(&mut iter)?,
        associated_token_program: next_account(&mut iter)?,
        system_program: next_account(&mut iter)?,
        remaining: iter.as_slice().to_vec(),
    })
}

#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[derive(Debug, Clone, Copy, PartialEq, Eq, BorshSerialize, BorshDeserialize)]
pub struct PrepareMeteoraGraduation {}

impl PrepareMeteoraGraduation {
    pub fn decode(data: &[u8]) -> Option<Self> {
        if data.len() < 8 || data[..8] != PREPARE_METEORA_GRADUATION_DISCRIMINATOR {
            return None;
        }
        Some(Self {})
    }
}

impl ArrangeAccounts for PrepareMeteoraGraduation {
    type ArrangedAccounts = MeteoraGraduationInstructionAccounts;

    fn arrange_accounts(accounts: &[AccountMeta]) -> Option<Self::ArrangedAccounts> {
        arrange_meteora_graduation_accounts(accounts)
    }
}

#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[derive(Debug, Clone, Copy, PartialEq, Eq, BorshSerialize, BorshDeserialize)]
pub struct GraduateMeteora {
    pub params: GraduateMeteoraParams,
}

impl GraduateMeteora {
    pub fn decode(data: &[u8]) -> Option<Self> {
        if data.len() < 8 || data[..8] != GRADUATE_METEORA_DISCRIMINATOR {
            return None;
        }
        let mut slice = &data[8..];
        Self::deserialize(&mut slice).ok()
    }
}

impl ArrangeAccounts for GraduateMeteora {
    type ArrangedAccounts = MeteoraGraduationInstructionAccounts;

    fn arrange_accounts(accounts: &[AccountMeta]) -> Option<Self::ArrangedAccounts> {
        arrange_meteora_graduation_accounts(accounts)
    }
}

#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[derive(Debug, Clone, Copy, PartialEq, Eq, BorshSerialize, BorshDeserialize)]
pub struct ClaimMeteoraFees {
    pub partner: bool,
}

#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ClaimMeteoraFeesInstructionAccounts {
    pub recipient: Pubkey,
    pub graduation: Pubkey,
    pub launched_coin: Pubkey,
    pub mint: Pubkey,
    pub wsol_mint: Pubkey,
    pub pool: Pubkey,
    pub position: Pubkey,
    pub position_mint: Pubkey,
    pub position_nft_account: Pubkey,
    pub token_a_account: Pubkey,
    pub token_b_account: Pubkey,
    pub token_a_vault: Pubkey,
    pub token_b_vault: Pubkey,
    pub pool_authority: Pubkey,
    pub event_authority: Pubkey,
    pub meteora_program: Pubkey,
    pub token_program: Pubkey,
    pub token_2022_program: Pubkey,
    pub remaining: Vec<AccountMeta>,
}

impl ClaimMeteoraFees {
    pub fn decode(data: &[u8]) -> Option<Self> {
        if data.len() < 8 || data[..8] != CLAIM_METEORA_FEES_DISCRIMINATOR {
            return None;
        }
        let mut slice = &data[8..];
        Self::deserialize(&mut slice).ok()
    }
}

impl ArrangeAccounts for ClaimMeteoraFees {
    type ArrangedAccounts = ClaimMeteoraFeesInstructionAccounts;

    fn arrange_accounts(accounts: &[AccountMeta]) -> Option<Self::ArrangedAccounts> {
        let mut iter = accounts.iter();
        Some(ClaimMeteoraFeesInstructionAccounts {
            recipient: next_account(&mut iter)?,
            graduation: next_account(&mut iter)?,
            launched_coin: next_account(&mut iter)?,
            mint: next_account(&mut iter)?,
            wsol_mint: next_account(&mut iter)?,
            pool: next_account(&mut iter)?,
            position: next_account(&mut iter)?,
            position_mint: next_account(&mut iter)?,
            position_nft_account: next_account(&mut iter)?,
            token_a_account: next_account(&mut iter)?,
            token_b_account: next_account(&mut iter)?,
            token_a_vault: next_account(&mut iter)?,
            token_b_vault: next_account(&mut iter)?,
            pool_authority: next_account(&mut iter)?,
            event_authority: next_account(&mut iter)?,
            meteora_program: next_account(&mut iter)?,
            token_program: next_account(&mut iter)?,
            token_2022_program: next_account(&mut iter)?,
            remaining: iter.as_slice().to_vec(),
        })
    }
}
