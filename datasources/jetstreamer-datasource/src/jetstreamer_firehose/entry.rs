use {
    crate::jetstreamer_firehose::{
        SharedError,
        node::Kind,
        node_reader::cid_from_cbor_link,
        utils::{self, Hash},
    },
    cid::Cid,
    std::vec::Vec,
};

// type Entry struct {
// 	Kind         int
// 	NumHashes    int
// 	Hash         []uint8
// 	Transactions List__Link
// }
/// Representation of a `Kind::Entry` node emitted by the firehose.
#[derive(Clone, PartialEq, Eq, Hash, Debug)]
pub struct Entry {
    /// Kind discriminator copied from the CBOR payload.
    pub kind: u64,
    /// Number of hashes stored in the entry.
    pub num_hashes: u64,
    /// Entry hash encoded as bytes.
    pub hash: Hash,
    /// Transactions referenced by this entry.
    pub transactions: Vec<Cid>,
}

impl Entry {
    /// Decodes an [`Entry`] from raw CBOR bytes.
    pub fn from_bytes(data: Vec<u8>) -> Result<Entry, SharedError> {
        let decoded_data: serde_cbor::Value = serde_cbor::from_slice(&data).unwrap();
        let entry = Entry::from_cbor(decoded_data)?;
        Ok(entry)
    }

    /// Decodes an [`Entry`] from a CBOR [`serde_cbor::Value`].
    pub fn from_cbor(val: serde_cbor::Value) -> Result<Entry, SharedError> {
        let mut entry = Entry {
            kind: 0,
            num_hashes: 0,
            hash: utils::Hash(vec![]),
            transactions: vec![],
        };

        if let serde_cbor::Value::Array(array) = val {
            // println!("Kind: {:?}", array[0]);
            if let Some(serde_cbor::Value::Integer(kind)) = array.first() {
                // println!("Kind: {:?}", Kind::from_u64(kind as u64).unwrap().to_string());
                entry.kind = *kind as u64;

                if *kind as u64 != Kind::Entry as u64 {
                    return Err(Box::new(std::io::Error::other(std::format!(
                        "Wrong kind for Entry. Expected {:?}, got {:?}",
                        Kind::Entry,
                        kind
                    ))) as SharedError);
                }
            }
            if let Some(serde_cbor::Value::Integer(num_hashes)) = array.get(1) {
                entry.num_hashes = *num_hashes as u64;
            }
            if let Some(serde_cbor::Value::Bytes(hash)) = &array.get(2) {
                entry.hash = Hash(hash.to_vec());
            }

            if let Some(serde_cbor::Value::Array(transactions)) = &array.get(3) {
                for transaction in transactions {
                    entry.transactions.push(cid_from_cbor_link(transaction)?);
                }
            }
        }
        Ok(entry)
    }

    /// Renders the entry as a JSON object for debugging.
    pub fn to_json(&self) -> serde_json::Value {
        let mut transactions = vec![];
        for transaction in &self.transactions {
            transactions.push(serde_json::json!({
                "/": transaction.to_string()
            }));
        }

        let mut map = serde_json::Map::new();
        map.insert("kind".to_string(), serde_json::Value::from(self.kind));
        map.insert(
            "num_hashes".to_string(),
            serde_json::Value::from(self.num_hashes),
        );
        map.insert(
            "hash".to_string(),
            serde_json::Value::from(self.hash.clone().to_string()),
        );
        if self.transactions.is_empty() {
            map.insert("transactions".to_string(), serde_json::Value::Null);
        } else {
            map.insert(
                "transactions".to_string(),
                serde_json::Value::from(transactions),
            );
        }

        serde_json::Value::from(map)
    }
}
