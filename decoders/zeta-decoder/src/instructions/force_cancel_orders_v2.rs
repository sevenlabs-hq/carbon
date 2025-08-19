use {
    super::super::types::*,
    carbon_core::{borsh, CarbonDeserialize},
};

#[derive(
    CarbonDeserialize, Debug, serde::Serialize, serde::Deserialize, PartialEq, Eq, Clone, Hash,
)]
#[carbon(discriminator = "0x0e3e95ca8f113873")]
pub struct ForceCancelOrdersV2 {
    pub asset: Asset,
}

pub struct ForceCancelOrdersV2InstructionAccounts {
    pub pricing: solana_pubkey::Pubkey,
    pub oracle: solana_pubkey::Pubkey,
    pub oracle_backup_feed: solana_pubkey::Pubkey,
    pub oracle_backup_program: solana_pubkey::Pubkey,
    pub cancel_accounts: solana_pubkey::Pubkey,
}

impl carbon_core::deserialize::ArrangeAccounts for ForceCancelOrdersV2 {
    type ArrangedAccounts = ForceCancelOrdersV2InstructionAccounts;

    fn arrange_accounts(
        accounts: &[solana_instruction::AccountMeta],
    ) -> Option<Self::ArrangedAccounts> {
        let [pricing, oracle, oracle_backup_feed, oracle_backup_program, cancel_accounts, _remaining @ ..] =
            accounts
        else {
            return None;
        };

        Some(ForceCancelOrdersV2InstructionAccounts {
            pricing: pricing.pubkey,
            oracle: oracle.pubkey,
            oracle_backup_feed: oracle_backup_feed.pubkey,
            oracle_backup_program: oracle_backup_program.pubkey,
            cancel_accounts: cancel_accounts.pubkey,
        })
    }
}
