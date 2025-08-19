use {
    super::RaydiumLaunchpadDecoder,
    crate::PROGRAM_ID,
    alloc::boxed::Box,
    carbon_core::{account::AccountDecoder, deserialize::CarbonDeserialize},
};
pub mod global_config;
pub mod platform_config;
pub mod pool_state;
pub mod vesting_record;

pub enum RaydiumLaunchpadAccount {
    GlobalConfig(Box<global_config::GlobalConfig>),
    PlatformConfig(Box<platform_config::PlatformConfig>),
    PoolState(Box<pool_state::PoolState>),
    VestingRecord(vesting_record::VestingRecord),
}

impl AccountDecoder<'_> for RaydiumLaunchpadDecoder {
    type AccountType = RaydiumLaunchpadAccount;
    fn decode_account(
        &self,
        account: &solana_account::Account,
    ) -> Option<carbon_core::account::DecodedAccount<Self::AccountType>> {
        if !account.owner.eq(&PROGRAM_ID) {
            return None;
        }

        if let Some(decoded_account) =
            global_config::GlobalConfig::deserialize(account.data.as_slice())
        {
            return Some(carbon_core::account::DecodedAccount {
                lamports: account.lamports,
                data: RaydiumLaunchpadAccount::GlobalConfig(Box::new(decoded_account)),
                owner: account.owner,
                executable: account.executable,
                rent_epoch: account.rent_epoch,
            });
        }

        if let Some(decoded_account) =
            platform_config::PlatformConfig::deserialize(account.data.as_slice())
        {
            return Some(carbon_core::account::DecodedAccount {
                lamports: account.lamports,
                data: RaydiumLaunchpadAccount::PlatformConfig(Box::new(decoded_account)),
                owner: account.owner,
                executable: account.executable,
                rent_epoch: account.rent_epoch,
            });
        }

        if let Some(decoded_account) = pool_state::PoolState::deserialize(account.data.as_slice()) {
            return Some(carbon_core::account::DecodedAccount {
                lamports: account.lamports,
                data: RaydiumLaunchpadAccount::PoolState(Box::new(decoded_account)),
                owner: account.owner,
                executable: account.executable,
                rent_epoch: account.rent_epoch,
            });
        }

        if let Some(decoded_account) =
            vesting_record::VestingRecord::deserialize(account.data.as_slice())
        {
            return Some(carbon_core::account::DecodedAccount {
                lamports: account.lamports,
                data: RaydiumLaunchpadAccount::VestingRecord(decoded_account),
                owner: account.owner,
                executable: account.executable,
                rent_epoch: account.rent_epoch,
            });
        }

        None
    }
}
