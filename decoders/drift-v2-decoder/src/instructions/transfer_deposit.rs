use carbon_core::{borsh, CarbonDeserialize};

#[derive(
    CarbonDeserialize, Debug, serde::Serialize, serde::Deserialize, PartialEq, Eq, Clone, Hash,
)]
#[carbon(discriminator = "0x141493df293fcc6f")]
pub struct TransferDeposit {
    pub market_index: u16,
    pub amount: u64,
}

pub struct TransferDepositInstructionAccounts {
    pub from_user: solana_pubkey::Pubkey,
    pub to_user: solana_pubkey::Pubkey,
    pub user_stats: solana_pubkey::Pubkey,
    pub authority: solana_pubkey::Pubkey,
    pub state: solana_pubkey::Pubkey,
    pub spot_market_vault: solana_pubkey::Pubkey,
}

impl carbon_core::deserialize::ArrangeAccounts for TransferDeposit {
    type ArrangedAccounts = TransferDepositInstructionAccounts;

    fn arrange_accounts(
        accounts: &[solana_instruction::AccountMeta],
    ) -> Option<Self::ArrangedAccounts> {
        let [from_user, to_user, user_stats, authority, state, spot_market_vault, _remaining @ ..] =
            accounts
        else {
            return None;
        };

        Some(TransferDepositInstructionAccounts {
            from_user: from_user.pubkey,
            to_user: to_user.pubkey,
            user_stats: user_stats.pubkey,
            authority: authority.pubkey,
            state: state.pubkey,
            spot_market_vault: spot_market_vault.pubkey,
        })
    }
}
