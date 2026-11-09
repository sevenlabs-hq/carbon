use reqwest::Client;
use ripget::{WindowedDownload, WindowedDownloadOptions, download_url_windowed};
use rseek::Seekable;
use serde::Deserialize;
use std::{fmt, io, pin::Pin};
use tokio::io::{AsyncRead, AsyncSeek, BufReader, ReadBuf, SeekFrom};

use crate::jetstreamer_firehose::SharedError;
use crate::jetstreamer_firehose::archive;
use crate::jetstreamer_firehose::node_reader::Len;

#[cfg(feature = "s3-backend")]
use {
    crate::jetstreamer_firehose::archive::S3Location,
    aws_sdk_s3::Client as S3Client,
    std::{future::Future, sync::Arc},
};

/// Default base URL used to fetch compact epoch CAR archives hosted by Old Faithful.
pub const BASE_URL: &str = "https://files.old-faithful.net";

#[inline(always)]
/// Returns the inclusive slot range covered by a Solana epoch.
///
/// The tuple contains the first slot and the final slot of the epoch.
pub const fn epoch_to_slot_range(epoch: u64) -> (u64, u64) {
    let first = epoch * 432000;
    (first, first + 431999)
}

#[inline(always)]
/// Converts a slot back into the epoch that contains it.
pub const fn slot_to_epoch(slot: u64) -> u64 {
    slot / 432000
}

trait EpochReader: AsyncRead + AsyncSeek + Send + Unpin {}
impl<T> EpochReader for T where T: AsyncRead + AsyncSeek + Send + Unpin {}
type DynEpochReader = dyn EpochReader;

/// Seekable reader returned by [`fetch_epoch_stream`] regardless of backend.
pub struct EpochStream {
    inner: Pin<Box<DynEpochReader>>,
    total_len: u64,
}

impl EpochStream {
    fn new<R>(reader: R) -> Self
    where
        R: AsyncRead + AsyncSeek + Len + Send + Unpin + 'static,
    {
        let len = reader.len();
        Self {
            inner: Box::pin(reader),
            total_len: len,
        }
    }
}

impl AsyncRead for EpochStream {
    fn poll_read(
        self: Pin<&mut Self>,
        cx: &mut std::task::Context<'_>,
        buf: &mut ReadBuf<'_>,
    ) -> std::task::Poll<io::Result<()>> {
        let this = unsafe { self.get_unchecked_mut() };
        this.inner.as_mut().poll_read(cx, buf)
    }
}

impl AsyncSeek for EpochStream {
    fn start_seek(self: Pin<&mut Self>, position: SeekFrom) -> io::Result<()> {
        let this = unsafe { self.get_unchecked_mut() };
        this.inner.as_mut().start_seek(position)
    }

    fn poll_complete(
        self: Pin<&mut Self>,
        cx: &mut std::task::Context<'_>,
    ) -> std::task::Poll<io::Result<u64>> {
        let this = unsafe { self.get_unchecked_mut() };
        this.inner.as_mut().poll_complete(cx)
    }
}

impl Len for EpochStream {
    fn len(&self) -> u64 {
        self.total_len
    }
}

impl Unpin for EpochStream {}

/* ────────────────────────────────────────────────────────────────────────── */
/*  Blanket Len impl so BufReader<T> keeps the .len() we rely on            */
/* ────────────────────────────────────────────────────────────────────────── */
impl<T: Len + AsyncRead> Len for BufReader<T> {
    #[inline]
    fn len(&self) -> u64 {
        self.get_ref().len()
    }
}

/// Controls how epoch CAR streams are opened for [`fetch_epoch_stream_with_options`].
#[derive(Clone, Debug)]
pub struct FetchEpochStreamOptions {
    /// When `true`, stream bytes sequentially through ripget's windowed downloader.
    pub sequential: bool,
    /// Parallel range request count used by ripget when `sequential` is enabled.
    pub ripget_threads: usize,
    /// Total hot/cold window size in bytes for ripget windowed streaming.
    pub buffer_window_bytes: u64,
    /// Pre-built ripget HTTP client for connection reuse across epoch downloads.
    pub ripget_client: Option<ripget::Client>,
}

