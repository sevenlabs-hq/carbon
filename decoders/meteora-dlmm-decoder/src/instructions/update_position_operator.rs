use carbon_core::{borsh, CarbonDeserialize};

#[derive(
    CarbonDeserialize, Debug, serde::Serialize, serde::Deserialize, PartialEq, Eq, Clone, Hash,
)]
#[carbon(discriminator = "0xcab8678fb4bf74d9")]
pub struct UpdatePositionOperator {
    pub operator: solana_sdk::pubkey::Pubkey,
}

pub struct UpdatePositionOperatorInstructionAccounts {
    pub position: solana_sdk::pubkey::Pubkey,
    pub owner: solana_sdk::pubkey::Pubkey,
    pub event_authority: solana_sdk::pubkey::Pubkey,
    pub program: solana_sdk::pubkey::Pubkey,
}

impl carbon_core::deserialize::ArrangeAccounts for UpdatePositionOperator {
    type ArrangedAccounts = UpdatePositionOperatorInstructionAccounts;

    fn arrange_accounts(
        accounts: &[solana_sdk::instruction::AccountMeta],
    ) -> Option<Self::ArrangedAccounts> {
        let [position, owner, event_authority, program, _remaining @ ..] = accounts else {
            return None;
        };

        Some(UpdatePositionOperatorInstructionAccounts {
            position: position.pubkey,
            owner: owner.pubkey,
            event_authority: event_authority.pubkey,
            program: program.pubkey,
        })
    }
}
