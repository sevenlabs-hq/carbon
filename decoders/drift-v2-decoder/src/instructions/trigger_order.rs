use carbon_core::{borsh, CarbonDeserialize};

#[derive(
    CarbonDeserialize, Debug, serde::Serialize, serde::Deserialize, PartialEq, Eq, Clone, Hash,
)]
#[carbon(discriminator = "0x3f7033e9e82ff0c7")]
pub struct TriggerOrder {
    pub order_id: u32,
}

pub struct TriggerOrderInstructionAccounts {
    pub state: solana_pubkey::Pubkey,
    pub authority: solana_pubkey::Pubkey,
    pub filler: solana_pubkey::Pubkey,
    pub user: solana_pubkey::Pubkey,
}

impl carbon_core::deserialize::ArrangeAccounts for TriggerOrder {
    type ArrangedAccounts = TriggerOrderInstructionAccounts;

    fn arrange_accounts(
        accounts: &[solana_sdk::instruction::AccountMeta],
    ) -> Option<Self::ArrangedAccounts> {
        let [state, authority, filler, user, _remaining @ ..] = accounts else {
            return None;
        };

        Some(TriggerOrderInstructionAccounts {
            state: state.pubkey,
            authority: authority.pubkey,
            filler: filler.pubkey,
            user: user.pubkey,
        })
    }
}
