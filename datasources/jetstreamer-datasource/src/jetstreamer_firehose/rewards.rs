use {
    crate::jetstreamer_firehose::{SharedError, dataframe, node::Kind, utils::Buffer},
    std::vec::Vec,
};

// type Rewards struct {
// 	Kind int
// 	Slot int
// 	Data DataFrame
// }
/// Representation of a `Kind::Rewards` node.
#[derive(Clone, PartialEq, Eq, Hash, Debug)]
pub struct Rewards {
    /// Kind discriminator copied from the CBOR payload.
    pub kind: u64,
    /// Slot associated with the reward data.
    pub slot: u64,
    /// Reward payload encoded as a [`dataframe::DataFrame`].
    pub data: dataframe::DataFrame,
}

impl Rewards {
    /// Decodes [`Rewards`] from raw CBOR bytes.
    pub fn from_bytes(data: Vec<u8>) -> Result<Rewards, SharedError> {
        let decoded_data: serde_cbor::Value = serde_cbor::from_slice(&data).unwrap();
        let rewards = Rewards::from_cbor(decoded_data)?;
        Ok(rewards)
    }

    /// Decodes [`Rewards`] from a CBOR [`serde_cbor::Value`].
    pub fn from_cbor(val: serde_cbor::Value) -> Result<Rewards, SharedError> {
        let mut rewards = Rewards {
            kind: 0,
            slot: 0,
            data: dataframe::DataFrame {
                kind: 0,
                hash: None,
                index: None,
                total: None,
                data: Buffer::new(),
                next: None,
            },
        };

        if let serde_cbor::Value::Array(array) = val {
            // println!("Kind: {:?}", array[0]);
            if let Some(serde_cbor::Value::Integer(kind)) = array.first() {
                // println!("Kind: {:?}", Kind::from_u64(kind as u64).unwrap().to_string());
                rewards.kind = *kind as u64;

                if *kind as u64 != Kind::Rewards as u64 {
                    return Err(Box::new(std::io::Error::other(std::format!(
                        "Wrong kind for Rewards. Expected {:?}, got {:?}",
                        Kind::Rewards,
                        kind
                    ))) as SharedError);
                }
            }
            if let Some(serde_cbor::Value::Integer(slot)) = array.get(1) {
                rewards.slot = *slot as u64;
            }

            if let Some(serde_cbor::Value::Array(data)) = &array.get(2) {
                rewards.data =
                    dataframe::DataFrame::from_cbor(serde_cbor::Value::Array(data.clone()))?;
            }
        }
        Ok(rewards)
    }

    /// Renders the rewards data as a JSON object for debugging.
    pub fn to_json(&self) -> serde_json::Value {
        let mut map = serde_json::Map::new();
        map.insert("kind".to_string(), serde_json::Value::from(self.kind));
        map.insert("slot".to_string(), serde_json::Value::from(self.slot));
        map.insert("data".to_string(), self.data.to_json());

        serde_json::Value::from(map)
    }

    /// Returns `true` when the rewards data frame has no continuation CIDs.
    pub const fn is_complete(&self) -> bool {
        self.data.next.is_none() || self.data.next.as_ref().unwrap().is_empty()
    }
}
