use carbon_core::{borsh, CarbonDeserialize};

#[derive(
    CarbonDeserialize, Debug, serde::Serialize, serde::Deserialize, PartialEq, Eq, Clone, Hash,
)]
#[carbon(discriminator = "0xc8785d884526c79f")]
pub struct SettleRevenueToInsuranceFund {
    pub spot_market_index: u16,
}

pub struct SettleRevenueToInsuranceFundInstructionAccounts {
    pub state: solana_pubkey::Pubkey,
    pub spot_market: solana_pubkey::Pubkey,
    pub spot_market_vault: solana_pubkey::Pubkey,
    pub drift_signer: solana_pubkey::Pubkey,
    pub insurance_fund_vault: solana_pubkey::Pubkey,
    pub token_program: solana_pubkey::Pubkey,
}

impl carbon_core::deserialize::ArrangeAccounts for SettleRevenueToInsuranceFund {
    type ArrangedAccounts = SettleRevenueToInsuranceFundInstructionAccounts;

    fn arrange_accounts(
        accounts: &[solana_instruction::AccountMeta],
    ) -> Option<Self::ArrangedAccounts> {
        let [state, spot_market, spot_market_vault, drift_signer, insurance_fund_vault, token_program, _remaining @ ..] =
            accounts
        else {
            return None;
        };

        Some(SettleRevenueToInsuranceFundInstructionAccounts {
            state: state.pubkey,
            spot_market: spot_market.pubkey,
            spot_market_vault: spot_market_vault.pubkey,
            drift_signer: drift_signer.pubkey,
            insurance_fund_vault: insurance_fund_vault.pubkey,
            token_program: token_program.pubkey,
        })
    }
}
