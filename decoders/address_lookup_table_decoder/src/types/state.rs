use {
    super::LookupTableMeta,
    serde::{Deserialize, Serialize},
};

#[derive(Deserialize, Serialize, Debug, PartialEq, Eq, Clone)]
pub enum ProgramState {
    Uninitialized,
    LookupTable(LookupTableMeta),
}
