mod downloader;

use downloader::download_and_setup_ledger;

use {
    agave_snapshots::snapshot_config::{SnapshotConfig, SnapshotUsage},
    async_trait::async_trait,
    carbon_core::{
        datasource::{AccountUpdate, Datasource, DatasourceId, Update, UpdateType},
        error::{CarbonResult, Error},
        metrics::MetricsCollection,
    },
    solana_account::ReadableAccount,
    solana_accounts_db::{
        accounts_db::AccountsDbConfig,
        accounts_index::{ScanConfig, ScanOrder},
        is_loadable::IsLoadable,
        utils::create_all_accounts_run_and_snapshot_dirs,
    },
    solana_genesis_utils::open_genesis_config,
    solana_ledger::{
        bank_forks_utils,
        blockstore::Blockstore,
        blockstore_options::{AccessType, BlockstoreOptions},
        blockstore_processor::ProcessOptions,
    },
    solana_pubkey::Pubkey as SolanaPubkey,
    solana_pubkey::Pubkey,
    solana_runtime::bank::Bank,
    std::{
        collections::HashSet,
        path::PathBuf,
        sync::{atomic::AtomicBool, Arc},
    },
    tokio::sync::mpsc::Sender,
    tokio_util::sync::CancellationToken,
};

const MAX_GENESIS_ARCHIVE_UNPACKED_SIZE: u64 = 10_485_760; // 10MB

#[derive(Debug, Clone)]
pub enum SnapshotSource {
    LocalPath(PathBuf),
    Remote { url: String },
}

#[derive(Debug, Clone)]
pub struct SnapshotDatasource {
    pub source: SnapshotSource,
    pub owners: Vec<Pubkey>,
    pub accounts: Vec<Pubkey>,
}

impl SnapshotDatasource {
    pub fn new(source: SnapshotSource, owners: Vec<Pubkey>, accounts: Vec<Pubkey>) -> Self {
        Self {
            source,
            owners,
            accounts,
        }
    }
}

struct SnapshotState {
    bank: Arc<Bank>,
    slot: u64,
}

impl SnapshotState {
    fn load_from_path(ledger_directory: PathBuf) -> CarbonResult<Self> {
        log::info!("Initializing snapshot from ledger directory: {ledger_directory:?}");

        if !ledger_directory.exists() {
            return Err(Error::FailedToConsumeDatasource(format!(
                "Ledger directory does not exist: {ledger_directory:?}"
            )));
        }

        let genesis_archive = ledger_directory.join("genesis.tar.bz2");
        if !genesis_archive.exists() {
            return Err(Error::FailedToConsumeDatasource(format!(
                "Genesis archive not found: {genesis_archive:?}. Expected genesis.tar.bz2"
            )));
        }

        let genesis_config =
            open_genesis_config(&ledger_directory, MAX_GENESIS_ARCHIVE_UNPACKED_SIZE).map_err(
                |err| {
                    Error::FailedToConsumeDatasource(format!(
                        "Unable to load genesis configuration from {ledger_directory:?}: {err}"
                    ))
                },
            )?;

        let snapshots_directory = ledger_directory.join("snapshots");
        let accounts_directory = ledger_directory.join("accounts");

        if !snapshots_directory.parent().is_some_and(|p| p.exists()) {
            return Err(Error::FailedToConsumeDatasource(format!(
                "Cannot create snapshots directory: parent path does not exist: {snapshots_directory:?}"
            )));
        }

        let (account_run_directories, _snapshot_directories) =
            create_all_accounts_run_and_snapshot_dirs(&[accounts_directory]).map_err(|err| {
                Error::FailedToConsumeDatasource(format!(
                    "Failed to initialize account directory structure: {err}"
                ))
            })?;

        let blockstore = Blockstore::open_with_options(
            &ledger_directory,
            BlockstoreOptions {
                access_type: AccessType::PrimaryForMaintenance,
                ..BlockstoreOptions::default()
            },
        )
        .map_err(|err| {
            Error::FailedToConsumeDatasource(format!(
                "Failed to open blockstore at {ledger_directory:?}: {err}"
            ))
        })?;

        let accounts_db_settings = AccountsDbConfig {
            base_working_path: Some(ledger_directory.clone()),
            skip_initial_hash_calc: true,
            ..AccountsDbConfig::default()
        };

        let snapshot_settings = SnapshotConfig {
            usage: SnapshotUsage::LoadOnly,
            full_snapshot_archives_dir: snapshots_directory.clone(),
            incremental_snapshot_archives_dir: snapshots_directory.clone(),
            bank_snapshots_dir: snapshots_directory,
            ..SnapshotConfig::default()
        };

        let processing_settings = ProcessOptions {
            accounts_db_skip_shrink: true,
            accounts_db_config: accounts_db_settings,
            ..Default::default()
        };

        let (bank_forks, _leader_schedule_cache, _snapshot_hashes, ..) =
            bank_forks_utils::load_bank_forks(
                &genesis_config,
                &blockstore,
                account_run_directories,
                &snapshot_settings,
                &processing_settings,
                None,
                None,
                None,
                Arc::new(AtomicBool::new(false)),
            )
            .map_err(|err| {
                Error::FailedToConsumeDatasource(format!(
                    "Failed to load bank forks from snapshot: {err}"
                ))
            })?;

        let bank = bank_forks
            .read()
            .map_err(|_| {
                Error::FailedToConsumeDatasource(
                    "Bank forks lock was poisoned during snapshot load".to_string(),
                )
            })?
            .working_bank();
        let slot = bank.slot();

        if slot == 0 {
            return Err(Error::FailedToConsumeDatasource(
                "Invalid snapshot: loaded bank has slot 0, which indicates an uninitialized or corrupted snapshot".to_string(),
            ));
        }

        log::info!(
            "Snapshot initialization complete: loaded bank at slot {slot} from {ledger_directory:?}"
        );

        Ok(Self { bank, slot })
    }

