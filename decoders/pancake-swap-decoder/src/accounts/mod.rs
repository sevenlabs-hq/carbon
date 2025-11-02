use carbon_core::account::AccountDecoder;
use carbon_core::deserialize::CarbonDeserialize;

use super::PancakeSwapDecoder;
pub mod amm_config;
pub mod observation_state;
pub mod operation_state;
pub mod permissionless_farm_switch;
pub mod personal_position_state;
pub mod pool_state;
pub mod protocol_position_state;
pub mod support_mint_associated;
pub mod tick_array_bitmap_extension;
pub mod tick_array_state;

#[allow(clippy::large_enum_variant)]
pub enum PancakeSwapAccount {
    AmmConfig(amm_config::AmmConfig),
    ObservationState(observation_state::ObservationState),
    OperationState(operation_state::OperationState),
    PermissionlessFarmSwitch(permissionless_farm_switch::PermissionlessFarmSwitch),
    PersonalPositionState(personal_position_state::PersonalPositionState),
    PoolState(pool_state::PoolState),
    ProtocolPositionState(protocol_position_state::ProtocolPositionState),
    SupportMintAssociated(support_mint_associated::SupportMintAssociated),
    TickArrayBitmapExtension(tick_array_bitmap_extension::TickArrayBitmapExtension),
    TickArrayState(tick_array_state::TickArrayState),
}

impl AccountDecoder<'_> for PancakeSwapDecoder {
    type AccountType = PancakeSwapAccount;
    fn decode_account(
        &self,
        account: &solana_account::Account,
    ) -> Option<carbon_core::account::DecodedAccount<Self::AccountType>> {
        if let Some(decoded_account) = amm_config::AmmConfig::deserialize(account.data.as_slice()) {
            return Some(carbon_core::account::DecodedAccount {
                lamports: account.lamports,
                data: PancakeSwapAccount::AmmConfig(decoded_account),
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
                data: PancakeSwapAccount::ObservationState(decoded_account),
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
                data: PancakeSwapAccount::OperationState(decoded_account),
                owner: account.owner,
                executable: account.executable,
                rent_epoch: account.rent_epoch,
            });
        }

        if let Some(decoded_account) =
            permissionless_farm_switch::PermissionlessFarmSwitch::deserialize(
                account.data.as_slice(),
            )
        {
            return Some(carbon_core::account::DecodedAccount {
                lamports: account.lamports,
                data: PancakeSwapAccount::PermissionlessFarmSwitch(decoded_account),
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
                data: PancakeSwapAccount::PersonalPositionState(decoded_account),
                owner: account.owner,
                executable: account.executable,
                rent_epoch: account.rent_epoch,
            });
        }

        if let Some(decoded_account) = pool_state::PoolState::deserialize(account.data.as_slice()) {
            return Some(carbon_core::account::DecodedAccount {
                lamports: account.lamports,
                data: PancakeSwapAccount::PoolState(decoded_account),
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
                data: PancakeSwapAccount::ProtocolPositionState(decoded_account),
                owner: account.owner,
                executable: account.executable,
                rent_epoch: account.rent_epoch,
            });
        }

        if let Some(decoded_account) =
            support_mint_associated::SupportMintAssociated::deserialize(account.data.as_slice())
        {
            return Some(carbon_core::account::DecodedAccount {
                lamports: account.lamports,
                data: PancakeSwapAccount::SupportMintAssociated(decoded_account),
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
                data: PancakeSwapAccount::TickArrayBitmapExtension(decoded_account),
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
                data: PancakeSwapAccount::TickArrayState(decoded_account),
                owner: account.owner,
                executable: account.executable,
                rent_epoch: account.rent_epoch,
            });
        }

        None
    }
}
