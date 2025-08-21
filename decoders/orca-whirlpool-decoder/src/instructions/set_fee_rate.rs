use carbon_core::{account_utils::next_account, borsh, CarbonDeserialize};

#[derive(
    CarbonDeserialize, Debug, serde::Serialize, serde::Deserialize, PartialEq, Eq, Clone, Hash,
)]
#[carbon(discriminator = "0x35f38941088c9e06")]
pub struct SetFeeRate {
    pub fee_rate: u16,
}

#[derive(Debug, PartialEq, Eq, Clone, Hash, serde::Serialize, serde::Deserialize)]
pub struct SetFeeRateInstructionAccounts {
    pub whirlpools_config: solana_pubkey::Pubkey,
    pub whirlpool: solana_pubkey::Pubkey,
    pub fee_authority: solana_pubkey::Pubkey,
}

impl carbon_core::deserialize::ArrangeAccounts for SetFeeRate {
    type ArrangedAccounts = SetFeeRateInstructionAccounts;

    fn arrange_accounts(
        accounts: &[solana_instruction::AccountMeta],
    ) -> Option<Self::ArrangedAccounts> {
        let mut iter = accounts.iter();
        let whirlpools_config = next_account(&mut iter)?;
        let whirlpool = next_account(&mut iter)?;
        let fee_authority = next_account(&mut iter)?;

        Some(SetFeeRateInstructionAccounts {
            whirlpools_config,
            whirlpool,
            fee_authority,
        })
    }
}
