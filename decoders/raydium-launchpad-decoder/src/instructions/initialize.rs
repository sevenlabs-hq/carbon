use {
    super::super::types::*,
    carbon_core::{borsh, CarbonDeserialize},
};

#[derive(
    CarbonDeserialize, Debug, serde::Serialize, serde::Deserialize, PartialEq, Eq, Clone, Hash,
)]
#[carbon(discriminator = "0xafaf6d1f0d989bed")]
pub struct Initialize {
    pub base_mint_param: MintParams,
    pub curve_param: CurveParams,
    pub vesting_param: VestingParams,
}

#[derive(Debug, PartialEq, Eq, Clone, Hash, serde::Serialize, serde::Deserialize)]
pub struct InitializeInstructionAccounts {
    pub payer: solana_pubkey::Pubkey,
    pub creator: solana_pubkey::Pubkey,
    pub global_config: solana_pubkey::Pubkey,
    pub platform_config: solana_pubkey::Pubkey,
    pub authority: solana_pubkey::Pubkey,
    pub pool_state: solana_pubkey::Pubkey,
    pub base_mint: solana_pubkey::Pubkey,
    pub quote_mint: solana_pubkey::Pubkey,
    pub base_vault: solana_pubkey::Pubkey,
    pub quote_vault: solana_pubkey::Pubkey,
    pub metadata_account: solana_pubkey::Pubkey,
    pub base_token_program: solana_pubkey::Pubkey,
    pub quote_token_program: solana_pubkey::Pubkey,
    pub metadata_program: solana_pubkey::Pubkey,
    pub system_program: solana_pubkey::Pubkey,
    pub rent_program: solana_pubkey::Pubkey,
    pub event_authority: solana_pubkey::Pubkey,
    pub program: solana_pubkey::Pubkey,
}

impl carbon_core::deserialize::ArrangeAccounts for Initialize {
    type ArrangedAccounts = InitializeInstructionAccounts;

    fn arrange_accounts(
        accounts: &[solana_instruction::AccountMeta],
    ) -> Option<Self::ArrangedAccounts> {
        let [payer, creator, global_config, platform_config, authority, pool_state, base_mint, quote_mint, base_vault, quote_vault, metadata_account, base_token_program, quote_token_program, metadata_program, system_program, rent_program, event_authority, program, _remaining @ ..] =
            accounts
        else {
            return None;
        };

        Some(InitializeInstructionAccounts {
            payer: payer.pubkey,
            creator: creator.pubkey,
            global_config: global_config.pubkey,
            platform_config: platform_config.pubkey,
            authority: authority.pubkey,
            pool_state: pool_state.pubkey,
            base_mint: base_mint.pubkey,
            quote_mint: quote_mint.pubkey,
            base_vault: base_vault.pubkey,
            quote_vault: quote_vault.pubkey,
            metadata_account: metadata_account.pubkey,
            base_token_program: base_token_program.pubkey,
            quote_token_program: quote_token_program.pubkey,
            metadata_program: metadata_program.pubkey,
            system_program: system_program.pubkey,
            rent_program: rent_program.pubkey,
            event_authority: event_authority.pubkey,
            program: program.pubkey,
        })
    }
}
