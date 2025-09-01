use super::super::types::*;

use carbon_core::{account_utils::next_account, borsh, CarbonDeserialize};

#[derive(
    CarbonDeserialize, Debug, serde::Serialize, serde::Deserialize, PartialEq, Eq, Clone, Hash,
)]
#[carbon(discriminator = "0x63e4293fddf4c8c7")]
pub struct AdminUpdateStandardLiquidityPoolState {
    pub update: AdminUpdateLiquidityPoolState,
}

#[derive(Debug, PartialEq, Eq, Clone, Hash, serde::Serialize, serde::Deserialize)]
pub struct AdminUpdateStandardLiquidityPoolStateInstructionAccounts {
    pub liquidity_pool_state: solana_pubkey::Pubkey,
    pub protocol_config: solana_pubkey::Pubkey,
    pub protocol_admin: solana_pubkey::Pubkey,
    pub protocol_admin_state: solana_pubkey::Pubkey,
}

impl carbon_core::deserialize::ArrangeAccounts for AdminUpdateStandardLiquidityPoolState {
    type ArrangedAccounts = AdminUpdateStandardLiquidityPoolStateInstructionAccounts;

    fn arrange_accounts(
        accounts: &[solana_instruction::AccountMeta],
    ) -> Option<Self::ArrangedAccounts> {
        let mut iter = accounts.iter();
        let liquidity_pool_state = next_account(&mut iter)?;
        let protocol_config = next_account(&mut iter)?;
        let protocol_admin = next_account(&mut iter)?;
        let protocol_admin_state = next_account(&mut iter)?;

        Some(AdminUpdateStandardLiquidityPoolStateInstructionAccounts {
            liquidity_pool_state,
            protocol_config,
            protocol_admin,
            protocol_admin_state,
        })
    }
}
