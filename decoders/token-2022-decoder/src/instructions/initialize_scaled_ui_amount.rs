use carbon_core::{borsh, CarbonDeserialize};

#[derive(CarbonDeserialize, Debug, serde::Serialize, serde::Deserialize, PartialEq, Clone)]
#[carbon(discriminator = "0x2B")]
pub struct InitializedScaledUiAmount {
    pub multiplier: f64,
    pub authority: Option<solana_pubkey::Pubkey>,
}

pub struct InitializeScaledUiAmountInstructionAccounts {
    pub mint: solana_pubkey::Pubkey,
}

impl carbon_core::deserialize::ArrangeAccounts for InitializedScaledUiAmount {
    type ArrangedAccounts = InitializeScaledUiAmountInstructionAccounts;

    fn arrange_accounts(
        accounts: &[solana_instruction::AccountMeta],
    ) -> Option<Self::ArrangedAccounts> {
        let [mint, _remaining @ ..] = accounts else {
            return None;
        };

        Some(InitializeScaledUiAmountInstructionAccounts { mint: mint.pubkey })
    }
}
