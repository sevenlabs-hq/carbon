//! Helpers for resolving and loading slot index artifacts for the firehose replay engine.
//!
//! ## Index formats
//!
//! Old Faithful publishes a compact per-epoch *slot ranges* index
//! (`{epoch}/epoch-{epoch}-slot-ranges.raw`, ~5 MB): one 12-byte little-endian record per slot
//! — `offset: u64` followed by `length: u32` — describing the byte range of the epoch CAR file
//! that contains every node (transactions, entries, rewards, block) belonging to that slot.
//! Records are ordered by slot, starting at the epoch's first slot; skipped slots are encoded
//! as an all-zero record. This is the primary index used for slot lookups.
//!
//! When the slot-ranges file is missing (e.g. an older mirror), the loader falls back to the
//! legacy compactindex pair (`slot-to-cid` + `cid-to-offset-and-size`). Those indexes are
//! deprecated upstream and will eventually stop being generated. The fallback derives the same
//! byte-range semantics as the slot-ranges index, so lookups behave identically on both paths.
//!
//! ## Environment variables
//!
//! - `JETSTREAMER_COMPACT_INDEX_BASE_URL`: Preferred base URL for index files.
//! - `JETSTREAMER_NETWORK`: Network suffix appended to on-disk cache keys and legacy remote
//!   filenames.
//! - `JETSTREAMER_FORCE_LEGACY_INDEX`: Set to `1`/`true` to skip the slot-ranges index and use
//!   the legacy compactindex pair directly (debugging escape hatch).
//!
//! ### Example
//!
//! ```no_run
//! use jetstreamer_firehose::index::{get_index_base_url, SlotOffsetIndex};
//!
//! # fn main() -> Result<(), Box<dyn std::error::Error>> {
//! unsafe {
//!     std::env::set_var("JETSTREAMER_NETWORK", "testnet");
//!     std::env::set_var(
//!         "JETSTREAMER_COMPACT_INDEX_BASE_URL",
//!         "https://mirror.example.com/indexes",
//!     );
//! }
//!
//! let base = get_index_base_url()?;
//! let index = SlotOffsetIndex::new(base)?;
//! # let _ = index.base_url();
//! # Ok(())
//! # }
//! ```
#[cfg(feature = "s3-backend")]
use crate::jetstreamer_firehose::archive::S3Location;
use crate::jetstreamer_firehose::{
    LOG_MODULE, archive,
    epochs::{epoch_to_slot_range, slot_to_epoch},
};
use cid::{Cid, multibase::Base};
use dashmap::{DashMap, mapref::entry::Entry};
use futures_util::StreamExt;
use log::{info, warn};
use once_cell::sync::{Lazy, OnceCell as SyncOnceCell};
use reqwest::{Client, StatusCode, Url, header::RANGE};
use serde_cbor::Value;
use std::{
    collections::HashMap,
    ops::RangeInclusive,
    sync::{
        Arc,
        atomic::{AtomicU64, Ordering},
    },
    time::{Duration, Instant},
};
use thiserror::Error;
use tokio::{sync::OnceCell, task::yield_now, time::sleep};
use xxhash_rust::xxh64::xxh64;

const COMPACT_INDEX_MAGIC: &[u8; 8] = b"compiszd";
const SLOT_RANGES_RECORD_SIZE: usize = 12;
const BUCKET_HEADER_SIZE: usize = 16;
const HASH_PREFIX_SIZE: usize = 32;
const SLOT_TO_CID_KIND: &[u8] = b"slot-to-cid";
const CID_TO_OFFSET_KIND: &[u8] = b"cid-to-offset-and-size";
const METADATA_KEY_KIND: &[u8] = b"index_kind";
const METADATA_KEY_EPOCH: &[u8] = b"epoch";
const HTTP_PREFETCH_BYTES: u64 = 4 * 1024; // initial bytes to fetch for headers
const FETCH_RANGE_MAX_RETRIES: usize = 10;
const FETCH_RANGE_BASE_DELAY_MS: u64 = 10_000;
const MIN_HIT_SPACING_MS: u64 = 20;

/// Errors returned while accessing the compact slot offset index.
#[derive(Debug, Error)]
pub enum SlotOffsetIndexError {
    /// Environment provided an invalid base URL.
    #[error("invalid index base URL: {0}")]
    InvalidBaseUrl(String),
    /// Constructed index URL is invalid.
    #[error("invalid index URL: {0}")]
    InvalidIndexUrl(String),
    /// Epoch index file was not found at the expected location.
    #[error("epoch index file not found: {0}")]
    EpochIndexFileNotFound(Url),
    /// Slot was not present in the index.
    #[error("slot {0} not found in index {1}")]
    SlotNotFound(u64, Url),
    /// The index reports no present slot for an entire epoch, which cannot happen with valid
    /// index data; the cached epoch index should be invalidated and refetched.
    #[error("no slot in epoch {0} is present in the index")]
    EpochHasNoIndexedSlots(u64),
    /// Request failed while fetching the index.
    #[error("network error while fetching {0}: {1}")]
    NetworkError(Url, #[source] reqwest::Error),
    /// Unexpected HTTP status returned while fetching index data.
    #[error("unexpected HTTP status {1} when fetching {0}")]
    HttpStatusError(Url, StatusCode),
    /// Index payload was malformed.
    #[error("invalid index format at {0}: {1}")]
    IndexFormatError(Url, String),
    /// CAR header could not be read or decoded.
    #[error("failed to read CAR header {0}: {1}")]
    CarHeaderError(Url, String),
    /// Error returned by the S3 backend.
    #[cfg(feature = "s3-backend")]
    #[error("S3 error while fetching {0}: {1}")]
    S3Error(Url, String),
}

enum IndexBackend {
    Http,
    #[cfg(feature = "s3-backend")]
    S3(Arc<S3Location>),
}

struct RemoteObject {
    kind: RemoteObjectKind,
    url: Url,
}

enum RemoteObjectKind {
    Http {
        client: Client,
    },
    #[cfg(feature = "s3-backend")]
    S3 {
        location: Arc<S3Location>,
        key: String,
    },
}

impl RemoteObject {
    fn http(client: Client, url: Url) -> Self {
        Self {
            kind: RemoteObjectKind::Http { client },
            url,
        }
    }

    #[cfg(feature = "s3-backend")]
    fn s3(location: Arc<S3Location>, key: String, url: Url) -> Self {
        Self {
            kind: RemoteObjectKind::S3 { location, key },
            url,
        }
    }

    const fn url(&self) -> &Url {
        &self.url
    }

    async fn fetch_full(&self) -> Result<Vec<u8>, SlotOffsetIndexError> {
        match &self.kind {
            RemoteObjectKind::Http { client } => fetch_full(client, &self.url).await,
            #[cfg(feature = "s3-backend")]
            RemoteObjectKind::S3 { location, key } => {
                fetch_s3_full(location, key.as_str(), &self.url).await
            }
        }
    }

