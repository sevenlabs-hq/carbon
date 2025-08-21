use carbon_core::{account_utils::next_account, borsh, CarbonDeserialize};

#[derive(
    CarbonDeserialize, Debug, serde::Serialize, serde::Deserialize, PartialEq, Eq, Clone, Hash,
)]
#[carbon(discriminator = "0xe54254fba486b707")]
pub struct SetDefaultBaseFeeRate {
    pub default_base_fee_rate: u16,
}

#[derive(Debug, PartialEq, Eq, Clone, Hash, serde::Serialize, serde::Deserialize)]
pub struct SetDefaultBaseFeeRateInstructionAccounts {
    pub whirlpools_config: solana_pubkey::Pubkey,
    pub adaptive_fee_tier: solana_pubkey::Pubkey,
    pub fee_authority: solana_pubkey::Pubkey,
}

impl carbon_core::deserialize::ArrangeAccounts for SetDefaultBaseFeeRate {
    type ArrangedAccounts = SetDefaultBaseFeeRateInstructionAccounts;

    fn arrange_accounts(
        accounts: &[solana_instruction::AccountMeta],
    ) -> Option<Self::ArrangedAccounts> {
        let mut iter = accounts.iter();
        let whirlpools_config = next_account(&mut iter)?;
        let adaptive_fee_tier = next_account(&mut iter)?;
        let fee_authority = next_account(&mut iter)?;

        Some(SetDefaultBaseFeeRateInstructionAccounts {
            whirlpools_config,
            adaptive_fee_tier,
            fee_authority,
        })
    }
}
