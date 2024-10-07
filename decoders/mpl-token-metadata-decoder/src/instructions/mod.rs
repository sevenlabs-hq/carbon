use carbon_core::deserialize::CarbonDeserialize;
use carbon_core::instruction::InstructionDecoder;

use super::MplTokenMetadataDecoder;
pub mod approve_collection_authority;
pub mod approve_use_authority;
pub mod bubblegum_set_collection_size;
pub mod burn;
pub mod burn_edition_nft;
pub mod burn_nft;
pub mod close_escrow_account;
pub mod collect;
pub mod convert_master_edition_v1_to_v2;
pub mod create;
pub mod create_escrow_account;
pub mod create_master_edition;
pub mod create_master_edition_v3;
pub mod create_metadata_account;
pub mod create_metadata_account_v2;
pub mod create_metadata_account_v3;
pub mod delegate;
pub mod deprecated_create_master_edition;
pub mod deprecated_create_reservation_list;
pub mod deprecated_mint_new_edition_from_master_edition_via_printing_token;
pub mod deprecated_mint_printing_tokens;
pub mod deprecated_mint_printing_tokens_via_token;
pub mod deprecated_set_reservation_list;
pub mod freeze_delegated_account;
pub mod lock;
pub mod migrate;
pub mod mint;
pub mod mint_new_edition_from_master_edition_via_token;
pub mod mint_new_edition_from_master_edition_via_vault_proxy;
pub mod print;
pub mod puff_metadata;
pub mod remove_creator_verification;
pub mod revoke;
pub mod revoke_collection_authority;
pub mod revoke_use_authority;
pub mod set_and_verify_collection;
pub mod set_and_verify_sized_collection_item;
pub mod set_collection_size;
pub mod set_token_standard;
pub mod sign_metadata;
pub mod thaw_delegated_account;
pub mod transfer;
pub mod transfer_out_of_escrow;
pub mod unlock;
pub mod unverify;
pub mod unverify_collection;
pub mod unverify_sized_collection_item;
pub mod update;
pub mod update_metadata_account;
pub mod update_metadata_account_v2;
pub mod update_primary_sale_happened_via_token;
pub mod utilize;
pub mod verify;
pub mod verify_collection;
pub mod verify_sized_collection_item;

