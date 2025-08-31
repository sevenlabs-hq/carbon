use super::super::types::*;

use carbon_core::{borsh, CarbonDeserialize};

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
        let [liquidity_pool_state, protocol_config, protocol_admin, protocol_admin_state, _remaining @ ..] =
            accounts
        else {
            return None;
        };

        Some(AdminUpdateStandardLiquidityPoolStateInstructionAccounts {
            liquidity_pool_state: liquidity_pool_state.pubkey,
            protocol_config: protocol_config.pubkey,
            protocol_admin: protocol_admin.pubkey,
            protocol_admin_state: protocol_admin_state.pubkey,
        })
    }
}
