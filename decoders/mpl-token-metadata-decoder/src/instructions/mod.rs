use crate::PROGRAM_ID;

use super::TokenMetadataDecoder;
pub mod _use;
pub mod approve_collection_authority;
pub mod approve_use_authority;
pub mod bubblegum_set_collection_size;
pub mod burn;
pub mod burn_edition_nft;
pub mod burn_nft;
pub mod close_accounts;
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
pub mod resize;
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
    carbon_core::InstructionType,
    serde::Serialize,
    serde::Deserialize,
    PartialEq,
    Eq,
    Debug,
    Clone,
    Hash,
)]
pub enum TokenMetadataInstruction {
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
    Use(_use::Use),
    Verify(verify::Verify),
    Unverify(unverify::Unverify),
    Collect(collect::Collect),
    Print(print::Print),
    Resize(resize::Resize),
    CloseAccounts(close_accounts::CloseAccounts),
}

impl carbon_core::instruction::InstructionDecoder<'_> for TokenMetadataDecoder {
    type InstructionType = TokenMetadataInstruction;

    fn decode_instruction(
        &self,
        instruction: &solana_instruction::Instruction,
    ) -> Option<carbon_core::instruction::DecodedInstruction<Self::InstructionType>> {
        if !instruction.program_id.eq(&PROGRAM_ID) {
            return None;
        }

        carbon_core::try_decode_instructions!(instruction,
            TokenMetadataInstruction::CreateMetadataAccount => create_metadata_account::CreateMetadataAccount,
            TokenMetadataInstruction::UpdateMetadataAccount => update_metadata_account::UpdateMetadataAccount,
            TokenMetadataInstruction::DeprecatedCreateMasterEdition => deprecated_create_master_edition::DeprecatedCreateMasterEdition,
            TokenMetadataInstruction::DeprecatedMintNewEditionFromMasterEditionViaPrintingToken => deprecated_mint_new_edition_from_master_edition_via_printing_token::DeprecatedMintNewEditionFromMasterEditionViaPrintingToken,
            TokenMetadataInstruction::UpdatePrimarySaleHappenedViaToken => update_primary_sale_happened_via_token::UpdatePrimarySaleHappenedViaToken,
            TokenMetadataInstruction::DeprecatedSetReservationList => deprecated_set_reservation_list::DeprecatedSetReservationList,
            TokenMetadataInstruction::DeprecatedCreateReservationList => deprecated_create_reservation_list::DeprecatedCreateReservationList,
            TokenMetadataInstruction::SignMetadata => sign_metadata::SignMetadata,
            TokenMetadataInstruction::DeprecatedMintPrintingTokensViaToken => deprecated_mint_printing_tokens_via_token::DeprecatedMintPrintingTokensViaToken,
            TokenMetadataInstruction::DeprecatedMintPrintingTokens => deprecated_mint_printing_tokens::DeprecatedMintPrintingTokens,
            TokenMetadataInstruction::CreateMasterEdition => create_master_edition::CreateMasterEdition,
            TokenMetadataInstruction::MintNewEditionFromMasterEditionViaToken => mint_new_edition_from_master_edition_via_token::MintNewEditionFromMasterEditionViaToken,
            TokenMetadataInstruction::ConvertMasterEditionV1ToV2 => convert_master_edition_v1_to_v2::ConvertMasterEditionV1ToV2,
            TokenMetadataInstruction::MintNewEditionFromMasterEditionViaVaultProxy => mint_new_edition_from_master_edition_via_vault_proxy::MintNewEditionFromMasterEditionViaVaultProxy,
            TokenMetadataInstruction::PuffMetadata => puff_metadata::PuffMetadata,
            TokenMetadataInstruction::UpdateMetadataAccountV2 => update_metadata_account_v2::UpdateMetadataAccountV2,
            TokenMetadataInstruction::CreateMetadataAccountV2 => create_metadata_account_v2::CreateMetadataAccountV2,
            TokenMetadataInstruction::CreateMasterEditionV3 => create_master_edition_v3::CreateMasterEditionV3,
            TokenMetadataInstruction::VerifyCollection => verify_collection::VerifyCollection,
            TokenMetadataInstruction::Utilize => utilize::Utilize,
            TokenMetadataInstruction::ApproveUseAuthority => approve_use_authority::ApproveUseAuthority,
            TokenMetadataInstruction::RevokeUseAuthority => revoke_use_authority::RevokeUseAuthority,
            TokenMetadataInstruction::UnverifyCollection => unverify_collection::UnverifyCollection,
            TokenMetadataInstruction::ApproveCollectionAuthority => approve_collection_authority::ApproveCollectionAuthority,
            TokenMetadataInstruction::RevokeCollectionAuthority => revoke_collection_authority::RevokeCollectionAuthority,
            TokenMetadataInstruction::SetAndVerifyCollection => set_and_verify_collection::SetAndVerifyCollection,
            TokenMetadataInstruction::FreezeDelegatedAccount => freeze_delegated_account::FreezeDelegatedAccount,
            TokenMetadataInstruction::ThawDelegatedAccount => thaw_delegated_account::ThawDelegatedAccount,
            TokenMetadataInstruction::RemoveCreatorVerification => remove_creator_verification::RemoveCreatorVerification,
            TokenMetadataInstruction::BurnNft => burn_nft::BurnNft,
            TokenMetadataInstruction::VerifySizedCollectionItem => verify_sized_collection_item::VerifySizedCollectionItem,
            TokenMetadataInstruction::UnverifySizedCollectionItem => unverify_sized_collection_item::UnverifySizedCollectionItem,
            TokenMetadataInstruction::SetAndVerifySizedCollectionItem => set_and_verify_sized_collection_item::SetAndVerifySizedCollectionItem,
            TokenMetadataInstruction::CreateMetadataAccountV3 => create_metadata_account_v3::CreateMetadataAccountV3,
            TokenMetadataInstruction::SetCollectionSize => set_collection_size::SetCollectionSize,
            TokenMetadataInstruction::SetTokenStandard => set_token_standard::SetTokenStandard,
            TokenMetadataInstruction::BubblegumSetCollectionSize => bubblegum_set_collection_size::BubblegumSetCollectionSize,
            TokenMetadataInstruction::BurnEditionNft => burn_edition_nft::BurnEditionNft,
            TokenMetadataInstruction::CreateEscrowAccount => create_escrow_account::CreateEscrowAccount,
            TokenMetadataInstruction::CloseEscrowAccount => close_escrow_account::CloseEscrowAccount,
            TokenMetadataInstruction::TransferOutOfEscrow => transfer_out_of_escrow::TransferOutOfEscrow,
            TokenMetadataInstruction::Burn => burn::Burn,
            TokenMetadataInstruction::Create => create::Create,
            TokenMetadataInstruction::Mint => mint::Mint,
            TokenMetadataInstruction::Delegate => delegate::Delegate,
            TokenMetadataInstruction::Revoke => revoke::Revoke,
            TokenMetadataInstruction::Lock => lock::Lock,
            TokenMetadataInstruction::Unlock => unlock::Unlock,
            TokenMetadataInstruction::Migrate => migrate::Migrate,
            TokenMetadataInstruction::Transfer => transfer::Transfer,
            TokenMetadataInstruction::Update => update::Update,
            TokenMetadataInstruction::Use => _use::Use,
            TokenMetadataInstruction::Verify => verify::Verify,
            TokenMetadataInstruction::Unverify => unverify::Unverify,
            TokenMetadataInstruction::Collect => collect::Collect,
            TokenMetadataInstruction::Print => print::Print,
            TokenMetadataInstruction::Resize => resize::Resize,
            TokenMetadataInstruction::CloseAccounts => close_accounts::CloseAccounts,
        )
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::{
        instructions::{
            create_metadata_account_v3::{
                CreateMetadataAccountV3, CreateMetadataAccountV3InstructionAccounts,
            },
            TokenMetadataInstruction,
        },
        types::{CreateMetadataAccountArgsV3, Creator, DataV2},
        TokenMetadataDecoder,
    };
    use carbon_core::{deserialize::ArrangeAccounts, instruction::InstructionDecoder};
    use carbon_test_utils::read_instruction;
    use solana_instruction::AccountMeta;
    use solana_pubkey::pubkey;

    #[test]
    fn test_decode_create_metadata_v3_without_rent() {
        let expected_ix = TokenMetadataInstruction::CreateMetadataAccountV3(CreateMetadataAccountV3 {
            create_metadata_account_args_v3: CreateMetadataAccountArgsV3 {
                data: DataV2 {
                    name: "スキズー".to_owned(),
                    symbol: "SKZOO".to_owned(),
                    uri: "https://ipfs.io/ipfs/bafkreigwusbaqy7cgbh3mvij5gu4c2m4msaceoutpw5fyisypych7glzjm".to_owned(),
                    seller_fee_basis_points: 0,
                    creators: Some(
                        vec![
                            Creator {
                            address: pubkey!("WLHv2UAZm6z4KyaaELi5pjdbJh6RESMva1Rnn8pJVVh"),
                            verified: true,
                            share: 100,
                        }
                        ]
                    ),
                    collection: None,
                    uses: None,
                },
                is_mutable: false,
                collection_details: None,
            }
        });

        let expected_accounts = vec![
            AccountMeta::new(
                pubkey!("FURfzvnjVPdjrfMBRzeSgahzS1xietCqiv8SG9pCS8ke"),
                false,
            ),
            AccountMeta::new(
                pubkey!("A3p836DWHzDA3DY73QfJSLCqekkhGohNXVAHAa1Qbonk"),
                true,
            ),
            AccountMeta::new_readonly(
                pubkey!("WLHv2UAZm6z4KyaaELi5pjdbJh6RESMva1Rnn8pJVVh"),
                false,
            ),
            AccountMeta::new(
                pubkey!("5f5BPCCNeMkekfjFZi18kAYU95rRdM2jToGaNZZwYZX6"),
                true,
            ),
            AccountMeta::new_readonly(
                pubkey!("WLHv2UAZm6z4KyaaELi5pjdbJh6RESMva1Rnn8pJVVh"),
                false,
            ),
            AccountMeta::new_readonly(pubkey!("11111111111111111111111111111111"), false),
        ];

        let expected_arranged_accounts = CreateMetadataAccountV3InstructionAccounts {
            metadata: pubkey!("FURfzvnjVPdjrfMBRzeSgahzS1xietCqiv8SG9pCS8ke"),
            mint: pubkey!("A3p836DWHzDA3DY73QfJSLCqekkhGohNXVAHAa1Qbonk"),
            mint_authority: pubkey!("WLHv2UAZm6z4KyaaELi5pjdbJh6RESMva1Rnn8pJVVh"),
            payer: pubkey!("5f5BPCCNeMkekfjFZi18kAYU95rRdM2jToGaNZZwYZX6"),
            update_authority: pubkey!("WLHv2UAZm6z4KyaaELi5pjdbJh6RESMva1Rnn8pJVVh"),
            system_program: pubkey!("11111111111111111111111111111111"),
            rent: None,
        };

        let decoder = TokenMetadataDecoder;

        const FIXTURE_PATH: &str = "tests/fixtures/create_metadata_v3_ix.json";
        let instruction = read_instruction(FIXTURE_PATH).expect("read fixture");

        let decoded_instruction = decoder
            .decode_instruction(&instruction)
            .expect("decode instruction");

        let decoded_arranged_accounts =
            CreateMetadataAccountV3::arrange_accounts(&instruction.accounts)
                .expect("arrange accounts");

        assert_eq!(decoded_instruction.data, expected_ix);
        assert_eq!(decoded_instruction.accounts, expected_accounts);
        assert_eq!(decoded_instruction.program_id, PROGRAM_ID);
        assert!(decoded_arranged_accounts.rent.is_none());
        assert_eq!(decoded_arranged_accounts, expected_arranged_accounts);
    }
}
