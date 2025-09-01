use carbon_core::{account_utils::next_account, borsh, CarbonDeserialize};

#[derive(
    CarbonDeserialize, Debug, serde::Serialize, serde::Deserialize, PartialEq, Eq, Clone, Hash,
)]
#[carbon(discriminator = "0x00c9e2e47f2d456e")]
pub struct ClaimStandardCreatorTradingFeeProtocolFees {
    pub amount: u64,
}

#[derive(Debug, PartialEq, Eq, Clone, Hash, serde::Serialize, serde::Deserialize)]
pub struct ClaimStandardCreatorTradingFeeProtocolFeesInstructionAccounts {
    pub swap: solana_pubkey::Pubkey,
    pub protocol_admin: solana_pubkey::Pubkey,
    pub protocol_admin_state: solana_pubkey::Pubkey,
}

impl carbon_core::deserialize::ArrangeAccounts for ClaimStandardCreatorTradingFeeProtocolFees {
    type ArrangedAccounts = ClaimStandardCreatorTradingFeeProtocolFeesInstructionAccounts;

    fn arrange_accounts(
        accounts: &[solana_instruction::AccountMeta],
    ) -> Option<Self::ArrangedAccounts> {
        let mut iter = accounts.iter();
        let swap = next_account(&mut iter)?;
        let protocol_admin = next_account(&mut iter)?;
        let protocol_admin_state = next_account(&mut iter)?;

        Some(
            ClaimStandardCreatorTradingFeeProtocolFeesInstructionAccounts {
                swap,
                protocol_admin,
                protocol_admin_state,
            },
        )
    }
}
