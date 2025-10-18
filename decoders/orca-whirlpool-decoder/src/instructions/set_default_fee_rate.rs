use carbon_core::{account_utils::next_account, borsh, CarbonDeserialize};

#[derive(
    CarbonDeserialize, Debug, serde::Serialize, serde::Deserialize, PartialEq, Eq, Clone, Hash,
)]
#[carbon(discriminator = "0x76d7d69db6e5d0e4")]
pub struct SetDefaultFeeRate {
    pub default_fee_rate: u16,
}

#[derive(Debug, PartialEq, Eq, Clone, Hash, serde::Serialize, serde::Deserialize)]
pub struct SetDefaultFeeRateInstructionAccounts {
    pub whirlpools_config: solana_pubkey::Pubkey,
    pub fee_tier: solana_pubkey::Pubkey,
    pub fee_authority: solana_pubkey::Pubkey,
}

impl carbon_core::deserialize::ArrangeAccounts for SetDefaultFeeRate {
    type ArrangedAccounts = SetDefaultFeeRateInstructionAccounts;

    fn arrange_accounts(
        accounts: &[solana_instruction::AccountMeta],
    ) -> Option<Self::ArrangedAccounts> {
        let mut iter = accounts.iter();
        let whirlpools_config = next_account(&mut iter)?;
        let fee_tier = next_account(&mut iter)?;
        let fee_authority = next_account(&mut iter)?;

        Some(SetDefaultFeeRateInstructionAccounts {
            whirlpools_config,
            fee_tier,
            fee_authority,
        })
    }
}
