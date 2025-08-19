use {
    super::super::types::*,
    alloc::vec::Vec,
    carbon_core::{borsh, CarbonDeserialize},
};

#[derive(
    CarbonDeserialize, Debug, serde::Serialize, serde::Deserialize, PartialEq, Eq, Clone, Hash,
)]
#[carbon(discriminator = "0x75104bf9b37fab93")]
pub struct PositionMovement {
    pub movement_type: MovementType,
    pub movements: Vec<PositionMovementArg>,
}

pub struct PositionMovementInstructionAccounts {
    pub state: solana_pubkey::Pubkey,
    pub zeta_group: solana_pubkey::Pubkey,
    pub margin_account: solana_pubkey::Pubkey,
    pub spread_account: solana_pubkey::Pubkey,
    pub authority: solana_pubkey::Pubkey,
    pub greeks: solana_pubkey::Pubkey,
    pub oracle: solana_pubkey::Pubkey,
    pub oracle_backup_feed: solana_pubkey::Pubkey,
    pub oracle_backup_program: solana_pubkey::Pubkey,
}

impl carbon_core::deserialize::ArrangeAccounts for PositionMovement {
    type ArrangedAccounts = PositionMovementInstructionAccounts;

    fn arrange_accounts(
        accounts: &[solana_instruction::AccountMeta],
    ) -> Option<Self::ArrangedAccounts> {
        let [state, zeta_group, margin_account, spread_account, authority, greeks, oracle, oracle_backup_feed, oracle_backup_program, _remaining @ ..] =
            accounts
        else {
            return None;
        };

        Some(PositionMovementInstructionAccounts {
            state: state.pubkey,
            zeta_group: zeta_group.pubkey,
            margin_account: margin_account.pubkey,
            spread_account: spread_account.pubkey,
            authority: authority.pubkey,
            greeks: greeks.pubkey,
            oracle: oracle.pubkey,
            oracle_backup_feed: oracle_backup_feed.pubkey,
            oracle_backup_program: oracle_backup_program.pubkey,
        })
    }
}
