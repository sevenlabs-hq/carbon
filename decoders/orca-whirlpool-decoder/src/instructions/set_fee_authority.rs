use carbon_core::{borsh, CarbonDeserialize};
#[derive(
    CarbonDeserialize, Debug, serde::Serialize, serde::Deserialize, PartialEq, Eq, Clone, Hash,
)]
#[carbon(discriminator = "0x1f013257ed656184")]
pub struct SetFeeAuthority {}

pub struct SetFeeAuthorityInstructionAccounts {
    pub whirlpools_config: solana_sdk::pubkey::Pubkey,
    pub fee_authority: solana_sdk::pubkey::Pubkey,
    pub new_fee_authority: solana_sdk::pubkey::Pubkey,
}

impl carbon_core::deserialize::ArrangeAccounts for SetFeeAuthority {
    type ArrangedAccounts = SetFeeAuthorityInstructionAccounts;

fn arrange_accounts(
        accounts: Vec<solana_sdk::instruction::AccountMeta>,
    ) -> Option<Self::ArrangedAccounts> {
        let whirlpools_config = accounts.get(0)?;
        let fee_authority = accounts.get(1)?;
        let new_fee_authority = accounts.get(2)?;

        Some(SetFeeAuthorityInstructionAccounts {
            whirlpools_config: whirlpools_config.pubkey,
            fee_authority: fee_authority.pubkey,
            new_fee_authority: new_fee_authority.pubkey,
        })
    }
}
