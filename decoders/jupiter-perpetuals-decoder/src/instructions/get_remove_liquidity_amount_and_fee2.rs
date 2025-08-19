use {
    super::super::types::*,
    carbon_core::{borsh, CarbonDeserialize},
};

#[derive(
    CarbonDeserialize, Debug, serde::Serialize, serde::Deserialize, PartialEq, Eq, Clone, Hash,
)]
#[carbon(discriminator = "0xb73b486edff3968e")]
pub struct GetRemoveLiquidityAmountAndFee2 {
    pub params: GetRemoveLiquidityAmountAndFee2Params,
}

pub struct GetRemoveLiquidityAmountAndFee2InstructionAccounts {
    pub perpetuals: solana_pubkey::Pubkey,
    pub pool: solana_pubkey::Pubkey,
    pub custody: solana_pubkey::Pubkey,
    pub custody_doves_price_account: solana_pubkey::Pubkey,
    pub custody_pythnet_price_account: solana_pubkey::Pubkey,
    pub lp_token_mint: solana_pubkey::Pubkey,
}

impl carbon_core::deserialize::ArrangeAccounts for GetRemoveLiquidityAmountAndFee2 {
    type ArrangedAccounts = GetRemoveLiquidityAmountAndFee2InstructionAccounts;

    fn arrange_accounts(
        accounts: &[solana_instruction::AccountMeta],
    ) -> Option<Self::ArrangedAccounts> {
        let [perpetuals, pool, custody, custody_doves_price_account, custody_pythnet_price_account, lp_token_mint, _remaining @ ..] =
            accounts
        else {
            return None;
        };

        Some(GetRemoveLiquidityAmountAndFee2InstructionAccounts {
            perpetuals: perpetuals.pubkey,
            pool: pool.pubkey,
            custody: custody.pubkey,
            custody_doves_price_account: custody_doves_price_account.pubkey,
            custody_pythnet_price_account: custody_pythnet_price_account.pubkey,
            lp_token_mint: lp_token_mint.pubkey,
        })
    }
}
