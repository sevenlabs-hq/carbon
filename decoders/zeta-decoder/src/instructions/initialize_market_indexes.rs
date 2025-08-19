use {
    super::super::types::*,
    carbon_core::{borsh, CarbonDeserialize},
};

#[derive(
    CarbonDeserialize, Debug, serde::Serialize, serde::Deserialize, PartialEq, Eq, Clone, Hash,
)]
#[carbon(discriminator = "0x5b3fcd901453b178")]
pub struct InitializeMarketIndexes {
    pub nonce: u8,
    pub asset: Asset,
}

pub struct InitializeMarketIndexesInstructionAccounts {
    pub state: solana_pubkey::Pubkey,
    pub market_indexes: solana_pubkey::Pubkey,
    pub admin: solana_pubkey::Pubkey,
    pub system_program: solana_pubkey::Pubkey,
    pub pricing: solana_pubkey::Pubkey,
}

impl carbon_core::deserialize::ArrangeAccounts for InitializeMarketIndexes {
    type ArrangedAccounts = InitializeMarketIndexesInstructionAccounts;

    fn arrange_accounts(
        accounts: &[solana_instruction::AccountMeta],
    ) -> Option<Self::ArrangedAccounts> {
        let [state, market_indexes, admin, system_program, pricing, _remaining @ ..] = accounts
        else {
            return None;
        };

        Some(InitializeMarketIndexesInstructionAccounts {
            state: state.pubkey,
            market_indexes: market_indexes.pubkey,
            admin: admin.pubkey,
            system_program: system_program.pubkey,
            pricing: pricing.pubkey,
        })
    }
}
