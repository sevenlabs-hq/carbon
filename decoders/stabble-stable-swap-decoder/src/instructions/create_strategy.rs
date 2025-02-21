use carbon_core::{borsh, CarbonDeserialize};

#[derive(
    CarbonDeserialize, Debug, serde::Serialize, serde::Deserialize, PartialEq, Eq, Clone, Hash,
)]
#[carbon(discriminator = "0x98a06b94f5be7fe0")]
pub struct CreateStrategy {
    pub amp_min_factor: u16,
    pub amp_max_factor: u16,
    pub ramp_min_step: u16,
    pub ramp_max_step: u16,
    pub ramp_min_duration: u32,
    pub ramp_max_duration: u32,
}

pub struct CreateStrategyInstructionAccounts {
    pub owner_only: solana_sdk::pubkey::Pubkey,
    pub strategy: solana_sdk::pubkey::Pubkey,
}

impl carbon_core::deserialize::ArrangeAccounts for CreateStrategy {
    type ArrangedAccounts = CreateStrategyInstructionAccounts;

    fn arrange_accounts(
        accounts: &[solana_sdk::instruction::AccountMeta],
    ) -> Option<Self::ArrangedAccounts> {
        let [owner_only, strategy, _remaining @ ..] = accounts else {
            return None;
        };

        Some(CreateStrategyInstructionAccounts {
            owner_only: owner_only.pubkey,
            strategy: strategy.pubkey,
        })
    }
}
