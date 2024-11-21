use carbon_core::{borsh, CarbonDeserialize};
#[derive(
    CarbonDeserialize, Debug, serde::Serialize, serde::Deserialize, PartialEq, Eq, Clone, Hash,
)]
#[carbon(discriminator = "0xa9dc29fa23be85c6")]
pub struct CremaSwap {}

pub struct CremaSwapInstructionAccounts {
    pub swap_program: solana_sdk::pubkey::Pubkey,
    pub clmm_config: solana_sdk::pubkey::Pubkey,
    pub clmmpool: solana_sdk::pubkey::Pubkey,
    pub token_a: solana_sdk::pubkey::Pubkey,
    pub token_b: solana_sdk::pubkey::Pubkey,
    pub account_a: solana_sdk::pubkey::Pubkey,
    pub account_b: solana_sdk::pubkey::Pubkey,
    pub token_a_vault: solana_sdk::pubkey::Pubkey,
    pub token_b_vault: solana_sdk::pubkey::Pubkey,
    pub tick_array_map: solana_sdk::pubkey::Pubkey,
    pub owner: solana_sdk::pubkey::Pubkey,
    pub partner: solana_sdk::pubkey::Pubkey,
    pub partner_ata_a: solana_sdk::pubkey::Pubkey,
    pub partner_ata_b: solana_sdk::pubkey::Pubkey,
    pub token_program: solana_sdk::pubkey::Pubkey,
}

impl carbon_core::deserialize::ArrangeAccounts for CremaSwap {
    type ArrangedAccounts = CremaSwapInstructionAccounts;

fn arrange_accounts(
        accounts: Vec<solana_sdk::instruction::AccountMeta>,
    ) -> Option<Self::ArrangedAccounts> {
        let swap_program = accounts.get(0)?;
        let clmm_config = accounts.get(1)?;
        let clmmpool = accounts.get(2)?;
        let token_a = accounts.get(3)?;
        let token_b = accounts.get(4)?;
        let account_a = accounts.get(5)?;
        let account_b = accounts.get(6)?;
        let token_a_vault = accounts.get(7)?;
        let token_b_vault = accounts.get(8)?;
        let tick_array_map = accounts.get(9)?;
        let owner = accounts.get(10)?;
        let partner = accounts.get(11)?;
        let partner_ata_a = accounts.get(12)?;
        let partner_ata_b = accounts.get(13)?;
        let token_program = accounts.get(14)?;

        Some(CremaSwapInstructionAccounts {
            swap_program: swap_program.pubkey,
            clmm_config: clmm_config.pubkey,
            clmmpool: clmmpool.pubkey,
            token_a: token_a.pubkey,
            token_b: token_b.pubkey,
            account_a: account_a.pubkey,
            account_b: account_b.pubkey,
            token_a_vault: token_a_vault.pubkey,
            token_b_vault: token_b_vault.pubkey,
            tick_array_map: tick_array_map.pubkey,
            owner: owner.pubkey,
            partner: partner.pubkey,
            partner_ata_a: partner_ata_a.pubkey,
            partner_ata_b: partner_ata_b.pubkey,
            token_program: token_program.pubkey,
        })
    }
}
