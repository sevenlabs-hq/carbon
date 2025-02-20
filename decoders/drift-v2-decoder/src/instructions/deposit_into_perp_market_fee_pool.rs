use carbon_core::{borsh, CarbonDeserialize};

#[derive(
    CarbonDeserialize, Debug, serde::Serialize, serde::Deserialize, PartialEq, Eq, Clone, Hash,
)]
#[carbon(discriminator = "0x223a39446150f406")]
pub struct DepositIntoPerpMarketFeePool {
    pub amount: u64,
}

pub struct DepositIntoPerpMarketFeePoolInstructionAccounts {
    pub state: solana_sdk::pubkey::Pubkey,
    pub perp_market: solana_sdk::pubkey::Pubkey,
    pub admin: solana_sdk::pubkey::Pubkey,
    pub source_vault: solana_sdk::pubkey::Pubkey,
    pub drift_signer: solana_sdk::pubkey::Pubkey,
    pub quote_spot_market: solana_sdk::pubkey::Pubkey,
    pub spot_market_vault: solana_sdk::pubkey::Pubkey,
    pub token_program: solana_sdk::pubkey::Pubkey,
}

impl carbon_core::deserialize::ArrangeAccounts for DepositIntoPerpMarketFeePool {
    type ArrangedAccounts = DepositIntoPerpMarketFeePoolInstructionAccounts;

    fn arrange_accounts(
        accounts: &[solana_sdk::instruction::AccountMeta],
    ) -> Option<Self::ArrangedAccounts> {
        let [state, perp_market, admin, source_vault, drift_signer, quote_spot_market, spot_market_vault, token_program, _remaining @ ..] =
            accounts
        else {
            return None;
        };

        Some(DepositIntoPerpMarketFeePoolInstructionAccounts {
            state: state.pubkey,
            perp_market: perp_market.pubkey,
            admin: admin.pubkey,
            source_vault: source_vault.pubkey,
            drift_signer: drift_signer.pubkey,
            quote_spot_market: quote_spot_market.pubkey,
            spot_market_vault: spot_market_vault.pubkey,
            token_program: token_program.pubkey,
        })
    }
}
