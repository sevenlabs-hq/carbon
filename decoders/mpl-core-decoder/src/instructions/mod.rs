use super::MplCoreDecoder;
pub mod add_collection_plugin_v1;
pub mod add_plugin_v1;
pub mod approve_collection_plugin_authority_v1;
pub mod approve_plugin_authority_v1;
pub mod burn_collection_v1;
pub mod burn_v1;
pub mod collect;
pub mod compress_v1;
pub mod create_collection_v1;
pub mod create_v1;
pub mod decompress_v1;
pub mod remove_collection_plugin_v1;
pub mod remove_plugin_v1;
pub mod revoke_collection_plugin_authority_v1;
pub mod revoke_plugin_authority_v1;
pub mod transfer_v1;
pub mod update_collection_plugin_v1;
pub mod update_collection_v1;
pub mod update_plugin_v1;
pub mod update_v1;

#[derive(
    carbon_core::InstructionType,
    serde::Serialize,
    serde::Deserialize,
    PartialEq,
    Eq,
    Debug,
    Clone,
    Hash,
)]
pub enum MplCoreInstruction {
    CreateV1(create_v1::CreateV1),
    CreateCollectionV1(create_collection_v1::CreateCollectionV1),
    AddPluginV1(add_plugin_v1::AddPluginV1),
    AddCollectionPluginV1(add_collection_plugin_v1::AddCollectionPluginV1),
    RemovePluginV1(remove_plugin_v1::RemovePluginV1),
    RemoveCollectionPluginV1(remove_collection_plugin_v1::RemoveCollectionPluginV1),
    UpdatePluginV1(update_plugin_v1::UpdatePluginV1),
    UpdateCollectionPluginV1(update_collection_plugin_v1::UpdateCollectionPluginV1),
    ApprovePluginAuthorityV1(approve_plugin_authority_v1::ApprovePluginAuthorityV1),
    ApproveCollectionPluginAuthorityV1(
        approve_collection_plugin_authority_v1::ApproveCollectionPluginAuthorityV1,
    ),
    RevokePluginAuthorityV1(revoke_plugin_authority_v1::RevokePluginAuthorityV1),
    RevokeCollectionPluginAuthorityV1(
        revoke_collection_plugin_authority_v1::RevokeCollectionPluginAuthorityV1,
    ),
    BurnV1(burn_v1::BurnV1),
    BurnCollectionV1(burn_collection_v1::BurnCollectionV1),
    TransferV1(transfer_v1::TransferV1),
    UpdateV1(update_v1::UpdateV1),
    UpdateCollectionV1(update_collection_v1::UpdateCollectionV1),
    CompressV1(compress_v1::CompressV1),
    DecompressV1(decompress_v1::DecompressV1),
    Collect(collect::Collect),
}

impl<'a> carbon_core::instruction::InstructionDecoder<'a> for MplCoreDecoder {
    type InstructionType = MplCoreInstruction;

    fn decode_instruction(
        &self,
        instruction: &solana_sdk::instruction::Instruction,
    ) -> Option<carbon_core::instruction::DecodedInstruction<Self::InstructionType>> {
        carbon_core::try_decode_instructions!(instruction,
            MplCoreInstruction::CreateV1 => create_v1::CreateV1,
            MplCoreInstruction::CreateCollectionV1 => create_collection_v1::CreateCollectionV1,
            MplCoreInstruction::AddPluginV1 => add_plugin_v1::AddPluginV1,
            MplCoreInstruction::AddCollectionPluginV1 => add_collection_plugin_v1::AddCollectionPluginV1,
            MplCoreInstruction::RemovePluginV1 => remove_plugin_v1::RemovePluginV1,
            MplCoreInstruction::RemoveCollectionPluginV1 => remove_collection_plugin_v1::RemoveCollectionPluginV1,
            MplCoreInstruction::UpdatePluginV1 => update_plugin_v1::UpdatePluginV1,
            MplCoreInstruction::UpdateCollectionPluginV1 => update_collection_plugin_v1::UpdateCollectionPluginV1,
            MplCoreInstruction::ApprovePluginAuthorityV1 => approve_plugin_authority_v1::ApprovePluginAuthorityV1,
            MplCoreInstruction::ApproveCollectionPluginAuthorityV1 => approve_collection_plugin_authority_v1::ApproveCollectionPluginAuthorityV1,
            MplCoreInstruction::RevokePluginAuthorityV1 => revoke_plugin_authority_v1::RevokePluginAuthorityV1,
            MplCoreInstruction::RevokeCollectionPluginAuthorityV1 => revoke_collection_plugin_authority_v1::RevokeCollectionPluginAuthorityV1,
            MplCoreInstruction::BurnV1 => burn_v1::BurnV1,
            MplCoreInstruction::BurnCollectionV1 => burn_collection_v1::BurnCollectionV1,
            MplCoreInstruction::TransferV1 => transfer_v1::TransferV1,
            MplCoreInstruction::UpdateV1 => update_v1::UpdateV1,
            MplCoreInstruction::UpdateCollectionV1 => update_collection_v1::UpdateCollectionV1,
            MplCoreInstruction::CompressV1 => compress_v1::CompressV1,
            MplCoreInstruction::DecompressV1 => decompress_v1::DecompressV1,
            MplCoreInstruction::Collect => collect::Collect,
        )
    }
}
