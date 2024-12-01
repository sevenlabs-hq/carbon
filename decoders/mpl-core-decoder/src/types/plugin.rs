
use super::*;

use carbon_core::{CarbonDeserialize, borsh};


#[derive(CarbonDeserialize, Debug, serde::Serialize, serde::Deserialize, PartialEq, Eq, Clone, Hash)]
pub enum Plugin {
    Royalties
                (
                    Royalties,
                )
    ,
    FreezeDelegate
                (
                    FreezeDelegate,
                )
    ,
    BurnDelegate
                (
                    BurnDelegate,
                )
    ,
    TransferDelegate
                (
                    TransferDelegate,
                )
    ,
    UpdateDelegate
                (
                    UpdateDelegate,
                )
    ,
    PermanentFreezeDelegate
                (
                    PermanentFreezeDelegate,
                )
    ,
    Attributes
                (
                    Attributes,
                )
    ,
    PermanentTransferDelegate
                (
                    PermanentTransferDelegate,
                )
    ,
    PermanentBurnDelegate
                (
                    PermanentBurnDelegate,
                )
    ,
    Edition
                (
                    Edition,
                )
    ,
    MasterEdition
                (
                    MasterEdition,
                )
    ,
    AddBlocker
                (
                    AddBlocker,
                )
    ,
    ImmutableMetadata
                (
                    ImmutableMetadata,
                )
    ,
    VerifiedCreators
                (
                    VerifiedCreators,
                )
    ,
    Autograph
                (
                    Autograph,
                )
    ,
}


