use carbon_core::{borsh, CarbonDeserialize};

#[derive(
    CarbonDeserialize, Debug, serde::Serialize, serde::Deserialize, PartialEq, Eq, Clone, Hash,
)]
#[carbon(discriminator = "0x8e46cc5c496ab434")]
pub struct RequestRemoveInsuranceFundStake {
    pub market_index: u16,
    pub amount: u64,
}

pub struct RequestRemoveInsuranceFundStakeInstructionAccounts {
    pub spot_market: solana_sdk::pubkey::Pubkey,
    pub insurance_fund_stake: solana_sdk::pubkey::Pubkey,
    pub user_stats: solana_sdk::pubkey::Pubkey,
    pub authority: solana_sdk::pubkey::Pubkey,
    pub insurance_fund_vault: solana_sdk::pubkey::Pubkey,
}

impl carbon_core::deserialize::ArrangeAccounts for RequestRemoveInsuranceFundStake {
    type ArrangedAccounts = RequestRemoveInsuranceFundStakeInstructionAccounts;

    fn arrange_accounts(
        accounts: &[solana_sdk::instruction::AccountMeta],
    ) -> Option<Self::ArrangedAccounts> {
        let [spot_market, insurance_fund_stake, user_stats, authority, insurance_fund_vault, _remaining @ ..] =
            accounts
        else {
            return None;
        };

        Some(RequestRemoveInsuranceFundStakeInstructionAccounts {
            spot_market: spot_market.pubkey,
            insurance_fund_stake: insurance_fund_stake.pubkey,
            user_stats: user_stats.pubkey,
            authority: authority.pubkey,
            insurance_fund_vault: insurance_fund_vault.pubkey,
        })
    }
}
