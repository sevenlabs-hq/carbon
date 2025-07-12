use carbon_core::account::AccountDecoder;
use carbon_core::deserialize::CarbonDeserialize;

use super::TokenMetadataDecoder;
pub mod collection_authority_record;
pub mod edition;
pub mod edition_marker;
pub mod edition_marker_v2;
pub mod holder_delegate_record;
pub mod master_edition_v1;
pub mod master_edition_v2;
pub mod metadata;
pub mod metadata_delegate_record;
pub mod reservation_list_v1;
pub mod reservation_list_v2;
pub mod token_owned_escrow;
pub mod token_record;
pub mod use_authority_record;

pub enum TokenMetadataAccount {
    CollectionAuthorityRecord(collection_authority_record::CollectionAuthorityRecord),
    MetadataDelegateRecord(metadata_delegate_record::MetadataDelegateRecord),
    HolderDelegateRecord(holder_delegate_record::HolderDelegateRecord),
    Edition(edition::Edition),
    EditionMarker(edition_marker::EditionMarker),
    EditionMarkerV2(edition_marker_v2::EditionMarkerV2),
    TokenOwnedEscrow(token_owned_escrow::TokenOwnedEscrow),
    MasterEditionV2(master_edition_v2::MasterEditionV2),
    MasterEditionV1(master_edition_v1::MasterEditionV1),
    Metadata(metadata::Metadata),
    TokenRecord(token_record::TokenRecord),
    ReservationListV2(reservation_list_v2::ReservationListV2),
    ReservationListV1(reservation_list_v1::ReservationListV1),
    UseAuthorityRecord(use_authority_record::UseAuthorityRecord),
}

impl AccountDecoder<'_> for TokenMetadataDecoder {
    type AccountType = TokenMetadataAccount;
    fn decode_account(
        &self,
        account: &solana_account::Account,
    ) -> Option<carbon_core::account::DecodedAccount<Self::AccountType>> {
        if let Some(decoded_account) =
            collection_authority_record::CollectionAuthorityRecord::deserialize(
                account.data.as_slice(),
            )
        {
            return Some(carbon_core::account::DecodedAccount {
                lamports: account.lamports,
                data: TokenMetadataAccount::CollectionAuthorityRecord(decoded_account),
                owner: account.owner,
                executable: account.executable,
                rent_epoch: account.rent_epoch,
            });
        }

        if let Some(decoded_account) =
            metadata_delegate_record::MetadataDelegateRecord::deserialize(account.data.as_slice())
        {
            return Some(carbon_core::account::DecodedAccount {
                lamports: account.lamports,
                data: TokenMetadataAccount::MetadataDelegateRecord(decoded_account),
                owner: account.owner,
                executable: account.executable,
                rent_epoch: account.rent_epoch,
            });
        }

        if let Some(decoded_account) =
            holder_delegate_record::HolderDelegateRecord::deserialize(account.data.as_slice())
        {
            return Some(carbon_core::account::DecodedAccount {
                lamports: account.lamports,
                data: TokenMetadataAccount::HolderDelegateRecord(decoded_account),
                owner: account.owner,
                executable: account.executable,
                rent_epoch: account.rent_epoch,
            });
        }

        if let Some(decoded_account) = edition::Edition::deserialize(account.data.as_slice()) {
            return Some(carbon_core::account::DecodedAccount {
                lamports: account.lamports,
                data: TokenMetadataAccount::Edition(decoded_account),
                owner: account.owner,
                executable: account.executable,
                rent_epoch: account.rent_epoch,
            });
        }

        if let Some(decoded_account) =
            edition_marker::EditionMarker::deserialize(account.data.as_slice())
        {
            return Some(carbon_core::account::DecodedAccount {
                lamports: account.lamports,
                data: TokenMetadataAccount::EditionMarker(decoded_account),
                owner: account.owner,
                executable: account.executable,
                rent_epoch: account.rent_epoch,
            });
        }

        if let Some(decoded_account) =
            edition_marker_v2::EditionMarkerV2::deserialize(account.data.as_slice())
        {
            return Some(carbon_core::account::DecodedAccount {
                lamports: account.lamports,
                data: TokenMetadataAccount::EditionMarkerV2(decoded_account),
                owner: account.owner,
                executable: account.executable,
                rent_epoch: account.rent_epoch,
            });
        }

        if let Some(decoded_account) =
            token_owned_escrow::TokenOwnedEscrow::deserialize(account.data.as_slice())
        {
            return Some(carbon_core::account::DecodedAccount {
                lamports: account.lamports,
                data: TokenMetadataAccount::TokenOwnedEscrow(decoded_account),
                owner: account.owner,
                executable: account.executable,
                rent_epoch: account.rent_epoch,
            });
        }

        if let Some(decoded_account) =
            master_edition_v2::MasterEditionV2::deserialize(account.data.as_slice())
        {
            return Some(carbon_core::account::DecodedAccount {
                lamports: account.lamports,
                data: TokenMetadataAccount::MasterEditionV2(decoded_account),
                owner: account.owner,
                executable: account.executable,
                rent_epoch: account.rent_epoch,
            });
        }

        if let Some(decoded_account) =
            master_edition_v1::MasterEditionV1::deserialize(account.data.as_slice())
        {
            return Some(carbon_core::account::DecodedAccount {
                lamports: account.lamports,
                data: TokenMetadataAccount::MasterEditionV1(decoded_account),
                owner: account.owner,
                executable: account.executable,
                rent_epoch: account.rent_epoch,
            });
        }

        if let Some(decoded_account) = metadata::Metadata::deserialize(account.data.as_slice()) {
            return Some(carbon_core::account::DecodedAccount {
                lamports: account.lamports,
                data: TokenMetadataAccount::Metadata(decoded_account),
                owner: account.owner,
                executable: account.executable,
                rent_epoch: account.rent_epoch,
            });
        }

        if let Some(decoded_account) =
            token_record::TokenRecord::deserialize(account.data.as_slice())
        {
            return Some(carbon_core::account::DecodedAccount {
                lamports: account.lamports,
                data: TokenMetadataAccount::TokenRecord(decoded_account),
                owner: account.owner,
                executable: account.executable,
                rent_epoch: account.rent_epoch,
            });
        }

        if let Some(decoded_account) =
            reservation_list_v2::ReservationListV2::deserialize(account.data.as_slice())
        {
            return Some(carbon_core::account::DecodedAccount {
                lamports: account.lamports,
                data: TokenMetadataAccount::ReservationListV2(decoded_account),
                owner: account.owner,
                executable: account.executable,
                rent_epoch: account.rent_epoch,
            });
        }

        if let Some(decoded_account) =
            reservation_list_v1::ReservationListV1::deserialize(account.data.as_slice())
        {
            return Some(carbon_core::account::DecodedAccount {
                lamports: account.lamports,
                data: TokenMetadataAccount::ReservationListV1(decoded_account),
                owner: account.owner,
                executable: account.executable,
                rent_epoch: account.rent_epoch,
            });
        }

        if let Some(decoded_account) =
            use_authority_record::UseAuthorityRecord::deserialize(account.data.as_slice())
        {
            return Some(carbon_core::account::DecodedAccount {
                lamports: account.lamports,
                data: TokenMetadataAccount::UseAuthorityRecord(decoded_account),
                owner: account.owner,
                executable: account.executable,
                rent_epoch: account.rent_epoch,
            });
        }

        None
    }
}
