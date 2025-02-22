use carbon_core::account::AccountDecoder;
use carbon_core::deserialize::CarbonDeserialize;

use crate::PROGRAM_ID;

use super::RaydiumLiquidityLockingDecoder;
pub mod locked_clmm_position_state;
pub mod locked_cp_liquidity_state;

pub enum RaydiumLiquidityLockingAccount {
    LockedCpLiquidityState(locked_cp_liquidity_state::LockedCpLiquidityState),
    LockedClmmPositionState(locked_clmm_position_state::LockedClmmPositionState),
}

impl<'a> AccountDecoder<'a> for RaydiumLiquidityLockingDecoder {
    type AccountType = RaydiumLiquidityLockingAccount;
    fn decode_account(
        &self,
        account: &solana_sdk::account::Account,
    ) -> Option<carbon_core::account::DecodedAccount<Self::AccountType>> {
        if !account.owner.eq(&PROGRAM_ID) {
            return None;
        }

        if let Some(decoded_account) =
            locked_cp_liquidity_state::LockedCpLiquidityState::deserialize(account.data.as_slice())
        {
            return Some(carbon_core::account::DecodedAccount {
                lamports: account.lamports,
                data: RaydiumLiquidityLockingAccount::LockedCpLiquidityState(decoded_account),
                owner: account.owner,
                executable: account.executable,
                rent_epoch: account.rent_epoch,
            });
        }

        if let Some(decoded_account) =
            locked_clmm_position_state::LockedClmmPositionState::deserialize(
                account.data.as_slice(),
            )
        {
            return Some(carbon_core::account::DecodedAccount {
                lamports: account.lamports,
                data: RaydiumLiquidityLockingAccount::LockedClmmPositionState(decoded_account),
                owner: account.owner,
                executable: account.executable,
                rent_epoch: account.rent_epoch,
            });
        }

        None
    }
}
