use carbon_core::{borsh, CarbonDeserialize};

#[derive(
    CarbonDeserialize, Debug, serde::Serialize, serde::Deserialize, PartialEq, Eq, Clone, Hash,
)]
#[carbon(discriminator = "0x35e6ac544dae163d")]
pub struct DeployBondingCurveFallback {
    pub creator: solana_pubkey::Pubkey,
    pub salt: u64,
}

#[derive(Debug, PartialEq, Eq, Clone, Hash, serde::Serialize, serde::Deserialize)]
pub struct DeployBondingCurveFallbackInstructionAccounts {
    pub mint: solana_pubkey::Pubkey,
    pub vault_authority: solana_pubkey::Pubkey,
    pub bonding_curve: solana_pubkey::Pubkey,
    pub bonding_curve_sol_vault: solana_pubkey::Pubkey,
    pub bonding_curve_vault: solana_pubkey::Pubkey,
    pub config: solana_pubkey::Pubkey,
    pub payer: solana_pubkey::Pubkey,
    pub system_program: solana_pubkey::Pubkey,
    pub token_program: solana_pubkey::Pubkey,
    pub associated_token_program: solana_pubkey::Pubkey,
}

impl carbon_core::deserialize::ArrangeAccounts for DeployBondingCurveFallback {
    type ArrangedAccounts = DeployBondingCurveFallbackInstructionAccounts;

    fn arrange_accounts(
        accounts: &[solana_instruction::AccountMeta],
    ) -> Option<Self::ArrangedAccounts> {
        let [mint, vault_authority, bonding_curve, bonding_curve_sol_vault, bonding_curve_vault, config, payer, system_program, token_program, associated_token_program, _remaining @ ..] =
            accounts
        else {
            return None;
        };

        Some(DeployBondingCurveFallbackInstructionAccounts {
            mint: mint.pubkey,
            vault_authority: vault_authority.pubkey,
            bonding_curve: bonding_curve.pubkey,
            bonding_curve_sol_vault: bonding_curve_sol_vault.pubkey,
            bonding_curve_vault: bonding_curve_vault.pubkey,
            config: config.pubkey,
            payer: payer.pubkey,
            system_program: system_program.pubkey,
            token_program: token_program.pubkey,
            associated_token_program: associated_token_program.pubkey,
        })
    }
}
