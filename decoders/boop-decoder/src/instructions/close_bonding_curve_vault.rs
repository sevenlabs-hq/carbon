use carbon_core::{borsh, CarbonDeserialize};

#[derive(
    CarbonDeserialize, Debug, serde::Serialize, serde::Deserialize, PartialEq, Eq, Clone, Hash,
)]
#[carbon(discriminator = "0xbd47bdef71423bbd")]
pub struct CloseBondingCurveVault {}

#[derive(Debug, PartialEq, Eq, Clone, Hash, serde::Serialize, serde::Deserialize)]
pub struct CloseBondingCurveVaultInstructionAccounts {
    pub config: solana_pubkey::Pubkey,
    pub operator: solana_pubkey::Pubkey,
    pub vault_authority: solana_pubkey::Pubkey,
    pub bonding_curve: solana_pubkey::Pubkey,
    pub bonding_curve_vault: solana_pubkey::Pubkey,
    pub mint: solana_pubkey::Pubkey,
    pub recipient_token_account: solana_pubkey::Pubkey,
    pub recipient: solana_pubkey::Pubkey,
    pub token_program: solana_pubkey::Pubkey,
    pub system_program: solana_pubkey::Pubkey,
    pub associated_token_program: solana_pubkey::Pubkey,
    pub rent: solana_pubkey::Pubkey,
}

impl carbon_core::deserialize::ArrangeAccounts for CloseBondingCurveVault {
    type ArrangedAccounts = CloseBondingCurveVaultInstructionAccounts;

    fn arrange_accounts(
        accounts: &[solana_instruction::AccountMeta],
    ) -> Option<Self::ArrangedAccounts> {
        let [config, operator, vault_authority, bonding_curve, bonding_curve_vault, mint, recipient_token_account, recipient, token_program, system_program, associated_token_program, rent, _remaining @ ..] =
            accounts
        else {
            return None;
        };

        Some(CloseBondingCurveVaultInstructionAccounts {
            config: config.pubkey,
            operator: operator.pubkey,
            vault_authority: vault_authority.pubkey,
            bonding_curve: bonding_curve.pubkey,
            bonding_curve_vault: bonding_curve_vault.pubkey,
            mint: mint.pubkey,
            recipient_token_account: recipient_token_account.pubkey,
            recipient: recipient.pubkey,
            token_program: token_program.pubkey,
            system_program: system_program.pubkey,
            associated_token_program: associated_token_program.pubkey,
            rent: rent.pubkey,
        })
    }
}