    async fn fetch_range(
        &self,
        start: u64,
        end: u64,
        exact: bool,
    ) -> Result<Vec<u8>, SlotOffsetIndexError> {
        match &self.kind {
            RemoteObjectKind::Http { client } => {
                fetch_http_range(client, &self.url, start, end, exact).await
            }
            #[cfg(feature = "s3-backend")]
            RemoteObjectKind::S3 { location, key } => {
                fetch_s3_range(location, key.as_str(), &self.url, start, end, exact).await
            }
        }
    }
}

/// Lazily constructed global [`SlotOffsetIndex`] that honors environment configuration.
pub static SLOT_OFFSET_INDEX: Lazy<SlotOffsetIndex> = Lazy::new(|| {
    let base_url =
        get_index_base_url().expect("JETSTREAMER_COMPACT_INDEX_BASE_URL must be a valid URL");
    SlotOffsetIndex::new(base_url).expect("failed to initialize slot offset index")
});

static START_INSTANT: Lazy<Instant> = Lazy::new(Instant::now);
static LAST_HIT_TIME: AtomicU64 = AtomicU64::new(0);
static SLOT_OFFSET_RESULT_CACHE: Lazy<DashMap<u64, (u64, u64), ahash::RandomState>> =
    Lazy::new(|| DashMap::with_hasher(ahash::RandomState::new()));
static EPOCH_CACHE: Lazy<DashMap<EpochCacheKey, Arc<EpochEntry>, ahash::RandomState>> =
    Lazy::new(|| DashMap::with_hasher(ahash::RandomState::new()));

#[derive(Clone, Hash, PartialEq, Eq)]
struct EpochCacheKey {
    namespace: Arc<str>,
    epoch: u64,
}

impl EpochCacheKey {
    fn new(namespace: &Arc<str>, epoch: u64) -> Self {
        Self {
            namespace: Arc::clone(namespace),
            epoch,
        }
    }
}

/// Looks up the byte offset at which `slot`'s data begins within its Old Faithful epoch CAR
/// archive. The offset points at the first CAR section belonging to the slot; reading forward
/// from it yields all of the slot's nodes, ending with the slot's Block node.
pub async fn slot_to_offset(slot: u64) -> Result<u64, SlotOffsetIndexError> {
    slot_to_range(slot).await.map(|(offset, _)| offset)
}

/// Returns the first *present* slot strictly after `after` and at most `up_to_inclusive`,
/// or `None` when none exists or the answer is unknown (legacy-index epochs). See
/// [`SlotOffsetIndex::next_present_slot`].
pub async fn next_present_slot(after: u64, up_to_inclusive: u64) -> Option<u64> {
    SLOT_OFFSET_INDEX
        .next_present_slot(after, up_to_inclusive)
        .await
}

/// Looks up the byte range `(offset, length)` covering all of `slot`'s data within its Old
/// Faithful epoch CAR archive. Returns [`SlotOffsetIndexError::SlotNotFound`] for slots that
/// were skipped on-chain.
pub async fn slot_to_range(slot: u64) -> Result<(u64, u64), SlotOffsetIndexError> {
    if let Some(range) = SLOT_OFFSET_RESULT_CACHE.get(&slot) {
        return Ok(*range);
    }

    let range = SLOT_OFFSET_INDEX.get_range(slot).await?;
    SLOT_OFFSET_RESULT_CACHE.insert(slot, range);
    Ok(range)
}

/// Client that resolves slot byte ranges using per-epoch index files from Old Faithful.
pub struct SlotOffsetIndex {
    client: Client,
    base_url: Url,
    network: String,
    cache_namespace: Arc<str>,
    backend: IndexBackend,
    force_legacy: bool,
}

struct EpochEntry {
    indexes: OnceCell<Arc<EpochIndexes>>,
}

impl EpochEntry {
    fn new() -> Self {
        Self {
            indexes: OnceCell::new(),
        }
    }
}

impl SlotOffsetIndex {
    /// Constructs a new [`SlotOffsetIndex`] rooted at the provided base URL.
    ///
    /// The [`SlotOffsetIndex`] uses the `JETSTREAMER_NETWORK` variable to scope cache keys and
    /// remote filenames. When unset, the network defaults to `mainnet`.
    pub fn new(base_url: Url) -> Result<Self, SlotOffsetIndexError> {
        let network =
            std::env::var("JETSTREAMER_NETWORK").unwrap_or_else(|_| "mainnet".to_string());
        let cache_namespace =
            Arc::<str>::from(format!("{}|{}", base_url.as_str(), network.as_str()));
        let location = archive::index_location();
        let backend = if location.is_http() {
            IndexBackend::Http
        } else {
            #[cfg(feature = "s3-backend")]
            {
                let cfg = location
                    .as_s3()
                    .expect("S3 backend requested but not configured");
                IndexBackend::S3(cfg)
            }
            #[cfg(not(feature = "s3-backend"))]
            {
                return Err(SlotOffsetIndexError::InvalidBaseUrl(
                    "compiled without s3-backend support".into(),
                ));
            }
        };
        let force_legacy = std::env::var("JETSTREAMER_FORCE_LEGACY_INDEX")
            .map(|value| {
                let value = value.trim();
                value.eq_ignore_ascii_case("1")
                    || value.eq_ignore_ascii_case("true")
                    || value.eq_ignore_ascii_case("yes")
            })
            .unwrap_or(false);
        Ok(Self {
            client: crate::jetstreamer_firehose::network::create_http_client(),
            base_url,
            network,
            cache_namespace,
            backend,
            force_legacy,
        })
    }

    /// Returns the base URL that remote index files are fetched from.
    pub const fn base_url(&self) -> &Url {
        &self.base_url
    }

    async fn load_epoch_indexes(&self, epoch: u64) -> Result<EpochIndexes, SlotOffsetIndexError> {
        if !self.force_legacy {
            match self.load_slot_ranges_index(epoch).await {
                Ok(index) => return Ok(EpochIndexes::SlotRanges(index)),
                Err(SlotOffsetIndexError::EpochIndexFileNotFound(url)) => {
                    warn!(
                        target: LOG_MODULE,
                        "slot-ranges index not found at {url}; falling back to legacy compact \
                         indexes (deprecated upstream)"
                    );
                }
                Err(err) => return Err(err),
            }
        }
        self.load_legacy_epoch_indexes(epoch)
            .await
            .map(EpochIndexes::Legacy)
    }

    async fn load_slot_ranges_index(
        &self,
        epoch: u64,
    ) -> Result<SlotRangesIndex, SlotOffsetIndexError> {
        let path = format!("{0}/epoch-{0}-slot-ranges.raw", epoch);
        let object = self.remote_for_path(&path)?;
        SlotRangesIndex::open(object, epoch).await
    }

    async fn load_legacy_epoch_indexes(
        &self,
        epoch: u64,
    ) -> Result<LegacyEpochIndexes, SlotOffsetIndexError> {
        let car_path = format!("{epoch}/epoch-{epoch}.car");
        let car_object = self.remote_for_path(&car_path)?;
        let (root_cid, header_size, car_url) = fetch_epoch_header(&car_object).await?;
        let root_base32 = root_cid
            .to_string_of_base(Base::Base32Lower)
            .map_err(|err| {
                SlotOffsetIndexError::CarHeaderError(car_url.clone(), err.to_string())
            })?;

        let slot_index_path = format!(
            "{0}/epoch-{0}-{1}-{2}-slot-to-cid.index",
            epoch, root_base32, self.network
        );
        let cid_index_path = format!(
            "{0}/epoch-{0}-{1}-{2}-cid-to-offset-and-size.index",
            epoch, root_base32, self.network
        );

        let slot_object = self.remote_for_path(&slot_index_path)?;
        let cid_object = self.remote_for_path(&cid_index_path)?;

        let slot_index = SlotCidIndex::open(slot_object, SLOT_TO_CID_KIND, None).await?;

        let cid_index = CidOffsetIndex::open(
            cid_object,
            CID_TO_OFFSET_KIND,
            Some(CidOffsetIndex::OFFSET_AND_SIZE_VALUE_SIZE),
        )
        .await?;

        if let Some(meta_epoch) = slot_index.metadata_epoch()
            && meta_epoch != epoch
        {
            warn!(
                target: LOG_MODULE,
                "Slot index epoch metadata mismatch: expected {epoch}, got {meta_epoch}"
            );
        }
        if let Some(meta_epoch) = cid_index.metadata_epoch()
            && meta_epoch != epoch
        {
            warn!(
                target: LOG_MODULE,
                "CID index epoch metadata mismatch: expected {epoch}, got {meta_epoch}"
            );
        }

        let (slot_start, slot_end_inclusive) = epoch_to_slot_range(epoch);
        Ok(LegacyEpochIndexes::new(
            slot_index,
            cid_index,
            slot_start,
            slot_end_inclusive,
            header_size,
        ))
    }

    /// Resolves the byte offset at which `slot`'s data begins within its Old Faithful CAR
    /// archive.
    pub async fn get_offset(&self, slot: u64) -> Result<u64, SlotOffsetIndexError> {
        self.get_range(slot).await.map(|(offset, _)| offset)
    }

