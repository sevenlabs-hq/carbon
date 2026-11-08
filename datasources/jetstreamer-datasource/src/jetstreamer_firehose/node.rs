use {
    crate::jetstreamer_firehose::{SharedError, block, dataframe, entry, epoch, rewards, subset, transaction, utils},
    ahash::RandomState,
    cid::Cid,
    core::hash::Hasher,
    crc::{CRC_64_GO_ISO, Crc},
    fnv::FnvHasher,
    std::{
        collections::{HashMap, HashSet},
        fmt,
        io::{self, Read},
        vec::Vec,
    },
};

/// Pairing of a decoded [`Node`] with its [`Cid`].
pub struct NodeWithCid {
    cid: Cid,
    node: Node,
}

impl NodeWithCid {
    /// Creates a new `(CID, node)` pair.
    pub const fn new(cid: Cid, node: Node) -> NodeWithCid {
        NodeWithCid { cid, node }
    }

    /// Returns the CID associated with the node.
    pub const fn get_cid(&self) -> &Cid {
        &self.cid
    }

    /// Returns the decoded node.
    pub const fn get_node(&self) -> &Node {
        &self.node
    }
}

/// Convenience collection that retains the CID for every stored node.
#[derive(Default)]
pub struct NodesWithCids(
    #[doc = "Ordered collection of nodes paired with their content identifiers."]
    pub  Vec<NodeWithCid>,
    #[doc(hidden)] HashMap<Cid, usize, RandomState>,
);

impl NodesWithCids {
    /// Creates an empty [`NodesWithCids`].
    pub fn new() -> NodesWithCids {
        NodesWithCids(vec![], HashMap::with_hasher(RandomState::new()))
    }

    /// Appends a node to the collection.
    pub fn push(&mut self, node_with_cid: NodeWithCid) {
        let index = self.0.len();
        self.1.insert(*node_with_cid.get_cid(), index);
        self.0.push(node_with_cid);
    }

    /// Returns the number of stored nodes.
    pub const fn len(&self) -> usize {
        self.0.len()
    }

    /// Returns `true` if no nodes are stored.
    pub const fn is_empty(&self) -> bool {
        self.len() == 0
    }

    /// Returns the node at `index`.
    pub fn get(&self, index: usize) -> &NodeWithCid {
        &self.0[index]
    }

    /// Looks up a node by CID.
    pub fn get_by_cid(&self, cid: &Cid) -> Option<&NodeWithCid> {
        self.1.get(cid).and_then(|index| self.0.get(*index))
    }

    /// Reassembles a potentially multi-part dataframe using the nodes in the collection.
    pub fn reassemble_dataframes(
        &self,
        first_dataframe: &dataframe::DataFrame,
    ) -> Result<Vec<u8>, SharedError> {
        let mut data = Vec::with_capacity(first_dataframe.data.len());
        data.extend_from_slice(first_dataframe.data.as_slice());

        // A continuation may not revisit a frame the traversal already consumed; a cycle would reappend the same chunks forever.
        let mut visited = HashSet::new();
        let mut next_arr = first_dataframe.next.as_deref();
        while let Some(next_cids) = next_arr {
            let mut next_segment = None;
            for next_cid in next_cids {
                if !visited.insert(*next_cid) {
                    return Err(Box::new(std::io::Error::other(std::format!(
                        "Cyclic dataframe continuation: {:?}",
                        next_cid
                    ))) as SharedError);
                }
                let next_node = self.get_by_cid(next_cid).ok_or_else(|| {
                    Box::new(std::io::Error::other(std::format!(
                        "Missing CID: {:?}",
                        next_cid
                    ))) as SharedError
                })?;

                let next_dataframe = next_node.get_node().get_dataframe().ok_or_else(|| {
                    Box::new(std::io::Error::other(std::format!(
                        "Expected DataFrame, got {:?}",
                        next_node.get_node()
                    ))) as SharedError
                })?;

                data.extend_from_slice(next_dataframe.data.as_slice());
                next_segment = next_dataframe.next.as_deref();
            }
            next_arr = next_segment;
        }

        if let Some(wanted_hash) = first_dataframe.hash {
            verify_hash(&data, wanted_hash)?;
        }
        Ok(data)
    }

