use carbon_core::{borsh, CarbonDeserialize};

#[derive(
    CarbonDeserialize, Debug, serde::Serialize, serde::Deserialize, PartialEq, Eq, Clone, Hash,
)]
#[carbon(discriminator = "0xbe3d7d57674f9ead")]
pub struct IncreaseOracleLength {
    pub length_to_add: u64,
}

pub struct IncreaseOracleLengthInstructionAccounts {
    pub oracle: solana_sdk::pubkey::Pubkey,
    pub funder: solana_sdk::pubkey::Pubkey,
    pub system_program: solana_sdk::pubkey::Pubkey,
    pub event_authority: solana_sdk::pubkey::Pubkey,
    pub program: solana_sdk::pubkey::Pubkey,
}

impl carbon_core::deserialize::ArrangeAccounts for IncreaseOracleLength {
    type ArrangedAccounts = IncreaseOracleLengthInstructionAccounts;

    fn arrange_accounts(
        accounts: Vec<solana_sdk::instruction::AccountMeta>,
    ) -> Option<Self::ArrangedAccounts> {
        let oracle = accounts.get(0)?;
        let funder = accounts.get(1)?;
        let system_program = accounts.get(2)?;
        let event_authority = accounts.get(3)?;
        let program = accounts.get(4)?;

        Some(IncreaseOracleLengthInstructionAccounts {
            oracle: oracle.pubkey,
            funder: funder.pubkey,
            system_program: system_program.pubkey,
            event_authority: event_authority.pubkey,
            program: program.pubkey,
        })
    }
}
