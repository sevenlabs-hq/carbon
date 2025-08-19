use {
    super::super::types::*,
    carbon_core::{borsh, CarbonDeserialize},
};

#[derive(
    CarbonDeserialize, Debug, serde::Serialize, serde::Deserialize, PartialEq, Eq, Clone, Hash,
)]
#[carbon(discriminator = "0x0688050ce5926659")]
pub struct UpdateMinLot {
    pub asset: Asset,
    pub min_lot_size: u32,
}

pub struct UpdateMinLotInstructionAccounts {
    pub state: solana_pubkey::Pubkey,
    pub admin: solana_pubkey::Pubkey,
}

impl carbon_core::deserialize::ArrangeAccounts for UpdateMinLot {
    type ArrangedAccounts = UpdateMinLotInstructionAccounts;

    fn arrange_accounts(
        accounts: &[solana_instruction::AccountMeta],
    ) -> Option<Self::ArrangedAccounts> {
        let [state, admin, _remaining @ ..] = accounts else {
            return None;
        };

        Some(UpdateMinLotInstructionAccounts {
            state: state.pubkey,
            admin: admin.pubkey,
        })
    }
}
