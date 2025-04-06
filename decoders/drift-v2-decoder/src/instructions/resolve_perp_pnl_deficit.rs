use carbon_core::{borsh, CarbonDeserialize};

#[derive(
    CarbonDeserialize, Debug, serde::Serialize, serde::Deserialize, PartialEq, Eq, Clone, Hash,
)]
#[carbon(discriminator = "0xa8cc44969f7e5f94")]
pub struct ResolvePerpPnlDeficit {
    pub spot_market_index: u16,
    pub perp_market_index: u16,
}

pub struct ResolvePerpPnlDeficitInstructionAccounts {
    pub state: solana_pubkey::Pubkey,
    pub authority: solana_pubkey::Pubkey,
    pub spot_market_vault: solana_pubkey::Pubkey,
    pub insurance_fund_vault: solana_pubkey::Pubkey,
    pub drift_signer: solana_pubkey::Pubkey,
    pub token_program: solana_pubkey::Pubkey,
}

impl carbon_core::deserialize::ArrangeAccounts for ResolvePerpPnlDeficit {
    type ArrangedAccounts = ResolvePerpPnlDeficitInstructionAccounts;

    fn arrange_accounts(
        accounts: &[solana_instruction::AccountMeta],
    ) -> Option<Self::ArrangedAccounts> {
        let [state, authority, spot_market_vault, insurance_fund_vault, drift_signer, token_program, _remaining @ ..] =
            accounts
        else {
            return None;
        };

        Some(ResolvePerpPnlDeficitInstructionAccounts {
            state: state.pubkey,
            authority: authority.pubkey,
            spot_market_vault: spot_market_vault.pubkey,
            insurance_fund_vault: insurance_fund_vault.pubkey,
            drift_signer: drift_signer.pubkey,
            token_program: token_program.pubkey,
        })
    }
}
