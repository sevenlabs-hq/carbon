use carbon_core::{account_utils::next_account, borsh, CarbonDeserialize};

#[derive(
    CarbonDeserialize, Debug, serde::Serialize, serde::Deserialize, PartialEq, Eq, Clone, Hash,
)]
#[carbon(discriminator = "0x84b94294535886c6")]
pub struct SetPresetAdaptiveFeeConstants {
    pub filter_period: u16,
    pub decay_period: u16,
    pub reduction_factor: u16,
    pub adaptive_fee_control_factor: u32,
    pub max_volatility_accumulator: u32,
    pub tick_group_size: u16,
    pub major_swap_threshold_ticks: u16,
}

#[derive(Debug, PartialEq, Eq, Clone, Hash, serde::Serialize, serde::Deserialize)]
pub struct SetPresetAdaptiveFeeConstantsInstructionAccounts {
    pub whirlpools_config: solana_pubkey::Pubkey,
    pub adaptive_fee_tier: solana_pubkey::Pubkey,
    pub fee_authority: solana_pubkey::Pubkey,
}

impl carbon_core::deserialize::ArrangeAccounts for SetPresetAdaptiveFeeConstants {
    type ArrangedAccounts = SetPresetAdaptiveFeeConstantsInstructionAccounts;

    fn arrange_accounts(
        accounts: &[solana_instruction::AccountMeta],
    ) -> Option<Self::ArrangedAccounts> {
        let mut iter = accounts.iter();
        let whirlpools_config = next_account(&mut iter)?;
        let adaptive_fee_tier = next_account(&mut iter)?;
        let fee_authority = next_account(&mut iter)?;

        Some(SetPresetAdaptiveFeeConstantsInstructionAccounts {
            whirlpools_config,
            adaptive_fee_tier,
            fee_authority,
        })
    }
}
