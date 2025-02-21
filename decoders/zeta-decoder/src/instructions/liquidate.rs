use carbon_core::{borsh, CarbonDeserialize};

#[derive(
    CarbonDeserialize, Debug, serde::Serialize, serde::Deserialize, PartialEq, Eq, Clone, Hash,
)]
#[carbon(discriminator = "0xdfb3e27d302e274a")]
pub struct Liquidate {
    pub size: u64,
}

pub struct LiquidateInstructionAccounts {
    pub state: solana_sdk::pubkey::Pubkey,
    pub liquidator: solana_sdk::pubkey::Pubkey,
    pub liquidator_margin_account: solana_sdk::pubkey::Pubkey,
    pub greeks: solana_sdk::pubkey::Pubkey,
    pub oracle: solana_sdk::pubkey::Pubkey,
    pub oracle_backup_feed: solana_sdk::pubkey::Pubkey,
    pub oracle_backup_program: solana_sdk::pubkey::Pubkey,
    pub market: solana_sdk::pubkey::Pubkey,
    pub zeta_group: solana_sdk::pubkey::Pubkey,
    pub liquidated_margin_account: solana_sdk::pubkey::Pubkey,
}

impl carbon_core::deserialize::ArrangeAccounts for Liquidate {
    type ArrangedAccounts = LiquidateInstructionAccounts;

    fn arrange_accounts(
        accounts: &[solana_sdk::instruction::AccountMeta],
    ) -> Option<Self::ArrangedAccounts> {
        let [state, liquidator, liquidator_margin_account, greeks, oracle, oracle_backup_feed, oracle_backup_program, market, zeta_group, liquidated_margin_account, _remaining @ ..] =
            accounts
        else {
            return None;
        };

        Some(LiquidateInstructionAccounts {
            state: state.pubkey,
            liquidator: liquidator.pubkey,
            liquidator_margin_account: liquidator_margin_account.pubkey,
            greeks: greeks.pubkey,
            oracle: oracle.pubkey,
            oracle_backup_feed: oracle_backup_feed.pubkey,
            oracle_backup_program: oracle_backup_program.pubkey,
            market: market.pubkey,
            zeta_group: zeta_group.pubkey,
            liquidated_margin_account: liquidated_margin_account.pubkey,
        })
    }
}
