use {
    super::super::types::*,
    carbon_core::{borsh, CarbonDeserialize},
};

#[derive(
    CarbonDeserialize, Debug, serde::Serialize, serde::Deserialize, PartialEq, Eq, Clone, Hash,
)]
#[carbon(discriminator = "0xd857417d716eb978")]
pub struct SetPoolConfig {
    pub params: SetPoolConfigParams,
}

pub struct SetPoolConfigInstructionAccounts {
    pub admin: solana_pubkey::Pubkey,
    pub perpetuals: solana_pubkey::Pubkey,
    pub pool: solana_pubkey::Pubkey,
}

impl carbon_core::deserialize::ArrangeAccounts for SetPoolConfig {
    type ArrangedAccounts = SetPoolConfigInstructionAccounts;

    fn arrange_accounts(
        accounts: &[solana_instruction::AccountMeta],
    ) -> Option<Self::ArrangedAccounts> {
        let [admin, perpetuals, pool, _remaining @ ..] = accounts else {
            return None;
        };

        Some(SetPoolConfigInstructionAccounts {
            admin: admin.pubkey,
            perpetuals: perpetuals.pubkey,
            pool: pool.pubkey,
        })
    }
}