    /// Iterates over every node and invokes `f`.
    pub fn each<F>(&self, mut f: F) -> Result<(), SharedError>
    where
        F: FnMut(&NodeWithCid) -> Result<(), SharedError>,
    {
        for node_with_cid in &self.0 {
            f(node_with_cid)?;
        }
        Ok(())
    }

    /// Returns the CIDs for all stored nodes.
    pub fn get_cids(&self) -> Vec<Cid> {
        let mut cids = vec![];
        for node_with_cid in &self.0 {
            cids.push(*node_with_cid.get_cid());
        }
        cids
    }

    /// Returns a reference to the final [`block::Block`] in the collection.
    pub fn get_block(&self) -> Result<&block::Block, SharedError> {
        // the last node should be a block
        let last_node = self.0.last();
        if last_node.is_none() {
            return Err(Box::new(std::io::Error::other("No nodes".to_owned())));
        }
        let last_node_un = last_node.unwrap();
        if !last_node_un.get_node().is_block() {
            return Err(Box::new(std::io::Error::other(std::format!(
                "Expected Block, got {:?}",
                last_node_un.get_node()
            ))));
        }
        let block = last_node_un.get_node().get_block().unwrap();
        Ok(block)
    }
}

/// Validates the provided data against the expected CRC64 (or legacy FNV) hash.
pub fn verify_hash(data: &[u8], hash: u64) -> Result<(), SharedError> {
    let crc64 = checksum_crc64(data);
    if crc64 != hash {
        // Maybe it's the legacy checksum function?
        let fnv = checksum_fnv(data);
        if fnv != hash {
            return Err(Box::new(std::io::Error::other(std::format!(
                "data hash mismatch: wanted {:?}, got crc64={:?}, fnv={:?}",
                hash,
                crc64,
                fnv
            ))));
        }
    }
    Ok(())
}

fn checksum_crc64(data: &[u8]) -> u64 {
    let crc = Crc::<u64>::new(&CRC_64_GO_ISO);
    let mut digest = crc.digest();
    digest.update(data);
    digest.finalize()
}

fn checksum_fnv(data: &[u8]) -> u64 {
    let mut hasher = FnvHasher::default();
    hasher.write(data);
    hasher.finish()
}

/// Unified representation of all decoded firehose node types.
#[derive(Clone, PartialEq, Eq, Hash, Debug)]
pub enum Node {
    /// Raw transaction node.
    Transaction(transaction::Transaction),
    /// Ledger entry node.
    Entry(entry::Entry),
    /// Block node containing ledger metadata and entries.
    Block(block::Block),
    /// Subset node linking to a contiguous block range.
    Subset(subset::Subset),
    /// Epoch node referencing subset descriptors.
    Epoch(epoch::Epoch),
    /// Rewards node containing per-account payouts.
    Rewards(rewards::Rewards),
    /// Data frame node wrapping arbitrary binary payloads.
    DataFrame(dataframe::DataFrame),
}

impl Node {
    /// Returns `true` if this node is a [`transaction::Transaction`].
    pub const fn is_transaction(&self) -> bool {
        matches!(self, Node::Transaction(_))
    }

    /// Returns `true` if this node is an [`entry::Entry`].
    pub const fn is_entry(&self) -> bool {
        matches!(self, Node::Entry(_))
    }

    /// Returns `true` if this node is a [`block::Block`].
    pub const fn is_block(&self) -> bool {
        matches!(self, Node::Block(_))
    }

    /// Returns `true` if this node is a [`subset::Subset`].
    pub const fn is_subset(&self) -> bool {
        matches!(self, Node::Subset(_))
    }

