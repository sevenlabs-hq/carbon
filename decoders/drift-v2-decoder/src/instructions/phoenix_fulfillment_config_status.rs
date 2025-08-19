use {
    super::super::types::*,
    carbon_core::{borsh, CarbonDeserialize},
};

#[derive(
    CarbonDeserialize, Debug, serde::Serialize, serde::Deserialize, PartialEq, Eq, Clone, Hash,
)]
#[carbon(discriminator = "0x601f71200ccb079a")]
pub struct PhoenixFulfillmentConfigStatus {
    pub status: SpotFulfillmentConfigStatus,
}

pub struct PhoenixFulfillmentConfigStatusInstructionAccounts {
    pub state: solana_pubkey::Pubkey,
    pub phoenix_fulfillment_config: solana_pubkey::Pubkey,
    pub admin: solana_pubkey::Pubkey,
}

impl carbon_core::deserialize::ArrangeAccounts for PhoenixFulfillmentConfigStatus {
    type ArrangedAccounts = PhoenixFulfillmentConfigStatusInstructionAccounts;

    fn arrange_accounts(
        accounts: &[solana_instruction::AccountMeta],
    ) -> Option<Self::ArrangedAccounts> {
        let [state, phoenix_fulfillment_config, admin, _remaining @ ..] = accounts else {
            return None;
        };

        Some(PhoenixFulfillmentConfigStatusInstructionAccounts {
            state: state.pubkey,
            phoenix_fulfillment_config: phoenix_fulfillment_config.pubkey,
            admin: admin.pubkey,
        })
    }
}
