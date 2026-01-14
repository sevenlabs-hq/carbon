use {
    async_trait::async_trait,
    carbon_core::{
        account::{AccountDecoder, AccountProcessorInputType, DecodedAccount},
        error::CarbonResult,
        metrics::MetricsCollection,
        pipeline::Pipeline,
        processor::Processor,
    },
    carbon_log_metrics::LogMetrics,
    carbon_validator_snapshot_datasource::{SnapshotDatasource, SnapshotSource},
    solana_account::Account,
    solana_pubkey::Pubkey,
    std::{env, sync::Arc},
};

/// Generic account decoder that passes through raw account data
pub struct GenericAccountDecoder;

impl<'a> AccountDecoder<'a> for GenericAccountDecoder {
    type AccountType = Account;

    fn decode_account(&self, account: &'a Account) -> Option<DecodedAccount<Self::AccountType>> {
        Some(DecodedAccount {
            lamports: account.lamports,
            data: account.clone(),
            owner: account.owner,
            executable: account.executable,
            rent_epoch: account.rent_epoch,
        })
    }
}

/// Account processor that logs each account
pub struct SnapshotAccountProcessor;

#[async_trait]
impl Processor for SnapshotAccountProcessor {
    type InputType = AccountProcessorInputType<Account>;

    async fn process(
        &mut self,
        (metadata, decoded_account, raw_account): Self::InputType,
        _metrics: Arc<MetricsCollection>,
    ) -> CarbonResult<()> {
        let pubkey_str = format!(
            "{}...{}",
            &metadata.pubkey.to_string()[..4],
            &metadata.pubkey.to_string()[metadata.pubkey.to_string().len() - 4..]
        );

        log::info!(
            "Account: pubkey={} slot={} lamports={} data_size={} owner={} executable={} rent_epoch={}",
            pubkey_str,
            metadata.slot,
            decoded_account.lamports,
            raw_account.data.len(),
            decoded_account.owner,
            decoded_account.executable,
            decoded_account.rent_epoch
        );

        Ok(())
    }
}

fn parse_pubkeys(env_var: &str) -> Vec<Pubkey> {
    env::var(env_var)
        .unwrap_or_default()
        .split(',')
        .map(|s| s.trim())
        .filter(|s| !s.is_empty())
        .map(|s| {
            s.parse::<Pubkey>()
                .unwrap_or_else(|_| panic!("Invalid pubkey in {env_var}: {s}"))
        })
        .collect()
}

#[tokio::main]
async fn main() -> CarbonResult<()> {
    dotenv::dotenv().ok();
    env_logger::init();

    let owners = parse_pubkeys("PROGRAM_OWNERS");
    let accounts = parse_pubkeys("ACCOUNT_IDS");

    if owners.is_empty() && accounts.is_empty() {
        panic!("Either PROGRAM_OWNERS or ACCOUNT_IDS must be set");
    }

    let datasource = if let Ok(validator_url) = env::var("VALIDATOR_URL") {
        log::info!("Using remote snapshot from: {validator_url}");
        SnapshotDatasource::new(
            SnapshotSource::Remote { url: validator_url },
            owners,
            accounts,
        )
    } else if let Ok(snapshot_path) = env::var("SNAPSHOT_PATH") {
        log::info!("Using local snapshot from: {snapshot_path}");
        SnapshotDatasource::new(
            SnapshotSource::LocalPath(snapshot_path.into()),
            owners,
            accounts,
        )
    } else {
        panic!("Either VALIDATOR_URL or SNAPSHOT_PATH must be set");
    };

    Pipeline::builder()
        .datasource(datasource)
        .account(GenericAccountDecoder, SnapshotAccountProcessor)
        .metrics(Arc::new(LogMetrics::new()))
        .metrics_flush_interval(
            env::var("METRICS_FLUSH_INTERVAL")
                .ok()
                .and_then(|v| v.parse().ok())
                .unwrap_or(10),
        )
        .build()?
        .run()
        .await?;

    Ok(())
}
