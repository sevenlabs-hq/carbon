use carbon_core::{borsh, CarbonDeserialize};

#[derive(CarbonDeserialize, Debug, serde::Serialize, serde::Deserialize, PartialEq, Clone)]
#[carbon(discriminator = "0x2B")]
pub struct UpdateMultiplierScaledUiAmount {
    pub new_multiplier: f64,
    pub effective_timestamp: i64,
}

pub struct UpdateMultiplierScaledUiAmountInstructionAccounts {
    pub mint: solana_pubkey::Pubkey,
    pub owner: solana_pubkey::Pubkey,
}

impl carbon_core::deserialize::ArrangeAccounts for UpdateMultiplierScaledUiAmount {
    type ArrangedAccounts = UpdateMultiplierScaledUiAmountInstructionAccounts;

    fn arrange_accounts(
        accounts: &[solana_instruction::AccountMeta],
    ) -> Option<Self::ArrangedAccounts> {
        let [mint, owner, _remaining @ ..] = accounts else {
            return None;
        };

        Some(UpdateMultiplierScaledUiAmountInstructionAccounts {
            mint: mint.pubkey,
            owner: owner.pubkey,
        })
    }
}
