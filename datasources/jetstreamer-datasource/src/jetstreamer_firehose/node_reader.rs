use crate::jetstreamer_firehose::LOG_MODULE;
use crate::jetstreamer_firehose::SharedError;
use crate::jetstreamer_firehose::epochs::{epoch_to_slot_range, slot_to_epoch};
use crate::jetstreamer_firehose::firehose::FirehoseError;
use crate::jetstreamer_firehose::index::{SlotOffsetIndexError, slot_to_range};
use crate::jetstreamer_firehose::node::{Node, NodeWithCid, NodesWithCids, parse_any_from_cbordata};
use crate::jetstreamer_firehose::pace::pace;
use crate::jetstreamer_firehose::utils;
use cid::Cid;
use once_cell::sync::Lazy;
use reqwest::RequestBuilder;
use rseek::Seekable;
use std::io;
use std::io::SeekFrom;
use std::sync::atomic::{AtomicU64, Ordering};
use std::time::Instant;
use std::vec::Vec;
use tokio::io::{AsyncRead, AsyncReadExt, AsyncSeek, AsyncSeekExt};
use tokio::time::sleep;

const MAX_VARINT_LEN_64: usize = 10;
static REQUEST_START_INSTANT: Lazy<Instant> = Lazy::new(Instant::now);
static LAST_REQUEST_TIME: AtomicU64 = AtomicU64::new(0);

/// Total CAR section bytes read across all readers and threads since process start. Frontends
/// (e.g. the TUI) sample this to derive an overall data rate.
pub static TOTAL_BYTES_READ: AtomicU64 = AtomicU64::new(0);

/// Reads an unsigned LEB128-encoded integer from the provided async reader.
pub async fn read_uvarint<R: AsyncRead + Unpin>(reader: &mut R) -> io::Result<u64> {
    let mut x = 0u64;
    let mut s = 0u32;
    let mut buffer = [0u8; 1];

    for i in 0..MAX_VARINT_LEN_64 {
        reader.read_exact(&mut buffer).await?;
        let b = buffer[0];
        if b < 0x80 {
            if i == MAX_VARINT_LEN_64 - 1 && b > 1 {
                return Err(io::Error::new(
                    io::ErrorKind::InvalidData,
                    "uvarint overflow",
                ));
            }
            return Ok(x | ((b as u64) << s));
        }
        x |= ((b & 0x7f) as u64) << s;
        s += 7;

        if s > 63 {
            return Err(io::Error::new(
                io::ErrorKind::InvalidData,
                "uvarint too long",
            ));
        }
    }
    Err(io::Error::new(
        io::ErrorKind::InvalidData,
        "uvarint overflow",
    ))
}

/// Raw DAG-CBOR node paired with its [`Cid`].
#[derive(Clone, PartialEq, Eq, Hash)]
pub struct RawNode {
    /// Content identifier for the node.
    pub cid: Cid,
    /// Raw CBOR-encoded bytes for the node.
    pub data: Vec<u8>,
}

// Debug trait for RawNode
impl core::fmt::Debug for RawNode {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        f.debug_struct("RawNode")
            .field("cid", &self.cid)
            .field("data", &self.data)
            .finish()
    }
}

impl RawNode {
    /// Creates a new [`RawNode`] from a CID and CBOR payload.
    pub const fn new(cid: Cid, data: Vec<u8>) -> RawNode {
        RawNode { cid, data }
    }

    /// Parses the CBOR payload into a typed [`Node`].
    pub fn parse(&self) -> Result<Node, SharedError> {
        match parse_any_from_cbordata(self.data.clone()) {
            Ok(node) => Ok(node),
            Err(err) => {
                println!("Error: {:?}", err);
                Err(Box::new(std::io::Error::other("Unknown type".to_owned())))
            }
        }
    }

