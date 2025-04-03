use carbon_core::{borsh, CarbonDeserialize};

#[derive(
    CarbonDeserialize, Debug, serde::Serialize, serde::Deserialize, PartialEq, Eq, Clone, Hash,
)]
#[carbon(discriminator = "0xfb90730bde2f3eec")]
pub struct AddInsuranceFundStake {
    pub market_index: u16,
    pub amount: u64,
}

pub struct AddInsuranceFundStakeInstructionAccounts {
    pub state: solana_pubkey::Pubkey,
    pub spot_market: solana_pubkey::Pubkey,
    pub insurance_fund_stake: solana_pubkey::Pubkey,
    pub user_stats: solana_pubkey::Pubkey,
    pub authority: solana_pubkey::Pubkey,
    pub spot_market_vault: solana_pubkey::Pubkey,
    pub insurance_fund_vault: solana_pubkey::Pubkey,
    pub drift_signer: solana_pubkey::Pubkey,
    pub user_token_account: solana_pubkey::Pubkey,
    pub token_program: solana_pubkey::Pubkey,
}

impl carbon_core::deserialize::ArrangeAccounts for AddInsuranceFundStake {
    type ArrangedAccounts = AddInsuranceFundStakeInstructionAccounts;

    fn arrange_accounts(
        accounts: &[solana_instruction::AccountMeta],
    ) -> Option<Self::ArrangedAccounts> {
        let [state, spot_market, insurance_fund_stake, user_stats, authority, spot_market_vault, insurance_fund_vault, drift_signer, user_token_account, token_program, _remaining @ ..] =
            accounts
        else {
            return None;
        };

        Some(AddInsuranceFundStakeInstructionAccounts {
            state: state.pubkey,
            spot_market: spot_market.pubkey,
            insurance_fund_stake: insurance_fund_stake.pubkey,
            user_stats: user_stats.pubkey,
            authority: authority.pubkey,
            spot_market_vault: spot_market_vault.pubkey,
            insurance_fund_vault: insurance_fund_vault.pubkey,
            drift_signer: drift_signer.pubkey,
            user_token_account: user_token_account.pubkey,
            token_program: token_program.pubkey,
        })
    }
}
