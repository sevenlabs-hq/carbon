use carbon_core::{borsh, CarbonDeserialize};

#[derive(
    CarbonDeserialize, Debug, serde::Serialize, serde::Deserialize, PartialEq, Eq, Clone, Hash,
)]
#[carbon(discriminator = "0x2debe1b511da4082")]
pub struct Graduate {}

#[derive(Debug, PartialEq, Eq, Clone, Hash, serde::Serialize, serde::Deserialize)]
pub struct GraduateInstructionAccounts {
    pub mint: solana_pubkey::Pubkey,
    pub wsol: solana_pubkey::Pubkey,
    pub protocol_fee_recipient: solana_pubkey::Pubkey,
    pub token_distributor: solana_pubkey::Pubkey,
    pub token_distributor_token_account: solana_pubkey::Pubkey,
    pub vault_authority: solana_pubkey::Pubkey,
    pub bonding_curve_sol_vault: solana_pubkey::Pubkey,
    pub bonding_curve: solana_pubkey::Pubkey,
    pub bonding_curve_vault: solana_pubkey::Pubkey,
    pub bonding_curve_wsol_account: solana_pubkey::Pubkey,
    pub operator: solana_pubkey::Pubkey,
    pub config: solana_pubkey::Pubkey,
    pub system_program: solana_pubkey::Pubkey,
    pub token_program: solana_pubkey::Pubkey,
    pub associated_token_program: solana_pubkey::Pubkey,
}

impl carbon_core::deserialize::ArrangeAccounts for Graduate {
    type ArrangedAccounts = GraduateInstructionAccounts;

    fn arrange_accounts(
        accounts: &[solana_instruction::AccountMeta],
    ) -> Option<Self::ArrangedAccounts> {
        let [mint, wsol, protocol_fee_recipient, token_distributor, token_distributor_token_account, vault_authority, bonding_curve_sol_vault, bonding_curve, bonding_curve_vault, bonding_curve_wsol_account, operator, config, system_program, token_program, associated_token_program, _remaining @ ..] =
            accounts
        else {
            return None;
        };

        Some(GraduateInstructionAccounts {
            mint: mint.pubkey,
            wsol: wsol.pubkey,
            protocol_fee_recipient: protocol_fee_recipient.pubkey,
            token_distributor: token_distributor.pubkey,
            token_distributor_token_account: token_distributor_token_account.pubkey,
            vault_authority: vault_authority.pubkey,
            bonding_curve_sol_vault: bonding_curve_sol_vault.pubkey,
            bonding_curve: bonding_curve.pubkey,
            bonding_curve_vault: bonding_curve_vault.pubkey,
            bonding_curve_wsol_account: bonding_curve_wsol_account.pubkey,
            operator: operator.pubkey,
            config: config.pubkey,
            system_program: system_program.pubkey,
            token_program: token_program.pubkey,
            associated_token_program: associated_token_program.pubkey,
        })
    }
}
