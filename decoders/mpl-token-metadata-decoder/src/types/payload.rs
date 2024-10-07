use super::*;
use carbon_core::borsh;
use carbon_core::deserialize::CarbonDeserialize;
use carbon_proc_macros::CarbonDeserialize;
use std::{
    collections::HashMap,
    hash::{Hash, Hasher},
};

#[derive(CarbonDeserialize, Debug, serde::Serialize, serde::Deserialize, PartialEq, Eq, Clone)]
pub struct Payload {
    pub map: HashMap<String, PayloadType>,
}

impl Hash for Payload {
    fn hash<H: Hasher>(&self, state: &mut H) {
        for (key, value) in &self.map {
            key.hash(state);
            value.hash(state);
        }
    }
}
