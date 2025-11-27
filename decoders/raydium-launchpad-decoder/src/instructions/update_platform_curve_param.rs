use super::super::types::*;

use carbon_core::{account_utils::next_account, borsh, CarbonDeserialize};

#[derive(
    CarbonDeserialize, Debug, serde::Serialize, serde::Deserialize, PartialEq, Eq, Clone, Hash,
)]
#[carbon(discriminator = "0x8a908afadc800439")]
pub struct UpdatePlatformCurveParam {
    pub index: u8,
    pub bonding_curve_param: BondingCurveParam,
}

#[derive(Debug, PartialEq, Eq, Clone, Hash, serde::Serialize, serde::Deserialize)]
pub struct UpdatePlatformCurveParamInstructionAccounts {
    pub platform_admin: solana_pubkey::Pubkey,
    pub platform_config: solana_pubkey::Pubkey,
    pub global_config: solana_pubkey::Pubkey,
    pub system_program: solana_pubkey::Pubkey,
}

impl carbon_core::deserialize::ArrangeAccounts for UpdatePlatformCurveParam {
    type ArrangedAccounts = UpdatePlatformCurveParamInstructionAccounts;

    fn arrange_accounts(
        accounts: &[solana_instruction::AccountMeta],
    ) -> Option<Self::ArrangedAccounts> {
        let mut iter = accounts.iter();
        let platform_admin = next_account(&mut iter)?;
        let platform_config = next_account(&mut iter)?;
        let global_config = next_account(&mut iter)?;
        let system_program = next_account(&mut iter)?;

        Some(UpdatePlatformCurveParamInstructionAccounts {
            platform_admin,
            platform_config,
            global_config,
            system_program,
        })
    }
}
