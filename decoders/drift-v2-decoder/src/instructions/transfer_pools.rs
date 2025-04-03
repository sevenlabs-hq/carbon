use carbon_core::{borsh, CarbonDeserialize};

#[derive(
    CarbonDeserialize, Debug, serde::Serialize, serde::Deserialize, PartialEq, Eq, Clone, Hash,
)]
#[carbon(discriminator = "0xc5679a196b5a3c5e")]
pub struct TransferPools {
    pub deposit_from_market_index: u16,
    pub deposit_to_market_index: u16,
    pub borrow_from_market_index: u16,
    pub borrow_to_market_index: u16,
    pub deposit_amount: Option<u64>,
    pub borrow_amount: Option<u64>,
}

pub struct TransferPoolsInstructionAccounts {
    pub from_user: solana_pubkey::Pubkey,
    pub to_user: solana_pubkey::Pubkey,
    pub user_stats: solana_pubkey::Pubkey,
    pub authority: solana_pubkey::Pubkey,
    pub state: solana_pubkey::Pubkey,
    pub deposit_from_spot_market_vault: solana_pubkey::Pubkey,
    pub deposit_to_spot_market_vault: solana_pubkey::Pubkey,
    pub borrow_from_spot_market_vault: solana_pubkey::Pubkey,
    pub borrow_to_spot_market_vault: solana_pubkey::Pubkey,
    pub drift_signer: solana_pubkey::Pubkey,
}

impl carbon_core::deserialize::ArrangeAccounts for TransferPools {
    type ArrangedAccounts = TransferPoolsInstructionAccounts;

    fn arrange_accounts(
        accounts: &[solana_sdk::instruction::AccountMeta],
    ) -> Option<Self::ArrangedAccounts> {
        let [from_user, to_user, user_stats, authority, state, deposit_from_spot_market_vault, deposit_to_spot_market_vault, borrow_from_spot_market_vault, borrow_to_spot_market_vault, drift_signer, _remaining @ ..] =
            accounts
        else {
            return None;
        };

        Some(TransferPoolsInstructionAccounts {
            from_user: from_user.pubkey,
            to_user: to_user.pubkey,
            user_stats: user_stats.pubkey,
            authority: authority.pubkey,
            state: state.pubkey,
            deposit_from_spot_market_vault: deposit_from_spot_market_vault.pubkey,
            deposit_to_spot_market_vault: deposit_to_spot_market_vault.pubkey,
            borrow_from_spot_market_vault: borrow_from_spot_market_vault.pubkey,
            borrow_to_spot_market_vault: borrow_to_spot_market_vault.pubkey,
            drift_signer: drift_signer.pubkey,
        })
    }
}
