mod downloader;

use {
    agave_snapshots::{
        paths::get_highest_full_snapshot_archive_info,
        snapshot_config::{SnapshotConfig, SnapshotUsage},
        unpack_genesis_archive,
    },
    async_trait::async_trait,
    carbon_core::{
        datasource::{AccountUpdate, Datasource, DatasourceId, Update, UpdateType},
        error::{CarbonResult, Error},
        metrics::{Counter, Histogram, MetricsRegistry},
    },
    downloader::download_and_setup_ledger,
    solana_account::ReadableAccount,
    solana_accounts_db::{
        accounts_db::AccountsDbConfig, is_loadable::IsLoadable,
        utils::create_all_accounts_run_and_snapshot_dirs,
    },
    solana_genesis_utils::open_genesis_config,
    solana_ledger::{bank_forks_utils, blockstore_processor::ProcessOptions},
    solana_pubkey::{Pubkey as SolanaPubkey, Pubkey},
    solana_runtime::bank::Bank,
    std::{
        collections::HashSet,
        path::PathBuf,
        sync::{atomic::AtomicBool, Arc, LazyLock},
    },
    tempfile::TempDir,
    tokio::sync::mpsc::Sender,
    tokio_util::sync::CancellationToken,
};

const MAX_GENESIS_ARCHIVE_UNPACKED_SIZE: u64 = 10_485_760; // 10MB
const WORK_DIRECTORY: &str = "carbon-snapshot";

static LOAD_DURATION_MILLIS: LazyLock<Histogram> = LazyLock::new(|| {
    Histogram::new(
        "validator_snapshot_load_duration_milliseconds",
        "Time to load snapshot (from path or remote) in milliseconds",
        vec![
            1.0, 5.0, 10.0, 25.0, 50.0, 100.0, 250.0, 500.0, 1000.0, 5000.0, 10_000.0,
        ],
    )
});
static SCAN_DURATION_MILLIS: LazyLock<Histogram> = LazyLock::new(|| {
    Histogram::new(
        "validator_snapshot_scan_duration_milliseconds",
        "Time to scan accounts after snapshot load in milliseconds",
        vec![
            1.0, 5.0, 10.0, 25.0, 50.0, 100.0, 250.0, 500.0, 1000.0, 5000.0, 10_000.0,
        ],
    )
});
static ACCOUNTS_PROCESSED: Counter = Counter::new(
    "validator_snapshot_accounts_processed_total",
    "Account updates processed (sent) by validator-snapshot datasource",
);

fn register_validator_snapshot_metrics() {
    let registry = MetricsRegistry::global();
    registry.register_counter(&ACCOUNTS_PROCESSED);
    registry.register_histogram(&LOAD_DURATION_MILLIS);
    registry.register_histogram(&SCAN_DURATION_MILLIS);
}

