use carbon_core::{borsh, CarbonDeserialize};

#[derive(
    CarbonDeserialize, Debug, serde::Serialize, serde::Deserialize, PartialEq, Eq, Clone, Hash,
)]
#[carbon(discriminator = "0xc96ad9fd04afe461")]
pub struct UpdateAmms {
    pub market_indexes: [u16; 5],
}

pub struct UpdateAmmsInstructionAccounts {
    pub state: solana_pubkey::Pubkey,
    pub authority: solana_pubkey::Pubkey,
}

impl carbon_core::deserialize::ArrangeAccounts for UpdateAmms {
    type ArrangedAccounts = UpdateAmmsInstructionAccounts;

    fn arrange_accounts(
        accounts: &[solana_instruction::AccountMeta],
    ) -> Option<Self::ArrangedAccounts> {
        let [state, authority, _remaining @ ..] = accounts else {
            return None;
        };

        Some(UpdateAmmsInstructionAccounts {
            state: state.pubkey,
            authority: authority.pubkey,
        })
    }
}
