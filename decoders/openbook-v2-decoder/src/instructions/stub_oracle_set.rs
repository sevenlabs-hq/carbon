use carbon_core::{borsh, CarbonDeserialize};

#[derive(CarbonDeserialize, Debug, serde::Serialize, serde::Deserialize, PartialEq, Clone)]
#[carbon(discriminator = "0x6dc64f7941caa18e")]
pub struct StubOracleSet {
    pub price: f64,
}

pub struct StubOracleSetInstructionAccounts {
    pub owner: solana_sdk::pubkey::Pubkey,
    pub oracle: solana_sdk::pubkey::Pubkey,
}

impl carbon_core::deserialize::ArrangeAccounts for StubOracleSet {
    type ArrangedAccounts = StubOracleSetInstructionAccounts;

    fn arrange_accounts(
        accounts: Vec<solana_sdk::instruction::AccountMeta>,
    ) -> Option<Self::ArrangedAccounts> {
        let owner = accounts.get(0)?;
        let oracle = accounts.get(1)?;

        Some(StubOracleSetInstructionAccounts {
            owner: owner.pubkey,
            oracle: oracle.pubkey,
        })
    }
}