    /// Returns `true` if this node is an [`epoch::Epoch`].
    pub const fn is_epoch(&self) -> bool {
        matches!(self, Node::Epoch(_))
    }

    /// Returns `true` if this node is a [`rewards::Rewards`].
    pub const fn is_rewards(&self) -> bool {
        matches!(self, Node::Rewards(_))
    }

    /// Returns `true` if this node is a [`dataframe::DataFrame`].
    pub const fn is_dataframe(&self) -> bool {
        matches!(self, Node::DataFrame(_))
    }

    /// Returns the transaction if this node is [`Node::Transaction`].
    pub const fn get_transaction(&self) -> Option<&transaction::Transaction> {
        match self {
            Node::Transaction(transaction) => Some(transaction),
            _ => None,
        }
    }

    /// Returns the entry if this node is [`Node::Entry`].
    pub const fn get_entry(&self) -> Option<&entry::Entry> {
        match self {
            Node::Entry(entry) => Some(entry),
            _ => None,
        }
    }

    /// Returns the block if this node is [`Node::Block`].
    pub const fn get_block(&self) -> Option<&block::Block> {
        match self {
            Node::Block(block) => Some(block),
            _ => None,
        }
    }

    /// Returns the subset if this node is [`Node::Subset`].
    pub const fn get_subset(&self) -> Option<&subset::Subset> {
        match self {
            Node::Subset(subset) => Some(subset),
            _ => None,
        }
    }

    /// Returns the epoch if this node is [`Node::Epoch`].
    pub const fn get_epoch(&self) -> Option<&epoch::Epoch> {
        match self {
            Node::Epoch(epoch) => Some(epoch),
            _ => None,
        }
    }

    /// Returns the rewards data if this node is [`Node::Rewards`].
    pub const fn get_rewards(&self) -> Option<&rewards::Rewards> {
        match self {
            Node::Rewards(rewards) => Some(rewards),
            _ => None,
        }
    }

    /// Returns the dataframe if this node is [`Node::DataFrame`].
    pub const fn get_dataframe(&self) -> Option<&dataframe::DataFrame> {
        match self {
            Node::DataFrame(dataframe) => Some(dataframe),
            _ => None,
        }
    }
}

// parse_any_from_cbordata parses any CBOR data into either a Epoch, Subset, Block, Rewards, Entry, or Transaction
/// Parses the raw CBOR payload into the appropriate [`Node`] variant.
pub fn parse_any_from_cbordata(data: Vec<u8>) -> Result<Node, SharedError> {
    let decoded_data: serde_cbor::Value = serde_cbor::from_slice(&data)?;
    // Process the decoded data
    // println!("Data: {:?}", decoded_data);
    let cloned_data = decoded_data.clone();

    // decoded_data is an serde_cbor.Array; print the kind, which is the first element of the array
    if let serde_cbor::Value::Array(array) = decoded_data {
        // println!("Kind: {:?}", array[0]);
        if let Some(serde_cbor::Value::Integer(kind)) = array.first() {
            // println!(
            //     "Kind: {:?}",
            //     Kind::from_u64(kind as u64).unwrap().to_string()
            // );

            // based on the kind, we can decode the rest of the data
            let Some(kind) = Kind::from_u64(*kind as u64) else {
                return Err(Box::new(std::io::Error::other(std::format!(
                    "Invalid kind: {:?}",
                    kind
                ))));
            };
            match kind {
                Kind::Transaction => {
                    let transaction = transaction::Transaction::from_cbor(cloned_data)?;
                    return Ok(Node::Transaction(transaction));
                }
                Kind::Entry => {
                    let entry = entry::Entry::from_cbor(cloned_data)?;
                    return Ok(Node::Entry(entry));
                }
                Kind::Block => {
                    let block = block::Block::from_cbor(cloned_data)?;
                    return Ok(Node::Block(block));
                }
                Kind::Subset => {
                    let subset = subset::Subset::from_cbor(cloned_data)?;
                    return Ok(Node::Subset(subset));
                }
                Kind::Epoch => {
                    let epoch = epoch::Epoch::from_cbor(cloned_data)?;
                    return Ok(Node::Epoch(epoch));
                }
                Kind::Rewards => {
                    let rewards = rewards::Rewards::from_cbor(cloned_data)?;
                    return Ok(Node::Rewards(rewards));
                }
                Kind::DataFrame => {
                    let dataframe = dataframe::DataFrame::from_cbor(cloned_data)?;
                    return Ok(Node::DataFrame(dataframe));
                } // unknown => {
                  //     return Err(Box::new(std::io::Error::new(
                  //         std::io::ErrorKind::Other,
                  //         std::format!("Unknown type: {:?}", unknown),
                  //     )))
                  // }
            }
        }
    }

    Err(Box::new(std::io::Error::other("Unknown type".to_owned())))
}

