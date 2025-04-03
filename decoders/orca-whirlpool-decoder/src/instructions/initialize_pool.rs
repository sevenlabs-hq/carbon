use {
    super::super::types::*,
    carbon_core::{borsh, CarbonDeserialize},
};

#[derive(
    CarbonDeserialize, Debug, serde::Serialize, serde::Deserialize, PartialEq, Eq, Clone, Hash,
)]
#[carbon(discriminator = "0x5fb40aac54aee828")]
pub struct InitializePool {
    pub bumps: WhirlpoolBumps,
    pub tick_spacing: u16,
    pub initial_sqrt_price: u128,
}

pub struct InitializePoolInstructionAccounts {
    pub whirlpools_config: solana_pubkey::Pubkey,
    pub token_mint_a: solana_pubkey::Pubkey,
    pub token_mint_b: solana_pubkey::Pubkey,
    pub funder: solana_pubkey::Pubkey,
    pub whirlpool: solana_pubkey::Pubkey,
    pub token_vault_a: solana_pubkey::Pubkey,
    pub token_vault_b: solana_pubkey::Pubkey,
    pub fee_tier: solana_pubkey::Pubkey,
    pub token_program: solana_pubkey::Pubkey,
    pub system_program: solana_pubkey::Pubkey,
    pub rent: solana_pubkey::Pubkey,
}

impl carbon_core::deserialize::ArrangeAccounts for InitializePool {
    type ArrangedAccounts = InitializePoolInstructionAccounts;

    fn arrange_accounts(
        accounts: &[solana_instruction::AccountMeta],
    ) -> Option<Self::ArrangedAccounts> {
        let [whirlpools_config, token_mint_a, token_mint_b, funder, whirlpool, token_vault_a, token_vault_b, fee_tier, token_program, system_program, rent, _remaining @ ..] =
            accounts
        else {
            return None;
        };

        Some(InitializePoolInstructionAccounts {
            whirlpools_config: whirlpools_config.pubkey,
            token_mint_a: token_mint_a.pubkey,
            token_mint_b: token_mint_b.pubkey,
            funder: funder.pubkey,
            whirlpool: whirlpool.pubkey,
            token_vault_a: token_vault_a.pubkey,
            token_vault_b: token_vault_b.pubkey,
            fee_tier: fee_tier.pubkey,
            token_program: token_program.pubkey,
            system_program: system_program.pubkey,
            rent: rent.pubkey,
        })
    }
}
