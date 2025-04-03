use carbon_core::{borsh, CarbonDeserialize};

#[derive(CarbonDeserialize, Debug, serde::Serialize, serde::Deserialize, PartialEq, Clone)]
#[carbon(discriminator = "0xac3f65538d4cc7d8")]
pub struct StubOracleCreate {
    pub price: f64,
}

pub struct StubOracleCreateInstructionAccounts {
    pub payer: solana_pubkey::Pubkey,
    pub owner: solana_pubkey::Pubkey,
    pub oracle: solana_pubkey::Pubkey,
    pub mint: solana_pubkey::Pubkey,
    pub system_program: solana_pubkey::Pubkey,
}

impl carbon_core::deserialize::ArrangeAccounts for StubOracleCreate {
    type ArrangedAccounts = StubOracleCreateInstructionAccounts;

    fn arrange_accounts(
        accounts: &[solana_sdk::instruction::AccountMeta],
    ) -> Option<Self::ArrangedAccounts> {
        let [payer, owner, oracle, mint, system_program, _remaining @ ..] = accounts else {
            return None;
        };

        Some(StubOracleCreateInstructionAccounts {
            payer: payer.pubkey,
            owner: owner.pubkey,
            oracle: oracle.pubkey,
            mint: mint.pubkey,
            system_program: system_program.pubkey,
        })
    }
}