/// Numeric discriminant used in the CBOR encoding of [`Node`] variants.
pub enum Kind {
    /// Transaction node discriminant.
    Transaction,
    /// Entry node discriminant.
    Entry,
    /// Block node discriminant.
    Block,
    /// Subset node discriminant.
    Subset,
    /// Epoch node discriminant.
    Epoch,
    /// Rewards node discriminant.
    Rewards,
    /// Data frame node discriminant.
    DataFrame,
}

impl fmt::Debug for Kind {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_struct("Kind")
            .field("kind", &self.to_string())
            .finish()
    }
}

impl fmt::Display for Kind {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let kind = match self {
            Kind::Transaction => "Transaction",
            Kind::Entry => "Entry",
            Kind::Block => "Block",
            Kind::Subset => "Subset",
            Kind::Epoch => "Epoch",
            Kind::Rewards => "Rewards",
            Kind::DataFrame => "DataFrame",
        };
        write!(f, "{}", kind)
    }
}

impl Kind {
    /// Converts a numeric discriminant into a [`Kind`].
    pub const fn from_u64(kind: u64) -> Option<Kind> {
        match kind {
            0 => Some(Kind::Transaction),
            1 => Some(Kind::Entry),
            2 => Some(Kind::Block),
            3 => Some(Kind::Subset),
            4 => Some(Kind::Epoch),
            5 => Some(Kind::Rewards),
            6 => Some(Kind::DataFrame),
            _ => None,
        }
    }

    /// Returns the numeric discriminant for this [`Kind`].
    pub const fn to_u64(&self) -> u64 {
        match self {
            Kind::Transaction => 0,
            Kind::Entry => 1,
            Kind::Block => 2,
            Kind::Subset => 3,
            Kind::Epoch => 4,
            Kind::Rewards => 5,
            Kind::DataFrame => 6,
        }
    }
}

/// Raw node extracted from an Old Faithful CAR segment.
pub struct RawNode {
    cid: Cid,
    data: Vec<u8>,
}

// Debug trait for RawNode
impl fmt::Debug for RawNode {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_struct("RawNode")
            .field("cid", &self.cid)
            .field("data", &self.data)
            .finish()
    }
}

impl RawNode {
    /// Creates a [`RawNode`] from the provided CID and raw bytes read from Old Faithful.
    pub const fn new(cid: Cid, data: Vec<u8>) -> RawNode {
        RawNode { cid, data }
    }

    /// Parses the node into a typed [`Node`].
    pub fn parse(&self) -> Result<Node, SharedError> {
        let parsed = parse_any_from_cbordata(self.data.clone());
        match parsed {
            Ok(node) => Ok(node),
            Err(_) => Err(Box::new(std::io::Error::other("Unknown type".to_owned()))),
        }
    }