    /// Resolves the byte range `(offset, length)` covering all of `slot`'s data within its Old
    /// Faithful CAR archive.
    pub async fn get_range(&self, slot: u64) -> Result<(u64, u64), SlotOffsetIndexError> {
        if let Some(range) = SLOT_OFFSET_RESULT_CACHE.get(&slot) {
            return Ok(*range);
        }

        let epoch = slot_to_epoch(slot);
        let cache_key = EpochCacheKey::new(&self.cache_namespace, epoch);
        let entry_arc = EPOCH_CACHE
            .entry(cache_key.clone())
            .or_insert_with(|| Arc::new(EpochEntry::new()))
            .clone();

        let indexes = entry_arc
            .indexes
            .get_or_try_init(|| async { Ok(Arc::new(self.load_epoch_indexes(epoch).await?)) })
            .await?
            .clone();

        let range = indexes.lookup_range(slot).await?;
        SLOT_OFFSET_RESULT_CACHE.insert(slot, range);

        Ok(range)
    }

    /// Returns the first *present* slot strictly after `after` and at most `up_to_inclusive`,
    /// or `None` when no such slot exists **or the answer is unknown** (legacy compactindex
    /// epochs cannot answer this without per-slot network lookups). Used to distinguish a
    /// genuine end-of-epoch from a prematurely closed HTTP stream, and by the end-of-run
    /// coverage audit; both treat `None` as "nothing provably missing".
    pub async fn next_present_slot(&self, after: u64, up_to_inclusive: u64) -> Option<u64> {
        let mut probe = after.saturating_add(1);
        while probe <= up_to_inclusive {
            let epoch = slot_to_epoch(probe);
            let (_, epoch_end_inclusive) = epoch_to_slot_range(epoch);
            let cache_key = EpochCacheKey::new(&self.cache_namespace, epoch);
            let entry_arc = EPOCH_CACHE
                .entry(cache_key.clone())
                .or_insert_with(|| Arc::new(EpochEntry::new()))
                .clone();
            let indexes = entry_arc
                .indexes
                .get_or_try_init(|| async {
                    Ok::<_, SlotOffsetIndexError>(Arc::new(self.load_epoch_indexes(epoch).await?))
                })
                .await
                .ok()?
                .clone();
            match indexes.as_ref() {
                EpochIndexes::SlotRanges(index) => {
                    let scan_end = up_to_inclusive.min(epoch_end_inclusive);
                    for slot in probe..=scan_end {
                        if index.lookup_range(slot).is_ok() {
                            return Some(slot);
                        }
                    }
                }
                // Legacy epochs cannot be scanned cheaply; report "unknown".
                EpochIndexes::Legacy(_) => return None,
            }
            probe = epoch_end_inclusive.saturating_add(1);
        }
        None
    }

    fn remote_for_path(&self, path: &str) -> Result<RemoteObject, SlotOffsetIndexError> {
        match &self.backend {
            IndexBackend::Http => {
                let url = self.base_url.join(path).map_err(|err| {
                    SlotOffsetIndexError::InvalidIndexUrl(format!("{path} ({err})"))
                })?;
                Ok(RemoteObject::http(self.client.clone(), url))
            }
            #[cfg(feature = "s3-backend")]
            IndexBackend::S3(location) => {
                let url = self.base_url.join(path).map_err(|err| {
                    SlotOffsetIndexError::InvalidIndexUrl(format!("{path} ({err})"))
                })?;
                let key = location.key_for(path);
                Ok(RemoteObject::s3(Arc::clone(location), key, url))
            }
        }
    }

    /// Clears cached index data for the given epoch, forcing a refetch on next lookup.
    pub fn invalidate_epoch(&self, epoch: u64) {
        let key = EpochCacheKey::new(&self.cache_namespace, epoch);
        EPOCH_CACHE.remove(&key);

        let (epoch_start, epoch_end_inclusive) = epoch_to_slot_range(epoch);
        SLOT_OFFSET_RESULT_CACHE
            .retain(|slot, _| *slot < epoch_start || *slot > epoch_end_inclusive);
    }
}

enum EpochIndexes {
    SlotRanges(SlotRangesIndex),
    Legacy(LegacyEpochIndexes),
}

impl EpochIndexes {
    async fn lookup_range(&self, slot: u64) -> Result<(u64, u64), SlotOffsetIndexError> {
        match self {
            EpochIndexes::SlotRanges(index) => index.lookup_range(slot),
            EpochIndexes::Legacy(indexes) => indexes.lookup_range(slot).await,
        }
    }
}

/// In-memory copy of an Old Faithful slot-ranges index file: one 12-byte little-endian
/// `(offset: u64, length: u32)` record per slot of the epoch, ordered by slot. A record with
/// `length == 0` marks a slot that was skipped on-chain.
struct SlotRangesIndex {
    file: RemoteIndexFile,
    slot_range: RangeInclusive<u64>,
}

impl SlotRangesIndex {
    async fn open(object: RemoteObject, epoch: u64) -> Result<Self, SlotOffsetIndexError> {
        info!(
            target: LOG_MODULE,
            "Fetching slot-ranges index for epoch {epoch}"
        );
        let file = RemoteIndexFile::download(&object).await?;
        let (slot_start, slot_end_inclusive) = epoch_to_slot_range(epoch);
        Self::from_file(file, slot_start..=slot_end_inclusive)
    }

    fn from_file(
        file: RemoteIndexFile,
        slot_range: RangeInclusive<u64>,
    ) -> Result<Self, SlotOffsetIndexError> {
        let num_slots = slot_range.end() - slot_range.start() + 1;
        let expected_len = num_slots.saturating_mul(SLOT_RANGES_RECORD_SIZE as u64);
        let actual_len = file.bytes().len() as u64;
        if actual_len != expected_len {
            return Err(SlotOffsetIndexError::IndexFormatError(
                file.url().clone(),
                format!(
                    "slot-ranges index is {actual_len} bytes, expected {expected_len} \
                     ({num_slots} records of {SLOT_RANGES_RECORD_SIZE} bytes)"
                ),
            ));
        }
        Ok(Self { file, slot_range })
    }

    fn lookup_range(&self, slot: u64) -> Result<(u64, u64), SlotOffsetIndexError> {
        if !self.slot_range.contains(&slot) {
            return Err(SlotOffsetIndexError::IndexFormatError(
                self.file.url().clone(),
                format!("slot {slot} not in epoch slot range"),
            ));
        }
        let record_index = (slot - self.slot_range.start()) as usize;
        let record = self.file.slice(
            record_index * SLOT_RANGES_RECORD_SIZE,
            SLOT_RANGES_RECORD_SIZE,
        )?;
        let offset = u64::from_le_bytes(record[..8].try_into().unwrap());
        let length = u32::from_le_bytes(record[8..12].try_into().unwrap());
        if length == 0 {
            return Err(SlotOffsetIndexError::SlotNotFound(
                slot,
                self.file.url().clone(),
            ));
        }
        Ok((offset, length as u64))
    }
}

struct LegacyEpochIndexes {
    slot_index: Arc<SlotCidIndex>,
    cid_index: Arc<CidOffsetIndex>,
    slot_cid_cache: DashMap<u64, Arc<[u8]>, ahash::RandomState>,
    slot_range: RangeInclusive<u64>,
    /// Total size of the epoch CAR header (varint prefix plus header bytes); the offset of the
    /// first byte of slot data in the archive.
    data_start: u64,
}

impl LegacyEpochIndexes {
    fn new(
        slot_index: SlotCidIndex,
        cid_index: CidOffsetIndex,
        slot_start: u64,
        slot_end_inclusive: u64,
        data_start: u64,
    ) -> Self {
        Self {
            slot_index: Arc::new(slot_index),
            cid_index: Arc::new(cid_index),
            slot_cid_cache: DashMap::with_hasher(ahash::RandomState::new()),
            slot_range: slot_start..=slot_end_inclusive,
            data_start,
        }
    }

    fn slot_cid(&self, slot: u64) -> Result<Arc<[u8]>, SlotOffsetIndexError> {
        if let Some(entry) = self.slot_cid_cache.get(&slot) {
            return Ok(Arc::clone(entry.value()));
        }
        let bytes = self.slot_index.lookup(slot)?;
        let cid = Arc::<[u8]>::from(bytes);
        self.slot_cid_cache.insert(slot, Arc::clone(&cid));
        Ok(cid)
    }

