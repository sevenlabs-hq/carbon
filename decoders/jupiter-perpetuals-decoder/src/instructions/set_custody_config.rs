use {
    super::super::types::*,
    carbon_core::{borsh, CarbonDeserialize},
};

#[derive(
    CarbonDeserialize, Debug, serde::Serialize, serde::Deserialize, PartialEq, Eq, Clone, Hash,
)]
#[carbon(discriminator = "0x8561828fd7e524b0")]
pub struct SetCustodyConfig {
    pub params: SetCustodyConfigParams,
}

pub struct SetCustodyConfigInstructionAccounts {
    pub admin: solana_pubkey::Pubkey,
    pub perpetuals: solana_pubkey::Pubkey,
    pub custody: solana_pubkey::Pubkey,
}

impl carbon_core::deserialize::ArrangeAccounts for SetCustodyConfig {
    type ArrangedAccounts = SetCustodyConfigInstructionAccounts;

    fn arrange_accounts(
        accounts: &[solana_instruction::AccountMeta],
    ) -> Option<Self::ArrangedAccounts> {
        let [admin, perpetuals, custody, _remaining @ ..] = accounts else {
            return None;
        };

        Some(SetCustodyConfigInstructionAccounts {
            admin: admin.pubkey,
            perpetuals: perpetuals.pubkey,
            custody: custody.pubkey,
        })
    }
}