impl FetchEpochStreamOptions {
    /// Returns default options that preserve the legacy seekable behavior.
    pub fn parallel_default() -> Self {
        Self {
            sequential: false,
            ripget_threads: 1,
            buffer_window_bytes: 2,
            ripget_client: None,
        }
    }
}

struct RipgetEpochReader {
    inner: WindowedDownload,
    len: u64,
    position: u64,
}

impl RipgetEpochReader {
    async fn new(
        url: impl AsRef<str>,
        threads: usize,
        buffer_window_bytes: u64,
        ripget_client: Option<ripget::Client>,
    ) -> Result<Self, ripget::RipgetError> {
        let mut options = WindowedDownloadOptions::new(buffer_window_bytes.max(2))
            .threads(std::cmp::max(1, threads))
            .user_agent(format!(
                "jetstreamer-firehose/{}",
                env!("CARGO_PKG_VERSION")
            ));
        if let Some(c) = ripget_client {
            options = options.client(c);
        }
        let inner = download_url_windowed(url.as_ref(), options).await?;
        let len = inner.expected_len();
        Ok(Self {
            inner,
            len,
            position: 0,
        })
    }
}

impl AsyncRead for RipgetEpochReader {
    fn poll_read(
        self: Pin<&mut Self>,
        cx: &mut std::task::Context<'_>,
        buf: &mut ReadBuf<'_>,
    ) -> std::task::Poll<io::Result<()>> {
        let this = self.get_mut();
        let before = buf.filled().len();
        let result = Pin::new(&mut this.inner).poll_read(cx, buf);
        if let std::task::Poll::Ready(Ok(())) = &result {
            let after = buf.filled().len();
            let delta = after.saturating_sub(before) as u64;
            this.position = this.position.saturating_add(delta);
        }
        result
    }
}

impl AsyncSeek for RipgetEpochReader {
    fn start_seek(self: Pin<&mut Self>, position: SeekFrom) -> io::Result<()> {
        if matches!(position, SeekFrom::Current(0)) {
            return Ok(());
        }
        Err(io::Error::new(
            io::ErrorKind::Unsupported,
            "seek is not supported for ripget windowed streams",
        ))
    }

    fn poll_complete(
        self: Pin<&mut Self>,
        _cx: &mut std::task::Context<'_>,
    ) -> std::task::Poll<io::Result<u64>> {
        let this = self.get_mut();
        std::task::Poll::Ready(Ok(this.position))
    }
}

impl Len for RipgetEpochReader {
    fn len(&self) -> u64 {
        self.len
    }
}

/// Checks whether an epoch CAR exists. Any failure, including throttling, reads as `false`;
/// [`check_epoch_exists`] tells the two apart.
pub async fn epoch_exists(epoch: u64, client: &Client) -> bool {
    epoch_exists_at(epoch, client, archive::car_location()).await
}

async fn epoch_exists_at(epoch: u64, client: &Client, location: &archive::Location) -> bool {
    check_epoch_exists_at(epoch, client, location)
        .await
        .unwrap_or(false)
}

/// Checks whether an epoch CAR exists. `Ok(false)` means the archive reports it missing;
/// throttling (HTTP 429), server and network errors return `Err`.
pub async fn check_epoch_exists(epoch: u64, client: &Client) -> Result<bool, SharedError> {
    check_epoch_exists_at(epoch, client, archive::car_location()).await
}