    async fn block_section(
        &self,
        cid: &[u8],
        slot: u64,
    ) -> Result<(u64, u32), SlotOffsetIndexError> {
        match self.cid_index.lookup(cid).await? {
            Some(bytes) => decode_offset_and_size(&bytes, self.cid_index.url()),
            None => Err(SlotOffsetIndexError::SlotNotFound(
                slot,
                self.cid_index.url().clone(),
            )),
        }
    }

    /// Derives the same `(offset, length)` byte range that the slot-ranges index encodes.
    ///
    /// The legacy `cid-to-offset-and-size` index locates a slot's Block node section, which is
    /// the *last* section of the slot's data in the CAR. The start of the slot's data is
    /// therefore the end of the previous present slot's Block section, or the end of the CAR
    /// header for the first present slot of the epoch.
    async fn lookup_range(&self, slot: u64) -> Result<(u64, u64), SlotOffsetIndexError> {
        if !self.slot_range.contains(&slot) {
            return Err(SlotOffsetIndexError::IndexFormatError(
                self.slot_index.url().clone(),
                format!("slot {slot} not in epoch slot range"),
            ));
        }

        let cid = self.slot_cid(slot)?;
        let (block_offset, block_size) = self.block_section(cid.as_ref(), slot).await?;
        let end = block_offset + block_size as u64;

        let mut candidate = slot;
        let start = loop {
            if candidate == *self.slot_range.start() {
                break self.data_start;
            }
            candidate -= 1;
            match self.slot_cid(candidate) {
                Ok(prev_cid) => {
                    // The legacy compactindexes store truncated entry hashes, so lookups of
                    // absent keys (skipped slots) can false-positive and return an unrelated
                    // entry. Treat results that cannot precede `slot`'s data as absent and keep
                    // scanning instead of deriving an impossible range.
                    match self.block_section(prev_cid.as_ref(), candidate).await {
                        Ok((prev_offset, prev_size)) => {
                            let candidate_end = prev_offset + prev_size as u64;
                            if candidate_end > end {
                                warn!(
                                    target: LOG_MODULE,
                                    "ignoring implausible legacy index entry for slot \
                                     {candidate} (section end {candidate_end} exceeds slot \
                                     {slot} end {end}); treating it as skipped"
                                );
                                continue;
                            }
                            break candidate_end;
                        }
                        Err(SlotOffsetIndexError::SlotNotFound(..)) => continue,
                        Err(err) => return Err(err),
                    }
                }
                Err(SlotOffsetIndexError::SlotNotFound(..)) => continue,
                Err(err) => return Err(err),
            }
        };

        let length = end.checked_sub(start).ok_or_else(|| {
            SlotOffsetIndexError::IndexFormatError(
                self.cid_index.url().clone(),
                format!("slot {slot} range end {end} precedes start {start}"),
            )
        })?;
        Ok((start, length))
    }
}

fn decode_offset_and_size(value: &[u8], url: &Url) -> Result<(u64, u32), SlotOffsetIndexError> {
    if value.len() != CidOffsetIndex::OFFSET_AND_SIZE_VALUE_SIZE as usize {
        return Err(SlotOffsetIndexError::IndexFormatError(
            url.clone(),
            format!(
                "offset-and-size entry expected {} bytes, got {}",
                CidOffsetIndex::OFFSET_AND_SIZE_VALUE_SIZE,
                value.len()
            ),
        ));
    }
    let mut offset_bytes = [0u8; 8];
    offset_bytes[..6].copy_from_slice(&value[..6]);
    let offset = u64::from_le_bytes(offset_bytes);

    let mut size_bytes = [0u8; 4];
    size_bytes[..3].copy_from_slice(&value[6..9]);
    let size = u32::from_le_bytes(size_bytes);
    Ok((offset, size))
}

struct RemoteIndexFile {
    url: Url,
    data: Arc<[u8]>,
}

impl RemoteIndexFile {
    async fn download(object: &RemoteObject) -> Result<Self, SlotOffsetIndexError> {
        let url = object.url().clone();
        let bytes = object.fetch_full().await?;
        Ok(Self {
            url,
            data: Arc::from(bytes),
        })
    }

    const fn url(&self) -> &Url {
        &self.url
    }

    fn slice(&self, offset: usize, len: usize) -> Result<&[u8], SlotOffsetIndexError> {
        let end = offset.checked_add(len).ok_or_else(|| {
            SlotOffsetIndexError::IndexFormatError(
                self.url.clone(),
                "slice range overflowed usize".into(),
            )
        })?;
        if end > self.data.len() {
            return Err(SlotOffsetIndexError::IndexFormatError(
                self.url.clone(),
                format!("slice {offset}-{end} exceeds file size {}", self.data.len()),
            ));
        }
        Ok(&self.data[offset..end])
    }

    fn slice_u64(&self, offset: u64, len: usize) -> Result<&[u8], SlotOffsetIndexError> {
        let start = usize::try_from(offset).map_err(|_| {
            SlotOffsetIndexError::IndexFormatError(
                self.url.clone(),
                format!("offset {offset} exceeds usize bounds"),
            )
        })?;
        self.slice(start, len)
    }

    fn bytes(&self) -> &[u8] {
        &self.data
    }
}

struct SlotCidIndex {
    file: Arc<RemoteIndexFile>,
    header: CompactIndexHeader,
    bucket_entries: DashMap<u32, Arc<SlotBucketEntry>, ahash::RandomState>,
}

impl SlotCidIndex {
    async fn open(
        object: RemoteObject,
        expected_kind: &[u8],
        expected_value_size: Option<u64>,
    ) -> Result<Self, SlotOffsetIndexError> {
        let kind_label = std::str::from_utf8(expected_kind).unwrap_or("index");
        let url = object.url().clone();
        let epoch_hint = url
            .path_segments()
            .and_then(|mut segments| segments.next_back())
            .and_then(|name| name.split('-').nth(1))
            .and_then(|value| value.parse::<u64>().ok());
        if let Some(epoch) = epoch_hint {
            info!(
                target: LOG_MODULE,
                "Fetching {kind_label} compact index for epoch {epoch}"
            );
        } else {
            info!(
                target: LOG_MODULE,
                "Fetching {kind_label} compact index"
            );
        }

        let file = Arc::new(RemoteIndexFile::download(&object).await?);
        let header =
            parse_compact_index_header(file.bytes(), &url, expected_kind, expected_value_size)?;
        Ok(Self {
            file,
            header,
            bucket_entries: DashMap::with_hasher(ahash::RandomState::new()),
        })
    }

    fn lookup(&self, slot: u64) -> Result<Vec<u8>, SlotOffsetIndexError> {
        let key = slot.to_le_bytes();
        match self.lookup_raw(&key)? {
            Some(bytes) => Ok(bytes),
            None => Err(SlotOffsetIndexError::SlotNotFound(slot, self.url().clone())),
        }
    }

    fn lookup_raw(&self, key: &[u8]) -> Result<Option<Vec<u8>>, SlotOffsetIndexError> {
        let bucket_index = self.bucket_hash(key);
        let bucket = self.get_bucket(bucket_index)?;
        bucket.lookup(key)
    }

    fn metadata_epoch(&self) -> Option<u64> {
        self.header.metadata_epoch()
    }

    fn url(&self) -> &Url {
        self.file.url()
    }

    fn bucket_hash(&self, key: &[u8]) -> u32 {
        let h = xxh64(key, 0);
        let n = self.header.num_buckets as u64;
        let mut u = h % n;
        if ((h - u) / n) < u {
            u = hash_uint64(u);
        }
        (u % n) as u32
    }

    fn get_bucket(&self, index: u32) -> Result<Arc<SlotBucketData>, SlotOffsetIndexError> {
        let entry = match self.bucket_entries.entry(index) {
            Entry::Occupied(occupied) => Arc::clone(occupied.get()),
            Entry::Vacant(vacant) => {
                let new_entry = Arc::new(SlotBucketEntry::new());
                vacant.insert(Arc::clone(&new_entry));
                new_entry
            }
        };
        entry.get_or_load(|| self.load_bucket(index))
    }

