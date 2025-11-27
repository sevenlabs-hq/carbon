use carbon_core::{CarbonDeserialize, borsh};

#[derive(
    CarbonDeserialize, Debug, serde::Serialize, serde::Deserialize, PartialEq, Eq, Clone, Hash,
)]
#[carbon(discriminator = "0xd6b0bcf4cb3be6cf")]
pub struct UpdateFarmConfig {
    pub mode: u16,
    pub data: Vec<u8>,
}

pub struct UpdateFarmConfigInstructionAccounts {
    pub signer: solana_pubkey::Pubkey,
    pub farm_state: solana_pubkey::Pubkey,
    pub scope_prices: solana_pubkey::Pubkey,
}

impl carbon_core::deserialize::ArrangeAccounts for UpdateFarmConfig {
    type ArrangedAccounts = UpdateFarmConfigInstructionAccounts;

    fn arrange_accounts(
        accounts: &[solana_instruction::AccountMeta],
    ) -> Option<Self::ArrangedAccounts> {
        let [signer, farm_state, scope_prices, _remaining @ ..] = accounts else {
            return None;
        };

        Some(UpdateFarmConfigInstructionAccounts {
            signer: signer.pubkey,
            farm_state: farm_state.pubkey,
            scope_prices: scope_prices.pubkey,
        })
    }
}
