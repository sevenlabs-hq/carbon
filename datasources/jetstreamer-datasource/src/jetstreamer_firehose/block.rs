use {
    crate::jetstreamer_firehose::{SharedError, node::Kind, node_reader::cid_from_cbor_link},
    base64::engine::{Engine, general_purpose::STANDARD},
    cid::Cid,
    std::vec::Vec,
};

// type (
// 	List__Shredding []Shredding
// 	Block           struct {
// 		Kind      int
// 		Slot      int
// 		Shredding List__Shredding
// 		Entries   List__Link
// 		Meta      SlotMeta
// 		Rewards   datamodel.Link
// 	}
// )
/// Representation of a `Kind::Block` node, including shredding metadata.
#[derive(Clone, PartialEq, Eq, Hash, Debug)]
pub struct Block {
    /// Kind discriminator copied from the CBOR payload.
    pub kind: u64,
    /// Slot number for the block.
    pub slot: u64,
    /// Shredding metadata describing entry and shred indexes.
    pub shredding: Vec<Shredding>,
    /// Transaction entry CIDs referenced by the block.
    pub entries: Vec<Cid>,
    /// Associated [`SlotMeta`] describing ancestry and block timing.
    pub meta: SlotMeta,
    /// CID pointing to the block rewards node.
    pub rewards: Cid,
}

impl Block {
    /// Decodes a [`Block`] from raw CBOR bytes.
    pub fn from_bytes(data: Vec<u8>) -> Result<Block, SharedError> {
        let decoded_data: serde_cbor::Value = serde_cbor::from_slice(&data).unwrap();
        let block = Block::from_cbor(decoded_data)?;
        Ok(block)
    }

    /// Decodes a [`Block`] from a CBOR [`serde_cbor::Value`].
    pub fn from_cbor(val: serde_cbor::Value) -> Result<Block, SharedError> {
        let mut block = Block {
            kind: 0,
            slot: 0,
            shredding: vec![],
            entries: vec![],
            meta: SlotMeta {
                parent_slot: 0,
                blocktime: 0,
                block_height: None,
                block_markers: None,
                block_id: None,
            },
            rewards: Cid::try_from(
                vec![
                    1, 113, 18, 32, 56, 148, 167, 251, 237, 117, 200, 226, 181, 134, 79, 115, 131,
                    220, 232, 143, 20, 67, 224, 179, 48, 130, 197, 123, 226, 85, 85, 56, 38, 84,
                    106, 225,
                ]
                .as_slice(),
            )
            .unwrap(),
        };

        if let serde_cbor::Value::Array(array) = val {
            // println!("Kind: {:?}", array[0]);
            if let Some(serde_cbor::Value::Integer(kind)) = array.first() {
                // println!("Kind: {:?}", Kind::from_u64(kind as u64).unwrap().to_string());
                block.kind = *kind as u64;

                if *kind as u64 != Kind::Block as u64 {
                    return Err(Box::new(std::io::Error::other(std::format!(
                        "Wrong kind for Block. Expected {:?}, got {:?}",
                        Kind::Block,
                        kind
                    ))) as SharedError);
                }
            }
            if let Some(serde_cbor::Value::Integer(slot)) = array.get(1) {
                block.slot = *slot as u64;
            }

            if let Some(serde_cbor::Value::Array(shredding)) = &array.get(2) {
                for shred in shredding {
                    if let serde_cbor::Value::Array(shred) = shred {
                        block
                            .shredding
                            .push(Shredding::from_cbor(serde_cbor::Value::Array(
                                shred.clone(),
                            )));
                    }
                }
            }

            if let Some(serde_cbor::Value::Array(entries)) = &array.get(3) {
                for entry in entries {
                    block.entries.push(cid_from_cbor_link(entry)?);
                }
            }

            if let Some(serde_cbor::Value::Array(meta)) = &array.get(4) {
                block.meta = SlotMeta::from_cbor(serde_cbor::Value::Array(meta.clone()));
            }

            if let Some(rewards @ serde_cbor::Value::Bytes(_)) = array.get(5) {
                block.rewards = cid_from_cbor_link(rewards)?;
            }
        }
        Ok(block)
    }

    /// Renders the block as a JSON object for debugging.
    pub fn to_json(&self) -> serde_json::Value {
        let mut shredding = vec![];
        for shred in &self.shredding {
            shredding.push(shred.to_json());
        }

        let mut entries = vec![];
        for entry in &self.entries {
            entries.push(serde_json::json!({
                "/": entry.to_string()
            }));
        }

        let mut map = serde_json::Map::new();
        map.insert("kind".to_string(), serde_json::Value::from(self.kind));
        map.insert("slot".to_string(), serde_json::Value::from(self.slot));
        map.insert("shredding".to_string(), serde_json::Value::from(shredding));
        map.insert("entries".to_string(), serde_json::Value::from(entries));
        map.insert("meta".to_string(), self.meta.to_json());
        map.insert(
            "rewards".to_string(),
            serde_json::json!({
                "/": self.rewards.to_string()
            }),
        );

        serde_json::Value::from(map)
    }
}

