use {
    super::DriftDecoder,
    crate::PROGRAM_ID,
    carbon_core::{account::AccountDecoder, deserialize::CarbonDeserialize},
};
pub mod fuel_overflow;
pub mod high_leverage_mode_config;
pub mod insurance_fund_stake;
pub mod openbook_v2_fulfillment_config;
pub mod perp_market;
pub mod phoenix_v1_fulfillment_config;
pub mod prelaunch_oracle;
pub mod protected_maker_mode_config;
pub mod protocol_if_shares_transfer_config;
pub mod pyth_lazer_oracle;
pub mod referrer_name;
pub mod serum_v3_fulfillment_config;
pub mod signed_msg_user_orders;
pub mod spot_market;
pub mod state;
pub mod user;
pub mod user_stats;

pub enum DriftAccount {
    OpenbookV2FulfillmentConfig(openbook_v2_fulfillment_config::OpenbookV2FulfillmentConfig),
    PhoenixV1FulfillmentConfig(phoenix_v1_fulfillment_config::PhoenixV1FulfillmentConfig),
    SerumV3FulfillmentConfig(serum_v3_fulfillment_config::SerumV3FulfillmentConfig),
    HighLeverageModeConfig(high_leverage_mode_config::HighLeverageModeConfig),
    InsuranceFundStake(insurance_fund_stake::InsuranceFundStake),
    ProtocolIfSharesTransferConfig(
        protocol_if_shares_transfer_config::ProtocolIfSharesTransferConfig,
    ),
    PrelaunchOracle(prelaunch_oracle::PrelaunchOracle),
    PerpMarket(Box<perp_market::PerpMarket>),
    ProtectedMakerModeConfig(protected_maker_mode_config::ProtectedMakerModeConfig),
    PythLazerOracle(pyth_lazer_oracle::PythLazerOracle),
    SignedMsgUserOrders(signed_msg_user_orders::SignedMsgUserOrders),
    SpotMarket(Box<spot_market::SpotMarket>),
    State(Box<state::State>),
    User(Box<user::User>),
    UserStats(user_stats::UserStats),
    ReferrerName(referrer_name::ReferrerName),
    FuelOverflow(fuel_overflow::FuelOverflow),
}