    /// Reads a [`RawNode`] from a CAR section cursor.
    pub async fn from_cursor(cursor: &mut io::Cursor<Vec<u8>>) -> Result<RawNode, SharedError> {
        let cid_version = read_uvarint(cursor).await?;
        // println!("CID version: {}", cid_version);

        let multicodec = read_uvarint(cursor).await?;
        // println!("Multicodec: {}", multicodec);

        // Multihash hash function code.
        let hash_function = read_uvarint(cursor).await?;
        // println!("Hash function: {}", hash_function);

        // Multihash digest length.
        let digest_length = read_uvarint(cursor).await?;
        // println!("Digest length: {}", digest_length);

        if digest_length > 64 {
            return Err(Box::new(std::io::Error::other(format!(
                "Digest length too long, position={}",
                cursor.position()
            ))));
        }

        // reac actual digest
        let mut digest = vec![0u8; digest_length as usize];
        cursor.read_exact(&mut digest).await?;

        // the rest is the data
        let mut data = vec![];
        cursor.read_to_end(&mut data).await?;

        // println!("Data: {:?}", data);

        let ha = multihash::Multihash::wrap(hash_function, digest.as_slice())?;

        match cid_version {
            0 => {
                let cid = Cid::new_v0(ha)?;
                let raw_node = RawNode::new(cid, data);
                Ok(raw_node)
            }
            1 => {
                let cid = Cid::new_v1(multicodec, ha);
                let raw_node = RawNode::new(cid, data);
                Ok(raw_node)
            }
            _ => Err(Box::new(std::io::Error::other(
                "Unknown CID version".to_owned(),
            ))),
        }
    }
}

/// Trait for readers that can report their total length.
pub trait Len {
    /// Returns the total number of bytes available.
    fn len(&self) -> u64;
    /// Returns `true` when the length is zero.
    fn is_empty(&self) -> bool {
        self.len() == 0
    }
}

impl<F> Len for Seekable<F>
where
    F: Fn() -> RequestBuilder + Send + Sync + 'static,
{
    fn len(&self) -> u64 {
        self.file_size.unwrap_or(0)
    }
}

/// Incremental reader that produces typed nodes from an Old Faithful CAR stream.
pub struct NodeReader<R: AsyncRead + AsyncSeek + Len> {
    /// Underlying stream yielding Old Faithful CAR bytes.
    pub reader: R,
    /// Cached Old Faithful CAR header data.
    pub header: Vec<u8>,
    /// Number of Old Faithful items that have been read so far.
    pub item_index: u64,
    /// Whether the next seek already holds the global request-spacing permit.
    seek_permit_primed: bool,
}

impl<R: AsyncRead + Unpin + AsyncSeek + Len> NodeReader<R> {
    /// Wraps an async reader and primes it for Old Faithful CAR decoding.
    pub const fn new(reader: R) -> NodeReader<R> {
        NodeReader {
            reader,
            header: vec![],
            item_index: 0,
            seek_permit_primed: false,
        }
    }

    /// Returns the raw Old Faithful CAR header, fetching and caching it on first use.
    pub async fn read_raw_header(&mut self) -> Result<Vec<u8>, SharedError> {
        if !self.header.is_empty() {
            return Ok(self.header.clone());
        };
        let header_length = read_uvarint(&mut self.reader).await?;
        if header_length > 1024 {
            return Err(Box::new(std::io::Error::other(
                "Header length too long".to_owned(),
            )));
        }
        let mut header = vec![0u8; header_length as usize];
        self.reader.read_exact(&mut header).await?;

        self.header.clone_from(&header);

        let clone = header.clone();
        Ok(clone.as_slice().to_owned())
    }

    /// Seeks the underlying reader to the first Old Faithful CAR section belonging to `slot`,
    /// so that reading forward yields all of the slot's nodes (transactions, entries, rewards)
    /// followed by its Block node. If `slot` is missing from the index (skipped on-chain), the
    /// seek advances to the next present slot.
    pub async fn seek_to_slot(&mut self, slot: u64) -> Result<(), FirehoseError> {
        self.seek_to_slot_inner(slot).await
    }

