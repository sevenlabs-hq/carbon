use carbon_core::{CarbonDeserialize, borsh};

#[derive(
    CarbonDeserialize, Debug, serde::Serialize, serde::Deserialize, PartialEq, Eq, Clone, Hash,
)]
#[carbon(discriminator = "0xe010b0d6a2d5b7de")]
pub struct ResolvePerpBankruptcy {
    pub quote_spot_market_index: u16,
    pub market_index: u16,
}

pub struct ResolvePerpBankruptcyInstructionAccounts {
    pub state: solana_pubkey::Pubkey,
    pub authority: solana_pubkey::Pubkey,
    pub liquidator: solana_pubkey::Pubkey,
    pub liquidator_stats: solana_pubkey::Pubkey,
    pub user: solana_pubkey::Pubkey,
    pub user_stats: solana_pubkey::Pubkey,
    pub spot_market_vault: solana_pubkey::Pubkey,
    pub insurance_fund_vault: solana_pubkey::Pubkey,
    pub drift_signer: solana_pubkey::Pubkey,
    pub token_program: solana_pubkey::Pubkey,
}

impl carbon_core::deserialize::ArrangeAccounts for ResolvePerpBankruptcy {
    type ArrangedAccounts = ResolvePerpBankruptcyInstructionAccounts;

    fn arrange_accounts(
        accounts: &[solana_instruction::AccountMeta],
    ) -> Option<Self::ArrangedAccounts> {
        let [
            state,
            authority,
            liquidator,
            liquidator_stats,
            user,
            user_stats,
            spot_market_vault,
            insurance_fund_vault,
            drift_signer,
            token_program,
            _remaining @ ..,
        ] = accounts
        else {
            return None;
        };

        Some(ResolvePerpBankruptcyInstructionAccounts {
            state: state.pubkey,
            authority: authority.pubkey,
            liquidator: liquidator.pubkey,
            liquidator_stats: liquidator_stats.pubkey,
            user: user.pubkey,
            user_stats: user_stats.pubkey,
            spot_market_vault: spot_market_vault.pubkey,
            insurance_fund_vault: insurance_fund_vault.pubkey,
            drift_signer: drift_signer.pubkey,
            token_program: token_program.pubkey,
        })
    }
}
