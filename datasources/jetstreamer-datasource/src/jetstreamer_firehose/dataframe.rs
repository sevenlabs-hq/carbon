use {
    crate::jetstreamer_firehose::{SharedError, node::Kind, node_reader::cid_from_cbor_link, utils::Buffer},
    cid::Cid,
    std::vec::Vec,
};

// type DataFrame struct {
// 	Kind  int
// 	Hash  **int
// 	Index **int
// 	Total **int
// 	Data  []uint8
// 	Next  **List__Link
// }
/// Representation of a `Kind::DataFrame` node from the firehose.
#[derive(Clone, PartialEq, Eq, Hash, Debug)]
pub struct DataFrame {
    /// Kind discriminator copied from the CBOR payload.
    pub kind: u64,
    /// Optional rolling hash for the data chunk.
    pub hash: Option<u64>,
    /// Optional chunk index within the full payload.
    pub index: Option<u64>,
    /// Optional total number of chunks.
    pub total: Option<u64>,
    /// Raw bytes contained in this chunk.
    pub data: Buffer,
    /// Optional list of CIDs pointing to continuation chunks.
    pub next: Option<Vec<Cid>>,
}

impl DataFrame {
    /// Decodes a [`DataFrame`] from raw CBOR bytes.
    pub fn from_bytes(data: Vec<u8>) -> Result<DataFrame, SharedError> {
        let decoded_data: serde_cbor::Value = serde_cbor::from_slice(&data).unwrap();
        let data_frame = DataFrame::from_cbor(decoded_data)?;
        Ok(data_frame)
    }

    /// Decodes a [`DataFrame`] from a parsed CBOR [`serde_cbor::Value`].
    pub fn from_cbor(val: serde_cbor::Value) -> Result<DataFrame, SharedError> {
        let mut data_frame = DataFrame {
            kind: 0,
            hash: None,
            index: None,
            total: None,
            data: Buffer::new(),
            next: None,
        };

        if let serde_cbor::Value::Array(array) = val {
            // println!("Kind: {:?}", array[0]);
            if let Some(serde_cbor::Value::Integer(kind)) = array.first() {
                // println!("Kind: {:?}", Kind::from_u64(kind as u64).unwrap().to_string());
                data_frame.kind = *kind as u64;

                if *kind as u64 != Kind::DataFrame as u64 {
                    return Err(Box::new(std::io::Error::other(std::format!(
                        "Wrong kind for DataFrame. Expected {:?}, got {:?}",
                        Kind::DataFrame,
                        kind
                    ))) as SharedError);
                }
            }
            if let Some(serde_cbor::Value::Integer(hash)) = array.get(1) {
                data_frame.hash = Some(*hash as u64);
            }
            if let Some(serde_cbor::Value::Integer(index)) = array.get(2) {
                data_frame.index = Some(*index as u64);
            }
            if let Some(serde_cbor::Value::Integer(total)) = array.get(3) {
                data_frame.total = Some(*total as u64);
            }
            if let Some(serde_cbor::Value::Bytes(data)) = &array.get(4) {
                data_frame.data = Buffer::from_vec(data.clone());
            }

            if array.len() > 5
                && let Some(serde_cbor::Value::Array(next)) = &array.get(5)
            {
                if next.is_empty() {
                    data_frame.next = None;
                } else {
                    let mut nexts = vec![];
                    for cid in next {
                        nexts.push(cid_from_cbor_link(cid)?);
                    }
                    data_frame.next = Some(nexts);
                }
            }
        }
        Ok(data_frame)
    }

    /// Renders the dataframe as a JSON value for debugging.
    pub fn to_json(&self) -> serde_json::Value {
        let mut next = vec![];
        if let Some(nexts) = &self.next {
            for cid in nexts {
                next.push(serde_json::json!({
                    "/": cid.to_string()
                }));
            }
        }

        let mut map = serde_json::Map::new();
        map.insert("kind".to_string(), serde_json::Value::from(self.kind));
        if self.hash.is_none() {
            map.insert("hash".to_string(), serde_json::Value::Null);
        } else {
            let hash_as_string = self.hash.unwrap().to_string();
            map.insert("hash".to_string(), serde_json::Value::from(hash_as_string));
        }
        if self.index.is_none() {
            map.insert("index".to_string(), serde_json::Value::Null);
        } else {
            map.insert("index".to_string(), serde_json::Value::from(self.index));
        }
        if self.total.is_none() {
            map.insert("total".to_string(), serde_json::Value::Null);
        } else {
            map.insert("total".to_string(), serde_json::Value::from(self.total));
        }
        map.insert(
            "data".to_string(),
            serde_json::Value::from(self.data.to_string()),
        );
        if next.is_empty() {
            map.insert("next".to_string(), serde_json::Value::Null);
        } else {
            map.insert("next".to_string(), serde_json::Value::from(next));
        }

        serde_json::Value::from(map)
    }
}
