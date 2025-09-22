use carbon_core::{account_utils::next_account, borsh, CarbonDeserialize};

#[derive(
    CarbonDeserialize, Debug, serde::Serialize, serde::Deserialize, PartialEq, Eq, Clone, Hash,
)]
#[carbon(discriminator = "0xc1eae7938a39037a")]
pub struct SetDelegatedFeeAuthority {}

#[derive(Debug, PartialEq, Eq, Clone, Hash, serde::Serialize, serde::Deserialize)]
pub struct SetDelegatedFeeAuthorityInstructionAccounts {
    pub whirlpools_config: solana_pubkey::Pubkey,
    pub adaptive_fee_tier: solana_pubkey::Pubkey,
    pub fee_authority: solana_pubkey::Pubkey,
    pub new_delegated_fee_authority: solana_pubkey::Pubkey,
}

impl carbon_core::deserialize::ArrangeAccounts for SetDelegatedFeeAuthority {
    type ArrangedAccounts = SetDelegatedFeeAuthorityInstructionAccounts;

    fn arrange_accounts(
        accounts: &[solana_instruction::AccountMeta],
    ) -> Option<Self::ArrangedAccounts> {
        let mut iter = accounts.iter();
        let whirlpools_config = next_account(&mut iter)?;
        let adaptive_fee_tier = next_account(&mut iter)?;
        let fee_authority = next_account(&mut iter)?;
        let new_delegated_fee_authority = next_account(&mut iter)?;

        Some(SetDelegatedFeeAuthorityInstructionAccounts {
            whirlpools_config,
            adaptive_fee_tier,
            fee_authority,
            new_delegated_fee_authority,
        })
    }
}
