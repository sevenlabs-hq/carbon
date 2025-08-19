use {
    super::super::types::*,
    carbon_core::{borsh, CarbonDeserialize},
};

#[derive(
    CarbonDeserialize, Debug, serde::Serialize, serde::Deserialize, PartialEq, Eq, Clone, Hash,
)]
#[carbon(discriminator = "0x0122f269d7d39d12")]
pub struct TreasuryMovement {
    pub treasury_movement_type: TreasuryMovementType,
    pub amount: u64,
}

pub struct TreasuryMovementInstructionAccounts {
    pub state: solana_pubkey::Pubkey,
    pub insurance_vault: solana_pubkey::Pubkey,
    pub treasury_wallet: solana_pubkey::Pubkey,
    pub referrals_rewards_wallet: solana_pubkey::Pubkey,
    pub token_program: solana_pubkey::Pubkey,
    pub admin: solana_pubkey::Pubkey,
}

impl carbon_core::deserialize::ArrangeAccounts for TreasuryMovement {
    type ArrangedAccounts = TreasuryMovementInstructionAccounts;

    fn arrange_accounts(
        accounts: &[solana_instruction::AccountMeta],
    ) -> Option<Self::ArrangedAccounts> {
        let [state, insurance_vault, treasury_wallet, referrals_rewards_wallet, token_program, admin, _remaining @ ..] =
            accounts
        else {
            return None;
        };

        Some(TreasuryMovementInstructionAccounts {
            state: state.pubkey,
            insurance_vault: insurance_vault.pubkey,
            treasury_wallet: treasury_wallet.pubkey,
            referrals_rewards_wallet: referrals_rewards_wallet.pubkey,
            token_program: token_program.pubkey,
            admin: admin.pubkey,
        })
    }
}
