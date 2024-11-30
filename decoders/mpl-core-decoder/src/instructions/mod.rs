use super::MplCoreProgramDecoder;
pub mod add_collection_external_plugin_adapter_v1;
pub mod add_collection_plugin_v1;
pub mod add_external_plugin_adapter_v1;
pub mod add_plugin_v1;
pub mod approve_collection_plugin_authority_v1;
pub mod approve_plugin_authority_v1;
pub mod burn_collection_v1;
pub mod burn_v1;
pub mod collect;
pub mod compress_v1;
pub mod create_collection_v1;
pub mod create_collection_v2;
pub mod create_v1;
pub mod create_v2;
pub mod decompress_v1;
pub mod remove_collection_external_plugin_adapter_v1;
pub mod remove_collection_plugin_v1;
pub mod remove_external_plugin_adapter_v1;
pub mod remove_plugin_v1;
pub mod revoke_collection_plugin_authority_v1;
pub mod revoke_plugin_authority_v1;
pub mod transfer_v1;
pub mod update_collection_external_plugin_adapter_v1;
pub mod update_collection_plugin_v1;
pub mod update_collection_v1;
pub mod update_external_plugin_adapter_v1;
pub mod update_plugin_v1;
pub mod update_v1;
pub mod update_v2;
pub mod write_collection_external_plugin_adapter_data_v1;
pub mod write_external_plugin_adapter_data_v1;

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
pub enum MplCoreProgramInstruction {
    CreateV1(create_v1::CreateV1),
    CreateCollectionV1(create_collection_v1::CreateCollectionV1),
    AddPluginV1(add_plugin_v1::AddPluginV1),
    AddCollectionPluginV1(add_collection_plugin_v1::AddCollectionPluginV1),
    RemovePluginV1(remove_plugin_v1::RemovePluginV1),
    RemoveCollectionPluginV1(remove_collection_plugin_v1::RemoveCollectionPluginV1),
    UpdatePluginV1(update_plugin_v1::UpdatePluginV1),
    UpdateCollectionPluginV1(update_collection_plugin_v1::UpdateCollectionPluginV1),
    ApprovePluginAuthorityV1(approve_plugin_authority_v1::ApprovePluginAuthorityV1),
    ApproveCollectionPluginAuthorityV1(approve_collection_plugin_authority_v1::ApproveCollectionPluginAuthorityV1),
    RevokePluginAuthorityV1(revoke_plugin_authority_v1::RevokePluginAuthorityV1),
    RevokeCollectionPluginAuthorityV1(revoke_collection_plugin_authority_v1::RevokeCollectionPluginAuthorityV1),
    BurnV1(burn_v1::BurnV1),
    BurnCollectionV1(burn_collection_v1::BurnCollectionV1),
    TransferV1(transfer_v1::TransferV1),
    UpdateV1(update_v1::UpdateV1),
    UpdateCollectionV1(update_collection_v1::UpdateCollectionV1),
    CompressV1(compress_v1::CompressV1),
    DecompressV1(decompress_v1::DecompressV1),
    Collect(collect::Collect),
    CreateV2(create_v2::CreateV2),
    CreateCollectionV2(create_collection_v2::CreateCollectionV2),
    AddExternalPluginAdapterV1(add_external_plugin_adapter_v1::AddExternalPluginAdapterV1),
    AddCollectionExternalPluginAdapterV1(add_collection_external_plugin_adapter_v1::AddCollectionExternalPluginAdapterV1),
    RemoveExternalPluginAdapterV1(remove_external_plugin_adapter_v1::RemoveExternalPluginAdapterV1),
    RemoveCollectionExternalPluginAdapterV1(remove_collection_external_plugin_adapter_v1::RemoveCollectionExternalPluginAdapterV1),
    UpdateExternalPluginAdapterV1(update_external_plugin_adapter_v1::UpdateExternalPluginAdapterV1),
    UpdateCollectionExternalPluginAdapterV1(update_collection_external_plugin_adapter_v1::UpdateCollectionExternalPluginAdapterV1),
    WriteExternalPluginAdapterDataV1(write_external_plugin_adapter_data_v1::WriteExternalPluginAdapterDataV1),
    WriteCollectionExternalPluginAdapterDataV1(write_collection_external_plugin_adapter_data_v1::WriteCollectionExternalPluginAdapterDataV1),
    UpdateV2(update_v2::UpdateV2),
}