    fn load_bucket(&self, index: u32) -> Result<Arc<SlotBucketData>, SlotOffsetIndexError> {
        let bucket_header_offset =
            self.header.header_size + (index as u64) * BUCKET_HEADER_SIZE as u64;
        let header_slice = self
            .file
            .slice_u64(bucket_header_offset, BUCKET_HEADER_SIZE)?;
        let mut header_bytes = [0u8; BUCKET_HEADER_SIZE];
        header_bytes.copy_from_slice(header_slice);
        let bucket_header = BucketHeader::from_bytes(header_bytes);
        let stride = bucket_header.hash_len as usize + self.header.value_size as usize;
        let data_len = stride * bucket_header.num_entries as usize;
        let data_start = usize::try_from(bucket_header.file_offset).map_err(|_| {
            SlotOffsetIndexError::IndexFormatError(
                self.url().clone(),
                format!(
                    "bucket data offset {} exceeds usize bounds",
                    bucket_header.file_offset
                ),
            )
        })?;
        // Ensure the bucket contents reside inside the downloaded file.
        let _ = self.file.slice(data_start, data_len)?;

        Ok(Arc::new(SlotBucketData::new(
            bucket_header,
            Arc::clone(&self.file),
            data_start,
            stride,
            self.header.value_size as usize,
        )))
    }
}

struct CidOffsetIndex {
    object: RemoteObject,
    header: CompactIndexHeader,
    bucket_entries: DashMap<u32, Arc<RemoteBucketEntry>, ahash::RandomState>,
}

impl CidOffsetIndex {
    const OFFSET_AND_SIZE_VALUE_SIZE: u64 = 9;

    async fn open(
        object: RemoteObject,
        expected_kind: &[u8],
        expected_value_size: Option<u64>,
    ) -> Result<Self, SlotOffsetIndexError> {
        let kind_label = std::str::from_utf8(expected_kind).unwrap_or("index");
        let url = object.url().clone();
        let epoch_hint = url
            .path_segments()
            .and_then(|mut segments| segments.next_back())
            .and_then(|name| name.split('-').nth(1))
            .and_then(|value| value.parse::<u64>().ok());
        if let Some(epoch) = epoch_hint {
            info!(
                target: LOG_MODULE,
                "Fetching {kind_label} compact index for epoch {epoch}"
            );
        } else {
            info!(
                target: LOG_MODULE,
                "Fetching {kind_label} compact index"
            );
        }

        let header = fetch_and_parse_header(&object, expected_kind, expected_value_size).await?;
        Ok(Self {
            object,
            header,
            bucket_entries: DashMap::with_hasher(ahash::RandomState::new()),
        })
    }

    async fn lookup(&self, key: &[u8]) -> Result<Option<Vec<u8>>, SlotOffsetIndexError> {
        let bucket_index = self.bucket_hash(key);
        let bucket = self.get_bucket(bucket_index).await?;
        bucket.lookup(key)
    }

    fn metadata_epoch(&self) -> Option<u64> {
        self.header.metadata_epoch()
    }

    fn url(&self) -> &Url {
        self.object.url()
    }

    fn bucket_hash(&self, key: &[u8]) -> u32 {
        let h = xxh64(key, 0);
        let n = self.header.num_buckets as u64;
        let mut u = h % n;
        if ((h - u) / n) < u {
            u = hash_uint64(u);
        }
        (u % n) as u32
    }

    async fn get_bucket(&self, index: u32) -> Result<Arc<RemoteBucketData>, SlotOffsetIndexError> {
        let entry = match self.bucket_entries.entry(index) {
            Entry::Occupied(occupied) => Arc::clone(occupied.get()),
            Entry::Vacant(vacant) => {
                let new_entry = Arc::new(RemoteBucketEntry::new());
                vacant.insert(Arc::clone(&new_entry));
                new_entry
            }
        };
        entry
            .get_or_load(|| async { self.load_bucket(index).await })
            .await
    }

    async fn load_bucket(&self, index: u32) -> Result<Arc<RemoteBucketData>, SlotOffsetIndexError> {
        let bucket_header_offset =
            self.header.header_size + (index as u64) * BUCKET_HEADER_SIZE as u64;
        let header_bytes = self
            .object
            .fetch_range(
                bucket_header_offset,
                bucket_header_offset + BUCKET_HEADER_SIZE as u64 - 1,
                true,
            )
            .await?;
        if header_bytes.len() != BUCKET_HEADER_SIZE {
            return Err(SlotOffsetIndexError::IndexFormatError(
                self.object.url().clone(),
                format!(
                    "expected {BUCKET_HEADER_SIZE} bucket header bytes, got {}",
                    header_bytes.len()
                ),
            ));
        }
        let bucket_header = BucketHeader::from_bytes(header_bytes.try_into().unwrap());
        let stride = bucket_header.hash_len as usize + self.header.value_size as usize;
        if stride == 0 {
            if bucket_header.num_entries == 0 {
                return Ok(Arc::new(RemoteBucketData::new(
                    bucket_header,
                    Vec::new(),
                    stride,
                    self.header.value_size as usize,
                )));
            }
            return Err(SlotOffsetIndexError::IndexFormatError(
                self.object.url().clone(),
                "bucket stride is zero for non-empty bucket".to_string(),
            ));
        }
        let data_len = stride * bucket_header.num_entries as usize;
        if data_len == 0 {
            return Ok(Arc::new(RemoteBucketData::new(
                bucket_header,
                Vec::new(),
                stride,
                self.header.value_size as usize,
            )));
        }
        let data_start = bucket_header.file_offset;
        let data_end = data_start
            .checked_add(data_len as u64)
            .and_then(|end| end.checked_sub(1))
            .ok_or_else(|| {
                SlotOffsetIndexError::IndexFormatError(
                    self.object.url().clone(),
                    format!("bucket data range overflow (offset {data_start}, len {data_len})"),
                )
            })?;
        let data = self.object.fetch_range(data_start, data_end, true).await?;
        if data.len() != data_len {
            return Err(SlotOffsetIndexError::IndexFormatError(
                self.object.url().clone(),
                format!(
                    "expected {} bytes of bucket data, got {}",
                    data_len,
                    data.len()
                ),
            ));
        }

        Ok(Arc::new(RemoteBucketData::new(
            bucket_header,
            data,
            stride,
            self.header.value_size as usize,
        )))
    }
}

struct CompactIndexHeader {
    value_size: u64,
    num_buckets: u32,
    header_size: u64,
    metadata: HashMap<Vec<u8>, Vec<u8>>,
}

impl CompactIndexHeader {
    fn metadata_epoch(&self) -> Option<u64> {
        self.metadata
            .get(METADATA_KEY_EPOCH)
            .and_then(|bytes| bytes.get(..8))
            .map(|slice| {
                let mut buf = [0u8; 8];
                buf.copy_from_slice(slice);
                u64::from_le_bytes(buf)
            })
    }
}

struct SlotBucketData {
    header: BucketHeader,
    file: Arc<RemoteIndexFile>,
    data_offset: usize,
    stride: usize,
    value_width: usize,
}

impl SlotBucketData {
    const fn new(
        header: BucketHeader,
        file: Arc<RemoteIndexFile>,
        data_offset: usize,
        stride: usize,
        value_width: usize,
    ) -> Self {
        Self {
            header,
            file,
            data_offset,
            stride,
            value_width,
        }
    }

    fn lookup(&self, key: &[u8]) -> Result<Option<Vec<u8>>, SlotOffsetIndexError> {
        let target_hash = truncated_entry_hash(self.header.hash_domain, key, self.header.hash_len);
        let max = self.header.num_entries as usize;
        let hash_len = self.header.hash_len as usize;

        let mut index = 0usize;
        while index < max {
            let offset = index * self.stride;
            let hash_slice = self.file.slice(self.data_offset + offset, hash_len)?;
            let hash = read_hash(hash_slice);
            if hash == target_hash {
                let value_start = self.data_offset + offset + hash_len;
                let value_slice = self.file.slice(value_start, self.value_width)?;
                return Ok(Some(value_slice.to_vec()));
            }
            index = (index << 1) | 1;
            if hash < target_hash {
                index += 1;
            }
        }
        Ok(None)
    }
}

struct RemoteBucketData {
    header: BucketHeader,
    data: Vec<u8>,
    stride: usize,
    value_width: usize,
}

impl RemoteBucketData {
    const fn new(header: BucketHeader, data: Vec<u8>, stride: usize, value_width: usize) -> Self {
        Self {
            header,
            data,
            stride,
            value_width,
        }
    }

