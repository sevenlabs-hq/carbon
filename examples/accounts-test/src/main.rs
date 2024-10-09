mod db;
mod pump_decoder;
use async_trait::async_trait;
use carbon_core::account::{AccountMetadata, DecodedAccount};
use carbon_core::datasource::AccountDeletion;
use carbon_core::error::CarbonResult;
use carbon_core::processor::Processor;
use carbon_yellowstone_grpc_datasource::YellowstoneGrpcGeyserClient;
use db::models::{BondingCurve, GlobalAccount};
use db::schema::{bonding_curve, global_account};
use diesel::{ExpressionMethods, QueryDsl, RunQueryDsl};
use pump_decoder::accounts::PumpAccount;
use pump_decoder::PumpDecoder;
use solana_sdk::pubkey::Pubkey;
use std::collections::{HashMap, HashSet};
use std::str::FromStr;
use std::sync::Arc;
use tokio::sync::RwLock;
use yellowstone_grpc_proto::geyser::{CommitmentLevel, SubscribeRequestFilterTransactions};

pub struct PumpfunAccountProcessor {
    pub accounts_tracked: Arc<RwLock<HashSet<Pubkey>>>,
}

#[async_trait]
impl Processor for PumpfunAccountProcessor {
    type InputType = (AccountMetadata, DecodedAccount<PumpAccount>);

    async fn process(&mut self, data: Self::InputType) -> CarbonResult<()> {
        match data.1.data {
            PumpAccount::Global(account) => {
                let global_account = GlobalAccount::from_account(account, data.0.pubkey);

                diesel::insert_into(global_account::table)
                    .values(&global_account)
                    .on_conflict(global_account::id)
                    .do_update()
                    .set(&global_account)
                    .execute(&mut db::get_conn())
                    .unwrap();
            }
            PumpAccount::BondingCurve(account) => {
                let bonding_curve = BondingCurve::from_account(account, data.0.pubkey);

                diesel::insert_into(bonding_curve::table)
                    .values(&bonding_curve)
                    .on_conflict(bonding_curve::id)
                    .do_update()
                    .set(&bonding_curve)
                    .execute(&mut db::get_conn())
                    .unwrap();
            }
        }

        if !self.accounts_tracked.read().await.contains(&data.0.pubkey) {
            self.accounts_tracked.write().await.insert(data.0.pubkey);
        }

        Ok(())
    }
}

pub struct PumpfunAccountDeletionProcessor {
    pub accounts_tracked: Arc<RwLock<HashSet<Pubkey>>>,
}

#[async_trait]
impl Processor for PumpfunAccountDeletionProcessor {
    type InputType = AccountDeletion;

    async fn process(&mut self, data: Self::InputType) -> CarbonResult<()> {
        if self.accounts_tracked.read().await.contains(&data.pubkey) {
            if diesel::select(diesel::dsl::exists(
                bonding_curve::table.filter(bonding_curve::pubkey.eq(data.pubkey.to_string())),
            ))
            .get_result(&mut db::get_conn())
            .unwrap_or(false)
            {
                diesel::delete(bonding_curve::table)
                    .filter(bonding_curve::pubkey.eq(data.pubkey.to_string()))
                    .execute(&mut db::get_conn())
                    .unwrap();
            } else if diesel::select(diesel::dsl::exists(
                global_account::table.filter(global_account::pubkey.eq(data.pubkey.to_string())),
            ))
            .get_result(&mut db::get_conn())
            .unwrap_or(false)
            {
                diesel::delete(global_account::table)
                    .filter(global_account::pubkey.eq(data.pubkey.to_string()))
                    .execute(&mut db::get_conn())
                    .unwrap();
            }

            self.accounts_tracked.write().await.remove(&data.pubkey);
        }

        Ok(())
    }
}

#[tokio::main]
pub async fn main() -> CarbonResult<()> {
    env_logger::init();

    let mut bonding_curve_pubkeys = diesel::query_dsl::methods::FilterDsl::filter(
        crate::db::schema::bonding_curve::dsl::bonding_curve,
        crate::db::schema::bonding_curve::dsl::id.is_not_null(),
    )
    .select(crate::db::schema::bonding_curve::dsl::pubkey)
    .load::<String>(&mut db::get_conn())
    .expect("Failed to load tracked wallets")
    .into_iter()
    .collect::<HashSet<String>>();
    log::trace!("Fetched wallets that are tracked...");

    let global_account_pubkeys = diesel::query_dsl::methods::FilterDsl::filter(
        crate::db::schema::global_account::dsl::global_account,
        crate::db::schema::global_account::dsl::id.is_not_null(),
    )
    .select(crate::db::schema::global_account::dsl::pubkey)
    .load::<String>(&mut db::get_conn())
    .expect("Failed to load tracked wallets")
    .into_iter()
    .collect::<HashSet<String>>();
    log::trace!("Fetched wallets that are tracked...");

    bonding_curve_pubkeys.extend(global_account_pubkeys);

    let account_deletions = bonding_curve_pubkeys
        .iter()
        .map(|str| Pubkey::from_str(str).unwrap())
        .collect::<HashSet<Pubkey>>();

    let tracked_accounts = Arc::new(RwLock::new(account_deletions));

    let account_filters = HashMap::new();
    let mut transaction_filters = HashMap::new();
    transaction_filters.insert(
        "pumpfun".to_string(),
        SubscribeRequestFilterTransactions {
            vote: None,
            failed: None,
            signature: None,
            account_include: vec![],
            account_exclude: vec![],
            account_required: vec!["6EF8rrecthR5Dkzon8Nwu78hRvfCKubJ14M5uBEwF6P".to_owned()],
        },
    );

    let grpc = YellowstoneGrpcGeyserClient {
        commitment: Some(CommitmentLevel::Processed),
        account_filters,
        transaction_filters,
        account_deletions_tracked: tracked_accounts.clone(),
        // TODO: Replace with actual endpoint
        endpoint: "".to_string(),
        x_token: Some("".to_string()),
    };

    carbon_core::pipeline::Pipeline::builder()
        .datasource(grpc)
        .account(
            PumpDecoder,
            PumpfunAccountProcessor {
                accounts_tracked: tracked_accounts.clone(),
            },
        )
        .account_deletions(PumpfunAccountDeletionProcessor {
            accounts_tracked: tracked_accounts.clone(),
        })
        .build()?
        .run()
        .await?;

    Ok(())
}
