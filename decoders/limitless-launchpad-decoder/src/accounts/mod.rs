use crate::{LimitlessLaunchpadDecoder, PROGRAM_ID};

pub mod config;
pub mod launched_coin;
pub mod meteora_graduation;

#[derive(Debug, Clone, PartialEq, Eq)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[cfg_attr(feature = "serde", serde(tag = "type", content = "data"))]
pub enum LimitlessLaunchpadAccount {
    Config(Box<config::Config>),
    LaunchedCoin(Box<launched_coin::LaunchedCoin>),
    MeteoraGraduation(Box<meteora_graduation::MeteoraGraduation>),
}

impl<'a> carbon_core::account::AccountDecoder<'a> for LimitlessLaunchpadDecoder {
    type AccountType = LimitlessLaunchpadAccount;

    fn decode_account(
        &self,
        account: &'a solana_account::Account,
    ) -> Option<carbon_core::account::DecodedAccount<Self::AccountType>> {
        if account.owner != PROGRAM_ID {
            return None;
        }

        let data = account.data.as_slice();

        if let Some(decoded) = launched_coin::LaunchedCoin::decode(data) {
            return Some(carbon_core::account::DecodedAccount {
                lamports: account.lamports,
                data: LimitlessLaunchpadAccount::LaunchedCoin(Box::new(decoded)),
                owner: account.owner,
                executable: account.executable,
                rent_epoch: account.rent_epoch,
            });
        }

        if let Some(decoded) = meteora_graduation::MeteoraGraduation::decode(data) {
            return Some(carbon_core::account::DecodedAccount {
                lamports: account.lamports,
                data: LimitlessLaunchpadAccount::MeteoraGraduation(Box::new(decoded)),
                owner: account.owner,
                executable: account.executable,
                rent_epoch: account.rent_epoch,
            });
        }

        if let Some(decoded) = config::Config::decode(data) {
            return Some(carbon_core::account::DecodedAccount {
                lamports: account.lamports,
                data: LimitlessLaunchpadAccount::Config(Box::new(decoded)),
                owner: account.owner,
                executable: account.executable,
                rent_epoch: account.rent_epoch,
            });
        }

        None
    }
}
