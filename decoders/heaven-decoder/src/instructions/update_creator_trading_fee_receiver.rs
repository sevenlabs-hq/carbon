use carbon_core::{borsh, CarbonDeserialize};

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
        let [swap, new_receiver, _remaining @ ..] = accounts else {
            return None;
        };

        Some(UpdateCreatorTradingFeeReceiverInstructionAccounts {
            swap: swap.pubkey,
            new_receiver: new_receiver.pubkey,
        })
    }
}
