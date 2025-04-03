use carbon_core::{borsh, CarbonDeserialize};

#[derive(
    CarbonDeserialize, Debug, serde::Serialize, serde::Deserialize, PartialEq, Eq, Clone, Hash,
)]
#[carbon(discriminator = "0xaf6b1338a5f12b45")]
pub struct SweepFuel {}

pub struct SweepFuelInstructionAccounts {
    pub fuel_overflow: solana_pubkey::Pubkey,
    pub user_stats: solana_pubkey::Pubkey,
    pub authority: solana_pubkey::Pubkey,
    pub signer: solana_pubkey::Pubkey,
}

impl carbon_core::deserialize::ArrangeAccounts for SweepFuel {
    type ArrangedAccounts = SweepFuelInstructionAccounts;

    fn arrange_accounts(
        accounts: &[solana_sdk::instruction::AccountMeta],
    ) -> Option<Self::ArrangedAccounts> {
        let [fuel_overflow, user_stats, authority, signer, _remaining @ ..] = accounts else {
            return None;
        };

        Some(SweepFuelInstructionAccounts {
            fuel_overflow: fuel_overflow.pubkey,
            user_stats: user_stats.pubkey,
            authority: authority.pubkey,
            signer: signer.pubkey,
        })
    }
}
