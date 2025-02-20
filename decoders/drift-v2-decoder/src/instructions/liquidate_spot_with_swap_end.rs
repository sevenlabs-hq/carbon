use carbon_core::{borsh, CarbonDeserialize};

#[derive(
    CarbonDeserialize, Debug, serde::Serialize, serde::Deserialize, PartialEq, Eq, Clone, Hash,
)]
#[carbon(discriminator = "0x8e58a3a0df4b37e1")]
pub struct LiquidateSpotWithSwapEnd {
    pub asset_market_index: u16,
    pub liability_market_index: u16,
}

pub struct LiquidateSpotWithSwapEndInstructionAccounts {
    pub state: solana_sdk::pubkey::Pubkey,
    pub authority: solana_sdk::pubkey::Pubkey,
    pub liquidator: solana_sdk::pubkey::Pubkey,
    pub liquidator_stats: solana_sdk::pubkey::Pubkey,
    pub user: solana_sdk::pubkey::Pubkey,
    pub user_stats: solana_sdk::pubkey::Pubkey,
    pub liability_spot_market_vault: solana_sdk::pubkey::Pubkey,
    pub asset_spot_market_vault: solana_sdk::pubkey::Pubkey,
    pub liability_token_account: solana_sdk::pubkey::Pubkey,
    pub asset_token_account: solana_sdk::pubkey::Pubkey,
    pub token_program: solana_sdk::pubkey::Pubkey,
    pub drift_signer: solana_sdk::pubkey::Pubkey,
    pub instructions: solana_sdk::pubkey::Pubkey,
}

impl carbon_core::deserialize::ArrangeAccounts for LiquidateSpotWithSwapEnd {
    type ArrangedAccounts = LiquidateSpotWithSwapEndInstructionAccounts;

    fn arrange_accounts(
        accounts: &[solana_sdk::instruction::AccountMeta],
    ) -> Option<Self::ArrangedAccounts> {
        let [state, authority, liquidator, liquidator_stats, user, user_stats, liability_spot_market_vault, asset_spot_market_vault, liability_token_account, asset_token_account, token_program, drift_signer, instructions, _remaining @ ..] =
            accounts
        else {
            return None;
        };

        Some(LiquidateSpotWithSwapEndInstructionAccounts {
            state: state.pubkey,
            authority: authority.pubkey,
            liquidator: liquidator.pubkey,
            liquidator_stats: liquidator_stats.pubkey,
            user: user.pubkey,
            user_stats: user_stats.pubkey,
            liability_spot_market_vault: liability_spot_market_vault.pubkey,
            asset_spot_market_vault: asset_spot_market_vault.pubkey,
            liability_token_account: liability_token_account.pubkey,
            asset_token_account: asset_token_account.pubkey,
            token_program: token_program.pubkey,
            drift_signer: drift_signer.pubkey,
            instructions: instructions.pubkey,
        })
    }
}
