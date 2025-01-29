use carbon_core::account::AccountDecoder;
use carbon_core::deserialize::CarbonDeserialize;

use super::RaydiumClmmDecoder;
pub mod amm_config;
pub mod observation_state;
pub mod operation_state;
pub mod personal_position_state;
pub mod pool_state;
pub mod protocol_position_state;
pub mod tick_array_bitmap_extension;
pub mod tick_array_state;

#[derive(Debug)]
pub enum RaydiumClmmAccount {
    AmmConfig(amm_config::AmmConfig),
    OperationState(operation_state::OperationState),
    ObservationState(observation_state::ObservationState),
    PersonalPositionState(personal_position_state::PersonalPositionState),
    PoolState(pool_state::PoolState),
    ProtocolPositionState(protocol_position_state::ProtocolPositionState),
    TickArrayState(tick_array_state::TickArrayState),
    TickArrayBitmapExtension(tick_array_bitmap_extension::TickArrayBitmapExtension),
}

impl<'a> AccountDecoder<'a> for RaydiumClmmDecoder {
    type AccountType = RaydiumClmmAccount;
    fn decode_account(
        &self,
        account: &solana_sdk::account::Account,
    ) -> Option<carbon_core::account::DecodedAccount<Self::AccountType>> {
        if let Some(decoded_account) = amm_config::AmmConfig::deserialize(account.data.as_slice()) {
            return Some(carbon_core::account::DecodedAccount {
                lamports: account.lamports,
                data: RaydiumClmmAccount::AmmConfig(decoded_account),
                owner: account.owner,
                executable: account.executable,
                rent_epoch: account.rent_epoch,
            });
        }

        if let Some(decoded_account) =
            operation_state::OperationState::deserialize(account.data.as_slice())
        {
            return Some(carbon_core::account::DecodedAccount {
                lamports: account.lamports,
                data: RaydiumClmmAccount::OperationState(decoded_account),
                owner: account.owner,
                executable: account.executable,
                rent_epoch: account.rent_epoch,
            });
        }

        if let Some(decoded_account) =
            observation_state::ObservationState::deserialize(account.data.as_slice())
        {
            return Some(carbon_core::account::DecodedAccount {
                lamports: account.lamports,
                data: RaydiumClmmAccount::ObservationState(decoded_account),
                owner: account.owner,
                executable: account.executable,
                rent_epoch: account.rent_epoch,
            });
        }

        if let Some(decoded_account) =
            personal_position_state::PersonalPositionState::deserialize(account.data.as_slice())
        {
            return Some(carbon_core::account::DecodedAccount {
                lamports: account.lamports,
                data: RaydiumClmmAccount::PersonalPositionState(decoded_account),
                owner: account.owner,
                executable: account.executable,
                rent_epoch: account.rent_epoch,
            });
        }

        if let Some(decoded_account) = pool_state::PoolState::deserialize(account.data.as_slice()) {
            return Some(carbon_core::account::DecodedAccount {
                lamports: account.lamports,
                data: RaydiumClmmAccount::PoolState(decoded_account),
                owner: account.owner,
                executable: account.executable,
                rent_epoch: account.rent_epoch,
            });
        }

        if let Some(decoded_account) =
            protocol_position_state::ProtocolPositionState::deserialize(account.data.as_slice())
        {
            return Some(carbon_core::account::DecodedAccount {
                lamports: account.lamports,
                data: RaydiumClmmAccount::ProtocolPositionState(decoded_account),
                owner: account.owner,
                executable: account.executable,
                rent_epoch: account.rent_epoch,
            });
        }

        if let Some(decoded_account) =
            tick_array_state::TickArrayState::deserialize(account.data.as_slice())
        {
            return Some(carbon_core::account::DecodedAccount {
                lamports: account.lamports,
                data: RaydiumClmmAccount::TickArrayState(decoded_account),
                owner: account.owner,
                executable: account.executable,
                rent_epoch: account.rent_epoch,
            });
        }

        if let Some(decoded_account) =
            tick_array_bitmap_extension::TickArrayBitmapExtension::deserialize(
                account.data.as_slice(),
            )
        {
            return Some(carbon_core::account::DecodedAccount {
                lamports: account.lamports,
                data: RaydiumClmmAccount::TickArrayBitmapExtension(decoded_account),
                owner: account.owner,
                executable: account.executable,
                rent_epoch: account.rent_epoch,
            });
        }

        None
    }
}