#[cfg(unix)]
fn raise_open_file_limit() {
    const AGAVE_OPEN_FILE_LIMIT: libc::rlim_t = 1_000_000;

    let mut limit = libc::rlimit {
        rlim_cur: 0,
        rlim_max: 0,
    };
    if unsafe { libc::getrlimit(libc::RLIMIT_NOFILE, &mut limit) } != 0 {
        log::warn!(
            "Failed to read the open file limit: {}",
            std::io::Error::last_os_error()
        );
        return;
    }

    let wanted = limit.rlim_max.min(AGAVE_OPEN_FILE_LIMIT);
    if limit.rlim_cur < wanted {
        let raised = libc::rlimit {
            rlim_cur: wanted,
            rlim_max: limit.rlim_max,
        };
        if unsafe { libc::setrlimit(libc::RLIMIT_NOFILE, &raised) } == 0 {
            limit = raised;
        } else {
            log::warn!(
                "Failed to raise the open file limit from {} to {wanted}: {}",
                limit.rlim_cur,
                std::io::Error::last_os_error()
            );
        }
    }

    if limit.rlim_cur < AGAVE_OPEN_FILE_LIMIT {
        log::warn!(
            "The open file limit is {}, below the 1,000,000 Agave asks for. The loader keeps a file open for each account storage, so a large snapshot can fail with `Too many open files`; raise the hard limit",
            limit.rlim_cur
        );
    }
}

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
    _work_directory: TempDir,
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

        let snapshots_directory = ledger_directory.join("snapshots");
        if get_highest_full_snapshot_archive_info(&snapshots_directory).is_none() {
            return Err(Error::FailedToConsumeDatasource(format!(
                "No loadable full snapshot archive found in {snapshots_directory:?}"
            )));
        }

        let leftover = ledger_directory.join(WORK_DIRECTORY);
        if leftover.exists() {
            std::fs::remove_dir_all(&leftover).map_err(|err| {
                Error::FailedToConsumeDatasource(format!(
                    "Failed to remove {leftover:?}, left by an earlier run: {err}"
                ))
            })?;
        }

        let work_directory = tempfile::Builder::new()
            .prefix(WORK_DIRECTORY)
            .rand_bytes(0)
            .tempdir_in(&ledger_directory)
            .map_err(|err| {
                Error::FailedToConsumeDatasource(format!(
                    "Failed to create a working directory in {ledger_directory:?}: {err}"
                ))
            })?;

        unpack_genesis_archive(
            &genesis_archive,
            work_directory.path(),
            MAX_GENESIS_ARCHIVE_UNPACKED_SIZE,
        )
        .map_err(|err| {
            Error::FailedToConsumeDatasource(format!(
                "Unable to load genesis configuration from {ledger_directory:?}: {err}"
            ))
        })?;
        let genesis_config =
            open_genesis_config(work_directory.path(), MAX_GENESIS_ARCHIVE_UNPACKED_SIZE).map_err(
                |err| {
                    Error::FailedToConsumeDatasource(format!(
                        "Unable to load genesis configuration from {ledger_directory:?}: {err}"
                    ))
                },
            )?;

        let accounts_directory = work_directory.path().join("accounts");
        std::fs::create_dir(&accounts_directory).map_err(|err| {
            Error::FailedToConsumeDatasource(format!(
                "Failed to initialize account directory structure: {err}"
            ))
        })?;

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

        let accounts_db_settings = AccountsDbConfig {
            skip_initial_hash_calc: true,
            ..AccountsDbConfig::default()
        };

        let snapshot_settings = SnapshotConfig {
            usage: SnapshotUsage::LoadOnly,
            full_snapshot_archives_dir: snapshots_directory.clone(),
            incremental_snapshot_archives_dir: snapshots_directory.clone(),
            bank_snapshots_dir: work_directory.path().join("snapshots"),
            ..SnapshotConfig::default()
        };

        let processing_settings = ProcessOptions {
            accounts_db_skip_shrink: true,
            accounts_db_config: accounts_db_settings,
            ..Default::default()
        };

        #[cfg(unix)]
        raise_open_file_limit();

        let (bank_forks, _snapshot_hashes) = bank_forks_utils::try_load_bank_forks_from_snapshot(
            &genesis_config,
            &account_run_directories,
            &snapshot_settings,
            &processing_settings,
            None,
            Arc::new(AtomicBool::new(false)),
        )
        .map_err(|err| {
            Error::FailedToConsumeDatasource(format!(
                "Failed to load bank forks from snapshot: {err}"
            ))
        })?
        .ok_or_else(|| {
            Error::FailedToConsumeDatasource(format!(
                "No loadable full snapshot archive found in {snapshots_directory:?}"
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

        Ok(Self {
            bank,
            slot,
            _work_directory: work_directory,
        })
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
    ) -> CarbonResult<()> {
        register_validator_snapshot_metrics();

        let load_start = std::time::Instant::now();

        let snapshot = match &self.source {
            SnapshotSource::LocalPath(path) => SnapshotState::load_from_path(path.clone())?,
            SnapshotSource::Remote { url } => {
                match SnapshotState::load_from_remote(url, cancellation_token.clone()).await {
                    Ok(snapshot) => snapshot,
                    Err(_) if cancellation_token.is_cancelled() => return Ok(()),
                    Err(error) => return Err(error),
                }
            }
        };

        let load_duration = load_start.elapsed();
        LOAD_DURATION_MILLIS.record(load_duration.as_millis() as f64);

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

            let mut channel_closed = false;

            bank.scan_all_accounts(|option| {
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
                        bank_id: None,
                    };

                    let update = Update::Account(account_update);

                    if let Err(e) = sender_clone.blocking_send((update, id_clone.clone())) {
                        log::error!("Failed to send account update: {e:?}");
                        channel_closed = true;
                    } else {
                        ACCOUNTS_PROCESSED.inc();
                    }
                }
            })
            .map_err(|e| Error::FailedToConsumeDatasource(format!("Scan failed: {e}")))?;

            log::info!("Account scan completed");

            Ok(())
        })
        .await
        .map_err(|e| Error::FailedToConsumeDatasource(format!("Scan task panicked: {e}")))??;

        let scan_duration = scan_start.elapsed();
        SCAN_DURATION_MILLIS.record(scan_duration.as_millis() as f64);

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
