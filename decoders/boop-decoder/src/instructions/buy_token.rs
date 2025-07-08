use carbon_core::{borsh, CarbonDeserialize};

#[derive(
    CarbonDeserialize, Debug, serde::Serialize, serde::Deserialize, PartialEq, Eq, Clone, Hash,
)]
#[carbon(discriminator = "0x8a7f0e5b26577369")]
pub struct BuyToken {
    pub buy_amount: u64,
    pub amount_out_min: u64,
}

#[derive(Debug, PartialEq, Eq, Clone, Hash, serde::Serialize, serde::Deserialize)]
pub struct BuyTokenInstructionAccounts {
    pub mint: solana_pubkey::Pubkey,
    pub bonding_curve: solana_pubkey::Pubkey,
    pub trading_fees_vault: solana_pubkey::Pubkey,
    pub bonding_curve_vault: solana_pubkey::Pubkey,
    pub bonding_curve_sol_vault: solana_pubkey::Pubkey,
    pub recipient_token_account: solana_pubkey::Pubkey,
    pub buyer: solana_pubkey::Pubkey,
    pub config: solana_pubkey::Pubkey,
    pub vault_authority: solana_pubkey::Pubkey,
    pub wsol: solana_pubkey::Pubkey,
    pub system_program: solana_pubkey::Pubkey,
    pub token_program: solana_pubkey::Pubkey,
    pub associated_token_program: solana_pubkey::Pubkey,
}

impl carbon_core::deserialize::ArrangeAccounts for BuyToken {
    type ArrangedAccounts = BuyTokenInstructionAccounts;

    fn arrange_accounts(
        accounts: &[solana_instruction::AccountMeta],
    ) -> Option<Self::ArrangedAccounts> {
        let [mint, bonding_curve, trading_fees_vault, bonding_curve_vault, bonding_curve_sol_vault, recipient_token_account, buyer, config, vault_authority, wsol, system_program, token_program, associated_token_program, _remaining @ ..] =
            accounts
        else {
            return None;
        };

        Some(BuyTokenInstructionAccounts {
            mint: mint.pubkey,
            bonding_curve: bonding_curve.pubkey,
            trading_fees_vault: trading_fees_vault.pubkey,
            bonding_curve_vault: bonding_curve_vault.pubkey,
            bonding_curve_sol_vault: bonding_curve_sol_vault.pubkey,
            recipient_token_account: recipient_token_account.pubkey,
            buyer: buyer.pubkey,
            config: config.pubkey,
            vault_authority: vault_authority.pubkey,
            wsol: wsol.pubkey,
            system_program: system_program.pubkey,
            token_program: token_program.pubkey,
            associated_token_program: associated_token_program.pubkey,
        })
    }
}
