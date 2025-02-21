use carbon_core::{borsh, CarbonDeserialize};

#[derive(
    CarbonDeserialize, Debug, serde::Serialize, serde::Deserialize, PartialEq, Eq, Clone, Hash,
)]
#[carbon(discriminator = "0xf92e37391f263d1b")]
pub struct ExecStrategy {
    pub ramp_step: u16,
    pub ramp_duration: u32,
}

pub struct ExecStrategyInstructionAccounts {
    pub strategy: solana_sdk::pubkey::Pubkey,
    pub pool: solana_sdk::pubkey::Pubkey,
}

impl carbon_core::deserialize::ArrangeAccounts for ExecStrategy {
    type ArrangedAccounts = ExecStrategyInstructionAccounts;

    fn arrange_accounts(
        accounts: &[solana_sdk::instruction::AccountMeta],
    ) -> Option<Self::ArrangedAccounts> {
        let [strategy, pool, _remaining @ ..] = accounts else {
            return None;
        };

        Some(ExecStrategyInstructionAccounts {
            strategy: strategy.pubkey,
            pool: pool.pubkey,
        })
    }
}