// type Shredding struct {
// 	EntryEndIdx int
// 	ShredEndIdx int
// }
/// Entry and shred index metadata attached to a block.
#[derive(Clone, PartialEq, Eq, Hash, Debug)]
pub struct Shredding {
    /// Index of the final entry contained in the shred.
    pub entry_end_idx: i64,
    /// Index of the final shred written for the entry.
    pub shred_end_idx: i64,
}

impl Shredding {
    /// Decodes [`Shredding`] metadata from a CBOR array.
    pub fn from_cbor(val: serde_cbor::Value) -> Shredding {
        let mut shredding = Shredding {
            entry_end_idx: 0,
            shred_end_idx: 0,
        };

        if let serde_cbor::Value::Array(array) = val {
            // println!("Kind: {:?}", array[0]);
            if let Some(serde_cbor::Value::Integer(entry_end_idx)) = array.first() {
                // println!("Kind: {:?}", Kind::from_u64(kind as u64).unwrap().to_string());
                shredding.entry_end_idx = *entry_end_idx as i64;
            }
            if let Some(serde_cbor::Value::Integer(shred_end_idx)) = array.get(1) {
                shredding.shred_end_idx = *shred_end_idx as i64;
            }
        }
        shredding
    }

    /// Serializes the shredding metadata into JSON for debugging.
    pub fn to_json(&self) -> serde_json::Value {
        let mut map = serde_json::Map::new();
        map.insert(
            "entry_end_idx".to_string(),
            serde_json::Value::from(self.entry_end_idx),
        );
        map.insert(
            "shred_end_idx".to_string(),
            serde_json::Value::from(self.shred_end_idx),
        );

        serde_json::Value::from(map)
    }
}

// type SlotMeta struct {
// 	Parent_slot   int
// 	Blocktime     int
// 	Block_height  **int
// 	Block_markers **[][]byte
// 	Block_id      **[]byte
// }
/// Slot-level metadata associated with a block.
#[derive(Clone, PartialEq, Eq, Hash, Debug)]
pub struct SlotMeta {
    /// Parent slot that produced this block.
    pub parent_slot: u64,
    /// Unix timestamp reported for the block.
    pub blocktime: u64,
    /// Optional block height.
    pub block_height: Option<u64>,
    /// Raw Alpenglow block markers (header, footer and others) in block order. Each is a
    /// little-endian `u16 version | u8 variant | u16 len | payload`, which
    /// `solana_entry::block_component::VersionedBlockMarker` decodes. `None` before Alpenglow.
    pub block_markers: Option<Vec<Vec<u8>>>,
    /// Alpenglow block id, the double merkle root that certificates vote on. `None` before
    /// Alpenglow.
    pub block_id: Option<[u8; 32]>,
}

impl SlotMeta {
    /// Decodes [`SlotMeta`] from a CBOR array.
    pub fn from_cbor(val: serde_cbor::Value) -> SlotMeta {
        let mut slot_meta = SlotMeta {
            parent_slot: 0,
            blocktime: 0,
            block_height: None,
            block_markers: None,
            block_id: None,
        };

        if let serde_cbor::Value::Array(array) = val {
            // println!("Kind: {:?}", array[0]);
            if let Some(serde_cbor::Value::Integer(parent_slot)) = array.first() {
                // println!("Kind: {:?}", Kind::from_u64(kind as u64).unwrap().to_string());
                slot_meta.parent_slot = *parent_slot as u64;
            }
            if let Some(serde_cbor::Value::Integer(blocktime)) = array.get(1) {
                slot_meta.blocktime = *blocktime as u64;
            }
            if let Some(serde_cbor::Value::Integer(block_height)) = array.get(2) {
                slot_meta.block_height = Some(*block_height as u64);
            }
            // Old Faithful fails the whole block on a malformed field. This stays lenient like the
            // fields above.
            if let Some(serde_cbor::Value::Array(markers)) = array.get(3) {
                slot_meta.block_markers = markers
                    .iter()
                    .map(|marker| match marker {
                        serde_cbor::Value::Bytes(bytes) => Some(bytes.clone()),
                        _ => None,
                    })
                    .collect();
            }
            if let Some(serde_cbor::Value::Bytes(block_id)) = array.get(4) {
                slot_meta.block_id = block_id.as_slice().try_into().ok();
            }
        }
        slot_meta
    }

    /// Renders the slot metadata as JSON for debugging.
    pub fn to_json(&self) -> serde_json::Value {
        let mut map = serde_json::Map::new();
        map.insert(
            "parent_slot".to_string(),
            serde_json::Value::from(self.parent_slot),
        );
        map.insert(
            "blocktime".to_string(),
            serde_json::Value::from(self.blocktime),
        );
        if self.block_height.is_none() {
            map.insert("block_height".to_string(), serde_json::Value::Null);
        } else {
            map.insert(
                "block_height".to_string(),
                serde_json::Value::from(self.block_height),
            );
        }
        if let Some(markers) = &self.block_markers {
            let markers = markers.iter().map(|marker| STANDARD.encode(marker));
            map.insert(
                "block_markers".to_string(),
                serde_json::Value::from_iter(markers),
            );
        }
        if let Some(block_id) = &self.block_id {
            map.insert(
                "block_id".to_string(),
                serde_json::Value::from(STANDARD.encode(block_id)),
            );
        }

        serde_json::Value::from(map)
    }
}