    fn lookup(&self, key: &[u8]) -> Result<Option<Vec<u8>>, SlotOffsetIndexError> {
        let target_hash = truncated_entry_hash(self.header.hash_domain, key, self.header.hash_len);
        let max = self.header.num_entries as usize;
        let hash_len = self.header.hash_len as usize;

        let mut index = 0usize;
        while index < max {
            let offset = index * self.stride;
            let hash = read_hash(&self.data[offset..offset + hash_len]);
            if hash == target_hash {
                let value_start = offset + hash_len;
                let value_end = value_start + self.value_width;
                return Ok(Some(self.data[value_start..value_end].to_vec()));
            }
            index = (index << 1) | 1;
            if hash < target_hash {
                index += 1;
            }
        }
        Ok(None)
    }
}

struct SlotBucketEntry {
    once: SyncOnceCell<Arc<SlotBucketData>>,
}

impl SlotBucketEntry {
    fn new() -> Self {
        Self {
            once: SyncOnceCell::new(),
        }
    }

    fn get_or_load<F>(&self, loader: F) -> Result<Arc<SlotBucketData>, SlotOffsetIndexError>
    where
        F: FnOnce() -> Result<Arc<SlotBucketData>, SlotOffsetIndexError>,
    {
        self.once.get_or_try_init(loader).map(Arc::clone)
    }
}

struct RemoteBucketEntry {
    once: OnceCell<Arc<RemoteBucketData>>,
}

impl RemoteBucketEntry {
    fn new() -> Self {
        Self {
            once: OnceCell::new(),
        }
    }

