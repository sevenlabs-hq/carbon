
use carbon_core::deserialize::CarbonDeserialize;
use carbon_core::instruction::InstructionDecoder;


use super::MplCoreProgramDecoder;
pub mod create_v1;
pub mod create_collection_v1;
pub mod add_plugin_v1;
pub mod add_collection_plugin_v1;
pub mod remove_plugin_v1;
pub mod remove_collection_plugin_v1;
pub mod update_plugin_v1;
pub mod update_collection_plugin_v1;
pub mod approve_plugin_authority_v1;
pub mod approve_collection_plugin_authority_v1;
pub mod revoke_plugin_authority_v1;
pub mod revoke_collection_plugin_authority_v1;
pub mod burn_v1;
pub mod burn_collection_v1;
pub mod transfer_v1;
pub mod update_v1;
pub mod update_collection_v1;
pub mod compress_v1;
pub mod decompress_v1;
pub mod collect;

#[derive(carbon_proc_macros::InstructionType, serde::Serialize, serde::Deserialize, PartialEq, Eq, Debug, Clone, Hash)]
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
}

impl InstructionDecoder for MplCoreProgramDecoder {
    type InstructionType = MplCoreProgramInstruction;

    fn decode_instruction(
        &self,
        instruction: solana_sdk::instruction::Instruction,
    ) -> Option<carbon_core::instruction::DecodedInstruction<Self::InstructionType>> {
        if let Some(decoded_instruction) = create_v1::CreateV1::deserialize(instruction.data.as_slice()) {
            return Some(carbon_core::instruction::DecodedInstruction {
                program_id: instruction.program_id,
                data: MplCoreProgramInstruction::CreateV1(decoded_instruction),
            });
        }
        if let Some(decoded_instruction) = create_collection_v1::CreateCollectionV1::deserialize(instruction.data.as_slice()) {
            return Some(carbon_core::instruction::DecodedInstruction {
                program_id: instruction.program_id,
                data: MplCoreProgramInstruction::CreateCollectionV1(decoded_instruction),
            });
        }
        if let Some(decoded_instruction) = add_plugin_v1::AddPluginV1::deserialize(instruction.data.as_slice()) {
            return Some(carbon_core::instruction::DecodedInstruction {
                program_id: instruction.program_id,
                data: MplCoreProgramInstruction::AddPluginV1(decoded_instruction),
            });
        }
        if let Some(decoded_instruction) = add_collection_plugin_v1::AddCollectionPluginV1::deserialize(instruction.data.as_slice()) {
            return Some(carbon_core::instruction::DecodedInstruction {
                program_id: instruction.program_id,
                data: MplCoreProgramInstruction::AddCollectionPluginV1(decoded_instruction),
            });
        }
        if let Some(decoded_instruction) = remove_plugin_v1::RemovePluginV1::deserialize(instruction.data.as_slice()) {
            return Some(carbon_core::instruction::DecodedInstruction {
                program_id: instruction.program_id,
                data: MplCoreProgramInstruction::RemovePluginV1(decoded_instruction),
            });
        }
        if let Some(decoded_instruction) = remove_collection_plugin_v1::RemoveCollectionPluginV1::deserialize(instruction.data.as_slice()) {
            return Some(carbon_core::instruction::DecodedInstruction {
                program_id: instruction.program_id,
                data: MplCoreProgramInstruction::RemoveCollectionPluginV1(decoded_instruction),
            });
        }
        if let Some(decoded_instruction) = update_plugin_v1::UpdatePluginV1::deserialize(instruction.data.as_slice()) {
            return Some(carbon_core::instruction::DecodedInstruction {
                program_id: instruction.program_id,
                data: MplCoreProgramInstruction::UpdatePluginV1(decoded_instruction),
            });
        }
        if let Some(decoded_instruction) = update_collection_plugin_v1::UpdateCollectionPluginV1::deserialize(instruction.data.as_slice()) {
            return Some(carbon_core::instruction::DecodedInstruction {
                program_id: instruction.program_id,
                data: MplCoreProgramInstruction::UpdateCollectionPluginV1(decoded_instruction),
            });
        }
        if let Some(decoded_instruction) = approve_plugin_authority_v1::ApprovePluginAuthorityV1::deserialize(instruction.data.as_slice()) {
            return Some(carbon_core::instruction::DecodedInstruction {
                program_id: instruction.program_id,
                data: MplCoreProgramInstruction::ApprovePluginAuthorityV1(decoded_instruction),
            });
        }
        if let Some(decoded_instruction) = approve_collection_plugin_authority_v1::ApproveCollectionPluginAuthorityV1::deserialize(instruction.data.as_slice()) {
            return Some(carbon_core::instruction::DecodedInstruction {
                program_id: instruction.program_id,
                data: MplCoreProgramInstruction::ApproveCollectionPluginAuthorityV1(decoded_instruction),
            });
        }
        if let Some(decoded_instruction) = revoke_plugin_authority_v1::RevokePluginAuthorityV1::deserialize(instruction.data.as_slice()) {
            return Some(carbon_core::instruction::DecodedInstruction {
                program_id: instruction.program_id,
                data: MplCoreProgramInstruction::RevokePluginAuthorityV1(decoded_instruction),
            });
        }
        if let Some(decoded_instruction) = revoke_collection_plugin_authority_v1::RevokeCollectionPluginAuthorityV1::deserialize(instruction.data.as_slice()) {
            return Some(carbon_core::instruction::DecodedInstruction {
                program_id: instruction.program_id,
                data: MplCoreProgramInstruction::RevokeCollectionPluginAuthorityV1(decoded_instruction),
            });
        }
        if let Some(decoded_instruction) = burn_v1::BurnV1::deserialize(instruction.data.as_slice()) {
            return Some(carbon_core::instruction::DecodedInstruction {
                program_id: instruction.program_id,
                data: MplCoreProgramInstruction::BurnV1(decoded_instruction),
            });
        }
        if let Some(decoded_instruction) = burn_collection_v1::BurnCollectionV1::deserialize(instruction.data.as_slice()) {
            return Some(carbon_core::instruction::DecodedInstruction {
                program_id: instruction.program_id,
                data: MplCoreProgramInstruction::BurnCollectionV1(decoded_instruction),
            });
        }
        if let Some(decoded_instruction) = transfer_v1::TransferV1::deserialize(instruction.data.as_slice()) {
            return Some(carbon_core::instruction::DecodedInstruction {
                program_id: instruction.program_id,
                data: MplCoreProgramInstruction::TransferV1(decoded_instruction),
            });
        }
        if let Some(decoded_instruction) = update_v1::UpdateV1::deserialize(instruction.data.as_slice()) {
            return Some(carbon_core::instruction::DecodedInstruction {
                program_id: instruction.program_id,
                data: MplCoreProgramInstruction::UpdateV1(decoded_instruction),
            });
        }
        if let Some(decoded_instruction) = update_collection_v1::UpdateCollectionV1::deserialize(instruction.data.as_slice()) {
            return Some(carbon_core::instruction::DecodedInstruction {
                program_id: instruction.program_id,
                data: MplCoreProgramInstruction::UpdateCollectionV1(decoded_instruction),
            });
        }
        if let Some(decoded_instruction) = compress_v1::CompressV1::deserialize(instruction.data.as_slice()) {
            return Some(carbon_core::instruction::DecodedInstruction {
                program_id: instruction.program_id,
                data: MplCoreProgramInstruction::CompressV1(decoded_instruction),
            });
        }
        if let Some(decoded_instruction) = decompress_v1::DecompressV1::deserialize(instruction.data.as_slice()) {
            return Some(carbon_core::instruction::DecodedInstruction {
                program_id: instruction.program_id,
                data: MplCoreProgramInstruction::DecompressV1(decoded_instruction),
            });
        }
        if let Some(decoded_instruction) = collect::Collect::deserialize(instruction.data.as_slice()) {
            return Some(carbon_core::instruction::DecodedInstruction {
                program_id: instruction.program_id,
                data: MplCoreProgramInstruction::Collect(decoded_instruction),
            });
        }

        None
    }
}