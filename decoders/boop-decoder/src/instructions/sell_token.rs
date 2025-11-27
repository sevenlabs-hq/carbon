use carbon_core::{borsh, CarbonDeserialize};

#[derive(
    CarbonDeserialize, Debug, serde::Serialize, serde::Deserialize, PartialEq, Eq, Clone, Hash,
)]
#[carbon(discriminator = "0x6d3d28bbe6b087ae")]
pub struct SellToken {
    pub sell_amount: u64,
    pub amount_out_min: u64,
}

#[derive(Debug, PartialEq, Eq, Clone, Hash, serde::Serialize, serde::Deserialize)]
pub struct SellTokenInstructionAccounts {
    pub mint: solana_pubkey::Pubkey,
    pub bonding_curve: solana_pubkey::Pubkey,
    pub trading_fees_vault: solana_pubkey::Pubkey,
    pub bonding_curve_vault: solana_pubkey::Pubkey,
    pub bonding_curve_sol_vault: solana_pubkey::Pubkey,
    pub seller_token_account: solana_pubkey::Pubkey,
    pub seller: solana_pubkey::Pubkey,
    pub recipient: solana_pubkey::Pubkey,
    pub config: solana_pubkey::Pubkey,
    pub system_program: solana_pubkey::Pubkey,
    pub token_program: solana_pubkey::Pubkey,
    pub associated_token_program: solana_pubkey::Pubkey,
}

impl carbon_core::deserialize::ArrangeAccounts for SellToken {
    type ArrangedAccounts = SellTokenInstructionAccounts;

    fn arrange_accounts(
        accounts: &[solana_instruction::AccountMeta],
    ) -> Option<Self::ArrangedAccounts> {
        let [mint, bonding_curve, trading_fees_vault, bonding_curve_vault, bonding_curve_sol_vault, seller_token_account, seller, recipient, config, system_program, token_program, associated_token_program, _remaining @ ..] =
            accounts
        else {
            return None;
        };

        Some(SellTokenInstructionAccounts {
            mint: mint.pubkey,
            bonding_curve: bonding_curve.pubkey,
            trading_fees_vault: trading_fees_vault.pubkey,
            bonding_curve_vault: bonding_curve_vault.pubkey,
            bonding_curve_sol_vault: bonding_curve_sol_vault.pubkey,
            seller_token_account: seller_token_account.pubkey,
            seller: seller.pubkey,
            recipient: recipient.pubkey,
            config: config.pubkey,
            system_program: system_program.pubkey,
            token_program: token_program.pubkey,
            associated_token_program: associated_token_program.pubkey,
        })
    }
}
