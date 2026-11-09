use {
    crate::jetstreamer_firehose::{SharedError, node::Kind, node_reader::cid_from_cbor_link},
    cid::Cid,
    std::vec::Vec,
};
// type (
// 	Epoch      struct {
// 		Kind    int
// 		Epoch   int
// 		Subsets List__Link
// 	}
// )
/// Representation of a `Kind::Epoch` node pointing to subset indexes.
#[derive(Clone, PartialEq, Eq, Hash, Debug)]
pub struct Epoch {
    /// Kind discriminator copied from the CBOR payload.
    pub kind: u64,
    /// Epoch number encoded in the payload.
    pub epoch: u64,
    /// Subset CIDs that compose the epoch.
    pub subsets: Vec<Cid>,
}

impl Epoch {
    /// Decodes an [`Epoch`] from raw CBOR bytes.
    pub fn from_bytes(data: Vec<u8>) -> Result<Epoch, SharedError> {
        let decoded_data: serde_cbor::Value = serde_cbor::from_slice(&data).unwrap();
        let epoch = Epoch::from_cbor(decoded_data)?;
        Ok(epoch)
    }

    /// Decodes an [`Epoch`] from a CBOR [`serde_cbor::Value`].
    pub fn from_cbor(val: serde_cbor::Value) -> Result<Epoch, SharedError> {
        let mut epoch = Epoch {
            kind: 0,
            epoch: 0,
            subsets: vec![],
        };

        if let serde_cbor::Value::Array(array) = val {
            // println!("Kind: {:?}", array[0]);
            if let Some(serde_cbor::Value::Integer(kind)) = array.first() {
                // println!("Kind: {:?}", Kind::from_u64(kind as u64).unwrap().to_string());
                epoch.kind = *kind as u64;

                if *kind as u64 != Kind::Epoch as u64 {
                    return Err(Box::new(std::io::Error::other(std::format!(
                        "Wrong kind for Epoch. Expected {:?}, got {:?}",
                        Kind::Epoch,
                        kind
                    ))) as SharedError);
                }
            }
            if let Some(serde_cbor::Value::Integer(num)) = array.get(1) {
                epoch.epoch = *num as u64;
            }

            if let Some(serde_cbor::Value::Array(subsets)) = &array.get(2) {
                for subset in subsets {
                    epoch.subsets.push(cid_from_cbor_link(subset)?);
                }
            }
        }
        Ok(epoch)
    }

    /// Renders the epoch as a JSON object for debugging.
    pub fn to_json(&self) -> serde_json::Value {
        let mut subsets = vec![];
        for subset in &self.subsets {
            subsets.push(serde_json::json!({
                "/": subset.to_string()
            }));
        }

        let mut map = serde_json::Map::new();
        map.insert("kind".to_string(), serde_json::Value::from(self.kind));
        map.insert("epoch".to_string(), serde_json::Value::from(self.epoch));
        map.insert("subsets".to_string(), serde_json::Value::from(subsets));

        serde_json::Value::from(map)
    }
}