impl<'a> carbon_core::instruction::InstructionDecoder<'a> for MplCoreProgramDecoder {
    type InstructionType = MplCoreProgramInstruction;

    fn decode_instruction(
        &self,
        instruction: &solana_sdk::instruction::Instruction,
    ) -> Option<carbon_core::instruction::DecodedInstruction<Self::InstructionType>> {
        carbon_core::try_decode_instructions!(instruction,
            MplCoreProgramInstruction::CreateV1 => create_v1::CreateV1,
            MplCoreProgramInstruction::CreateCollectionV1 => create_collection_v1::CreateCollectionV1,
            MplCoreProgramInstruction::AddPluginV1 => add_plugin_v1::AddPluginV1,
            MplCoreProgramInstruction::AddCollectionPluginV1 => add_collection_plugin_v1::AddCollectionPluginV1,
            MplCoreProgramInstruction::RemovePluginV1 => remove_plugin_v1::RemovePluginV1,
            MplCoreProgramInstruction::RemoveCollectionPluginV1 => remove_collection_plugin_v1::RemoveCollectionPluginV1,
            MplCoreProgramInstruction::UpdatePluginV1 => update_plugin_v1::UpdatePluginV1,
            MplCoreProgramInstruction::UpdateCollectionPluginV1 => update_collection_plugin_v1::UpdateCollectionPluginV1,
            MplCoreProgramInstruction::ApprovePluginAuthorityV1 => approve_plugin_authority_v1::ApprovePluginAuthorityV1,
            MplCoreProgramInstruction::ApproveCollectionPluginAuthorityV1 => approve_collection_plugin_authority_v1::ApproveCollectionPluginAuthorityV1,
            MplCoreProgramInstruction::RevokePluginAuthorityV1 => revoke_plugin_authority_v1::RevokePluginAuthorityV1,
            MplCoreProgramInstruction::RevokeCollectionPluginAuthorityV1 => revoke_collection_plugin_authority_v1::RevokeCollectionPluginAuthorityV1,
            MplCoreProgramInstruction::BurnV1 => burn_v1::BurnV1,
            MplCoreProgramInstruction::BurnCollectionV1 => burn_collection_v1::BurnCollectionV1,
            MplCoreProgramInstruction::TransferV1 => transfer_v1::TransferV1,
            MplCoreProgramInstruction::UpdateV1 => update_v1::UpdateV1,
            MplCoreProgramInstruction::UpdateCollectionV1 => update_collection_v1::UpdateCollectionV1,
            MplCoreProgramInstruction::CompressV1 => compress_v1::CompressV1,
            MplCoreProgramInstruction::DecompressV1 => decompress_v1::DecompressV1,
            MplCoreProgramInstruction::Collect => collect::Collect,
            MplCoreProgramInstruction::CreateV2 => create_v2::CreateV2,
            MplCoreProgramInstruction::CreateCollectionV2 => create_collection_v2::CreateCollectionV2,
            MplCoreProgramInstruction::AddExternalPluginAdapterV1 => add_external_plugin_adapter_v1::AddExternalPluginAdapterV1,
            MplCoreProgramInstruction::AddCollectionExternalPluginAdapterV1 => add_collection_external_plugin_adapter_v1::AddCollectionExternalPluginAdapterV1,
            MplCoreProgramInstruction::RemoveExternalPluginAdapterV1 => remove_external_plugin_adapter_v1::RemoveExternalPluginAdapterV1,
            MplCoreProgramInstruction::RemoveCollectionExternalPluginAdapterV1 => remove_collection_external_plugin_adapter_v1::RemoveCollectionExternalPluginAdapterV1,
            MplCoreProgramInstruction::UpdateExternalPluginAdapterV1 => update_external_plugin_adapter_v1::UpdateExternalPluginAdapterV1,
            MplCoreProgramInstruction::UpdateCollectionExternalPluginAdapterV1 => update_collection_external_plugin_adapter_v1::UpdateCollectionExternalPluginAdapterV1,
            MplCoreProgramInstruction::WriteExternalPluginAdapterDataV1 => write_external_plugin_adapter_data_v1::WriteExternalPluginAdapterDataV1,
            MplCoreProgramInstruction::WriteCollectionExternalPluginAdapterDataV1 => write_collection_external_plugin_adapter_data_v1::WriteCollectionExternalPluginAdapterDataV1,
            MplCoreProgramInstruction::UpdateV2 => update_v2::UpdateV2,
        )
    }
}
