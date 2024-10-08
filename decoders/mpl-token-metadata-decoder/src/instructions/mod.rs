use carbon_core::deserialize::CarbonDeserialize;
use carbon_core::instruction::InstructionDecoder;
use carbon_macros::try_decode_instructions;

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
pub mod use_ix;
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
    Use(use_ix::Use),
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
        try_decode_instructions!(instruction,
            MplTokenMetadataInstruction::CreateMetadataAccount => create_metadata_account::CreateMetadataAccount,
            MplTokenMetadataInstruction::UpdateMetadataAccount => update_metadata_account::UpdateMetadataAccount,
            MplTokenMetadataInstruction::DeprecatedCreateMasterEdition => deprecated_create_master_edition::DeprecatedCreateMasterEdition,
            MplTokenMetadataInstruction::DeprecatedMintNewEditionFromMasterEditionViaPrintingToken => deprecated_mint_new_edition_from_master_edition_via_printing_token::DeprecatedMintNewEditionFromMasterEditionViaPrintingToken,
            MplTokenMetadataInstruction::UpdatePrimarySaleHappenedViaToken => update_primary_sale_happened_via_token::UpdatePrimarySaleHappenedViaToken,
            MplTokenMetadataInstruction::DeprecatedSetReservationList => deprecated_set_reservation_list::DeprecatedSetReservationList,
            MplTokenMetadataInstruction::DeprecatedCreateReservationList => deprecated_create_reservation_list::DeprecatedCreateReservationList,
            MplTokenMetadataInstruction::SignMetadata => sign_metadata::SignMetadata,
            MplTokenMetadataInstruction::DeprecatedMintPrintingTokensViaToken => deprecated_mint_printing_tokens_via_token::DeprecatedMintPrintingTokensViaToken,
            MplTokenMetadataInstruction::DeprecatedMintPrintingTokens => deprecated_mint_printing_tokens::DeprecatedMintPrintingTokens,
            MplTokenMetadataInstruction::CreateMasterEdition => create_master_edition::CreateMasterEdition,
            MplTokenMetadataInstruction::MintNewEditionFromMasterEditionViaToken => mint_new_edition_from_master_edition_via_token::MintNewEditionFromMasterEditionViaToken,
            MplTokenMetadataInstruction::ConvertMasterEditionV1ToV2 => convert_master_edition_v1_to_v2::ConvertMasterEditionV1ToV2,
            MplTokenMetadataInstruction::MintNewEditionFromMasterEditionViaVaultProxy => mint_new_edition_from_master_edition_via_vault_proxy::MintNewEditionFromMasterEditionViaVaultProxy,
            MplTokenMetadataInstruction::PuffMetadata => puff_metadata::PuffMetadata,
            MplTokenMetadataInstruction::UpdateMetadataAccountV2 => update_metadata_account_v2::UpdateMetadataAccountV2,
            MplTokenMetadataInstruction::CreateMetadataAccountV2 => create_metadata_account_v2::CreateMetadataAccountV2,
            MplTokenMetadataInstruction::CreateMasterEditionV3 => create_master_edition_v3::CreateMasterEditionV3,
            MplTokenMetadataInstruction::VerifyCollection => verify_collection::VerifyCollection,
            MplTokenMetadataInstruction::Utilize => utilize::Utilize,
            MplTokenMetadataInstruction::ApproveUseAuthority => approve_use_authority::ApproveUseAuthority,
            MplTokenMetadataInstruction::RevokeUseAuthority => revoke_use_authority::RevokeUseAuthority,
            MplTokenMetadataInstruction::UnverifyCollection => unverify_collection::UnverifyCollection,
            MplTokenMetadataInstruction::ApproveCollectionAuthority => approve_collection_authority::ApproveCollectionAuthority,
            MplTokenMetadataInstruction::RevokeCollectionAuthority => revoke_collection_authority::RevokeCollectionAuthority,
            MplTokenMetadataInstruction::SetAndVerifyCollection => set_and_verify_collection::SetAndVerifyCollection,
            MplTokenMetadataInstruction::FreezeDelegatedAccount => freeze_delegated_account::FreezeDelegatedAccount,
            MplTokenMetadataInstruction::ThawDelegatedAccount => thaw_delegated_account::ThawDelegatedAccount,
            MplTokenMetadataInstruction::RemoveCreatorVerification => remove_creator_verification::RemoveCreatorVerification,
            MplTokenMetadataInstruction::BurnNft => burn_nft::BurnNft,
            MplTokenMetadataInstruction::VerifySizedCollectionItem => verify_sized_collection_item::VerifySizedCollectionItem,
            MplTokenMetadataInstruction::UnverifySizedCollectionItem => unverify_sized_collection_item::UnverifySizedCollectionItem,
            MplTokenMetadataInstruction::SetAndVerifySizedCollectionItem => set_and_verify_sized_collection_item::SetAndVerifySizedCollectionItem,
            MplTokenMetadataInstruction::CreateMetadataAccountV3 => create_metadata_account_v3::CreateMetadataAccountV3,
            MplTokenMetadataInstruction::SetCollectionSize => set_collection_size::SetCollectionSize,
            MplTokenMetadataInstruction::SetTokenStandard => set_token_standard::SetTokenStandard,
            MplTokenMetadataInstruction::BubblegumSetCollectionSize => bubblegum_set_collection_size::BubblegumSetCollectionSize,
            MplTokenMetadataInstruction::BurnEditionNft => burn_edition_nft::BurnEditionNft,
            MplTokenMetadataInstruction::CreateEscrowAccount => create_escrow_account::CreateEscrowAccount,
            MplTokenMetadataInstruction::CloseEscrowAccount => close_escrow_account::CloseEscrowAccount,
            MplTokenMetadataInstruction::TransferOutOfEscrow => transfer_out_of_escrow::TransferOutOfEscrow,
            MplTokenMetadataInstruction::Burn => burn::Burn,
            MplTokenMetadataInstruction::Create => create::Create,
            MplTokenMetadataInstruction::Mint => mint::Mint,
            MplTokenMetadataInstruction::Delegate => delegate::Delegate,
            MplTokenMetadataInstruction::Revoke => revoke::Revoke,
            MplTokenMetadataInstruction::Lock => lock::Lock,
            MplTokenMetadataInstruction::Unlock => unlock::Unlock,
            MplTokenMetadataInstruction::Migrate => migrate::Migrate,
            MplTokenMetadataInstruction::Transfer => transfer::Transfer,
            MplTokenMetadataInstruction::Update => update::Update,
            MplTokenMetadataInstruction::Use => use_ix::Use,
            MplTokenMetadataInstruction::Verify => verify::Verify,
            MplTokenMetadataInstruction::Unverify => unverify::Unverify,
            MplTokenMetadataInstruction::Collect => collect::Collect,
            MplTokenMetadataInstruction::Print => print::Print,
        )
    }
}
