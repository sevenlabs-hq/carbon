use alloc::boxed::Box;
use carbon_core::account::AccountDecoder;
use carbon_core::deserialize::CarbonDeserialize;

use crate::PROGRAM_ID;

use super::VirtualCurveDecoder;
pub mod claim_fee_operator;
pub mod config;
pub mod lock_escrow;
pub mod meteora_damm_migration_metadata;
pub mod meteora_damm_v2_metadata;
pub mod partner_metadata;
pub mod pool_config;
pub mod virtual_pool;
pub mod virtual_pool_metadata;

pub enum VirtualCurveAccount {
    ClaimFeeOperator(claim_fee_operator::ClaimFeeOperator),
    Config(config::Config),
    LockEscrow(lock_escrow::LockEscrow),
    MeteoraDammMigrationMetadata(meteora_damm_migration_metadata::MeteoraDammMigrationMetadata),
    MeteoraDammV2Metadata(meteora_damm_v2_metadata::MeteoraDammV2Metadata),
    PartnerMetadata(partner_metadata::PartnerMetadata),
    PoolConfig(Box<pool_config::PoolConfig>),
    VirtualPool(virtual_pool::VirtualPool),
    VirtualPoolMetadata(virtual_pool_metadata::VirtualPoolMetadata),
}

impl AccountDecoder<'_> for VirtualCurveDecoder {
    type AccountType = VirtualCurveAccount;
    fn decode_account(
        &self,
        account: &solana_account::Account,
    ) -> Option<carbon_core::account::DecodedAccount<Self::AccountType>> {
        if !account.owner.eq(&PROGRAM_ID) {
            return None;
        }

        if let Some(decoded_account) =
            claim_fee_operator::ClaimFeeOperator::deserialize(account.data.as_slice())
        {
            return Some(carbon_core::account::DecodedAccount {
                lamports: account.lamports,
                data: VirtualCurveAccount::ClaimFeeOperator(decoded_account),
                owner: account.owner,
                executable: account.executable,
                rent_epoch: account.rent_epoch,
            });
        }

        if let Some(decoded_account) = config::Config::deserialize(account.data.as_slice()) {
            return Some(carbon_core::account::DecodedAccount {
                lamports: account.lamports,
                data: VirtualCurveAccount::Config(decoded_account),
                owner: account.owner,
                executable: account.executable,
                rent_epoch: account.rent_epoch,
            });
        }

        if let Some(decoded_account) = lock_escrow::LockEscrow::deserialize(account.data.as_slice())
        {
            return Some(carbon_core::account::DecodedAccount {
                lamports: account.lamports,
                data: VirtualCurveAccount::LockEscrow(decoded_account),
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
                data: VirtualCurveAccount::MeteoraDammMigrationMetadata(decoded_account),
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
                data: VirtualCurveAccount::MeteoraDammV2Metadata(decoded_account),
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
                data: VirtualCurveAccount::PartnerMetadata(decoded_account),
                owner: account.owner,
                executable: account.executable,
                rent_epoch: account.rent_epoch,
            });
        }

        if let Some(decoded_account) = pool_config::PoolConfig::deserialize(account.data.as_slice())
        {
            return Some(carbon_core::account::DecodedAccount {
                lamports: account.lamports,
                data: VirtualCurveAccount::PoolConfig(Box::new(decoded_account)),
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
                data: VirtualCurveAccount::VirtualPool(decoded_account),
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
                data: VirtualCurveAccount::VirtualPoolMetadata(decoded_account),
                owner: account.owner,
                executable: account.executable,
                rent_epoch: account.rent_epoch,
            });
        }

        None
    }
}