    async fn get_or_load<F, Fut>(
        &self,
        loader: F,
    ) -> Result<Arc<RemoteBucketData>, SlotOffsetIndexError>
    where
        F: FnOnce() -> Fut,
        Fut: std::future::Future<Output = Result<Arc<RemoteBucketData>, SlotOffsetIndexError>>,
    {
        self.once
            .get_or_try_init(|| async { loader().await })
            .await
            .map(Arc::clone)
    }
}

#[derive(Clone, Copy)]
struct BucketHeader {
    hash_domain: u32,
    num_entries: u32,
    hash_len: u8,
    file_offset: u64,
}

impl BucketHeader {
    fn from_bytes(bytes: [u8; BUCKET_HEADER_SIZE]) -> Self {
        let hash_domain = u32::from_le_bytes(bytes[0..4].try_into().unwrap());
        let num_entries = u32::from_le_bytes(bytes[4..8].try_into().unwrap());
        let hash_len = bytes[8];
        let mut offset_bytes = [0u8; 8];
        offset_bytes[..6].copy_from_slice(&bytes[10..16]);
        let file_offset = u64::from_le_bytes(offset_bytes);
        Self {
            hash_domain,
            num_entries,
            hash_len,
            file_offset,
        }
    }
}

fn read_hash(bytes: &[u8]) -> u64 {
    let mut buf = 0u64;
    for (i, b) in bytes.iter().enumerate() {
        buf |= (*b as u64) << (8 * i);
    }
    buf
}

fn truncated_entry_hash(hash_domain: u32, key: &[u8], hash_len: u8) -> u64 {
    let raw = entry_hash64(hash_domain, key);
    if hash_len >= 8 {
        raw
    } else {
        let bits = (hash_len as usize) * 8;
        let mask = if bits == 64 {
            u64::MAX
        } else {
            (1u64 << bits) - 1
        };
        raw & mask
    }
}

fn entry_hash64(prefix: u32, key: &[u8]) -> u64 {
    let mut block = [0u8; HASH_PREFIX_SIZE];
    block[..4].copy_from_slice(&prefix.to_le_bytes());
    let mut data = Vec::with_capacity(HASH_PREFIX_SIZE + key.len());
    data.extend_from_slice(&block);
    data.extend_from_slice(key);
    xxh64(&data, 0)
}

const fn hash_uint64(mut x: u64) -> u64 {
    x ^= x >> 33;
    x = x.wrapping_mul(0xff51afd7ed558ccd);
    x ^= x >> 33;
    x = x.wrapping_mul(0xc4ceb9fe1a85ec53);
    x ^= x >> 33;
    x
}

fn monotonic_millis() -> u64 {
    let elapsed = START_INSTANT.elapsed().as_millis();
    if elapsed > u64::MAX as u128 {
        u64::MAX
    } else {
        elapsed as u64
    }
}

pub(crate) async fn wait_for_remote_hit_slot() {
    loop {
        let now_ms = monotonic_millis();
        let last_hit = LAST_HIT_TIME.load(Ordering::Relaxed);
        if now_ms.saturating_sub(last_hit) < MIN_HIT_SPACING_MS {
            yield_now().await;
            continue;
        }
        if LAST_HIT_TIME
            .compare_exchange(last_hit, now_ms, Ordering::SeqCst, Ordering::SeqCst)
            .is_ok()
        {
            return;
        }
        yield_now().await;
    }
}

async fn fetch_and_parse_header(
    object: &RemoteObject,
    expected_kind: &[u8],
    expected_value_size: Option<u64>,
) -> Result<CompactIndexHeader, SlotOffsetIndexError> {
    let url = object.url().clone();
    let mut bytes = object
        .fetch_range(0, HTTP_PREFETCH_BYTES - 1, false)
        .await?;
    if bytes.len() < 12 {
        return Err(SlotOffsetIndexError::IndexFormatError(
            url.clone(),
            "index header shorter than 12 bytes".into(),
        ));
    }
    if bytes[..8] != COMPACT_INDEX_MAGIC[..] {
        return Err(SlotOffsetIndexError::IndexFormatError(
            url.clone(),
            "invalid compactindex magic".into(),
        ));
    }
    let header_len = u32::from_le_bytes(bytes[8..12].try_into().unwrap()) as usize;
    let total_header_size = 8 + 4 + header_len;
    if bytes.len() < total_header_size {
        bytes = object
            .fetch_range(0, total_header_size as u64 - 1, false)
            .await?;
        if bytes.len() < total_header_size {
            return Err(SlotOffsetIndexError::IndexFormatError(
                url.clone(),
                format!(
                    "incomplete index header: expected {total_header_size} bytes, got {}",
                    bytes.len()
                ),
            ));
        }
    }

    parse_compact_index_header(&bytes, &url, expected_kind, expected_value_size)
}

fn parse_compact_index_header(
    data: &[u8],
    url: &Url,
    expected_kind: &[u8],
    expected_value_size: Option<u64>,
) -> Result<CompactIndexHeader, SlotOffsetIndexError> {
    if data.len() < 12 {
        return Err(SlotOffsetIndexError::IndexFormatError(
            url.clone(),
            "index header shorter than 12 bytes".into(),
        ));
    }
    if data[..8] != COMPACT_INDEX_MAGIC[..] {
        return Err(SlotOffsetIndexError::IndexFormatError(
            url.clone(),
            "invalid compactindex magic".into(),
        ));
    }
    let header_len = u32::from_le_bytes(data[8..12].try_into().unwrap()) as usize;
    let total_header_size = 8 + 4 + header_len;
    if data.len() < total_header_size {
        return Err(SlotOffsetIndexError::IndexFormatError(
            url.clone(),
            format!(
                "incomplete index header: expected {total_header_size} bytes, got {}",
                data.len()
            ),
        ));
    }

    let value_size = u64::from_le_bytes(data[12..20].try_into().unwrap());
    let num_buckets = u32::from_le_bytes(data[20..24].try_into().unwrap());
    let version = data[24];
    if version != 1 {
        return Err(SlotOffsetIndexError::IndexFormatError(
            url.clone(),
            format!("unsupported compactindex version {version}"),
        ));
    }
    let metadata_slice = &data[25..total_header_size];
    let metadata = parse_metadata(metadata_slice).map_err(|msg| {
        SlotOffsetIndexError::IndexFormatError(url.clone(), format!("invalid metadata: {msg}"))
    })?;

    if let Some(expected) = expected_value_size
        && value_size != expected
    {
        return Err(SlotOffsetIndexError::IndexFormatError(
            url.clone(),
            format!("unexpected value size: expected {expected}, got {value_size}"),
        ));
    }
    if let Some(kind) = metadata.get(METADATA_KEY_KIND)
        && kind.as_slice() != expected_kind
    {
        return Err(SlotOffsetIndexError::IndexFormatError(
            url.clone(),
            format!(
                "wrong index kind: expected {:?}, got {:?}",
                expected_kind, kind
            ),
        ));
    }

    Ok(CompactIndexHeader {
        value_size,
        num_buckets,
        header_size: total_header_size as u64,
        metadata,
    })
}

/// Reads the epoch CAR header, returning its root CID, the total header size in bytes (varint
/// prefix plus header body — i.e. the offset of the first byte of slot data), and the CAR URL.
async fn fetch_epoch_header(
    object: &RemoteObject,
) -> Result<(Cid, u64, Url), SlotOffsetIndexError> {
    let car_url = object.url().clone();
    let mut bytes = object
        .fetch_range(0, HTTP_PREFETCH_BYTES - 1, false)
        .await?;
    let (header_len, prefix) = decode_varint(&bytes)
        .map_err(|msg| SlotOffsetIndexError::CarHeaderError(car_url.clone(), msg))?;
    let total_needed = prefix + header_len as usize;
    if bytes.len() < total_needed {
        bytes = object
            .fetch_range(0, total_needed as u64 - 1, false)
            .await?;
        if bytes.len() < total_needed {
            return Err(SlotOffsetIndexError::CarHeaderError(
                car_url.clone(),
                format!(
                    "incomplete CAR header: expected {total_needed} bytes, got {}",
                    bytes.len()
                ),
            ));
        }
    }
    let header_bytes = &bytes[prefix..total_needed];
    let value: Value = serde_cbor::from_slice(header_bytes).map_err(|err| {
        SlotOffsetIndexError::CarHeaderError(
            car_url.clone(),
            format!("failed to decode CBOR: {err}"),
        )
    })?;
    let root_cid = extract_root_cid(&value)
        .map_err(|msg| SlotOffsetIndexError::CarHeaderError(car_url.clone(), msg.to_string()))?;
    Ok((root_cid, total_needed as u64, car_url))
}

fn decode_varint(bytes: &[u8]) -> Result<(u64, usize), String> {
    let mut value = 0u64;
    let mut shift = 0u32;
    for (idx, b) in bytes.iter().enumerate() {
        let byte = *b as u64;
        if byte < 0x80 {
            value |= byte << shift;
            return Ok((value, idx + 1));
        }
        value |= (byte & 0x7f) << shift;
        shift += 7;
        if shift > 63 {
            return Err("varint overflow".into());
        }
    }
    Err("buffer ended before varint terminated".into())
}

fn extract_root_cid(value: &Value) -> Result<Cid, String> {
    let map_entries = match value {
        Value::Map(entries) => entries,
        _ => return Err("CAR header is not a map".into()),
    };
    let roots_value = map_entries
        .iter()
        .find(|(k, _)| matches!(k, Value::Text(s) if s == "roots"))
        .map(|(_, v)| v)
        .ok_or_else(|| "CAR header missing 'roots'".to_string())?;
    let roots = match roots_value {
        Value::Array(items) => items,
        _ => return Err("CAR header 'roots' not an array".into()),
    };
    let first = roots
        .first()
        .ok_or_else(|| "CAR header 'roots' array empty".to_string())?;
    match first {
        Value::Tag(42, boxed) => match boxed.as_ref() {
            Value::Bytes(bytes) => decode_cid_bytes(bytes),
            _ => Err("CID tag did not contain bytes".into()),
        },
        Value::Bytes(bytes) => decode_cid_bytes(bytes),
        _ => Err("unexpected CID encoding in CAR header".into()),
    }
}

fn decode_cid_bytes(bytes: &[u8]) -> Result<Cid, String> {
    if bytes.is_empty() {
        return Err("CID bytes were empty".into());
    }
    // CID-in-CBOR is prefixed with a multibase identity (0x00) byte.
    let mut candidates: Vec<&[u8]> = Vec::with_capacity(2);
    if bytes[0] == 0 && bytes.len() > 1 {
        candidates.push(&bytes[1..]);
    }
    candidates.push(bytes);
    let mut last_err = None;
    for slice in candidates {
        match Cid::try_from(slice.to_vec()) {
            Ok(cid) => return Ok(cid),
            Err(err) => last_err = Some(err),
        }
    }
    Err(last_err
        .map(|err| format!("invalid CID: {err}"))
        .unwrap_or_else(|| "invalid CID bytes".into()))
}

async fn fetch_full(client: &Client, url: &Url) -> Result<Vec<u8>, SlotOffsetIndexError> {
    let mut attempt = 0usize;
    loop {
        wait_for_remote_hit_slot().await;
        let response = match client.get(url.clone()).send().await {
            Ok(resp) => resp,
            Err(err) => {
                if attempt < FETCH_RANGE_MAX_RETRIES {
                    let delay_ms =
                        FETCH_RANGE_BASE_DELAY_MS.saturating_mul(1u64 << attempt.min(10));
                    warn!(
                        target: LOG_MODULE,
                        "Network error fetching {}: {}; retrying in {} ms (attempt {}/{})",
                        url,
                        err,
                        delay_ms,
                        attempt + 1,
                        FETCH_RANGE_MAX_RETRIES
                    );
                    sleep(Duration::from_millis(delay_ms)).await;
                    attempt += 1;
                    continue;
                }
                return Err(SlotOffsetIndexError::NetworkError(url.clone(), err));
            }
        };

        if response.status() == StatusCode::NOT_FOUND {
            return Err(SlotOffsetIndexError::EpochIndexFileNotFound(url.clone()));
        }

        if response.status() == StatusCode::TOO_MANY_REQUESTS
            || response.status() == StatusCode::SERVICE_UNAVAILABLE
        {
            if attempt < FETCH_RANGE_MAX_RETRIES {
                let delay_ms = FETCH_RANGE_BASE_DELAY_MS.saturating_mul(1u64 << attempt.min(10));
                warn!(
                    target: LOG_MODULE,
                    "HTTP {} fetching {}; retrying in {} ms (attempt {}/{})",
                    response.status(),
                    url,
                    delay_ms,
                    attempt + 1,
                    FETCH_RANGE_MAX_RETRIES
                );
                sleep(Duration::from_millis(delay_ms)).await;
                attempt += 1;
                continue;
            }
            return Err(SlotOffsetIndexError::HttpStatusError(
                url.clone(),
                response.status(),
            ));
        }

        if !response.status().is_success() {
            return Err(SlotOffsetIndexError::HttpStatusError(
                url.clone(),
                response.status(),
            ));
        }

        match read_response_with_progress(response, url).await {
            Ok(bytes) => return Ok(bytes),
            Err(err @ SlotOffsetIndexError::NetworkError(_, _)) => {
                if attempt < FETCH_RANGE_MAX_RETRIES {
                    let delay_ms =
                        FETCH_RANGE_BASE_DELAY_MS.saturating_mul(1u64 << attempt.min(10));
                    warn!(
                        target: LOG_MODULE,
                        "Error reading {} body: {}; retrying in {} ms (attempt {}/{})",
                        url,
                        err,
                        delay_ms,
                        attempt + 1,
                        FETCH_RANGE_MAX_RETRIES
                    );
                    sleep(Duration::from_millis(delay_ms)).await;
                    attempt += 1;
                    continue;
                }
                return Err(err);
            }
            Err(err) => return Err(err),
        }
    }
}

async fn read_response_with_progress(
    response: reqwest::Response,
    url: &Url,
) -> Result<Vec<u8>, SlotOffsetIndexError> {
    let total_len = response.content_length();
    let mut bytes = match total_len.and_then(|len| usize::try_from(len).ok()) {
        Some(len) => Vec::with_capacity(len),
        None => Vec::new(),
    };
    let mut stream = response.bytes_stream();
    while let Some(chunk_result) = stream.next().await {
        let chunk =
            chunk_result.map_err(|err| SlotOffsetIndexError::NetworkError(url.clone(), err))?;
        bytes.extend_from_slice(&chunk);
    }

    Ok(bytes)
}

async fn fetch_http_range(
    client: &Client,
    url: &Url,
    start: u64,
    end: u64,
    exact: bool,
) -> Result<Vec<u8>, SlotOffsetIndexError> {
    if end < start {
        return Ok(Vec::new());
    }
    let range_header = format!("bytes={start}-{end}");
    let mut attempt = 0usize;
    loop {
        wait_for_remote_hit_slot().await;
        let response = match client
            .get(url.clone())
            .header(RANGE, range_header.clone())
            .send()
            .await
        {
            Ok(resp) => resp,
            Err(err) => {
                if attempt < FETCH_RANGE_MAX_RETRIES {
                    let delay_ms =
                        FETCH_RANGE_BASE_DELAY_MS.saturating_mul(1u64 << attempt.min(10));
                    warn!(
                        target: LOG_MODULE,
                        "Network error fetching {} (range {}-{}): {}; retrying in {} ms (attempt {}/{})",
                        url,
                        start,
                        end,
                        err,
                        delay_ms,
                        attempt + 1,
                        FETCH_RANGE_MAX_RETRIES
                    );
                    sleep(Duration::from_millis(delay_ms)).await;
                    attempt += 1;
                    continue;
                }
                return Err(SlotOffsetIndexError::NetworkError(url.clone(), err));
            }
        };

        if response.status() == StatusCode::NOT_FOUND {
            return Err(SlotOffsetIndexError::EpochIndexFileNotFound(url.clone()));
        }

        if response.status() == StatusCode::TOO_MANY_REQUESTS
            || response.status() == StatusCode::SERVICE_UNAVAILABLE
        {
            if attempt < FETCH_RANGE_MAX_RETRIES {
                let delay_ms = FETCH_RANGE_BASE_DELAY_MS.saturating_mul(1u64 << attempt.min(10));
                warn!(
                    target: LOG_MODULE,
                    "HTTP {} fetching {} (range {}-{}); retrying in {} ms (attempt {}/{})",
                    response.status(),
                    url,
                    start,
                    end,
                    delay_ms,
                    attempt + 1,
                    FETCH_RANGE_MAX_RETRIES
                );
                sleep(Duration::from_millis(delay_ms)).await;
                attempt += 1;
                continue;
            }
            return Err(SlotOffsetIndexError::HttpStatusError(
                url.clone(),
                response.status(),
            ));
        }

        if !(response.status().is_success() || response.status() == StatusCode::PARTIAL_CONTENT) {
            return Err(SlotOffsetIndexError::HttpStatusError(
                url.clone(),
                response.status(),
            ));
        }

        let bytes = match response.bytes().await {
            Ok(bytes) => bytes,
            Err(err) => {
                if attempt < FETCH_RANGE_MAX_RETRIES {
                    let delay_ms =
                        FETCH_RANGE_BASE_DELAY_MS.saturating_mul(1u64 << attempt.min(10));
                    warn!(
                        target: LOG_MODULE,
                        "Error reading {} (range {}-{}) body: {}; retrying in {} ms (attempt {}/{})",
                        url,
                        start,
                        end,
                        err,
                        delay_ms,
                        attempt + 1,
                        FETCH_RANGE_MAX_RETRIES
                    );
                    sleep(Duration::from_millis(delay_ms)).await;
                    attempt += 1;
                    continue;
                }
                return Err(SlotOffsetIndexError::NetworkError(url.clone(), err));
            }
        };
        let expected = (end - start + 1) as usize;
        if exact && bytes.len() != expected {
            return Err(SlotOffsetIndexError::IndexFormatError(
                url.clone(),
                format!("expected {expected} bytes, got {}", bytes.len()),
            ));
        }
        return Ok(bytes.to_vec());
    }
}

#[cfg(feature = "s3-backend")]
async fn fetch_s3_full(
    location: &Arc<S3Location>,
    key: &str,
    url: &Url,
) -> Result<Vec<u8>, SlotOffsetIndexError> {
    let resp = location
        .client
        .get_object()
        .bucket(location.bucket.as_ref())
        .key(key)
        .send()
        .await
        .map_err(|err| {
            let missing = err.as_service_error().is_some_and(|service_err| {
                // GetObject on a missing key returns NoSuchKey, or AccessDenied when the
                // credentials lack s3:ListBucket. Treat both as "not found" so missing
                // slot-ranges files fall back to the legacy indexes; genuinely broken
                // credentials will fail the fallback fetch as well and surface there.
                service_err.is_no_such_key() || service_err.meta().code() == Some("AccessDenied")
            });
            if missing {
                SlotOffsetIndexError::EpochIndexFileNotFound(url.clone())
            } else {
                SlotOffsetIndexError::S3Error(url.clone(), err.to_string())
            }
        })?;
    let body = resp
        .body
        .collect()
        .await
        .map_err(|err| SlotOffsetIndexError::S3Error(url.clone(), err.to_string()))?;
    Ok(body.into_bytes().to_vec())
}

#[cfg(feature = "s3-backend")]
async fn fetch_s3_range(
    location: &Arc<S3Location>,
    key: &str,
    url: &Url,
    start: u64,
    end: u64,
    exact: bool,
) -> Result<Vec<u8>, SlotOffsetIndexError> {
    if end < start {
        return Ok(Vec::new());
    }
    let range = format!("bytes={start}-{end}");
    let resp = location
        .client
        .get_object()
        .bucket(location.bucket.as_ref())
        .key(key)
        .range(range)
        .send()
        .await
        .map_err(|err| SlotOffsetIndexError::S3Error(url.clone(), err.to_string()))?;
    let body = resp
        .body
        .collect()
        .await
        .map_err(|err| SlotOffsetIndexError::S3Error(url.clone(), err.to_string()))?;
    let data = body.into_bytes().to_vec();
    let expected = (end - start + 1) as usize;
    if exact && data.len() != expected {
        return Err(SlotOffsetIndexError::IndexFormatError(
            url.clone(),
            format!("expected {expected} bytes, got {}", data.len()),
        ));
    }
    Ok(data)
}

fn parse_metadata(data: &[u8]) -> Result<HashMap<Vec<u8>, Vec<u8>>, String> {
    if data.is_empty() {
        return Ok(HashMap::new());
    }
    let mut map = HashMap::new();
    let mut offset = 0;
    let num_pairs = data[offset] as usize;
    offset += 1;
    for _ in 0..num_pairs {
        if offset >= data.len() {
            return Err("unexpected end while reading metadata key length".into());
        }
        let key_len = data[offset] as usize;
        offset += 1;
        if offset + key_len > data.len() {
            return Err("metadata key length out of bounds".into());
        }
        let key = data[offset..offset + key_len].to_vec();
        offset += key_len;
        if offset >= data.len() {
            return Err("unexpected end while reading metadata value length".into());
        }
        let value_len = data[offset] as usize;
        offset += 1;
        if offset + value_len > data.len() {
            return Err("metadata value length out of bounds".into());
        }
        let value = data[offset..offset + value_len].to_vec();
        offset += value_len;
        map.insert(key, value);
    }
    Ok(map)
}

/// Resolves the base URL used when constructing the global [`SLOT_OFFSET_INDEX`].
///
/// The resolution order is:
/// 1. `JETSTREAMER_COMPACT_INDEX_BASE_URL`
/// 2. `JETSTREAMER_ARCHIVE_BASE`
/// 3. `JETSTREAMER_HTTP_BASE_URL` (falling back to the built-in public mirror)
///
/// All three knobs accept either `https://` URLs or `s3://bucket/prefix` URIs.
///
/// # Examples
///
/// ```no_run
/// # use jetstreamer_firehose::index::get_index_base_url;
/// unsafe {
///     std::env::set_var("JETSTREAMER_COMPACT_INDEX_BASE_URL", "https://mirror.example.com/indexes");
/// }
///
/// let base = get_index_base_url().expect("valid URL");
/// assert_eq!(base.as_str(), "https://mirror.example.com/indexes");
/// ```
pub fn get_index_base_url() -> Result<Url, SlotOffsetIndexError> {
    Ok(archive::index_location().url().clone())
}
