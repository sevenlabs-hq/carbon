use carbon_core::{borsh, CarbonDeserialize};

#[derive(
    CarbonDeserialize, Debug, serde::Serialize, serde::Deserialize, PartialEq, Eq, Clone, Hash,
)]
#[carbon(discriminator = "0x9c27d0874ced3d48")]
pub struct ClaimPlatformFee {}

#[derive(Debug, PartialEq, Eq, Clone, Hash, serde::Serialize, serde::Deserialize)]
pub struct ClaimPlatformFeeInstructionAccounts {
    pub platform_fee_wallet: solana_pubkey::Pubkey,
    pub authority: solana_pubkey::Pubkey,
    pub pool_state: solana_pubkey::Pubkey,
    pub platform_config: solana_pubkey::Pubkey,
    pub quote_vault: solana_pubkey::Pubkey,
    pub recipient_token_account: solana_pubkey::Pubkey,
    pub quote_mint: solana_pubkey::Pubkey,
    pub token_program: solana_pubkey::Pubkey,
    pub system_program: solana_pubkey::Pubkey,
    pub associated_token_program: solana_pubkey::Pubkey,
}

impl carbon_core::deserialize::ArrangeAccounts for ClaimPlatformFee {
    type ArrangedAccounts = ClaimPlatformFeeInstructionAccounts;

    fn arrange_accounts(
        accounts: &[solana_instruction::AccountMeta],
    ) -> Option<Self::ArrangedAccounts> {
        let [platform_fee_wallet, authority, pool_state, platform_config, quote_vault, recipient_token_account, quote_mint, token_program, system_program, associated_token_program, _remaining @ ..] =
            accounts
        else {
            return None;
        };

        Some(ClaimPlatformFeeInstructionAccounts {
            platform_fee_wallet: platform_fee_wallet.pubkey,
            authority: authority.pubkey,
            pool_state: pool_state.pubkey,
            platform_config: platform_config.pubkey,
            quote_vault: quote_vault.pubkey,
            recipient_token_account: recipient_token_account.pubkey,
            quote_mint: quote_mint.pubkey,
            token_program: token_program.pubkey,
            system_program: system_program.pubkey,
            associated_token_program: associated_token_program.pubkey,
        })
    }
}