    pub async fn load_from_remote(
        url: &str,
        cancellation_token: CancellationToken,
    ) -> CarbonResult<Self> {
        let ledger_path = download_and_setup_ledger(url, cancellation_token).await?;
        Self::load_from_path(ledger_path)
    }
}

#[async_trait]
impl Datasource for SnapshotDatasource {
    async fn consume(
        &self,
        id: DatasourceId,
        sender: Sender<(Update, DatasourceId)>,
        cancellation_token: CancellationToken,
        metrics: Arc<MetricsCollection>,
    ) -> CarbonResult<()> {
        let load_start = std::time::Instant::now();

        let snapshot = match &self.source {
            SnapshotSource::LocalPath(path) => SnapshotState::load_from_path(path.clone())?,
            SnapshotSource::Remote { url } => {
                SnapshotState::load_from_remote(url, cancellation_token.clone()).await?
            }
        };

        let load_duration = load_start.elapsed();
        metrics
            .record_histogram(
                "snapshot_load_duration_seconds",
                load_duration.as_secs_f64(),
            )
            .await
            .unwrap_or_else(|e| log::error!("Error recording metric: {e}"));

        log::info!(
            "Snapshot loaded at slot {} in {:.2}s, starting account scan",
            snapshot.slot,
            load_duration.as_secs_f64()
        );

        let bank = snapshot.bank;
        let owners = self.owners.clone();
        let accounts = self.accounts.clone();
        let snapshot_slot = snapshot.slot;
        let sender_clone = sender.clone();
        let id_clone = id.clone();
        let cancellation_token_clone = cancellation_token.clone();

        let scan_start = std::time::Instant::now();
        tokio::task::spawn_blocking(move || {
            let owners_set: HashSet<SolanaPubkey> = owners.iter().copied().collect();
            let accounts_set: HashSet<SolanaPubkey> = accounts.iter().copied().collect();

            if owners_set.is_empty() && accounts_set.is_empty() {
                log::warn!("No owners or accounts specified, skipping account scan");
                return Ok::<(), Error>(());
            }

            log::info!(
                "Scanning accounts for {} owner(s) and {} specific account(s) at slot {}",
                owners_set.len(),
                accounts_set.len(),
                snapshot_slot
            );

            let scan_config = ScanConfig::new(ScanOrder::Sorted);
            let mut channel_closed = false;

            bank.rc
                .accounts
                .accounts_db
                .scan_accounts(
                    &bank.ancestors,
                    bank.bank_id(),
                    |option| {
                        if cancellation_token_clone.is_cancelled() {
                            return;
                        }

                        if channel_closed {
                            return;
                        }

                        if let Some((pubkey, account, _slot)) = option {
                            if !account.is_loadable() {
                                return;
                            }

                            let owner = account.owner();
                            if !owners_set.contains(owner) && !accounts_set.contains(pubkey) {
                                return;
                            }

                            let account_update = AccountUpdate {
                                pubkey: *pubkey,
                                account: solana_account::Account {
                                    lamports: account.lamports(),
                                    data: account.data().to_vec(),
                                    owner: *owner,
                                    executable: account.executable(),
                                    rent_epoch: account.rent_epoch(),
                                },
                                slot: snapshot_slot,
                                transaction_signature: None,
                            };

                            let update = Update::Account(account_update);

                            if let Err(e) = sender_clone.blocking_send((update, id_clone.clone())) {
                                log::error!("Failed to send account update: {e:?}");
                                channel_closed = true;
                            }
                        }
                    },
                    &scan_config,
                )
                .map_err(|e| Error::FailedToConsumeDatasource(format!("Scan failed: {e}")))?;

            log::info!("Account scan completed");

            Ok(())
        })
        .await
        .map_err(|e| Error::FailedToConsumeDatasource(format!("Scan task panicked: {e}")))??;

        let scan_duration = scan_start.elapsed();
        metrics
            .record_histogram(
                "snapshot_scan_duration_seconds",
                scan_duration.as_secs_f64(),
            )
            .await
            .unwrap_or_else(|e| log::error!("Error recording metric: {e}"));

        log::info!(
            "Snapshot processing completed in {:.2}s",
            scan_duration.as_secs_f64()
        );

        Ok(())
    }

    fn update_types(&self) -> Vec<UpdateType> {
        vec![UpdateType::AccountUpdate]
    }
}
