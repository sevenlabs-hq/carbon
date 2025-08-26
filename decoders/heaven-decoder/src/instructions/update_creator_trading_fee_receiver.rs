use carbon_core::{account_utils::next_account, borsh, CarbonDeserialize};

#[derive(
    CarbonDeserialize, Debug, serde::Serialize, serde::Deserialize, PartialEq, Eq, Clone, Hash,
)]
#[carbon(discriminator = "0xf6e5c84f1f157819")]
pub struct UpdateCreatorTradingFeeReceiver {}

#[derive(Debug, PartialEq, Eq, Clone, Hash, serde::Serialize, serde::Deserialize)]
pub struct UpdateCreatorTradingFeeReceiverInstructionAccounts {
    pub swap: solana_pubkey::Pubkey,
    pub new_receiver: solana_pubkey::Pubkey,
}

impl carbon_core::deserialize::ArrangeAccounts for UpdateCreatorTradingFeeReceiver {
    type ArrangedAccounts = UpdateCreatorTradingFeeReceiverInstructionAccounts;

    fn arrange_accounts(
        accounts: &[solana_instruction::AccountMeta],
    ) -> Option<Self::ArrangedAccounts> {
        let mut iter = accounts.iter();
        let swap = next_account(&mut iter)?;
        let new_receiver = next_account(&mut iter)?;

        Some(UpdateCreatorTradingFeeReceiverInstructionAccounts { swap, new_receiver })
    }
}
