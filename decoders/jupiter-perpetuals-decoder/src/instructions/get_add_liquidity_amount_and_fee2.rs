use {
    super::super::types::*,
    carbon_core::{borsh, CarbonDeserialize},
};

#[derive(
    CarbonDeserialize, Debug, serde::Serialize, serde::Deserialize, PartialEq, Eq, Clone, Hash,
)]
#[carbon(discriminator = "0x6d9d37a908510476")]
pub struct GetAddLiquidityAmountAndFee2 {
    pub params: GetAddLiquidityAmountAndFee2Params,
}

pub struct GetAddLiquidityAmountAndFee2InstructionAccounts {
    pub perpetuals: solana_pubkey::Pubkey,
    pub pool: solana_pubkey::Pubkey,
    pub custody: solana_pubkey::Pubkey,
    pub custody_doves_price_account: solana_pubkey::Pubkey,
    pub custody_pythnet_price_account: solana_pubkey::Pubkey,
    pub lp_token_mint: solana_pubkey::Pubkey,
}

impl carbon_core::deserialize::ArrangeAccounts for GetAddLiquidityAmountAndFee2 {
    type ArrangedAccounts = GetAddLiquidityAmountAndFee2InstructionAccounts;

    fn arrange_accounts(
        accounts: &[solana_instruction::AccountMeta],
    ) -> Option<Self::ArrangedAccounts> {
        let [perpetuals, pool, custody, custody_doves_price_account, custody_pythnet_price_account, lp_token_mint, _remaining @ ..] =
            accounts
        else {
            return None;
        };

        Some(GetAddLiquidityAmountAndFee2InstructionAccounts {
            perpetuals: perpetuals.pubkey,
            pool: pool.pubkey,
            custody: custody.pubkey,
            custody_doves_price_account: custody_doves_price_account.pubkey,
            custody_pythnet_price_account: custody_pythnet_price_account.pubkey,
            lp_token_mint: lp_token_mint.pubkey,
        })
    }
}
