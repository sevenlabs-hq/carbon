use carbon_core::{borsh, CarbonDeserialize};

#[derive(
    CarbonDeserialize, Debug, serde::Serialize, serde::Deserialize, PartialEq, Eq, Clone, Hash,
)]
#[carbon(discriminator = "0x8c6b21d6ebdb6714")]
pub struct InitializePythLazerOracle {
    pub feed_id: u32,
}

pub struct InitializePythLazerOracleInstructionAccounts {
    pub admin: solana_pubkey::Pubkey,
    pub lazer_oracle: solana_pubkey::Pubkey,
    pub state: solana_pubkey::Pubkey,
    pub rent: solana_pubkey::Pubkey,
    pub system_program: solana_pubkey::Pubkey,
}

impl carbon_core::deserialize::ArrangeAccounts for InitializePythLazerOracle {
    type ArrangedAccounts = InitializePythLazerOracleInstructionAccounts;

    fn arrange_accounts(
        accounts: &[solana_sdk::instruction::AccountMeta],
    ) -> Option<Self::ArrangedAccounts> {
        let [admin, lazer_oracle, state, rent, system_program, _remaining @ ..] = accounts else {
            return None;
        };

        Some(InitializePythLazerOracleInstructionAccounts {
            admin: admin.pubkey,
            lazer_oracle: lazer_oracle.pubkey,
            state: state.pubkey,
            rent: rent.pubkey,
            system_program: system_program.pubkey,
        })
    }
}
