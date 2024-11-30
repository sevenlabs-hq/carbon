

use carbon_core::{CarbonDeserialize, borsh};


#[derive(CarbonDeserialize, Debug, serde::Serialize, serde::Deserialize, PartialEq, Eq, Clone, Hash)]
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
    AddBlocker,
    ImmutableMetadata,
    VerifiedCreators,
    Autograph,
}


