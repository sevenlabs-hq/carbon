use carbon_core::{borsh, CarbonDeserialize};

#[derive(
    CarbonDeserialize, Debug, serde::Serialize, serde::Deserialize, PartialEq, Eq, Clone, Hash,
)]
#[carbon(discriminator = "0xafaf6d1f0d989bed")]
pub struct Initialize {}

pub struct InitializeInstructionAccounts {
    pub admin: solana_pubkey::Pubkey,
    pub state: solana_pubkey::Pubkey,
    pub quote_asset_mint: solana_pubkey::Pubkey,
    pub drift_signer: solana_pubkey::Pubkey,
    pub rent: solana_pubkey::Pubkey,
    pub system_program: solana_pubkey::Pubkey,
    pub token_program: solana_pubkey::Pubkey,
}

impl carbon_core::deserialize::ArrangeAccounts for Initialize {
    type ArrangedAccounts = InitializeInstructionAccounts;

    fn arrange_accounts(
        accounts: &[solana_sdk::instruction::AccountMeta],
    ) -> Option<Self::ArrangedAccounts> {
        let [admin, state, quote_asset_mint, drift_signer, rent, system_program, token_program, _remaining @ ..] =
            accounts
        else {
            return None;
        };

        Some(InitializeInstructionAccounts {
            admin: admin.pubkey,
            state: state.pubkey,
            quote_asset_mint: quote_asset_mint.pubkey,
            drift_signer: drift_signer.pubkey,
            rent: rent.pubkey,
            system_program: system_program.pubkey,
            token_program: token_program.pubkey,
        })
    }
}