    /// Decodes a [`RawNode`] from an Old Faithful CAR cursor.
    pub fn from_cursor(cursor: &mut io::Cursor<Vec<u8>>) -> Result<RawNode, SharedError> {
        let cid_version = utils::read_uvarint(cursor)?;
        // println!("CID version: {}", cid_version);

        let multicodec = utils::read_uvarint(cursor)?;
        // println!("Multicodec: {}", multicodec);

        // Multihash hash function code.
        let hash_function = utils::read_uvarint(cursor)?;
        // println!("Hash function: {}", hash_function);

        // Multihash digest length.
        let digest_length = utils::read_uvarint(cursor)?;
        // println!("Digest length: {}", digest_length);

        if digest_length > 64 {
            return Err(Box::new(std::io::Error::other(
                "Digest length too long".to_owned(),
            )));
        }

        // reac actual digest
        let mut digest = vec![0u8; digest_length as usize];
        cursor.read_exact(&mut digest)?;

        // the rest is the data
        let mut data = vec![];
        cursor.read_to_end(&mut data)?;

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

/// Old Faithful CAR reader that produces [`RawNode`] values from a synchronous source.
pub struct NodeReader<R: Read> {
    /// Underlying reader yielding Old Faithful CAR bytes.
    reader: R,
    /// Cached Old Faithful CAR header.
    header: Vec<u8>,
    /// Number of Old Faithful items that have been read.
    item_index: u64,
}

impl<R: Read> NodeReader<R> {
    /// Creates a new [`NodeReader`] around a blocking reader.
    pub fn new(reader: R) -> Result<NodeReader<R>, SharedError> {
        let node_reader = NodeReader {
            reader,
            header: vec![],
            item_index: 0,
        };
        Ok(node_reader)
    }

    /// Returns the raw Old Faithful CAR header, caching it for subsequent calls.
    pub fn read_raw_header(&mut self) -> Result<Vec<u8>, SharedError> {
        if !self.header.is_empty() {
            return Ok(self.header.clone());
        };
        let header_length = utils::read_uvarint(&mut self.reader)?;
        if header_length > 1024 {
            return Err(Box::new(std::io::Error::other(
                "Header length too long".to_owned(),
            )));
        }
        let mut header = vec![0u8; header_length as usize];
        self.reader.read_exact(&mut header)?;

        self.header.clone_from(&header);

        let clone = header.clone();
        Ok(clone.as_slice().to_owned())
    }

    #[allow(clippy::should_implement_trait)]
    /// Reads the next [`RawNode`] without parsing it from Old Faithful data.
    pub fn next(&mut self) -> Result<RawNode, SharedError> {
        if self.header.is_empty() {
            self.read_raw_header()?;
        };

        // println!("Item index: {}", item_index);
        self.item_index += 1;

        // Read and decode the uvarint prefix (length of CID + data)
        let section_size = utils::read_uvarint(&mut self.reader)?;
        // println!("Section size: {}", section_size);

        if section_size > utils::MAX_ALLOWED_SECTION_SIZE as u64 {
            return Err(Box::new(std::io::Error::other(
                "Section size too long".to_owned(),
            )));
        }

        // read whole item
        let mut item = vec![0u8; section_size as usize];
        self.reader.read_exact(&mut item)?;

        // dump item bytes as numbers
        // println!("Item bytes: {:?}", item);

        // now create a cursor over the item
        let mut cursor = io::Cursor::new(item);

        RawNode::from_cursor(&mut cursor)
    }

    /// Reads and parses the next node, returning it with its [`Cid`].
    pub fn next_parsed(&mut self) -> Result<NodeWithCid, SharedError> {
        let raw_node = self.next()?;
        let cid = raw_node.cid;
        Ok(NodeWithCid::new(cid, raw_node.parse()?))
    }

    /// Iterates Old Faithful nodes until a block is encountered, returning the collected list.
    pub fn read_until_block(&mut self) -> Result<NodesWithCids, SharedError> {
        let mut nodes = NodesWithCids::new();
        loop {
            let node = self.next_parsed()?;
            if node.get_node().is_block() {
                nodes.push(node);
                break;
            }
            nodes.push(node);
        }
        Ok(nodes)
    }

    /// Returns the number of CAR items read so far.
    pub const fn get_item_index(&self) -> u64 {
        self.item_index
    }
}
