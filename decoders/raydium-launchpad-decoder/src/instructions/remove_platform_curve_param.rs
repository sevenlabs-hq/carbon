use carbon_core::{account_utils::next_account, borsh, CarbonDeserialize};

#[derive(
    CarbonDeserialize, Debug, serde::Serialize, serde::Deserialize, PartialEq, Eq, Clone, Hash,
)]
#[carbon(discriminator = "0x1b1e3ea95de01891")]
pub struct RemovePlatformCurveParam {
    pub index: u8,
}

#[derive(Debug, PartialEq, Eq, Clone, Hash, serde::Serialize, serde::Deserialize)]
pub struct RemovePlatformCurveParamInstructionAccounts {
    pub platform_admin: solana_pubkey::Pubkey,
    pub platform_config: solana_pubkey::Pubkey,
}

impl carbon_core::deserialize::ArrangeAccounts for RemovePlatformCurveParam {
    type ArrangedAccounts = RemovePlatformCurveParamInstructionAccounts;

    fn arrange_accounts(
        accounts: &[solana_instruction::AccountMeta],
    ) -> Option<Self::ArrangedAccounts> {
        let mut iter = accounts.iter();
        let platform_admin = next_account(&mut iter)?;
        let platform_config = next_account(&mut iter)?;

        Some(RemovePlatformCurveParamInstructionAccounts {
            platform_admin,
            platform_config,
        })
    }
}
