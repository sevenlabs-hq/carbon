use carbon_core::{borsh, CarbonDeserialize};

#[derive(
    CarbonDeserialize, Debug, serde::Serialize, serde::Deserialize, PartialEq, Eq, Clone, Hash,
)]
pub enum PluginType {
    Royalties,
    FreezeDelegate,
    BurnDelegate,
    TransferDelegate,
    UpdateDelegate,
    PermanentFreezeDelegate,
    Attributes,
    PermanentTransferDelegate,
    PermanentBurnDelegate,
    Edition,
    MasterEdition,
}
