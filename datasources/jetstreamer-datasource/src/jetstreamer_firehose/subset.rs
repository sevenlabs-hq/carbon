use {
    crate::jetstreamer_firehose::{SharedError, node::Kind, node_reader::cid_from_cbor_link},
    cid::Cid,
    std::vec::Vec,
};

// type Subset struct {
// 	Kind   int
// 	First  int
// 	Last   int
// 	Blocks List__Link
// }
/// Representation of a `Kind::Subset` node referencing a range of blocks.
#[derive(Clone, PartialEq, Eq, Hash, Debug)]
pub struct Subset {
    /// Kind discriminator copied from the CBOR payload.
    pub kind: u64,
    /// First slot included in the subset.
    pub first: u64,
    /// Last slot included in the subset.
    pub last: u64,
    /// Block CIDs covered by this subset.
    pub blocks: Vec<Cid>,
}

impl Subset {
    /// Decodes a [`Subset`] from raw CBOR bytes.
    pub fn from_bytes(data: Vec<u8>) -> Result<Subset, SharedError> {
        let decoded_data: serde_cbor::Value = serde_cbor::from_slice(&data).unwrap();
        let subset = Subset::from_cbor(decoded_data)?;
        Ok(subset)
    }

    /// Decodes a [`Subset`] from a CBOR [`serde_cbor::Value`].
    pub fn from_cbor(val: serde_cbor::Value) -> Result<Subset, SharedError> {
        let mut subset = Subset {
            kind: 0,
            first: 0,
            last: 0,
            blocks: vec![],
        };

        if let serde_cbor::Value::Array(array) = val {
            // println!("Kind: {:?}", array[0]);
            if let Some(serde_cbor::Value::Integer(kind)) = array.first() {
                // println!("Kind: {:?}", Kind::from_u64(kind as u64).unwrap().to_string());
                subset.kind = *kind as u64;

                if *kind as u64 != Kind::Subset as u64 {
                    return Err(Box::new(std::io::Error::other(std::format!(
                        "Wrong kind for Subset. Expected {:?}, got {:?}",
                        Kind::Subset,
                        kind
                    ))) as SharedError);
                }
            }
            if let Some(serde_cbor::Value::Integer(first)) = array.get(1) {
                subset.first = *first as u64;
            }
            if let Some(serde_cbor::Value::Integer(last)) = array.get(2) {
                subset.last = *last as u64;
            }

            if let Some(serde_cbor::Value::Array(blocks)) = &array.get(3) {
                for block in blocks {
                    subset.blocks.push(cid_from_cbor_link(block)?);
                }
            }
        }
        Ok(subset)
    }

    /// Renders the subset as a JSON object for debugging.
    pub fn to_json(&self) -> serde_json::Value {
        let mut blocks = vec![];
        for block in &self.blocks {
            blocks.push(serde_json::json!({
                "/": block.to_string()
            }));
        }

        let mut map = serde_json::Map::new();
        map.insert("kind".to_string(), serde_json::Value::from(self.kind));
        map.insert("first".to_string(), serde_json::Value::from(self.first));
        map.insert("last".to_string(), serde_json::Value::from(self.last));
        map.insert("blocks".to_string(), serde_json::Value::from(blocks));

        serde_json::Value::from(map)
    }
}
