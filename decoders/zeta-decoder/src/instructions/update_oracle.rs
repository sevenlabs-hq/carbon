use carbon_core::{borsh, CarbonDeserialize};

#[derive(
    CarbonDeserialize, Debug, serde::Serialize, serde::Deserialize, PartialEq, Eq, Clone, Hash,
)]
#[carbon(discriminator = "0x7029d112f8e2fcbc")]
pub struct UpdateOracle {}

pub struct UpdateOracleInstructionAccounts {
    pub state: solana_sdk::pubkey::Pubkey,
    pub zeta_group: solana_sdk::pubkey::Pubkey,
    pub admin: solana_sdk::pubkey::Pubkey,
    pub oracle: solana_sdk::pubkey::Pubkey,
}

impl carbon_core::deserialize::ArrangeAccounts for UpdateOracle {
    type ArrangedAccounts = UpdateOracleInstructionAccounts;

    fn arrange_accounts(
        accounts: &[solana_sdk::instruction::AccountMeta],
    ) -> Option<Self::ArrangedAccounts> {
        let [state, zeta_group, admin, oracle, _remaining @ ..] = accounts else {
            return None;
        };

        Some(UpdateOracleInstructionAccounts {
            state: state.pubkey,
            zeta_group: zeta_group.pubkey,
            admin: admin.pubkey,
            oracle: oracle.pubkey,
        })
    }
}
