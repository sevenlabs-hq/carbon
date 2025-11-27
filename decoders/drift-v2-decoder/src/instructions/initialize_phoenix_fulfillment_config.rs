use carbon_core::{borsh, CarbonDeserialize};

#[derive(
    CarbonDeserialize, Debug, serde::Serialize, serde::Deserialize, PartialEq, Eq, Clone, Hash,
)]
#[carbon(discriminator = "0x87846e6bb9a0a99a")]
pub struct InitializePhoenixFulfillmentConfig {
    pub market_index: u16,
}

pub struct InitializePhoenixFulfillmentConfigInstructionAccounts {
    pub base_spot_market: solana_pubkey::Pubkey,
    pub quote_spot_market: solana_pubkey::Pubkey,
    pub state: solana_pubkey::Pubkey,
    pub phoenix_program: solana_pubkey::Pubkey,
    pub phoenix_market: solana_pubkey::Pubkey,
    pub drift_signer: solana_pubkey::Pubkey,
    pub phoenix_fulfillment_config: solana_pubkey::Pubkey,
    pub admin: solana_pubkey::Pubkey,
    pub rent: solana_pubkey::Pubkey,
    pub system_program: solana_pubkey::Pubkey,
}

impl carbon_core::deserialize::ArrangeAccounts for InitializePhoenixFulfillmentConfig {
    type ArrangedAccounts = InitializePhoenixFulfillmentConfigInstructionAccounts;

    fn arrange_accounts(
        accounts: &[solana_instruction::AccountMeta],
    ) -> Option<Self::ArrangedAccounts> {
        let [base_spot_market, quote_spot_market, state, phoenix_program, phoenix_market, drift_signer, phoenix_fulfillment_config, admin, rent, system_program, _remaining @ ..] =
            accounts
        else {
            return None;
        };

        Some(InitializePhoenixFulfillmentConfigInstructionAccounts {
            base_spot_market: base_spot_market.pubkey,
            quote_spot_market: quote_spot_market.pubkey,
            state: state.pubkey,
            phoenix_program: phoenix_program.pubkey,
            phoenix_market: phoenix_market.pubkey,
            drift_signer: drift_signer.pubkey,
            phoenix_fulfillment_config: phoenix_fulfillment_config.pubkey,
            admin: admin.pubkey,
            rent: rent.pubkey,
            system_program: system_program.pubkey,
        })
    }
}