pub(crate) async fn check_epoch_exists_at(
    epoch: u64,
    client: &Client,
    location: &archive::Location,
) -> Result<bool, SharedError> {
    let path = format!("{epoch}/epoch-{epoch}.car");

    if location.is_http() {
        let url = location.url().join(&path)?;
        let response = client.head(url.clone()).send().await?;
        let status = response.status();
        if status.is_success() {
            return Ok(true);
        }
        if status == reqwest::StatusCode::NOT_FOUND {
            return Ok(false);
        }
        return Err(format!("HTTP {status} checking {url}").into());
    }

    #[cfg(feature = "s3-backend")]
    if let Some(cfg) = location.as_s3() {
        let key = cfg.key_for(&path);
        return match cfg
            .client
            .head_object()
            .bucket(cfg.bucket.as_ref())
            .key(&key)
            .send()
            .await
        {
            Ok(_) => Ok(true),
            // Without s3:ListBucket a missing key answers 403 instead of 404.
            Err(err)
                if err.as_service_error().is_some_and(|e| e.is_not_found())
                    || err.raw_response().map(|r| r.status().as_u16()) == Some(403) =>
            {
                Ok(false)
            }
            Err(err) => Err(format!("S3 error checking {key}: {err}").into()),
        };
    }

    panic!(
        "unsupported archive backend for CAR location {}",
        location.url()
    );
}

/// Finds the newest epoch at or below `start` whose CAR exists. A failed check is retried with
/// backoff, then returned as an error; it never counts as a missing epoch.
pub(crate) async fn latest_epoch_at(
    start: u64,
    client: &Client,
    location: &archive::Location,
    retry_base: std::time::Duration,
) -> Result<u64, SharedError> {
    const ATTEMPTS: u32 = 5;
    let mut epoch = start;
    loop {
        let mut attempt = 0;
        let exists = loop {
            match check_epoch_exists_at(epoch, client, location).await {
                Ok(exists) => break exists,
                Err(err) if attempt + 1 >= ATTEMPTS => return Err(err),
                Err(err) => {
                    let delay = retry_base.saturating_mul(1 << attempt);
                    log::warn!(
                        target: crate::jetstreamer_firehose::LOG_MODULE,
                        "checking epoch {epoch} failed ({err}); retrying in {delay:?}"
                    );
                    tokio::time::sleep(delay).await;
                    attempt += 1;
                }
            }
        };
        if exists {
            return Ok(epoch);
        }
        epoch = epoch
            .checked_sub(1)
            .ok_or_else(|| format!("no Old Faithful epoch found at or below epoch {start}"))?;
    }
}

/// Fetches an epoch’s CAR file from the configured archive backend as a buffered, seekable stream.
pub async fn fetch_epoch_stream(epoch: u64, client: &Client) -> EpochStream {
    fetch_epoch_stream_with_options(epoch, client, None).await
}

/// Fetches an epoch’s CAR file with explicit stream options.
///
/// In sequential mode, arbitrary seeking is not supported and seek requests other than
/// `SeekFrom::Current(0)` return `io::ErrorKind::Unsupported`.
pub async fn fetch_epoch_stream_with_options(
    epoch: u64,
    client: &Client,
    options: Option<FetchEpochStreamOptions>,
) -> EpochStream {
    fetch_epoch_stream_at(epoch, client, options, archive::car_location()).await
}

async fn fetch_epoch_stream_at(
    epoch: u64,
    client: &Client,
    options: Option<FetchEpochStreamOptions>,
    location: &archive::Location,
) -> EpochStream {
    let options = options.unwrap_or_else(FetchEpochStreamOptions::parallel_default);
    let path = format!("{epoch}/epoch-{epoch}.car");

    if location.is_http() {
        let url = location
            .url()
            .join(&path)
            .unwrap_or_else(|err| panic!("invalid CAR URL for epoch {epoch}: {err}"));
        let request_url = url.to_string();
        if options.sequential {
            match RipgetEpochReader::new(
                request_url.clone(),
                options.ripget_threads,
                options.buffer_window_bytes,
                options.ripget_client,
            )
            .await
            {
                Ok(reader) => {
                    return EpochStream::new(BufReader::with_capacity(8 * 1024 * 1024, reader));
                }
                Err(err) => {
                    log::warn!(
                        target: crate::jetstreamer_firehose::LOG_MODULE,
                        "ripget windowed stream failed to initialize for epoch {} ({}), falling back to seekable stream",
                        epoch,
                        err
                    );
                }
            }
        }
        let http_client = client.clone();
        let seekable = Seekable::new(move || http_client.get(request_url.clone())).await;
        let reader = BufReader::with_capacity(8 * 1024 * 1024, seekable);
        return EpochStream::new(reader);
    }

    #[cfg(feature = "s3-backend")]
    if let Some(cfg) = location.as_s3() {
        let s3_reader = S3SeekableReader::new(cfg, path)
            .await
            .unwrap_or_else(|err| panic!("failed to open epoch {epoch} CAR via S3: {err}"));
        let reader = BufReader::with_capacity(8 * 1024 * 1024, s3_reader);
        return EpochStream::new(reader);
    }

    panic!(
        "unsupported archive backend for CAR location {}",
        location.url()
    );
}