    /// Acquires the global request-spacing permit ahead of a [`Self::seek_to_slot`] call, so the
    /// cross-thread pacing wait (which can queue for many seconds when hundreds of threads
    /// seek at once) happens outside any timeout the caller wraps the seek in. The permit is
    /// consumed by the next seek; an unprimed seek acquires it itself.
    pub async fn prime_seek_permit(&mut self) {
        wait_for_request_slot().await;
        self.seek_permit_primed = true;
    }

    async fn seek_to_slot_inner(&mut self, slot: u64) -> Result<(), FirehoseError> {
        if self.header.is_empty() {
            self.read_raw_header()
                .await
                .map_err(FirehoseError::SeekToSlotError)?;
        };

        let epoch = slot_to_epoch(slot);
        let (epoch_start, epoch_end_inclusive) = epoch_to_slot_range(epoch);
        let mut current = slot;
        loop {
            match slot_to_range(current).await {
                Ok((offset, _)) => {
                    log::info!(
                        target: LOG_MODULE,
                        "Seeking to slot {} in epoch {} @ offset {}",
                        current,
                        epoch,
                        offset
                    );
                    return self.seek_to_offset(offset).await;
                }
                Err(SlotOffsetIndexError::SlotNotFound(..)) => {
                    if current >= epoch_end_inclusive {
                        // No present slot remains between `slot` and the end of its epoch, and
                        // offsets from other epochs are meaningless for this reader's CAR
                        // stream. Position the reader just past the last present slot's data so
                        // callers read the epoch's trailing non-block nodes and observe a clean
                        // end-of-epoch.
                        return self.seek_to_epoch_tail(slot, epoch, epoch_start).await;
                    }
                    log::warn!(
                        target: LOG_MODULE,
                        "Slot {} not found in index, seeking to next slot",
                        current
                    );
                    current += 1;
                }
                // Surface index failures as SlotOffsetIndexError so callers can invalidate the
                // cached epoch index and retry with fresh data.
                Err(err) => return Err(FirehoseError::SlotOffsetIndexError(err)),
            }
        }
    }

    /// Seeks just past the end of the data of the last present slot preceding `slot`, for use
    /// when no slot at or after `slot` exists in its epoch.
    async fn seek_to_epoch_tail(
        &mut self,
        slot: u64,
        epoch: u64,
        epoch_start: u64,
    ) -> Result<(), FirehoseError> {
        let mut candidate = slot;
        while candidate > epoch_start {
            candidate -= 1;
            match slot_to_range(candidate).await {
                Ok((offset, length)) => {
                    log::warn!(
                        target: LOG_MODULE,
                        "No slot at or after {} is present in epoch {}; seeking past the last \
                         present slot {}",
                        slot,
                        epoch,
                        candidate
                    );
                    return self.seek_to_offset(offset + length).await;
                }
                Err(SlotOffsetIndexError::SlotNotFound(..)) => continue,
                Err(err) => return Err(FirehoseError::SlotOffsetIndexError(err)),
            }
        }
        // An index that reports every slot of an epoch as skipped is corrupt; surface an index
        // error so callers invalidate the cached epoch index and refetch instead of retrying
        // against the same bad data forever.
        Err(FirehoseError::SlotOffsetIndexError(
            SlotOffsetIndexError::EpochHasNoIndexedSlots(epoch),
        ))
    }

    async fn seek_to_offset(&mut self, offset: u64) -> Result<(), FirehoseError> {
        if !std::mem::take(&mut self.seek_permit_primed) {
            wait_for_request_slot().await;
        }
        self.reader
            .seek(SeekFrom::Start(offset))
            .await
            .map_err(|e| FirehoseError::SeekToSlotError(Box::new(e)))?;
        Ok(())
    }

