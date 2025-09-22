use carbon_core::{account_utils::next_account, borsh, CarbonDeserialize};

#[derive(
    CarbonDeserialize, Debug, serde::Serialize, serde::Deserialize, PartialEq, Eq, Clone, Hash,
)]
#[carbon(discriminator = "0x7979367283e6a268")]
pub struct SetFeeRateByDelegatedFeeAuthority {
    pub fee_rate: u16,
}

#[derive(Debug, PartialEq, Eq, Clone, Hash, serde::Serialize, serde::Deserialize)]
pub struct SetFeeRateByDelegatedFeeAuthorityInstructionAccounts {
    pub whirlpool: solana_pubkey::Pubkey,
    pub adaptive_fee_tier: solana_pubkey::Pubkey,
    pub delegated_fee_authority: solana_pubkey::Pubkey,
}

impl carbon_core::deserialize::ArrangeAccounts for SetFeeRateByDelegatedFeeAuthority {
    type ArrangedAccounts = SetFeeRateByDelegatedFeeAuthorityInstructionAccounts;

    fn arrange_accounts(
        accounts: &[solana_instruction::AccountMeta],
    ) -> Option<Self::ArrangedAccounts> {
        let mut iter = accounts.iter();
        let whirlpool = next_account(&mut iter)?;
        let adaptive_fee_tier = next_account(&mut iter)?;
        let delegated_fee_authority = next_account(&mut iter)?;

        Some(SetFeeRateByDelegatedFeeAuthorityInstructionAccounts {
            whirlpool,
            adaptive_fee_tier,
            delegated_fee_authority,
        })
    }
}
