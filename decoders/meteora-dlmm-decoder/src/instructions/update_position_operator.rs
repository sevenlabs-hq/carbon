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
        accounts: Vec<solana_sdk::instruction::AccountMeta>,
    ) -> Option<Self::ArrangedAccounts> {
        let position = accounts.get(0)?;
        let owner = accounts.get(1)?;
        let event_authority = accounts.get(2)?;
        let program = accounts.get(3)?;

        Some(UpdatePositionOperatorInstructionAccounts {
            position: position.pubkey,
            owner: owner.pubkey,
            event_authority: event_authority.pubkey,
            program: program.pubkey,
        })
    }
}