    #[allow(clippy::should_implement_trait)]
    /// Reads the next raw node from the Old Faithful stream without parsing it.
    pub async fn next(&mut self) -> Result<RawNode, SharedError> {
        if self.header.is_empty() {
            self.read_raw_header().await?;
        };

        // println!("Item index: {}", item_index);
        self.item_index += 1;

        // Read and decode the uvarint prefix (length of CID + data)
        let section_size = read_uvarint(&mut self.reader).await?;
        // println!("Section size: {}", section_size);

        if section_size > utils::MAX_ALLOWED_SECTION_SIZE as u64 {
            return Err(Box::new(std::io::Error::other(
                "Section size too long".to_owned(),
            )));
        }

        // read whole item
        let mut item = vec![0u8; section_size as usize];
        self.reader.read_exact(&mut item).await?;
        TOTAL_BYTES_READ.fetch_add(section_size, Ordering::Relaxed);

        // dump item bytes as numbers
        // println!("Item bytes: {:?}", item);

        // now create a cursor over the item
        let mut cursor = io::Cursor::new(item);

        RawNode::from_cursor(&mut cursor).await
    }

    /// Reads and parses the next node, returning it paired with its [`Cid`].
    pub async fn next_parsed(&mut self) -> Result<NodeWithCid, SharedError> {
        let raw_node = self.next().await?;
        let cid = raw_node.cid;
        Ok(NodeWithCid::new(cid, raw_node.parse()?))
    }

    /// Continues reading nodes until the next block is encountered.
    pub async fn read_until_block(&mut self) -> Result<NodesWithCids, SharedError> {
        let mut nodes = NodesWithCids::new();
        loop {
            let node = match self.next_parsed().await {
                Ok(node) => node,
                Err(e)
                    if e.downcast_ref::<io::Error>()
                        .is_some_and(|io_err| io_err.kind() == io::ErrorKind::UnexpectedEof) =>
                {
                    break;
                }
                Err(e) => return Err(e),
            };
            if node.get_node().is_block() {
                nodes.push(node);
                break;
            }
            nodes.push(node);
        }
        Ok(nodes)
    }

    /// Returns the number of Old Faithful CAR items that have been yielded so far.
    pub const fn get_item_index(&self) -> u64 {
        self.item_index
    }
}

fn request_monotonic_millis() -> u64 {
    let elapsed = REQUEST_START_INSTANT.elapsed().as_millis();
    if elapsed > u64::MAX as u128 {
        u64::MAX
    } else {
        elapsed as u64
    }
}

/// Waits for the process-wide request-spacing permit. Every stream open and seek takes one, so
/// reconnects happen at most once per [`crate::jetstreamer_firehose::pace::Pace::request_spacing`].
pub(crate) async fn wait_for_request_slot() {
    let spacing_ms = pace().request_spacing().as_millis() as u64;
    loop {
        let now_ms = request_monotonic_millis();
        let last_hit = LAST_REQUEST_TIME.load(Ordering::Relaxed);
        let since_last = now_ms.saturating_sub(last_hit);
        if since_last < spacing_ms {
            // Sleep out the remainder of the spacing window instead of spinning; with
            // hundreds of threads queued this otherwise burns CPU for the whole drain.
            sleep(std::time::Duration::from_millis(spacing_ms - since_last)).await;
            continue;
        }
        if LAST_REQUEST_TIME
            .compare_exchange(last_hit, now_ms, Ordering::SeqCst, Ordering::SeqCst)
            .is_ok()
        {
            return;
        }
        // Lost the race for this window; the winner's spacing interval starts now.
        sleep(std::time::Duration::from_millis(spacing_ms)).await;
    }
}

/// Extracts a CID from a DAG-CBOR link value.
pub fn cid_from_cbor_link(val: &serde_cbor::Value) -> Result<cid::Cid, SharedError> {
    if let serde_cbor::Value::Bytes(b) = val
        && b.first() == Some(&0)
    {
        return Ok(cid::Cid::try_from(b[1..].to_vec())?);
    }
    Err("invalid DAG‑CBOR link encoding".into())
}