#[derive(
    carbon_proc_macros::InstructionType,
    serde::Serialize,
    serde::Deserialize,
    PartialEq,
    Eq,
    Debug,
    Clone,
    Hash,
)]
pub enum MplTokenMetadataInstruction {
    CreateMetadataAccount(create_metadata_account::CreateMetadataAccount),
    UpdateMetadataAccount(update_metadata_account::UpdateMetadataAccount),
    DeprecatedCreateMasterEdition(deprecated_create_master_edition::DeprecatedCreateMasterEdition),
    DeprecatedMintNewEditionFromMasterEditionViaPrintingToken(deprecated_mint_new_edition_from_master_edition_via_printing_token::DeprecatedMintNewEditionFromMasterEditionViaPrintingToken),
    UpdatePrimarySaleHappenedViaToken(update_primary_sale_happened_via_token::UpdatePrimarySaleHappenedViaToken),
    DeprecatedSetReservationList(deprecated_set_reservation_list::DeprecatedSetReservationList),
    DeprecatedCreateReservationList(deprecated_create_reservation_list::DeprecatedCreateReservationList),
    SignMetadata(sign_metadata::SignMetadata),
    DeprecatedMintPrintingTokensViaToken(deprecated_mint_printing_tokens_via_token::DeprecatedMintPrintingTokensViaToken),
    DeprecatedMintPrintingTokens(deprecated_mint_printing_tokens::DeprecatedMintPrintingTokens),
    CreateMasterEdition(create_master_edition::CreateMasterEdition),
    MintNewEditionFromMasterEditionViaToken(mint_new_edition_from_master_edition_via_token::MintNewEditionFromMasterEditionViaToken),
    ConvertMasterEditionV1ToV2(convert_master_edition_v1_to_v2::ConvertMasterEditionV1ToV2),
    MintNewEditionFromMasterEditionViaVaultProxy(mint_new_edition_from_master_edition_via_vault_proxy::MintNewEditionFromMasterEditionViaVaultProxy),
    PuffMetadata(puff_metadata::PuffMetadata),
    UpdateMetadataAccountV2(update_metadata_account_v2::UpdateMetadataAccountV2),
    CreateMetadataAccountV2(create_metadata_account_v2::CreateMetadataAccountV2),
    CreateMasterEditionV3(create_master_edition_v3::CreateMasterEditionV3),
    VerifyCollection(verify_collection::VerifyCollection),
    Utilize(utilize::Utilize),
    ApproveUseAuthority(approve_use_authority::ApproveUseAuthority),
    RevokeUseAuthority(revoke_use_authority::RevokeUseAuthority),
    UnverifyCollection(unverify_collection::UnverifyCollection),
    ApproveCollectionAuthority(approve_collection_authority::ApproveCollectionAuthority),
    RevokeCollectionAuthority(revoke_collection_authority::RevokeCollectionAuthority),
    SetAndVerifyCollection(set_and_verify_collection::SetAndVerifyCollection),
    FreezeDelegatedAccount(freeze_delegated_account::FreezeDelegatedAccount),
    ThawDelegatedAccount(thaw_delegated_account::ThawDelegatedAccount),
    RemoveCreatorVerification(remove_creator_verification::RemoveCreatorVerification),
    BurnNft(burn_nft::BurnNft),
    VerifySizedCollectionItem(verify_sized_collection_item::VerifySizedCollectionItem),
    UnverifySizedCollectionItem(unverify_sized_collection_item::UnverifySizedCollectionItem),
    SetAndVerifySizedCollectionItem(set_and_verify_sized_collection_item::SetAndVerifySizedCollectionItem),
    CreateMetadataAccountV3(create_metadata_account_v3::CreateMetadataAccountV3),
    SetCollectionSize(set_collection_size::SetCollectionSize),
    SetTokenStandard(set_token_standard::SetTokenStandard),
    BubblegumSetCollectionSize(bubblegum_set_collection_size::BubblegumSetCollectionSize),
    BurnEditionNft(burn_edition_nft::BurnEditionNft),
    CreateEscrowAccount(create_escrow_account::CreateEscrowAccount),
    CloseEscrowAccount(close_escrow_account::CloseEscrowAccount),
    TransferOutOfEscrow(transfer_out_of_escrow::TransferOutOfEscrow),
    Burn(burn::Burn),
    Create(create::Create),
    Mint(mint::Mint),
    Delegate(delegate::Delegate),
    Revoke(revoke::Revoke),
    Lock(lock::Lock),
    Unlock(unlock::Unlock),
    Migrate(migrate::Migrate),
    Transfer(transfer::Transfer),
    Update(update::Update),
    Verify(verify::Verify),
    Unverify(unverify::Unverify),
    Collect(collect::Collect),
    Print(print::Print),
}

impl InstructionDecoder for MplTokenMetadataDecoder {
    type InstructionType = MplTokenMetadataInstruction;