#[cfg(feature = "s3-backend")]
type ReaderFuture =
    Pin<Box<dyn Future<Output = io::Result<Pin<Box<dyn AsyncRead + Send>>>> + Send>>;

#[cfg(feature = "s3-backend")]
struct S3SeekableReader {
    client: Arc<S3Client>,
    bucket: Arc<str>,
    key: String,
    len: u64,
    position: u64,
    reader: Option<Pin<Box<dyn AsyncRead + Send>>>,
    init_fetch: Option<ReaderFuture>,
}

#[cfg(feature = "s3-backend")]
impl S3SeekableReader {
    async fn new(location: Arc<S3Location>, path: String) -> io::Result<Self> {
        let key = location.key_for(&path);
        let head = location
            .client
            .head_object()
            .bucket(location.bucket.as_ref())
            .key(&key)
            .send()
            .await
            .map_err(|err| io::Error::other(err.to_string()))?;
        let len = head
            .content_length()
            .ok_or_else(|| io::Error::other("S3 object missing length"))? as u64;

        let mut reader = Self {
            client: Arc::clone(&location.client),
            bucket: Arc::clone(&location.bucket),
            key,
            len,
            position: 0,
            reader: None,
            init_fetch: None,
        };
        reader.schedule_fetch(0);
        Ok(reader)
    }

    fn schedule_fetch(&mut self, start: u64) {
        let client = Arc::clone(&self.client);
        let bucket = Arc::clone(&self.bucket);
        let key = self.key.clone();
        let len = self.len;
        self.reader = None;
        self.init_fetch = Some(Box::pin(async move {
            let end = len.saturating_sub(1);
            let range = if start >= len {
                format!("bytes={end}-{end}")
            } else {
                format!("bytes={start}-{end}")
            };
            let resp = client
                .get_object()
                .bucket(bucket.as_ref())
                .key(&key)
                .range(range)
                .send()
                .await
                .map_err(|err| io::Error::other(err.to_string()))?;
            let reader = tokio::io::BufReader::new(resp.body.into_async_read());
            Ok(Box::pin(reader) as Pin<Box<dyn AsyncRead + Send>>)
        }));
    }
}

#[cfg(feature = "s3-backend")]
impl AsyncRead for S3SeekableReader {
    fn poll_read(
        self: Pin<&mut Self>,
        cx: &mut std::task::Context<'_>,
        buf: &mut ReadBuf<'_>,
    ) -> std::task::Poll<io::Result<()>> {
        let this = unsafe { self.get_unchecked_mut() };

        if this.position >= this.len {
            return std::task::Poll::Ready(Ok(()));
        }

        loop {
            if let Some(reader) = this.reader.as_mut() {
                let before = buf.filled().len();
                match reader.as_mut().poll_read(cx, buf) {
                    std::task::Poll::Ready(Ok(())) => {
                        let after = buf.filled().len();
                        let delta = after.saturating_sub(before);
                        this.position = this.position.saturating_add(delta as u64);
                        return std::task::Poll::Ready(Ok(()));
                    }
                    other => return other,
                }
            }

            if let Some(fut) = this.init_fetch.as_mut() {
                match fut.as_mut().poll(cx) {
                    std::task::Poll::Ready(Ok(reader)) => {
                        this.reader = Some(reader);
                        this.init_fetch = None;
                        continue;
                    }
                    std::task::Poll::Ready(Err(err)) => {
                        this.init_fetch = None;
                        return std::task::Poll::Ready(Err(err));
                    }
                    std::task::Poll::Pending => return std::task::Poll::Pending,
                }
            } else {
                this.schedule_fetch(this.position);
            }
        }
    }
}

