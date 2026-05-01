use {
    carbon_core::{
        account::{AccountDecoder, AccountProcessorInputType, DecodedAccount},
        datasource::AccountDeletion,
        error::CarbonResult,
        pipeline::Pipeline,
        processor::Processor,
    },
    carbon_log_metrics::LogMetrics,
    carbon_validator_snapshot_datasource::{SnapshotDatasource, SnapshotSource},
    solana_account::Account,
    solana_pubkey::Pubkey,
    std::{env, sync::Arc},
};

#[tokio::main]
pub async fn main() -> CarbonResult<()> {
    dotenv::dotenv().ok();
    env_logger::init();

    let owners = parse_pubkeys("PROGRAM_OWNERS");
    let accounts = parse_pubkeys("ACCOUNT_IDS");

    if owners.is_empty() && accounts.is_empty() {
        panic!("set at least one of PROGRAM_OWNERS or ACCOUNT_IDS");
    }

    let source = match (
        env::var("VALIDATOR_URL").ok(),
        env::var("SNAPSHOT_PATH").ok(),
    ) {
        (Some(url), _) => SnapshotSource::Remote { url },
        (None, Some(path)) => SnapshotSource::LocalPath(path.into()),
        (None, None) => panic!("set VALIDATOR_URL or SNAPSHOT_PATH"),
    };

    let datasource = SnapshotDatasource::new(source, owners, accounts);

    Pipeline::builder()
        .datasource(datasource)
        .metrics(Arc::new(LogMetrics::new()))
        .account(GenericAccountDecoder, SnapshotAccountProcessor)
        .account_deletions(AccountDeletionProcessor)
        .shutdown_strategy(carbon_core::pipeline::ShutdownStrategy::ProcessPending)
        .build()?
        .run()
        .await?;

    log::info!("snapshot bootstrap complete");
    Ok(())
}

fn parse_pubkeys(var: &str) -> Vec<Pubkey> {
    env::var(var)
        .unwrap_or_default()
        .split(',')
        .map(str::trim)
        .filter(|s| !s.is_empty())
        .map(|s| {
            s.parse::<Pubkey>()
                .unwrap_or_else(|_| panic!("invalid pubkey in {var}: {s}"))
        })
        .collect()
}

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

pub struct SnapshotAccountProcessor;

impl Processor<AccountProcessorInputType<'_, Account>> for SnapshotAccountProcessor {
    async fn process(
        &mut self,
        input: &AccountProcessorInputType<'_, Account>,
    ) -> CarbonResult<()> {
        let raw = &input.decoded_account.data;
        log::info!(
            "account pubkey={} slot={} lamports={} data_size={} owner={}",
            input.metadata.pubkey,
            input.metadata.slot,
            raw.lamports,
            raw.data.len(),
            raw.owner,
        );
        Ok(())
    }
}

pub struct AccountDeletionProcessor;

impl Processor<AccountDeletion> for AccountDeletionProcessor {
    async fn process(&mut self, deletion: &AccountDeletion) -> CarbonResult<()> {
        log::info!(
            "account_deletion pubkey={} slot={}",
            deletion.pubkey,
            deletion.slot
        );
        Ok(())
    }
}
