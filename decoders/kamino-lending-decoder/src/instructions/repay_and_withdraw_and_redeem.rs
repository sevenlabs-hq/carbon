use carbon_core::{borsh, CarbonDeserialize};

#[derive(
    CarbonDeserialize, Debug, serde::Serialize, serde::Deserialize, PartialEq, Eq, Clone, Hash,
)]
#[carbon(discriminator = "0x0236980394606dda")]
pub struct RepayAndWithdrawAndRedeem {
    pub repay_amount: u64,
    pub withdraw_collateral_amount: u64,
}

pub struct RepayAndWithdrawAndRedeemInstructionAccounts {
    pub repay_accounts: solana_pubkey::Pubkey,
    pub withdraw_accounts: solana_pubkey::Pubkey,
}

impl carbon_core::deserialize::ArrangeAccounts for RepayAndWithdrawAndRedeem {
    type ArrangedAccounts = RepayAndWithdrawAndRedeemInstructionAccounts;

    fn arrange_accounts(
        accounts: &[solana_instruction::AccountMeta],
    ) -> Option<Self::ArrangedAccounts> {
        let [repay_accounts, withdraw_accounts, _remaining @ ..] = accounts else {
            return None;
        };

        Some(RepayAndWithdrawAndRedeemInstructionAccounts {
            repay_accounts: repay_accounts.pubkey,
            withdraw_accounts: withdraw_accounts.pubkey,
        })
    }
}
