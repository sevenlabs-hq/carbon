use carbon_core::{borsh, CarbonDeserialize};

#[derive(
    CarbonDeserialize, Debug, serde::Serialize, serde::Deserialize, PartialEq, Eq, Clone, Hash,
)]
#[carbon(discriminator = "0x6bcf3be219171fa1")]
pub struct TakeTriggerOrder {
    pub trigger_order_bit: u8,
}

pub struct TakeTriggerOrderInstructionAccounts {
    pub trigger_order: solana_pubkey::Pubkey,
    pub state: solana_pubkey::Pubkey,
    pub pricing: solana_pubkey::Pubkey,
    pub oracle: solana_pubkey::Pubkey,
    pub oracle_backup_feed: solana_pubkey::Pubkey,
    pub oracle_backup_program: solana_pubkey::Pubkey,
    pub bids: solana_pubkey::Pubkey,
    pub asks: solana_pubkey::Pubkey,
    pub taker: solana_pubkey::Pubkey,
    pub taker_margin_account: solana_pubkey::Pubkey,
    pub order_margin_account: solana_pubkey::Pubkey,
}

impl carbon_core::deserialize::ArrangeAccounts for TakeTriggerOrder {
    type ArrangedAccounts = TakeTriggerOrderInstructionAccounts;

    fn arrange_accounts(
        accounts: &[solana_sdk::instruction::AccountMeta],
    ) -> Option<Self::ArrangedAccounts> {
        let [trigger_order, state, pricing, oracle, oracle_backup_feed, oracle_backup_program, bids, asks, taker, taker_margin_account, order_margin_account, _remaining @ ..] =
            accounts
        else {
            return None;
        };

        Some(TakeTriggerOrderInstructionAccounts {
            trigger_order: trigger_order.pubkey,
            state: state.pubkey,
            pricing: pricing.pubkey,
            oracle: oracle.pubkey,
            oracle_backup_feed: oracle_backup_feed.pubkey,
            oracle_backup_program: oracle_backup_program.pubkey,
            bids: bids.pubkey,
            asks: asks.pubkey,
            taker: taker.pubkey,
            taker_margin_account: taker_margin_account.pubkey,
            order_margin_account: order_margin_account.pubkey,
        })
    }
}
