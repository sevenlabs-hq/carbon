use carbon_core::{CarbonDeserialize, borsh};

#[derive(
    CarbonDeserialize, Debug, serde::Serialize, serde::Deserialize, PartialEq, Eq, Clone, Hash,
)]
#[carbon(discriminator = "0x61eb4e3ed42af17f")]
pub struct CancelRequestRemoveInsuranceFundStake {
    pub market_index: u16,
}

pub struct CancelRequestRemoveInsuranceFundStakeInstructionAccounts {
    pub spot_market: solana_pubkey::Pubkey,
    pub insurance_fund_stake: solana_pubkey::Pubkey,
    pub user_stats: solana_pubkey::Pubkey,
    pub authority: solana_pubkey::Pubkey,
    pub insurance_fund_vault: solana_pubkey::Pubkey,
}

impl carbon_core::deserialize::ArrangeAccounts for CancelRequestRemoveInsuranceFundStake {
    type ArrangedAccounts = CancelRequestRemoveInsuranceFundStakeInstructionAccounts;

    fn arrange_accounts(
        accounts: &[solana_instruction::AccountMeta],
    ) -> Option<Self::ArrangedAccounts> {
        let [
            spot_market,
            insurance_fund_stake,
            user_stats,
            authority,
            insurance_fund_vault,
            _remaining @ ..,
        ] = accounts
        else {
            return None;
        };

        Some(CancelRequestRemoveInsuranceFundStakeInstructionAccounts {
            spot_market: spot_market.pubkey,
            insurance_fund_stake: insurance_fund_stake.pubkey,
            user_stats: user_stats.pubkey,
            authority: authority.pubkey,
            insurance_fund_vault: insurance_fund_vault.pubkey,
        })
    }
}
