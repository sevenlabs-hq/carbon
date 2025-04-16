use carbon_core::{borsh, CarbonDeserialize};

#[derive(
    CarbonDeserialize, Debug, serde::Serialize, serde::Deserialize, PartialEq, Eq, Clone, Hash,
)]
#[carbon(discriminator = "0x3121681ebd9d4f23")]
pub struct ClaimVestedToken {}

#[derive(Debug, PartialEq, Eq, Clone, Hash, serde::Serialize, serde::Deserialize)]
pub struct ClaimVestedTokenInstructionAccounts {
    pub beneficiary: solana_pubkey::Pubkey,
    pub authority: solana_pubkey::Pubkey,
    pub pool_state: solana_pubkey::Pubkey,
    pub vesting_record: solana_pubkey::Pubkey,
    pub base_vault: solana_pubkey::Pubkey,
    pub user_base_token: solana_pubkey::Pubkey,
    pub base_token_mint: solana_pubkey::Pubkey,
    pub base_token_program: solana_pubkey::Pubkey,
    pub system_program: solana_pubkey::Pubkey,
    pub associated_token_program: solana_pubkey::Pubkey,
}

impl carbon_core::deserialize::ArrangeAccounts for ClaimVestedToken {
    type ArrangedAccounts = ClaimVestedTokenInstructionAccounts;

    fn arrange_accounts(
        accounts: &[solana_instruction::AccountMeta],
    ) -> Option<Self::ArrangedAccounts> {
        let [beneficiary, authority, pool_state, vesting_record, base_vault, user_base_token, base_token_mint, base_token_program, system_program, associated_token_program, _remaining @ ..] =
            accounts
        else {
            return None;
        };

        Some(ClaimVestedTokenInstructionAccounts {
            beneficiary: beneficiary.pubkey,
            authority: authority.pubkey,
            pool_state: pool_state.pubkey,
            vesting_record: vesting_record.pubkey,
            base_vault: base_vault.pubkey,
            user_base_token: user_base_token.pubkey,
            base_token_mint: base_token_mint.pubkey,
            base_token_program: base_token_program.pubkey,
            system_program: system_program.pubkey,
            associated_token_program: associated_token_program.pubkey,
        })
    }
}
