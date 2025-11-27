use carbon_core::{CarbonDeserialize, borsh};

#[derive(
    CarbonDeserialize, Debug, serde::Serialize, serde::Deserialize, PartialEq, Eq, Clone, Hash,
)]
#[carbon(discriminator = "0x1f8c43bfbd1465dd")]
pub struct DeleteInitializedSpotMarket {
    pub market_index: u16,
}

pub struct DeleteInitializedSpotMarketInstructionAccounts {
    pub admin: solana_pubkey::Pubkey,
    pub state: solana_pubkey::Pubkey,
    pub spot_market: solana_pubkey::Pubkey,
    pub spot_market_vault: solana_pubkey::Pubkey,
    pub insurance_fund_vault: solana_pubkey::Pubkey,
    pub drift_signer: solana_pubkey::Pubkey,
    pub token_program: solana_pubkey::Pubkey,
}

impl carbon_core::deserialize::ArrangeAccounts for DeleteInitializedSpotMarket {
    type ArrangedAccounts = DeleteInitializedSpotMarketInstructionAccounts;

    fn arrange_accounts(
        accounts: &[solana_instruction::AccountMeta],
    ) -> Option<Self::ArrangedAccounts> {
        let [
            admin,
            state,
            spot_market,
            spot_market_vault,
            insurance_fund_vault,
            drift_signer,
            token_program,
            _remaining @ ..,
        ] = accounts
        else {
            return None;
        };

        Some(DeleteInitializedSpotMarketInstructionAccounts {
            admin: admin.pubkey,
            state: state.pubkey,
            spot_market: spot_market.pubkey,
            spot_market_vault: spot_market_vault.pubkey,
            insurance_fund_vault: insurance_fund_vault.pubkey,
            drift_signer: drift_signer.pubkey,
            token_program: token_program.pubkey,
        })
    }
}
