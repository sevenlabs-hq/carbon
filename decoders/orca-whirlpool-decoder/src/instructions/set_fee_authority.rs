use carbon_core::{account_utils::next_account, borsh, CarbonDeserialize};

#[derive(
    CarbonDeserialize, Debug, serde::Serialize, serde::Deserialize, PartialEq, Eq, Clone, Hash,
)]
#[carbon(discriminator = "0x1f013257ed656184")]
pub struct SetFeeAuthority {}

#[derive(Debug, PartialEq, Eq, Clone, Hash, serde::Serialize, serde::Deserialize)]
pub struct SetFeeAuthorityInstructionAccounts {
    pub whirlpools_config: solana_pubkey::Pubkey,
    pub fee_authority: solana_pubkey::Pubkey,
    pub new_fee_authority: solana_pubkey::Pubkey,
}

impl carbon_core::deserialize::ArrangeAccounts for SetFeeAuthority {
    type ArrangedAccounts = SetFeeAuthorityInstructionAccounts;

    fn arrange_accounts(
        accounts: &[solana_instruction::AccountMeta],
    ) -> Option<Self::ArrangedAccounts> {
        let mut iter = accounts.iter();
        let whirlpools_config = next_account(&mut iter)?;
        let fee_authority = next_account(&mut iter)?;
        let new_fee_authority = next_account(&mut iter)?;

        Some(SetFeeAuthorityInstructionAccounts {
            whirlpools_config,
            fee_authority,
            new_fee_authority,
        })
    }
}
