use carbon_core::{CarbonDeserialize, borsh};

#[derive(
    CarbonDeserialize, Debug, serde::Serialize, serde::Deserialize, PartialEq, Eq, Clone, Hash,
)]
#[carbon(discriminator = "0xbbb3f346f85a5c93")]
pub struct InitializeInsuranceFundStake {
    pub market_index: u16,
}

pub struct InitializeInsuranceFundStakeInstructionAccounts {
    pub spot_market: solana_pubkey::Pubkey,
    pub insurance_fund_stake: solana_pubkey::Pubkey,
    pub user_stats: solana_pubkey::Pubkey,
    pub state: solana_pubkey::Pubkey,
    pub authority: solana_pubkey::Pubkey,
    pub payer: solana_pubkey::Pubkey,
    pub rent: solana_pubkey::Pubkey,
    pub system_program: solana_pubkey::Pubkey,
}

impl carbon_core::deserialize::ArrangeAccounts for InitializeInsuranceFundStake {
    type ArrangedAccounts = InitializeInsuranceFundStakeInstructionAccounts;

    fn arrange_accounts(
        accounts: &[solana_instruction::AccountMeta],
    ) -> Option<Self::ArrangedAccounts> {
        let [
            spot_market,
            insurance_fund_stake,
            user_stats,
            state,
            authority,
            payer,
            rent,
            system_program,
            _remaining @ ..,
        ] = accounts
        else {
            return None;
        };

        Some(InitializeInsuranceFundStakeInstructionAccounts {
            spot_market: spot_market.pubkey,
            insurance_fund_stake: insurance_fund_stake.pubkey,
            user_stats: user_stats.pubkey,
            state: state.pubkey,
            authority: authority.pubkey,
            payer: payer.pubkey,
            rent: rent.pubkey,
            system_program: system_program.pubkey,
        })
    }
}
