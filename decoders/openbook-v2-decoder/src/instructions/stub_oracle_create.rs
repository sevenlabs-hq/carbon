use carbon_core::{borsh, CarbonDeserialize};

#[derive(CarbonDeserialize, Debug, serde::Serialize, serde::Deserialize, PartialEq, Clone)]
#[carbon(discriminator = "0xac3f65538d4cc7d8")]
pub struct StubOracleCreate {
    pub price: f64,
}

pub struct StubOracleCreateInstructionAccounts {
    pub payer: solana_sdk::pubkey::Pubkey,
    pub owner: solana_sdk::pubkey::Pubkey,
    pub oracle: solana_sdk::pubkey::Pubkey,
    pub mint: solana_sdk::pubkey::Pubkey,
    pub system_program: solana_sdk::pubkey::Pubkey,
}

impl carbon_core::deserialize::ArrangeAccounts for StubOracleCreate {
    type ArrangedAccounts = StubOracleCreateInstructionAccounts;

    fn arrange_accounts(
        accounts: Vec<solana_sdk::instruction::AccountMeta>,
    ) -> Option<Self::ArrangedAccounts> {
        let payer = accounts.get(0)?;
        let owner = accounts.get(1)?;
        let oracle = accounts.get(2)?;
        let mint = accounts.get(3)?;
        let system_program = accounts.get(4)?;

        Some(StubOracleCreateInstructionAccounts {
            payer: payer.pubkey,
            owner: owner.pubkey,
            oracle: oracle.pubkey,
            mint: mint.pubkey,
            system_program: system_program.pubkey,
        })
    }
}
