use carbon_core::{borsh, CarbonDeserialize};

#[derive(CarbonDeserialize, Debug, serde::Serialize, serde::Deserialize, PartialEq, Clone)]
#[carbon(discriminator = "0x6dc64f7941caa18e")]
pub struct StubOracleSet {
    pub price: f64,
}

pub struct StubOracleSetInstructionAccounts {
    pub owner: solana_pubkey::Pubkey,
    pub oracle: solana_pubkey::Pubkey,
}

impl carbon_core::deserialize::ArrangeAccounts for StubOracleSet {
    type ArrangedAccounts = StubOracleSetInstructionAccounts;

    fn arrange_accounts(
        accounts: &[solana_instruction::AccountMeta],
    ) -> Option<Self::ArrangedAccounts> {
        let [owner, oracle, _remaining @ ..] = accounts else {
            return None;
        };

        Some(StubOracleSetInstructionAccounts {
            owner: owner.pubkey,
            oracle: oracle.pubkey,
        })
    }
}
