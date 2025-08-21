use carbon_core::account::AccountDecoder;
use carbon_core::deserialize::CarbonDeserialize;

use super::DynamicBondingCurveDecoder;
pub mod claim_fee_operator;
pub mod config;
pub mod lock_escrow;
pub mod meteora_damm_migration_metadata;
pub mod meteora_damm_v2_metadata;
pub mod partner_metadata;
pub mod pool_config;
pub mod virtual_pool;
pub mod virtual_pool_metadata;

#[allow(clippy::large_enum_variant)]
pub enum DynamicBondingCurveAccount {
    ClaimFeeOperator(claim_fee_operator::ClaimFeeOperator),
    Config(config::Config),
    LockEscrow(lock_escrow::LockEscrow),
    MeteoraDammMigrationMetadata(meteora_damm_migration_metadata::MeteoraDammMigrationMetadata),
    MeteoraDammV2Metadata(meteora_damm_v2_metadata::MeteoraDammV2Metadata),
    PartnerMetadata(partner_metadata::PartnerMetadata),
    PoolConfig(pool_config::PoolConfig),
    VirtualPool(virtual_pool::VirtualPool),
    VirtualPoolMetadata(virtual_pool_metadata::VirtualPoolMetadata),
}

impl AccountDecoder<'_> for DynamicBondingCurveDecoder {
    type AccountType = DynamicBondingCurveAccount;
    fn decode_account(
        &self,
        account: &solana_account::Account,
    ) -> Option<carbon_core::account::DecodedAccount<Self::AccountType>> {
        if let Some(decoded_account) =
            claim_fee_operator::ClaimFeeOperator::deserialize(account.data.as_slice())
        {
            return Some(carbon_core::account::DecodedAccount {
                lamports: account.lamports,
                data: DynamicBondingCurveAccount::ClaimFeeOperator(decoded_account),
                owner: account.owner,
                executable: account.executable,
                rent_epoch: account.rent_epoch,
            });
        }

        if let Some(decoded_account) = config::Config::deserialize(account.data.as_slice()) {
            return Some(carbon_core::account::DecodedAccount {
                lamports: account.lamports,
                data: DynamicBondingCurveAccount::Config(decoded_account),
                owner: account.owner,
                executable: account.executable,
                rent_epoch: account.rent_epoch,
            });
        }

        if let Some(decoded_account) = lock_escrow::LockEscrow::deserialize(account.data.as_slice())
        {
            return Some(carbon_core::account::DecodedAccount {
                lamports: account.lamports,
                data: DynamicBondingCurveAccount::LockEscrow(decoded_account),
                owner: account.owner,
                executable: account.executable,
                rent_epoch: account.rent_epoch,
            });
        }

        if let Some(decoded_account) =
            meteora_damm_migration_metadata::MeteoraDammMigrationMetadata::deserialize(
                account.data.as_slice(),
            )
        {
            return Some(carbon_core::account::DecodedAccount {
                lamports: account.lamports,
                data: DynamicBondingCurveAccount::MeteoraDammMigrationMetadata(decoded_account),
                owner: account.owner,
                executable: account.executable,
                rent_epoch: account.rent_epoch,
            });
        }

        if let Some(decoded_account) =
            meteora_damm_v2_metadata::MeteoraDammV2Metadata::deserialize(account.data.as_slice())
        {
            return Some(carbon_core::account::DecodedAccount {
                lamports: account.lamports,
                data: DynamicBondingCurveAccount::MeteoraDammV2Metadata(decoded_account),
                owner: account.owner,
                executable: account.executable,
                rent_epoch: account.rent_epoch,
            });
        }

        if let Some(decoded_account) =
            partner_metadata::PartnerMetadata::deserialize(account.data.as_slice())
        {
            return Some(carbon_core::account::DecodedAccount {
                lamports: account.lamports,
                data: DynamicBondingCurveAccount::PartnerMetadata(decoded_account),
                owner: account.owner,
                executable: account.executable,
                rent_epoch: account.rent_epoch,
            });
        }

        if let Some(decoded_account) = pool_config::PoolConfig::deserialize(account.data.as_slice())
        {
            return Some(carbon_core::account::DecodedAccount {
                lamports: account.lamports,
                data: DynamicBondingCurveAccount::PoolConfig(decoded_account),
                owner: account.owner,
                executable: account.executable,
                rent_epoch: account.rent_epoch,
            });
        }

        if let Some(decoded_account) =
            virtual_pool::VirtualPool::deserialize(account.data.as_slice())
        {
            return Some(carbon_core::account::DecodedAccount {
                lamports: account.lamports,
                data: DynamicBondingCurveAccount::VirtualPool(decoded_account),
                owner: account.owner,
                executable: account.executable,
                rent_epoch: account.rent_epoch,
            });
        }

        if let Some(decoded_account) =
            virtual_pool_metadata::VirtualPoolMetadata::deserialize(account.data.as_slice())
        {
            return Some(carbon_core::account::DecodedAccount {
                lamports: account.lamports,
                data: DynamicBondingCurveAccount::VirtualPoolMetadata(decoded_account),
                owner: account.owner,
                executable: account.executable,
                rent_epoch: account.rent_epoch,
            });
        }

        None
    }
}
