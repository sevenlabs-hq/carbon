use super::BubblegumDecoder;
pub mod burn;
pub mod burn_v2;
pub mod cancel_redeem;
pub mod collect_v2;
pub mod compress;
pub mod create_tree;
pub mod create_tree_v2;
pub mod decompress_v1;
pub mod delegate;
pub mod delegate_and_freeze_v2;
pub mod delegate_v2;
pub mod freeze_v2;
pub mod mint_to_collection_v1;
pub mod mint_v1;
pub mod mint_v2;
pub mod redeem;
pub mod set_and_verify_collection;
pub mod set_collection_v2;
pub mod set_decompressable_state;
pub mod set_decompressible_state;
pub mod set_non_transferable_v2;
pub mod set_tree_delegate;
pub mod thaw_and_revoke_v2;
pub mod thaw_v2;
pub mod transfer;
pub mod transfer_v2;
pub mod unverify_collection;
pub mod unverify_creator;
pub mod unverify_creator_v2;
pub mod update_asset_data_v2;
pub mod update_metadata;
pub mod update_metadata_v2;
pub mod verify_collection;
pub mod verify_creator;
pub mod verify_creator_v2;

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
pub enum BubblegumInstruction {
    Burn(burn::Burn),
    BurnV2(burn_v2::BurnV2),
    CancelRedeem(cancel_redeem::CancelRedeem),
    CollectV2(collect_v2::CollectV2),
    Compress(compress::Compress),
    CreateTree(create_tree::CreateTree),
    CreateTreeV2(create_tree_v2::CreateTreeV2),
    DecompressV1(decompress_v1::DecompressV1),
    Delegate(delegate::Delegate),
    DelegateAndFreezeV2(delegate_and_freeze_v2::DelegateAndFreezeV2),
    DelegateV2(delegate_v2::DelegateV2),
    FreezeV2(freeze_v2::FreezeV2),
    MintToCollectionV1(mint_to_collection_v1::MintToCollectionV1),
    MintV1(mint_v1::MintV1),
    MintV2(mint_v2::MintV2),
    Redeem(redeem::Redeem),
    SetAndVerifyCollection(set_and_verify_collection::SetAndVerifyCollection),
    SetCollectionV2(set_collection_v2::SetCollectionV2),
    SetDecompressableState(set_decompressable_state::SetDecompressableState),
    SetDecompressibleState(set_decompressible_state::SetDecompressibleState),
    SetNonTransferableV2(set_non_transferable_v2::SetNonTransferableV2),
    SetTreeDelegate(set_tree_delegate::SetTreeDelegate),
    ThawAndRevokeV2(thaw_and_revoke_v2::ThawAndRevokeV2),
    ThawV2(thaw_v2::ThawV2),
    Transfer(transfer::Transfer),
    TransferV2(transfer_v2::TransferV2),
    UnverifyCollection(unverify_collection::UnverifyCollection),
    UnverifyCreator(unverify_creator::UnverifyCreator),
    UnverifyCreatorV2(unverify_creator_v2::UnverifyCreatorV2),
    UpdateAssetDataV2(update_asset_data_v2::UpdateAssetDataV2),
    UpdateMetadata(update_metadata::UpdateMetadata),
    UpdateMetadataV2(update_metadata_v2::UpdateMetadataV2),
    VerifyCollection(verify_collection::VerifyCollection),
    VerifyCreator(verify_creator::VerifyCreator),
    VerifyCreatorV2(verify_creator_v2::VerifyCreatorV2),
}

impl carbon_core::instruction::InstructionDecoder<'_> for BubblegumDecoder {
    type InstructionType = BubblegumInstruction;

    fn decode_instruction(
        &self,
        instruction: &solana_instruction::Instruction,
    ) -> Option<carbon_core::instruction::DecodedInstruction<Self::InstructionType>> {
        carbon_core::try_decode_instructions!(instruction,
            BubblegumInstruction::Burn => burn::Burn,
            BubblegumInstruction::BurnV2 => burn_v2::BurnV2,
            BubblegumInstruction::CancelRedeem => cancel_redeem::CancelRedeem,
            BubblegumInstruction::CollectV2 => collect_v2::CollectV2,
            BubblegumInstruction::Compress => compress::Compress,
            BubblegumInstruction::CreateTree => create_tree::CreateTree,
            BubblegumInstruction::CreateTreeV2 => create_tree_v2::CreateTreeV2,
            BubblegumInstruction::DecompressV1 => decompress_v1::DecompressV1,
            BubblegumInstruction::Delegate => delegate::Delegate,
            BubblegumInstruction::DelegateAndFreezeV2 => delegate_and_freeze_v2::DelegateAndFreezeV2,
            BubblegumInstruction::DelegateV2 => delegate_v2::DelegateV2,
            BubblegumInstruction::FreezeV2 => freeze_v2::FreezeV2,
            BubblegumInstruction::MintToCollectionV1 => mint_to_collection_v1::MintToCollectionV1,
            BubblegumInstruction::MintV1 => mint_v1::MintV1,
            BubblegumInstruction::MintV2 => mint_v2::MintV2,
            BubblegumInstruction::Redeem => redeem::Redeem,
            BubblegumInstruction::SetAndVerifyCollection => set_and_verify_collection::SetAndVerifyCollection,
            BubblegumInstruction::SetCollectionV2 => set_collection_v2::SetCollectionV2,
            BubblegumInstruction::SetDecompressableState => set_decompressable_state::SetDecompressableState,
            BubblegumInstruction::SetDecompressibleState => set_decompressible_state::SetDecompressibleState,
            BubblegumInstruction::SetNonTransferableV2 => set_non_transferable_v2::SetNonTransferableV2,
            BubblegumInstruction::SetTreeDelegate => set_tree_delegate::SetTreeDelegate,
            BubblegumInstruction::ThawAndRevokeV2 => thaw_and_revoke_v2::ThawAndRevokeV2,
            BubblegumInstruction::ThawV2 => thaw_v2::ThawV2,
            BubblegumInstruction::Transfer => transfer::Transfer,
            BubblegumInstruction::TransferV2 => transfer_v2::TransferV2,
            BubblegumInstruction::UnverifyCollection => unverify_collection::UnverifyCollection,
            BubblegumInstruction::UnverifyCreator => unverify_creator::UnverifyCreator,
            BubblegumInstruction::UnverifyCreatorV2 => unverify_creator_v2::UnverifyCreatorV2,
            BubblegumInstruction::UpdateAssetDataV2 => update_asset_data_v2::UpdateAssetDataV2,
            BubblegumInstruction::UpdateMetadata => update_metadata::UpdateMetadata,
            BubblegumInstruction::UpdateMetadataV2 => update_metadata_v2::UpdateMetadataV2,
            BubblegumInstruction::VerifyCollection => verify_collection::VerifyCollection,
            BubblegumInstruction::VerifyCreator => verify_creator::VerifyCreator,
            BubblegumInstruction::VerifyCreatorV2 => verify_creator_v2::VerifyCreatorV2,
        )
    }
}
