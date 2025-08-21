use carbon_core::account::AccountDecoder;
use carbon_core::deserialize::CarbonDeserialize;

use super::OrcaWhirlpoolDecoder;
pub mod adaptive_fee_tier;
pub mod dynamic_tick_array;
pub mod fee_tier;
pub mod fixed_tick_array;
pub mod lock_config;
pub mod oracle;
pub mod position;
pub mod position_bundle;
pub mod token_badge;
pub mod whirlpool;
pub mod whirlpools_config;
pub mod whirlpools_config_extension;

pub enum WhirlpoolAccount {
    AdaptiveFeeTier(adaptive_fee_tier::AdaptiveFeeTier),
    WhirlpoolsConfig(whirlpools_config::WhirlpoolsConfig),
    WhirlpoolsConfigExtension(whirlpools_config_extension::WhirlpoolsConfigExtension),
    FeeTier(fee_tier::FeeTier),
    LockConfig(lock_config::LockConfig),
    Oracle(oracle::Oracle),
    Position(position::Position),
    PositionBundle(position_bundle::PositionBundle),
    FixedTickArray(Box<fixed_tick_array::FixedTickArray>),
    DynamicTickArray(Box<dynamic_tick_array::DynamicTickArray>),
    TokenBadge(token_badge::TokenBadge),
    Whirlpool(Box<whirlpool::Whirlpool>),
}

impl AccountDecoder<'_> for OrcaWhirlpoolDecoder {
    type AccountType = WhirlpoolAccount;
    fn decode_account(
        &self,
        account: &solana_account::Account,
    ) -> Option<carbon_core::account::DecodedAccount<Self::AccountType>> {
        if let Some(decoded_account) =
            adaptive_fee_tier::AdaptiveFeeTier::deserialize(account.data.as_slice())
        {
            return Some(carbon_core::account::DecodedAccount {
                lamports: account.lamports,
                data: WhirlpoolAccount::AdaptiveFeeTier(decoded_account),
                owner: account.owner,
                executable: account.executable,
                rent_epoch: account.rent_epoch,
            });
        }

        if let Some(decoded_account) =
            whirlpools_config::WhirlpoolsConfig::deserialize(account.data.as_slice())
        {
            return Some(carbon_core::account::DecodedAccount {
                lamports: account.lamports,
                data: WhirlpoolAccount::WhirlpoolsConfig(decoded_account),
                owner: account.owner,
                executable: account.executable,
                rent_epoch: account.rent_epoch,
            });
        }

        if let Some(decoded_account) =
            whirlpools_config_extension::WhirlpoolsConfigExtension::deserialize(
                account.data.as_slice(),
            )
        {
            return Some(carbon_core::account::DecodedAccount {
                lamports: account.lamports,
                data: WhirlpoolAccount::WhirlpoolsConfigExtension(decoded_account),
                owner: account.owner,
                executable: account.executable,
                rent_epoch: account.rent_epoch,
            });
        }

        if let Some(decoded_account) = fee_tier::FeeTier::deserialize(account.data.as_slice()) {
            return Some(carbon_core::account::DecodedAccount {
                lamports: account.lamports,
                data: WhirlpoolAccount::FeeTier(decoded_account),
                owner: account.owner,
                executable: account.executable,
                rent_epoch: account.rent_epoch,
            });
        }

        if let Some(decoded_account) = lock_config::LockConfig::deserialize(account.data.as_slice())
        {
            return Some(carbon_core::account::DecodedAccount {
                lamports: account.lamports,
                data: WhirlpoolAccount::LockConfig(decoded_account),
                owner: account.owner,
                executable: account.executable,
                rent_epoch: account.rent_epoch,
            });
        }

        if let Some(decoded_account) = oracle::Oracle::deserialize(account.data.as_slice()) {
            return Some(carbon_core::account::DecodedAccount {
                lamports: account.lamports,
                data: WhirlpoolAccount::Oracle(decoded_account),
                owner: account.owner,
                executable: account.executable,
                rent_epoch: account.rent_epoch,
            });
        }

        if let Some(decoded_account) = position::Position::deserialize(account.data.as_slice()) {
            return Some(carbon_core::account::DecodedAccount {
                lamports: account.lamports,
                data: WhirlpoolAccount::Position(decoded_account),
                owner: account.owner,
                executable: account.executable,
                rent_epoch: account.rent_epoch,
            });
        }

        if let Some(decoded_account) =
            position_bundle::PositionBundle::deserialize(account.data.as_slice())
        {
            return Some(carbon_core::account::DecodedAccount {
                lamports: account.lamports,
                data: WhirlpoolAccount::PositionBundle(decoded_account),
                owner: account.owner,
                executable: account.executable,
                rent_epoch: account.rent_epoch,
            });
        }

        if let Some(decoded_account) =
            fixed_tick_array::FixedTickArray::deserialize(account.data.as_slice())
        {
            return Some(carbon_core::account::DecodedAccount {
                lamports: account.lamports,
                data: WhirlpoolAccount::FixedTickArray(Box::new(decoded_account)),
                owner: account.owner,
                executable: account.executable,
                rent_epoch: account.rent_epoch,
            });
        }

        if let Some(decoded_account) =
            dynamic_tick_array::DynamicTickArray::deserialize(account.data.as_slice())
        {
            return Some(carbon_core::account::DecodedAccount {
                lamports: account.lamports,
                data: WhirlpoolAccount::DynamicTickArray(Box::new(decoded_account)),
                owner: account.owner,
                executable: account.executable,
                rent_epoch: account.rent_epoch,
            });
        }

        if let Some(decoded_account) = token_badge::TokenBadge::deserialize(account.data.as_slice())
        {
            return Some(carbon_core::account::DecodedAccount {
                lamports: account.lamports,
                data: WhirlpoolAccount::TokenBadge(decoded_account),
                owner: account.owner,
                executable: account.executable,
                rent_epoch: account.rent_epoch,
            });
        }

        if let Some(decoded_account) = whirlpool::Whirlpool::deserialize(account.data.as_slice()) {
            return Some(carbon_core::account::DecodedAccount {
                lamports: account.lamports,
                data: WhirlpoolAccount::Whirlpool(Box::new(decoded_account)),
                owner: account.owner,
                executable: account.executable,
                rent_epoch: account.rent_epoch,
            });
        }

        None
    }
}
