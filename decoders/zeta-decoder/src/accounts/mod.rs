use {
    super::ZetaDecoder,
    crate::PROGRAM_ID,
    alloc::boxed::Box,
    carbon_core::{account::AccountDecoder, deserialize::CarbonDeserialize},
};
pub mod cross_margin_account;
pub mod cross_margin_account_manager;
pub mod cross_open_orders_map;
pub mod greeks;
pub mod insurance_deposit_account;
pub mod margin_account;
pub mod market_indexes;
pub mod market_node;
pub mod open_orders_map;
pub mod perp_sync_queue;
pub mod pricing;
pub mod referrer_id_account;
pub mod referrer_pubkey_account;
pub mod settlement_account;
pub mod socialized_loss_account;
pub mod spread_account;
pub mod state;
pub mod trigger_order;
pub mod underlying;
pub mod whitelist_deposit_account;
pub mod whitelist_insurance_account;
pub mod whitelist_trading_fees_account;
pub mod zeta_group;

pub enum ZetaAccount {
    Pricing(Box<pricing::Pricing>),
    Greeks(Box<greeks::Greeks>),
    MarketIndexes(market_indexes::MarketIndexes),
    OpenOrdersMap(open_orders_map::OpenOrdersMap),
    CrossOpenOrdersMap(cross_open_orders_map::CrossOpenOrdersMap),
    State(state::State),
    Underlying(underlying::Underlying),
    SettlementAccount(settlement_account::SettlementAccount),
    PerpSyncQueue(perp_sync_queue::PerpSyncQueue),
    ZetaGroup(zeta_group::ZetaGroup),
    MarketNode(market_node::MarketNode),
    SpreadAccount(spread_account::SpreadAccount),
    CrossMarginAccountManager(cross_margin_account_manager::CrossMarginAccountManager),
    CrossMarginAccount(cross_margin_account::CrossMarginAccount),
    MarginAccount(margin_account::MarginAccount),
    TriggerOrder(trigger_order::TriggerOrder),
    SocializedLossAccount(socialized_loss_account::SocializedLossAccount),
    WhitelistDepositAccount(whitelist_deposit_account::WhitelistDepositAccount),
    WhitelistInsuranceAccount(whitelist_insurance_account::WhitelistInsuranceAccount),
    InsuranceDepositAccount(insurance_deposit_account::InsuranceDepositAccount),
    WhitelistTradingFeesAccount(whitelist_trading_fees_account::WhitelistTradingFeesAccount),
    ReferrerIdAccount(referrer_id_account::ReferrerIdAccount),
    ReferrerPubkeyAccount(referrer_pubkey_account::ReferrerPubkeyAccount),
}