impl AccountDecoder<'_> for DriftDecoder {
    type AccountType = DriftAccount;
    fn decode_account(
        &self,
        account: &solana_account::Account,
    ) -> Option<carbon_core::account::DecodedAccount<Self::AccountType>> {
        if !account.owner.eq(&PROGRAM_ID) {
            return None;
        }

        if let Some(decoded_account) =
            openbook_v2_fulfillment_config::OpenbookV2FulfillmentConfig::deserialize(
                account.data.as_slice(),
            )
        {
            return Some(carbon_core::account::DecodedAccount {
                lamports: account.lamports,
                data: DriftAccount::OpenbookV2FulfillmentConfig(decoded_account),
                owner: account.owner,
                executable: account.executable,
                rent_epoch: account.rent_epoch,
            });
        }

        if let Some(decoded_account) =
            phoenix_v1_fulfillment_config::PhoenixV1FulfillmentConfig::deserialize(
                account.data.as_slice(),
            )
        {
            return Some(carbon_core::account::DecodedAccount {
                lamports: account.lamports,
                data: DriftAccount::PhoenixV1FulfillmentConfig(decoded_account),
                owner: account.owner,
                executable: account.executable,
                rent_epoch: account.rent_epoch,
            });
        }

        if let Some(decoded_account) =
            serum_v3_fulfillment_config::SerumV3FulfillmentConfig::deserialize(
                account.data.as_slice(),
            )
        {
            return Some(carbon_core::account::DecodedAccount {
                lamports: account.lamports,
                data: DriftAccount::SerumV3FulfillmentConfig(decoded_account),
                owner: account.owner,
                executable: account.executable,
                rent_epoch: account.rent_epoch,
            });
        }

        if let Some(decoded_account) =
            high_leverage_mode_config::HighLeverageModeConfig::deserialize(account.data.as_slice())
        {
            return Some(carbon_core::account::DecodedAccount {
                lamports: account.lamports,
                data: DriftAccount::HighLeverageModeConfig(decoded_account),
                owner: account.owner,
                executable: account.executable,
                rent_epoch: account.rent_epoch,
            });
        }

        if let Some(decoded_account) =
            insurance_fund_stake::InsuranceFundStake::deserialize(account.data.as_slice())
        {
            return Some(carbon_core::account::DecodedAccount {
                lamports: account.lamports,
                data: DriftAccount::InsuranceFundStake(decoded_account),
                owner: account.owner,
                executable: account.executable,
                rent_epoch: account.rent_epoch,
            });
        }

        if let Some(decoded_account) =
            protocol_if_shares_transfer_config::ProtocolIfSharesTransferConfig::deserialize(
                account.data.as_slice(),
            )
        {
            return Some(carbon_core::account::DecodedAccount {
                lamports: account.lamports,
                data: DriftAccount::ProtocolIfSharesTransferConfig(decoded_account),
                owner: account.owner,
                executable: account.executable,
                rent_epoch: account.rent_epoch,
            });
        }

        if let Some(decoded_account) =
            prelaunch_oracle::PrelaunchOracle::deserialize(account.data.as_slice())
        {
            return Some(carbon_core::account::DecodedAccount {
                lamports: account.lamports,
                data: DriftAccount::PrelaunchOracle(decoded_account),
                owner: account.owner,
                executable: account.executable,
                rent_epoch: account.rent_epoch,
            });
        }

        if let Some(decoded_account) = perp_market::PerpMarket::deserialize(account.data.as_slice())
        {
            return Some(carbon_core::account::DecodedAccount {
                lamports: account.lamports,
                data: DriftAccount::PerpMarket(Box::new(decoded_account)),
                owner: account.owner,
                executable: account.executable,
                rent_epoch: account.rent_epoch,
            });
        }

        if let Some(decoded_account) =
            protected_maker_mode_config::ProtectedMakerModeConfig::deserialize(
                account.data.as_slice(),
            )
        {
            return Some(carbon_core::account::DecodedAccount {
                lamports: account.lamports,
                data: DriftAccount::ProtectedMakerModeConfig(decoded_account),
                owner: account.owner,
                executable: account.executable,
                rent_epoch: account.rent_epoch,
            });
        }

        if let Some(decoded_account) =
            pyth_lazer_oracle::PythLazerOracle::deserialize(account.data.as_slice())
        {
            return Some(carbon_core::account::DecodedAccount {
                lamports: account.lamports,
                data: DriftAccount::PythLazerOracle(decoded_account),
                owner: account.owner,
                executable: account.executable,
                rent_epoch: account.rent_epoch,
            });
        }

        if let Some(decoded_account) =
            signed_msg_user_orders::SignedMsgUserOrders::deserialize(account.data.as_slice())
        {
            return Some(carbon_core::account::DecodedAccount {
                lamports: account.lamports,
                data: DriftAccount::SignedMsgUserOrders(decoded_account),
                owner: account.owner,
                executable: account.executable,
                rent_epoch: account.rent_epoch,
            });
        }

        if let Some(decoded_account) = spot_market::SpotMarket::deserialize(account.data.as_slice())
        {
            return Some(carbon_core::account::DecodedAccount {
                lamports: account.lamports,
                data: DriftAccount::SpotMarket(Box::new(decoded_account)),
                owner: account.owner,
                executable: account.executable,
                rent_epoch: account.rent_epoch,
            });
        }

        if let Some(decoded_account) = state::State::deserialize(account.data.as_slice()) {
            return Some(carbon_core::account::DecodedAccount {
                lamports: account.lamports,
                data: DriftAccount::State(Box::new(decoded_account)),
                owner: account.owner,
                executable: account.executable,
                rent_epoch: account.rent_epoch,
            });
        }

        if let Some(decoded_account) = user::User::deserialize(account.data.as_slice()) {
            return Some(carbon_core::account::DecodedAccount {
                lamports: account.lamports,
                data: DriftAccount::User(Box::new(decoded_account)),
                owner: account.owner,
                executable: account.executable,
                rent_epoch: account.rent_epoch,
            });
        }

        if let Some(decoded_account) = user_stats::UserStats::deserialize(account.data.as_slice()) {
            return Some(carbon_core::account::DecodedAccount {
                lamports: account.lamports,
                data: DriftAccount::UserStats(decoded_account),
                owner: account.owner,
                executable: account.executable,
                rent_epoch: account.rent_epoch,
            });
        }

        if let Some(decoded_account) =
            referrer_name::ReferrerName::deserialize(account.data.as_slice())
        {
            return Some(carbon_core::account::DecodedAccount {
                lamports: account.lamports,
                data: DriftAccount::ReferrerName(decoded_account),
                owner: account.owner,
                executable: account.executable,
                rent_epoch: account.rent_epoch,
            });
        }

        if let Some(decoded_account) =
            fuel_overflow::FuelOverflow::deserialize(account.data.as_slice())
        {
            return Some(carbon_core::account::DecodedAccount {
                lamports: account.lamports,
                data: DriftAccount::FuelOverflow(decoded_account),
                owner: account.owner,
                executable: account.executable,
                rent_epoch: account.rent_epoch,
            });
        }

        None
    }
}