#[cfg(feature = "s3-backend")]
impl AsyncSeek for S3SeekableReader {
    fn start_seek(self: Pin<&mut Self>, position: SeekFrom) -> io::Result<()> {
        let this = unsafe { self.get_unchecked_mut() };
        let target = match position {
            SeekFrom::Start(offset) => offset,
            SeekFrom::Current(delta) => {
                let tmp = this.position as i64 + delta;
                if tmp < 0 {
                    return Err(io::Error::new(io::ErrorKind::InvalidInput, "negative seek"));
                }
                tmp as u64
            }
            SeekFrom::End(delta) => {
                let tmp = this.len as i64 + delta;
                if tmp < 0 {
                    return Err(io::Error::new(io::ErrorKind::InvalidInput, "negative seek"));
                }
                tmp as u64
            }
        };
        this.position = target.min(this.len);
        this.reader = None;
        this.init_fetch = None;
        this.schedule_fetch(this.position);
        Ok(())
    }

    fn poll_complete(
        self: Pin<&mut Self>,
        _cx: &mut std::task::Context<'_>,
    ) -> std::task::Poll<io::Result<u64>> {
        let this = unsafe { self.get_unchecked_mut() };
        std::task::Poll::Ready(Ok(this.position))
    }
}

#[cfg(feature = "s3-backend")]
impl Len for S3SeekableReader {
    fn len(&self) -> u64 {
        self.len
    }
}

#[cfg(feature = "s3-backend")]
impl Unpin for S3SeekableReader {}

/// Errors that can occur when calling [`get_slot_timestamp`].
#[derive(Debug)]
pub enum SlotTimestampError {
    /// Network request failed while contacting the RPC endpoint.
    Transport(reqwest::Error),
    /// JSON payload could not be decoded.
    Decode(serde_json::Error),
    /// RPC returned an error object instead of a result.
    Rpc(Option<serde_json::Value>),
    /// The RPC response did not include a block time.
    NoBlockTime,
}
impl fmt::Display for SlotTimestampError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            SlotTimestampError::Transport(e) => write!(f, "RPC transport error: {e}"),
            SlotTimestampError::Decode(e) => write!(f, "RPC decode error: {e}"),
            SlotTimestampError::Rpc(e) => write!(f, "RPC error: {:?}", e),
            SlotTimestampError::NoBlockTime => write!(f, "No blockTime found in getBlock result"),
        }
    }
}
impl std::error::Error for SlotTimestampError {}

/// Get the true Unix timestamp (seconds since epoch, UTC) for a Solana slot.
/// Uses the validator RPC getBlock method, returns Ok(timestamp) or Err(reason).
pub async fn get_slot_timestamp(
    slot: u64,
    rpc_url: &str,
    client: &Client,
) -> Result<u64, SlotTimestampError> {
    #[derive(Deserialize)]
    struct BlockResult {
        #[serde(rename = "blockTime")]
        block_time: Option<u64>,
    }
    #[derive(Deserialize)]
    struct RpcResponse {
        result: Option<BlockResult>,
        error: Option<serde_json::Value>,
    }

    let req = serde_json::json!({
        "jsonrpc": "2.0",
        "id": 1,
        "method": "getBlock",
        "params": [slot, { "maxSupportedTransactionVersion": 0 }],
    });

    let resp = client
        .post(rpc_url)
        .json(&req)
        .send()
        .await
        .map_err(SlotTimestampError::Transport)?;

    let text = resp.text().await.map_err(SlotTimestampError::Transport)?;
    let resp_val: RpcResponse = serde_json::from_str(&text).map_err(SlotTimestampError::Decode)?;

    if resp_val.error.is_some() {
        return Err(SlotTimestampError::Rpc(resp_val.error));
    }
    resp_val
        .result
        .and_then(|r| r.block_time)
        .ok_or(SlotTimestampError::NoBlockTime)
}

/* ── Tests ──────────────────────────────────────────────────────────────── */
