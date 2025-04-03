use carbon_core::{borsh, CarbonDeserialize};

#[derive(
    CarbonDeserialize, Debug, serde::Serialize, serde::Deserialize, PartialEq, Eq, Clone, Hash,
)]
#[carbon(discriminator = "0xde5aca5e1c2d73b7")]
pub struct SettleFundingPayment {}

pub struct SettleFundingPaymentInstructionAccounts {
    pub state: solana_pubkey::Pubkey,
    pub user: solana_pubkey::Pubkey,
}

impl carbon_core::deserialize::ArrangeAccounts for SettleFundingPayment {
    type ArrangedAccounts = SettleFundingPaymentInstructionAccounts;

    fn arrange_accounts(
        accounts: &[solana_instruction::AccountMeta],
    ) -> Option<Self::ArrangedAccounts> {
        let [state, user, _remaining @ ..] = accounts else {
            return None;
        };

        Some(SettleFundingPaymentInstructionAccounts {
            state: state.pubkey,
            user: user.pubkey,
        })
    }
}
