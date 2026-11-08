use {
    crate::jetstreamer_firehose::{SharedError, dataframe::DataFrame, node::Kind, utils::Buffer},
    std::vec::Vec,
};

// type Transaction struct {
// 	Kind     int
// 	Data     DataFrame
// 	Metadata DataFrame
// 	Slot     int
// 	Index    **int
// }
/// Representation of a `Kind::Transaction` node containing the raw wire data.
#[derive(Clone, PartialEq, Eq, Hash, Debug)]
pub struct Transaction {
    /// Kind discriminator copied from the CBOR payload.
    pub kind: u64,
    /// Binary transaction payload stored as a [`DataFrame`].
    pub data: DataFrame,
    /// Associated metadata payload stored as a [`DataFrame`].
    pub metadata: DataFrame,
    /// Slot that produced the transaction.
    pub slot: u64,
    /// Optional within-slot transaction index.
    pub index: Option<u64>,
}

impl Transaction {
    /// Decodes a [`Transaction`] from raw CBOR bytes.
    pub fn from_bytes(data: Vec<u8>) -> Result<Transaction, SharedError> {
        let decoded_data: serde_cbor::Value = serde_cbor::from_slice(&data).unwrap();
        let transaction = Transaction::from_cbor(decoded_data)?;
        Ok(transaction)
    }

    /// Decodes a [`Transaction`] from a CBOR [`serde_cbor::Value`].
    pub fn from_cbor(val: serde_cbor::Value) -> Result<Transaction, SharedError> {
        let mut transaction = Transaction {
            kind: 0,
            data: DataFrame {
                kind: 0,
                hash: None,
                index: None,
                total: None,
                data: Buffer::new(),
                next: None,
            },
            metadata: DataFrame {
                kind: 0,
                hash: None,
                index: None,
                total: None,
                data: Buffer::new(),
                next: None,
            },
            slot: 0,
            index: None,
        };

        if let serde_cbor::Value::Array(array) = val {
            // println!("Kind: {:?}", array[0]);
            if let Some(serde_cbor::Value::Integer(kind)) = array.first() {
                // println!("Kind: {:?}", Kind::from_u64(kind as u64).unwrap().to_string());
                transaction.kind = *kind as u64;

                if *kind as u64 != Kind::Transaction as u64 {
                    return Err(Box::new(std::io::Error::other(std::format!(
                        "Wrong kind for Transaction; expected {:?}, got {:?}",
                        Kind::Transaction,
                        kind
                    ))) as SharedError);
                }
            }

            if let Some(serde_cbor::Value::Array(data)) = &array.get(1) {
                transaction.data = DataFrame::from_cbor(serde_cbor::Value::Array(data.clone()))?;
            }

            if let Some(serde_cbor::Value::Array(metadata)) = &array.get(2) {
                transaction.metadata =
                    DataFrame::from_cbor(serde_cbor::Value::Array(metadata.clone()))?;
            }

            if let Some(serde_cbor::Value::Integer(slot)) = array.get(3) {
                transaction.slot = *slot as u64;
            }

            if let Some(serde_cbor::Value::Integer(index)) = array.get(4) {
                transaction.index = Some(*index as u64);
            }
        }
        Ok(transaction)
    }

    /// Renders the transaction as a JSON object for debugging.
    pub fn to_json(&self) -> serde_json::Value {
        let mut map = serde_json::Map::new();
        map.insert("kind".to_string(), serde_json::Value::from(self.kind));
        map.insert("data".to_string(), self.data.to_json());
        map.insert("metadata".to_string(), self.metadata.to_json());
        map.insert("slot".to_string(), serde_json::Value::from(self.slot));
        map.insert("index".to_string(), serde_json::Value::from(self.index));

        serde_json::Value::from(map)
    }

    /// Deserializes the transaction payload into a [`solana_transaction::versioned::VersionedTransaction`].
    ///
    /// Uses the SDK's own wincode schema, which covers legacy, v0 and v1 message formats.
    pub fn as_parsed(
        &self,
    ) -> Result<solana_transaction::versioned::VersionedTransaction, SharedError> {
        Ok(wincode::deserialize(self.data.data.as_slice())?)
    }

    /// Returns `true` when the transaction data frame has no continuation CIDs.
    pub const fn is_complete_data(&self) -> bool {
        self.data.next.is_none() || self.data.next.as_ref().unwrap().is_empty()
    }
    /// Returns `true` when the transaction metadata frame has no continuation CIDs.
    pub const fn is_complete_metadata(&self) -> bool {
        self.metadata.next.is_none() || self.metadata.next.as_ref().unwrap().is_empty()
    }
}