impl AccountDecoder<'_> for ZetaDecoder {
    type AccountType = ZetaAccount;
    fn decode_account(
        &self,
        account: &solana_account::Account,
    ) -> Option<carbon_core::account::DecodedAccount<Self::AccountType>> {
        if !account.owner.eq(&PROGRAM_ID) {
            return None;
        }

        if let Some(decoded_account) = pricing::Pricing::deserialize(account.data.as_slice()) {
            return Some(carbon_core::account::DecodedAccount {
                lamports: account.lamports,
                data: ZetaAccount::Pricing(Box::new(decoded_account)),
                owner: account.owner,
                executable: account.executable,
                rent_epoch: account.rent_epoch,
            });
        }

        if let Some(decoded_account) = greeks::Greeks::deserialize(account.data.as_slice()) {
            return Some(carbon_core::account::DecodedAccount {
                lamports: account.lamports,
                data: ZetaAccount::Greeks(Box::new(decoded_account)),
                owner: account.owner,
                executable: account.executable,
                rent_epoch: account.rent_epoch,
            });
        }

        if let Some(decoded_account) =
            market_indexes::MarketIndexes::deserialize(account.data.as_slice())
        {
            return Some(carbon_core::account::DecodedAccount {
                lamports: account.lamports,
                data: ZetaAccount::MarketIndexes(decoded_account),
                owner: account.owner,
                executable: account.executable,
                rent_epoch: account.rent_epoch,
            });
        }

        if let Some(decoded_account) =
            open_orders_map::OpenOrdersMap::deserialize(account.data.as_slice())
        {
            return Some(carbon_core::account::DecodedAccount {
                lamports: account.lamports,
                data: ZetaAccount::OpenOrdersMap(decoded_account),
                owner: account.owner,
                executable: account.executable,
                rent_epoch: account.rent_epoch,
            });
        }

        if let Some(decoded_account) =
            cross_open_orders_map::CrossOpenOrdersMap::deserialize(account.data.as_slice())
        {
            return Some(carbon_core::account::DecodedAccount {
                lamports: account.lamports,
                data: ZetaAccount::CrossOpenOrdersMap(decoded_account),
                owner: account.owner,
                executable: account.executable,
                rent_epoch: account.rent_epoch,
            });
        }

        if let Some(decoded_account) = state::State::deserialize(account.data.as_slice()) {
            return Some(carbon_core::account::DecodedAccount {
                lamports: account.lamports,
                data: ZetaAccount::State(decoded_account),
                owner: account.owner,
                executable: account.executable,
                rent_epoch: account.rent_epoch,
            });
        }

        if let Some(decoded_account) = underlying::Underlying::deserialize(account.data.as_slice())
        {
            return Some(carbon_core::account::DecodedAccount {
                lamports: account.lamports,
                data: ZetaAccount::Underlying(decoded_account),
                owner: account.owner,
                executable: account.executable,
                rent_epoch: account.rent_epoch,
            });
        }

        if let Some(decoded_account) =
            settlement_account::SettlementAccount::deserialize(account.data.as_slice())
        {
            return Some(carbon_core::account::DecodedAccount {
                lamports: account.lamports,
                data: ZetaAccount::SettlementAccount(decoded_account),
                owner: account.owner,
                executable: account.executable,
                rent_epoch: account.rent_epoch,
            });
        }

        if let Some(decoded_account) =
            perp_sync_queue::PerpSyncQueue::deserialize(account.data.as_slice())
        {
            return Some(carbon_core::account::DecodedAccount {
                lamports: account.lamports,
                data: ZetaAccount::PerpSyncQueue(decoded_account),
                owner: account.owner,
                executable: account.executable,
                rent_epoch: account.rent_epoch,
            });
        }

        if let Some(decoded_account) = zeta_group::ZetaGroup::deserialize(account.data.as_slice()) {
            return Some(carbon_core::account::DecodedAccount {
                lamports: account.lamports,
                data: ZetaAccount::ZetaGroup(decoded_account),
                owner: account.owner,
                executable: account.executable,
                rent_epoch: account.rent_epoch,
            });
        }

        if let Some(decoded_account) = market_node::MarketNode::deserialize(account.data.as_slice())
        {
            return Some(carbon_core::account::DecodedAccount {
                lamports: account.lamports,
                data: ZetaAccount::MarketNode(decoded_account),
                owner: account.owner,
                executable: account.executable,
                rent_epoch: account.rent_epoch,
            });
        }

        if let Some(decoded_account) =
            spread_account::SpreadAccount::deserialize(account.data.as_slice())
        {
            return Some(carbon_core::account::DecodedAccount {
                lamports: account.lamports,
                data: ZetaAccount::SpreadAccount(decoded_account),
                owner: account.owner,
                executable: account.executable,
                rent_epoch: account.rent_epoch,
            });
        }

        if let Some(decoded_account) =
            cross_margin_account_manager::CrossMarginAccountManager::deserialize(
                account.data.as_slice(),
            )
        {
            return Some(carbon_core::account::DecodedAccount {
                lamports: account.lamports,
                data: ZetaAccount::CrossMarginAccountManager(decoded_account),
                owner: account.owner,
                executable: account.executable,
                rent_epoch: account.rent_epoch,
            });
        }

        if let Some(decoded_account) =
            cross_margin_account::CrossMarginAccount::deserialize(account.data.as_slice())
        {
            return Some(carbon_core::account::DecodedAccount {
                lamports: account.lamports,
                data: ZetaAccount::CrossMarginAccount(decoded_account),
                owner: account.owner,
                executable: account.executable,
                rent_epoch: account.rent_epoch,
            });
        }

        if let Some(decoded_account) =
            margin_account::MarginAccount::deserialize(account.data.as_slice())
        {
            return Some(carbon_core::account::DecodedAccount {
                lamports: account.lamports,
                data: ZetaAccount::MarginAccount(decoded_account),
                owner: account.owner,
                executable: account.executable,
                rent_epoch: account.rent_epoch,
            });
        }

        if let Some(decoded_account) =
            trigger_order::TriggerOrder::deserialize(account.data.as_slice())
        {
            return Some(carbon_core::account::DecodedAccount {
                lamports: account.lamports,
                data: ZetaAccount::TriggerOrder(decoded_account),
                owner: account.owner,
                executable: account.executable,
                rent_epoch: account.rent_epoch,
            });
        }

        if let Some(decoded_account) =
            socialized_loss_account::SocializedLossAccount::deserialize(account.data.as_slice())
        {
            return Some(carbon_core::account::DecodedAccount {
                lamports: account.lamports,
                data: ZetaAccount::SocializedLossAccount(decoded_account),
                owner: account.owner,
                executable: account.executable,
                rent_epoch: account.rent_epoch,
            });
        }

        if let Some(decoded_account) =
            whitelist_deposit_account::WhitelistDepositAccount::deserialize(account.data.as_slice())
        {
            return Some(carbon_core::account::DecodedAccount {
                lamports: account.lamports,
                data: ZetaAccount::WhitelistDepositAccount(decoded_account),
                owner: account.owner,
                executable: account.executable,
                rent_epoch: account.rent_epoch,
            });
        }

        if let Some(decoded_account) =
            whitelist_insurance_account::WhitelistInsuranceAccount::deserialize(
                account.data.as_slice(),
            )
        {
            return Some(carbon_core::account::DecodedAccount {
                lamports: account.lamports,
                data: ZetaAccount::WhitelistInsuranceAccount(decoded_account),
                owner: account.owner,
                executable: account.executable,
                rent_epoch: account.rent_epoch,
            });
        }

        if let Some(decoded_account) =
            insurance_deposit_account::InsuranceDepositAccount::deserialize(account.data.as_slice())
        {
            return Some(carbon_core::account::DecodedAccount {
                lamports: account.lamports,
                data: ZetaAccount::InsuranceDepositAccount(decoded_account),
                owner: account.owner,
                executable: account.executable,
                rent_epoch: account.rent_epoch,
            });
        }

        if let Some(decoded_account) =
            whitelist_trading_fees_account::WhitelistTradingFeesAccount::deserialize(
                account.data.as_slice(),
            )
        {
            return Some(carbon_core::account::DecodedAccount {
                lamports: account.lamports,
                data: ZetaAccount::WhitelistTradingFeesAccount(decoded_account),
                owner: account.owner,
                executable: account.executable,
                rent_epoch: account.rent_epoch,
            });
        }

        if let Some(decoded_account) =
            referrer_id_account::ReferrerIdAccount::deserialize(account.data.as_slice())
        {
            return Some(carbon_core::account::DecodedAccount {
                lamports: account.lamports,
                data: ZetaAccount::ReferrerIdAccount(decoded_account),
                owner: account.owner,
                executable: account.executable,
                rent_epoch: account.rent_epoch,
            });
        }

        if let Some(decoded_account) =
            referrer_pubkey_account::ReferrerPubkeyAccount::deserialize(account.data.as_slice())
        {
            return Some(carbon_core::account::DecodedAccount {
                lamports: account.lamports,
                data: ZetaAccount::ReferrerPubkeyAccount(decoded_account),
                owner: account.owner,
                executable: account.executable,
                rent_epoch: account.rent_epoch,
            });
        }

        None
    }
}
