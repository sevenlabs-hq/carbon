use {
    super::super::types::*,
    carbon_core::{borsh, CarbonDeserialize},
};

#[derive(
    CarbonDeserialize, Debug, serde::Serialize, serde::Deserialize, PartialEq, Eq, Clone, Hash,
)]
#[carbon(discriminator = "0x0f56553702e1a1eb")]
pub struct LiquidateV2 {
    pub size: u64,
    pub asset: Asset,
}

pub struct LiquidateV2InstructionAccounts {
    pub state: solana_pubkey::Pubkey,
    pub liquidator: solana_pubkey::Pubkey,
    pub liquidator_account: solana_pubkey::Pubkey,
    pub pricing: solana_pubkey::Pubkey,
    pub oracle: solana_pubkey::Pubkey,
    pub oracle_backup_feed: solana_pubkey::Pubkey,
    pub oracle_backup_program: solana_pubkey::Pubkey,
    pub market: solana_pubkey::Pubkey,
    pub liquidated_account: solana_pubkey::Pubkey,
}

impl carbon_core::deserialize::ArrangeAccounts for LiquidateV2 {
    type ArrangedAccounts = LiquidateV2InstructionAccounts;

    fn arrange_accounts(
        accounts: &[solana_instruction::AccountMeta],
    ) -> Option<Self::ArrangedAccounts> {
        let [state, liquidator, liquidator_account, pricing, oracle, oracle_backup_feed, oracle_backup_program, market, liquidated_account, _remaining @ ..] =
            accounts
        else {
            return None;
        };

        Some(LiquidateV2InstructionAccounts {
            state: state.pubkey,
            liquidator: liquidator.pubkey,
            liquidator_account: liquidator_account.pubkey,
            pricing: pricing.pubkey,
            oracle: oracle.pubkey,
            oracle_backup_feed: oracle_backup_feed.pubkey,
            oracle_backup_program: oracle_backup_program.pubkey,
            market: market.pubkey,
            liquidated_account: liquidated_account.pubkey,
        })
    }
}
