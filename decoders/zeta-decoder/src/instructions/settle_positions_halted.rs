use {
    super::super::types::*,
    carbon_core::{borsh, CarbonDeserialize},
};

#[derive(
    CarbonDeserialize, Debug, serde::Serialize, serde::Deserialize, PartialEq, Eq, Clone, Hash,
)]
#[carbon(discriminator = "0xaa938ba31368a74d")]
pub struct SettlePositionsHalted {
    pub asset: Asset,
}

pub struct SettlePositionsHaltedInstructionAccounts {
    pub state: solana_pubkey::Pubkey,
    pub pricing: solana_pubkey::Pubkey,
    pub admin: solana_pubkey::Pubkey,
}

impl carbon_core::deserialize::ArrangeAccounts for SettlePositionsHalted {
    type ArrangedAccounts = SettlePositionsHaltedInstructionAccounts;

    fn arrange_accounts(
        accounts: &[solana_instruction::AccountMeta],
    ) -> Option<Self::ArrangedAccounts> {
        let [state, pricing, admin, _remaining @ ..] = accounts else {
            return None;
        };

        Some(SettlePositionsHaltedInstructionAccounts {
            state: state.pubkey,
            pricing: pricing.pubkey,
            admin: admin.pubkey,
        })
    }
}
