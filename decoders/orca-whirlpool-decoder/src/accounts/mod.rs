use carbon_core::account::AccountDecoder;
use carbon_core::deserialize::CarbonDeserialize;

use super::OrcaWhirlpoolDecoder;
pub mod fee_tier;
pub mod position;
pub mod position_bundle;
pub mod tick_array;
pub mod token_badge;
pub mod whirlpool;
pub mod whirlpools_config;
pub mod whirlpools_config_extension;

pub enum OrcaWhirlpoolAccount {
    WhirlpoolsConfigExtension(whirlpools_config_extension::WhirlpoolsConfigExtension),
    WhirlpoolsConfig(whirlpools_config::WhirlpoolsConfig),
    FeeTier(fee_tier::FeeTier),
    PositionBundle(position_bundle::PositionBundle),
    Position(position::Position),
    TickArray(tick_array::TickArray),
    TokenBadge(token_badge::TokenBadge),
    Whirlpool(whirlpool::Whirlpool),
}

impl<'a> AccountDecoder<'a> for OrcaWhirlpoolDecoder {
    type AccountType = OrcaWhirlpoolAccount;
    fn decode_account(
        &self,
        account: &solana_sdk::account::Account,
    ) -> Option<carbon_core::account::DecodedAccount<Self::AccountType>> {
        if let Some(decoded_account) =
            whirlpools_config_extension::WhirlpoolsConfigExtension::deserialize(
                account.data.as_slice(),
            )
        {
            return Some(carbon_core::account::DecodedAccount {
                lamports: account.lamports,
                data: OrcaWhirlpoolAccount::WhirlpoolsConfigExtension(decoded_account),
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
                data: OrcaWhirlpoolAccount::WhirlpoolsConfig(decoded_account),
                owner: account.owner,
                executable: account.executable,
                rent_epoch: account.rent_epoch,
            });
        }

        if let Some(decoded_account) = fee_tier::FeeTier::deserialize(account.data.as_slice()) {
            return Some(carbon_core::account::DecodedAccount {
                lamports: account.lamports,
                data: OrcaWhirlpoolAccount::FeeTier(decoded_account),
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
                data: OrcaWhirlpoolAccount::PositionBundle(decoded_account),
                owner: account.owner,
                executable: account.executable,
                rent_epoch: account.rent_epoch,
            });
        }

        if let Some(decoded_account) = position::Position::deserialize(account.data.as_slice()) {
            return Some(carbon_core::account::DecodedAccount {
                lamports: account.lamports,
                data: OrcaWhirlpoolAccount::Position(decoded_account),
                owner: account.owner,
                executable: account.executable,
                rent_epoch: account.rent_epoch,
            });
        }

        if let Some(decoded_account) = tick_array::TickArray::deserialize(account.data.as_slice()) {
            return Some(carbon_core::account::DecodedAccount {
                lamports: account.lamports,
                data: OrcaWhirlpoolAccount::TickArray(decoded_account),
                owner: account.owner,
                executable: account.executable,
                rent_epoch: account.rent_epoch,
            });
        }

        if let Some(decoded_account) = token_badge::TokenBadge::deserialize(account.data.as_slice())
        {
            return Some(carbon_core::account::DecodedAccount {
                lamports: account.lamports,
                data: OrcaWhirlpoolAccount::TokenBadge(decoded_account),
                owner: account.owner,
                executable: account.executable,
                rent_epoch: account.rent_epoch,
            });
        }

        if let Some(decoded_account) = whirlpool::Whirlpool::deserialize(account.data.as_slice()) {
            return Some(carbon_core::account::DecodedAccount {
                lamports: account.lamports,
                data: OrcaWhirlpoolAccount::Whirlpool(decoded_account),
                owner: account.owner,
                executable: account.executable,
                rent_epoch: account.rent_epoch,
            });
        }

        None
    }
}
