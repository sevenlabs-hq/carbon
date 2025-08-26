use carbon_core::{account_utils::next_account, borsh, CarbonDeserialize};

#[derive(
    CarbonDeserialize, Debug, serde::Serialize, serde::Deserialize, PartialEq, Eq, Clone, Hash,
)]
#[carbon(discriminator = "0xb627a819603f4c11")]
pub struct AdminClaimStandardCreatorTradingFees {
    pub amount: u64,
}

#[derive(Debug, PartialEq, Eq, Clone, Hash, serde::Serialize, serde::Deserialize)]
pub struct AdminClaimStandardCreatorTradingFeesInstructionAccounts {
    pub swap: solana_pubkey::Pubkey,
    pub protocol_admin: solana_pubkey::Pubkey,
    pub protocol_admin_state: solana_pubkey::Pubkey,
}

impl carbon_core::deserialize::ArrangeAccounts for AdminClaimStandardCreatorTradingFees {
    type ArrangedAccounts = AdminClaimStandardCreatorTradingFeesInstructionAccounts;

    fn arrange_accounts(
        accounts: &[solana_instruction::AccountMeta],
    ) -> Option<Self::ArrangedAccounts> {
        let mut iter = accounts.iter();
        let swap = next_account(&mut iter)?;
        let protocol_admin = next_account(&mut iter)?;
        let protocol_admin_state = next_account(&mut iter)?;

        Some(AdminClaimStandardCreatorTradingFeesInstructionAccounts {
            swap,
            protocol_admin,
            protocol_admin_state,
        })
    }
}