    fn decode_instruction(
        &self,
        instruction: solana_sdk::instruction::Instruction,
    ) -> Option<carbon_core::instruction::DecodedInstruction<Self::InstructionType>> {
        if let Some(decoded_instruction) =
            create_metadata_account::CreateMetadataAccount::deserialize(instruction.data.as_slice())
        {
            return Some(carbon_core::instruction::DecodedInstruction {
                program_id: instruction.program_id,
                data: MplTokenMetadataInstruction::CreateMetadataAccount(decoded_instruction),
            });
        }
        if let Some(decoded_instruction) =
            update_metadata_account::UpdateMetadataAccount::deserialize(instruction.data.as_slice())
        {
            return Some(carbon_core::instruction::DecodedInstruction {
                program_id: instruction.program_id,
                data: MplTokenMetadataInstruction::UpdateMetadataAccount(decoded_instruction),
            });
        }
        if let Some(decoded_instruction) =
            deprecated_create_master_edition::DeprecatedCreateMasterEdition::deserialize(
                instruction.data.as_slice(),
            )
        {
            return Some(carbon_core::instruction::DecodedInstruction {
                program_id: instruction.program_id,
                data: MplTokenMetadataInstruction::DeprecatedCreateMasterEdition(
                    decoded_instruction,
                ),
            });
        }
        if let Some(decoded_instruction) = deprecated_mint_new_edition_from_master_edition_via_printing_token::DeprecatedMintNewEditionFromMasterEditionViaPrintingToken::deserialize(instruction.data.as_slice()) {
            return Some(carbon_core::instruction::DecodedInstruction {
                program_id: instruction.program_id,
                data: MplTokenMetadataInstruction::DeprecatedMintNewEditionFromMasterEditionViaPrintingToken(decoded_instruction),
            });
        }
        if let Some(decoded_instruction) =
            update_primary_sale_happened_via_token::UpdatePrimarySaleHappenedViaToken::deserialize(
                instruction.data.as_slice(),
            )
        {
            return Some(carbon_core::instruction::DecodedInstruction {
                program_id: instruction.program_id,
                data: MplTokenMetadataInstruction::UpdatePrimarySaleHappenedViaToken(
                    decoded_instruction,
                ),
            });
        }
        if let Some(decoded_instruction) =
            deprecated_set_reservation_list::DeprecatedSetReservationList::deserialize(
                instruction.data.as_slice(),
            )
        {
            return Some(carbon_core::instruction::DecodedInstruction {
                program_id: instruction.program_id,
                data: MplTokenMetadataInstruction::DeprecatedSetReservationList(
                    decoded_instruction,
                ),
            });
        }
        if let Some(decoded_instruction) =
            deprecated_create_reservation_list::DeprecatedCreateReservationList::deserialize(
                instruction.data.as_slice(),
            )
        {
            return Some(carbon_core::instruction::DecodedInstruction {
                program_id: instruction.program_id,
                data: MplTokenMetadataInstruction::DeprecatedCreateReservationList(
                    decoded_instruction,
                ),
            });
        }
        if let Some(decoded_instruction) =
            sign_metadata::SignMetadata::deserialize(instruction.data.as_slice())
        {
            return Some(carbon_core::instruction::DecodedInstruction {
                program_id: instruction.program_id,
                data: MplTokenMetadataInstruction::SignMetadata(decoded_instruction),
            });
        }
        if let Some(decoded_instruction) = deprecated_mint_printing_tokens_via_token::DeprecatedMintPrintingTokensViaToken::deserialize(instruction.data.as_slice()) {
            return Some(carbon_core::instruction::DecodedInstruction {
                program_id: instruction.program_id,
                data: MplTokenMetadataInstruction::DeprecatedMintPrintingTokensViaToken(decoded_instruction),
            });
        }
        if let Some(decoded_instruction) =
            deprecated_mint_printing_tokens::DeprecatedMintPrintingTokens::deserialize(
                instruction.data.as_slice(),
            )
        {
            return Some(carbon_core::instruction::DecodedInstruction {
                program_id: instruction.program_id,
                data: MplTokenMetadataInstruction::DeprecatedMintPrintingTokens(
                    decoded_instruction,
                ),
            });
        }
        if let Some(decoded_instruction) =
            create_master_edition::CreateMasterEdition::deserialize(instruction.data.as_slice())
        {
            return Some(carbon_core::instruction::DecodedInstruction {
                program_id: instruction.program_id,
                data: MplTokenMetadataInstruction::CreateMasterEdition(decoded_instruction),
            });
        }
        if let Some(decoded_instruction) = mint_new_edition_from_master_edition_via_token::MintNewEditionFromMasterEditionViaToken::deserialize(instruction.data.as_slice()) {
            return Some(carbon_core::instruction::DecodedInstruction {
                program_id: instruction.program_id,
                data: MplTokenMetadataInstruction::MintNewEditionFromMasterEditionViaToken(decoded_instruction),
            });
        }
        if let Some(decoded_instruction) =
            convert_master_edition_v1_to_v2::ConvertMasterEditionV1ToV2::deserialize(
                instruction.data.as_slice(),
            )
        {
            return Some(carbon_core::instruction::DecodedInstruction {
                program_id: instruction.program_id,
                data: MplTokenMetadataInstruction::ConvertMasterEditionV1ToV2(decoded_instruction),
            });
        }
        if let Some(decoded_instruction) = mint_new_edition_from_master_edition_via_vault_proxy::MintNewEditionFromMasterEditionViaVaultProxy::deserialize(instruction.data.as_slice()) {
            return Some(carbon_core::instruction::DecodedInstruction {
                program_id: instruction.program_id,
                data: MplTokenMetadataInstruction::MintNewEditionFromMasterEditionViaVaultProxy(decoded_instruction),
            });
        }
        if let Some(decoded_instruction) =
            puff_metadata::PuffMetadata::deserialize(instruction.data.as_slice())
        {
            return Some(carbon_core::instruction::DecodedInstruction {
                program_id: instruction.program_id,
                data: MplTokenMetadataInstruction::PuffMetadata(decoded_instruction),
            });
        }
        if let Some(decoded_instruction) =
            update_metadata_account_v2::UpdateMetadataAccountV2::deserialize(
                instruction.data.as_slice(),
            )
        {
            return Some(carbon_core::instruction::DecodedInstruction {
                program_id: instruction.program_id,
                data: MplTokenMetadataInstruction::UpdateMetadataAccountV2(decoded_instruction),
            });
        }
        if let Some(decoded_instruction) =
            create_metadata_account_v2::CreateMetadataAccountV2::deserialize(
                instruction.data.as_slice(),
            )
        {
            return Some(carbon_core::instruction::DecodedInstruction {
                program_id: instruction.program_id,
                data: MplTokenMetadataInstruction::CreateMetadataAccountV2(decoded_instruction),
            });
        }
        if let Some(decoded_instruction) =
            create_master_edition_v3::CreateMasterEditionV3::deserialize(
                instruction.data.as_slice(),
            )
        {
            return Some(carbon_core::instruction::DecodedInstruction {
                program_id: instruction.program_id,
                data: MplTokenMetadataInstruction::CreateMasterEditionV3(decoded_instruction),
            });
        }
        if let Some(decoded_instruction) =
            verify_collection::VerifyCollection::deserialize(instruction.data.as_slice())
        {
            return Some(carbon_core::instruction::DecodedInstruction {
                program_id: instruction.program_id,
                data: MplTokenMetadataInstruction::VerifyCollection(decoded_instruction),
            });
        }
        if let Some(decoded_instruction) =
            utilize::Utilize::deserialize(instruction.data.as_slice())
        {
            return Some(carbon_core::instruction::DecodedInstruction {
                program_id: instruction.program_id,
                data: MplTokenMetadataInstruction::Utilize(decoded_instruction),
            });
        }
        if let Some(decoded_instruction) =
            approve_use_authority::ApproveUseAuthority::deserialize(instruction.data.as_slice())
        {
            return Some(carbon_core::instruction::DecodedInstruction {
                program_id: instruction.program_id,
                data: MplTokenMetadataInstruction::ApproveUseAuthority(decoded_instruction),
            });
        }
        if let Some(decoded_instruction) =
            revoke_use_authority::RevokeUseAuthority::deserialize(instruction.data.as_slice())
        {
            return Some(carbon_core::instruction::DecodedInstruction {
                program_id: instruction.program_id,
                data: MplTokenMetadataInstruction::RevokeUseAuthority(decoded_instruction),
            });
        }
        if let Some(decoded_instruction) =
            unverify_collection::UnverifyCollection::deserialize(instruction.data.as_slice())
        {
            return Some(carbon_core::instruction::DecodedInstruction {
                program_id: instruction.program_id,
                data: MplTokenMetadataInstruction::UnverifyCollection(decoded_instruction),
            });
        }
        if let Some(decoded_instruction) =
            approve_collection_authority::ApproveCollectionAuthority::deserialize(
                instruction.data.as_slice(),
            )
        {
            return Some(carbon_core::instruction::DecodedInstruction {
                program_id: instruction.program_id,
                data: MplTokenMetadataInstruction::ApproveCollectionAuthority(decoded_instruction),
            });
        }
        if let Some(decoded_instruction) =
            revoke_collection_authority::RevokeCollectionAuthority::deserialize(
                instruction.data.as_slice(),
            )
        {
            return Some(carbon_core::instruction::DecodedInstruction {
                program_id: instruction.program_id,
                data: MplTokenMetadataInstruction::RevokeCollectionAuthority(decoded_instruction),
            });
        }
        if let Some(decoded_instruction) =
            set_and_verify_collection::SetAndVerifyCollection::deserialize(
                instruction.data.as_slice(),
            )
        {
            return Some(carbon_core::instruction::DecodedInstruction {
                program_id: instruction.program_id,
                data: MplTokenMetadataInstruction::SetAndVerifyCollection(decoded_instruction),
            });
        }
        if let Some(decoded_instruction) =
            freeze_delegated_account::FreezeDelegatedAccount::deserialize(
                instruction.data.as_slice(),
            )
        {
            return Some(carbon_core::instruction::DecodedInstruction {
                program_id: instruction.program_id,
                data: MplTokenMetadataInstruction::FreezeDelegatedAccount(decoded_instruction),
            });
        }
        if let Some(decoded_instruction) =
            thaw_delegated_account::ThawDelegatedAccount::deserialize(instruction.data.as_slice())
        {
            return Some(carbon_core::instruction::DecodedInstruction {
                program_id: instruction.program_id,
                data: MplTokenMetadataInstruction::ThawDelegatedAccount(decoded_instruction),
            });
        }
        if let Some(decoded_instruction) =
            remove_creator_verification::RemoveCreatorVerification::deserialize(
                instruction.data.as_slice(),
            )
        {
            return Some(carbon_core::instruction::DecodedInstruction {
                program_id: instruction.program_id,
                data: MplTokenMetadataInstruction::RemoveCreatorVerification(decoded_instruction),
            });
        }
        if let Some(decoded_instruction) =
            burn_nft::BurnNft::deserialize(instruction.data.as_slice())
        {
            return Some(carbon_core::instruction::DecodedInstruction {
                program_id: instruction.program_id,
                data: MplTokenMetadataInstruction::BurnNft(decoded_instruction),
            });
        }
        if let Some(decoded_instruction) =
            verify_sized_collection_item::VerifySizedCollectionItem::deserialize(
                instruction.data.as_slice(),
            )
        {
            return Some(carbon_core::instruction::DecodedInstruction {
                program_id: instruction.program_id,
                data: MplTokenMetadataInstruction::VerifySizedCollectionItem(decoded_instruction),
            });
        }
        if let Some(decoded_instruction) =
            unverify_sized_collection_item::UnverifySizedCollectionItem::deserialize(
                instruction.data.as_slice(),
            )
        {
            return Some(carbon_core::instruction::DecodedInstruction {
                program_id: instruction.program_id,
                data: MplTokenMetadataInstruction::UnverifySizedCollectionItem(decoded_instruction),
            });
        }
        if let Some(decoded_instruction) =
            set_and_verify_sized_collection_item::SetAndVerifySizedCollectionItem::deserialize(
                instruction.data.as_slice(),
            )
        {
            return Some(carbon_core::instruction::DecodedInstruction {
                program_id: instruction.program_id,
                data: MplTokenMetadataInstruction::SetAndVerifySizedCollectionItem(
                    decoded_instruction,
                ),
            });
        }
        if let Some(decoded_instruction) =
            create_metadata_account_v3::CreateMetadataAccountV3::deserialize(
                instruction.data.as_slice(),
            )
        {
            return Some(carbon_core::instruction::DecodedInstruction {
                program_id: instruction.program_id,
                data: MplTokenMetadataInstruction::CreateMetadataAccountV3(decoded_instruction),
            });
        }
        if let Some(decoded_instruction) =
            set_collection_size::SetCollectionSize::deserialize(instruction.data.as_slice())
        {
            return Some(carbon_core::instruction::DecodedInstruction {
                program_id: instruction.program_id,
                data: MplTokenMetadataInstruction::SetCollectionSize(decoded_instruction),
            });
        }
        if let Some(decoded_instruction) =
            set_token_standard::SetTokenStandard::deserialize(instruction.data.as_slice())
        {
            return Some(carbon_core::instruction::DecodedInstruction {
                program_id: instruction.program_id,
                data: MplTokenMetadataInstruction::SetTokenStandard(decoded_instruction),
            });
        }
        if let Some(decoded_instruction) =
            bubblegum_set_collection_size::BubblegumSetCollectionSize::deserialize(
                instruction.data.as_slice(),
            )
        {
            return Some(carbon_core::instruction::DecodedInstruction {
                program_id: instruction.program_id,
                data: MplTokenMetadataInstruction::BubblegumSetCollectionSize(decoded_instruction),
            });
        }
        if let Some(decoded_instruction) =
            burn_edition_nft::BurnEditionNft::deserialize(instruction.data.as_slice())
        {
            return Some(carbon_core::instruction::DecodedInstruction {
                program_id: instruction.program_id,
                data: MplTokenMetadataInstruction::BurnEditionNft(decoded_instruction),
            });
        }
        if let Some(decoded_instruction) =
            create_escrow_account::CreateEscrowAccount::deserialize(instruction.data.as_slice())
        {
            return Some(carbon_core::instruction::DecodedInstruction {
                program_id: instruction.program_id,
                data: MplTokenMetadataInstruction::CreateEscrowAccount(decoded_instruction),
            });
        }
        if let Some(decoded_instruction) =
            close_escrow_account::CloseEscrowAccount::deserialize(instruction.data.as_slice())
        {
            return Some(carbon_core::instruction::DecodedInstruction {
                program_id: instruction.program_id,
                data: MplTokenMetadataInstruction::CloseEscrowAccount(decoded_instruction),
            });
        }
        if let Some(decoded_instruction) =
            transfer_out_of_escrow::TransferOutOfEscrow::deserialize(instruction.data.as_slice())
        {
            return Some(carbon_core::instruction::DecodedInstruction {
                program_id: instruction.program_id,
                data: MplTokenMetadataInstruction::TransferOutOfEscrow(decoded_instruction),
            });
        }
        if let Some(decoded_instruction) = burn::Burn::deserialize(instruction.data.as_slice()) {
            return Some(carbon_core::instruction::DecodedInstruction {
                program_id: instruction.program_id,
                data: MplTokenMetadataInstruction::Burn(decoded_instruction),
            });
        }
        if let Some(decoded_instruction) = create::Create::deserialize(instruction.data.as_slice())
        {
            return Some(carbon_core::instruction::DecodedInstruction {
                program_id: instruction.program_id,
                data: MplTokenMetadataInstruction::Create(decoded_instruction),
            });
        }
        if let Some(decoded_instruction) = mint::Mint::deserialize(instruction.data.as_slice()) {
            return Some(carbon_core::instruction::DecodedInstruction {
                program_id: instruction.program_id,
                data: MplTokenMetadataInstruction::Mint(decoded_instruction),
            });
        }
        if let Some(decoded_instruction) =
            delegate::Delegate::deserialize(instruction.data.as_slice())
        {
            return Some(carbon_core::instruction::DecodedInstruction {
                program_id: instruction.program_id,
                data: MplTokenMetadataInstruction::Delegate(decoded_instruction),
            });
        }
        if let Some(decoded_instruction) = revoke::Revoke::deserialize(instruction.data.as_slice())
        {
            return Some(carbon_core::instruction::DecodedInstruction {
                program_id: instruction.program_id,
                data: MplTokenMetadataInstruction::Revoke(decoded_instruction),
            });
        }
        if let Some(decoded_instruction) = lock::Lock::deserialize(instruction.data.as_slice()) {
            return Some(carbon_core::instruction::DecodedInstruction {
                program_id: instruction.program_id,
                data: MplTokenMetadataInstruction::Lock(decoded_instruction),
            });
        }
        if let Some(decoded_instruction) = unlock::Unlock::deserialize(instruction.data.as_slice())
        {
            return Some(carbon_core::instruction::DecodedInstruction {
                program_id: instruction.program_id,
                data: MplTokenMetadataInstruction::Unlock(decoded_instruction),
            });
        }
        if let Some(decoded_instruction) =
            migrate::Migrate::deserialize(instruction.data.as_slice())
        {
            return Some(carbon_core::instruction::DecodedInstruction {
                program_id: instruction.program_id,
                data: MplTokenMetadataInstruction::Migrate(decoded_instruction),
            });
        }
        if let Some(decoded_instruction) =
            transfer::Transfer::deserialize(instruction.data.as_slice())
        {
            return Some(carbon_core::instruction::DecodedInstruction {
                program_id: instruction.program_id,
                data: MplTokenMetadataInstruction::Transfer(decoded_instruction),
            });
        }
        if let Some(decoded_instruction) = update::Update::deserialize(instruction.data.as_slice())
        {
            return Some(carbon_core::instruction::DecodedInstruction {
                program_id: instruction.program_id,
                data: MplTokenMetadataInstruction::Update(decoded_instruction),
            });
        }
        if let Some(decoded_instruction) = verify::Verify::deserialize(instruction.data.as_slice())
        {
            return Some(carbon_core::instruction::DecodedInstruction {
                program_id: instruction.program_id,
                data: MplTokenMetadataInstruction::Verify(decoded_instruction),
            });
        }
        if let Some(decoded_instruction) =
            unverify::Unverify::deserialize(instruction.data.as_slice())
        {
            return Some(carbon_core::instruction::DecodedInstruction {
                program_id: instruction.program_id,
                data: MplTokenMetadataInstruction::Unverify(decoded_instruction),
            });
        }
        if let Some(decoded_instruction) =
            collect::Collect::deserialize(instruction.data.as_slice())
        {
            return Some(carbon_core::instruction::DecodedInstruction {
                program_id: instruction.program_id,
                data: MplTokenMetadataInstruction::Collect(decoded_instruction),
            });
        }
        if let Some(decoded_instruction) = print::Print::deserialize(instruction.data.as_slice()) {
            return Some(carbon_core::instruction::DecodedInstruction {
                program_id: instruction.program_id,
                data: MplTokenMetadataInstruction::Print(decoded_instruction),
            });
        }

        None
    }
}
