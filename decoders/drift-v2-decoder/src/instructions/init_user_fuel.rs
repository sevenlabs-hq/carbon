use carbon_core::{borsh, CarbonDeserialize};

#[derive(
    CarbonDeserialize, Debug, serde::Serialize, serde::Deserialize, PartialEq, Eq, Clone, Hash,
)]
#[carbon(discriminator = "0x84bfe48dc98a3c30")]
pub struct InitUserFuel {
    pub fuel_boost_deposits: Option<i32>,
    pub fuel_boost_borrows: Option<u32>,
    pub fuel_boost_taker: Option<u32>,
    pub fuel_boost_maker: Option<u32>,
    pub fuel_boost_insurance: Option<u32>,
}

pub struct InitUserFuelInstructionAccounts {
    pub admin: solana_pubkey::Pubkey,
    pub state: solana_pubkey::Pubkey,
    pub user: solana_pubkey::Pubkey,
    pub user_stats: solana_pubkey::Pubkey,
}

impl carbon_core::deserialize::ArrangeAccounts for InitUserFuel {
    type ArrangedAccounts = InitUserFuelInstructionAccounts;

    fn arrange_accounts(
        accounts: &[solana_sdk::instruction::AccountMeta],
    ) -> Option<Self::ArrangedAccounts> {
        let [admin, state, user, user_stats, _remaining @ ..] = accounts else {
            return None;
        };

        Some(InitUserFuelInstructionAccounts {
            admin: admin.pubkey,
            state: state.pubkey,
            user: user.pubkey,
            user_stats: user_stats.pubkey,
        })
    }
}
