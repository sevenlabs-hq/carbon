 
use carbon_core::account::AccountDecoder; 
use carbon_core::deserialize::CarbonDeserialize;
 

use super::MplTokenMetadataDecoder; 
pub mod uninitialized; 
pub mod edition; 
pub mod master_edition_v1; 
pub mod reservation_list_v1; 
pub mod metadata; 
pub mod reservation_list_v2; 
pub mod master_edition_v2; 
pub mod edition_marker; 
pub mod use_authority_record; 
pub mod collection_authority_record; 
pub mod token_owned_escrow; 
pub mod token_record; 
pub mod metadata_delegate_record; 
pub mod edition_marker_v2; 

pub enum MplTokenMetadataAccount { 
        Uninitialized(uninitialized::Uninitialized), 
        Edition(edition::Edition), 
        MasterEditionV1(master_edition_v1::MasterEditionV1), 
        ReservationListV1(reservation_list_v1::ReservationListV1), 
        Metadata(metadata::Metadata), 
        ReservationListV2(reservation_list_v2::ReservationListV2), 
        MasterEditionV2(master_edition_v2::MasterEditionV2), 
        EditionMarker(edition_marker::EditionMarker), 
        UseAuthorityRecord(use_authority_record::UseAuthorityRecord), 
        CollectionAuthorityRecord(collection_authority_record::CollectionAuthorityRecord), 
        TokenOwnedEscrow(token_owned_escrow::TokenOwnedEscrow), 
        TokenRecord(token_record::TokenRecord), 
        MetadataDelegateRecord(metadata_delegate_record::MetadataDelegateRecord), 
        EditionMarkerV2(edition_marker_v2::EditionMarkerV2), 
}


impl AccountDecoder for MplTokenMetadataDecoder { 
    type AccountType = MplTokenMetadataAccount;
     fn decode_account( &self, account: solana_sdk::account::Account, ) -> Option<carbon_core::account::DecodedAccount<Self::AccountType>> { 
         
            if let Some(decoded_account) = uninitialized::Uninitialized::deserialize(account.data.as_slice()) { 
            return Some(carbon_core::account::DecodedAccount { 
                lamports: account.lamports, 
                data: MplTokenMetadataAccount::Uninitialized(decoded_account), 
                owner: account.owner, 
                executable: account.executable, 
                rent_epoch: account.rent_epoch, 
            }); 
        } 
         
            if let Some(decoded_account) = edition::Edition::deserialize(account.data.as_slice()) { 
            return Some(carbon_core::account::DecodedAccount { 
                lamports: account.lamports, 
                data: MplTokenMetadataAccount::Edition(decoded_account), 
                owner: account.owner, 
                executable: account.executable, 
                rent_epoch: account.rent_epoch, 
            }); 
        } 
         
            if let Some(decoded_account) = master_edition_v1::MasterEditionV1::deserialize(account.data.as_slice()) { 
            return Some(carbon_core::account::DecodedAccount { 
                lamports: account.lamports, 
                data: MplTokenMetadataAccount::MasterEditionV1(decoded_account), 
                owner: account.owner, 
                executable: account.executable, 
                rent_epoch: account.rent_epoch, 
            }); 
        } 
         
            if let Some(decoded_account) = reservation_list_v1::ReservationListV1::deserialize(account.data.as_slice()) { 
            return Some(carbon_core::account::DecodedAccount { 
                lamports: account.lamports, 
                data: MplTokenMetadataAccount::ReservationListV1(decoded_account), 
                owner: account.owner, 
                executable: account.executable, 
                rent_epoch: account.rent_epoch, 
            }); 
        } 
         
            if let Some(decoded_account) = metadata::Metadata::deserialize(account.data.as_slice()) { 
            return Some(carbon_core::account::DecodedAccount { 
                lamports: account.lamports, 
                data: MplTokenMetadataAccount::Metadata(decoded_account), 
                owner: account.owner, 
                executable: account.executable, 
                rent_epoch: account.rent_epoch, 
            }); 
        } 
         
            if let Some(decoded_account) = reservation_list_v2::ReservationListV2::deserialize(account.data.as_slice()) { 
            return Some(carbon_core::account::DecodedAccount { 
                lamports: account.lamports, 
                data: MplTokenMetadataAccount::ReservationListV2(decoded_account), 
                owner: account.owner, 
                executable: account.executable, 
                rent_epoch: account.rent_epoch, 
            }); 
        } 
         
            if let Some(decoded_account) = master_edition_v2::MasterEditionV2::deserialize(account.data.as_slice()) { 
            return Some(carbon_core::account::DecodedAccount { 
                lamports: account.lamports, 
                data: MplTokenMetadataAccount::MasterEditionV2(decoded_account), 
                owner: account.owner, 
                executable: account.executable, 
                rent_epoch: account.rent_epoch, 
            }); 
        } 
         
            if let Some(decoded_account) = edition_marker::EditionMarker::deserialize(account.data.as_slice()) { 
            return Some(carbon_core::account::DecodedAccount { 
                lamports: account.lamports, 
                data: MplTokenMetadataAccount::EditionMarker(decoded_account), 
                owner: account.owner, 
                executable: account.executable, 
                rent_epoch: account.rent_epoch, 
            }); 
        } 
         
            if let Some(decoded_account) = use_authority_record::UseAuthorityRecord::deserialize(account.data.as_slice()) { 
            return Some(carbon_core::account::DecodedAccount { 
                lamports: account.lamports, 
                data: MplTokenMetadataAccount::UseAuthorityRecord(decoded_account), 
                owner: account.owner, 
                executable: account.executable, 
                rent_epoch: account.rent_epoch, 
            }); 
        } 
         
            if let Some(decoded_account) = collection_authority_record::CollectionAuthorityRecord::deserialize(account.data.as_slice()) { 
            return Some(carbon_core::account::DecodedAccount { 
                lamports: account.lamports, 
                data: MplTokenMetadataAccount::CollectionAuthorityRecord(decoded_account), 
                owner: account.owner, 
                executable: account.executable, 
                rent_epoch: account.rent_epoch, 
            }); 
        } 
         
            if let Some(decoded_account) = token_owned_escrow::TokenOwnedEscrow::deserialize(account.data.as_slice()) { 
            return Some(carbon_core::account::DecodedAccount { 
                lamports: account.lamports, 
                data: MplTokenMetadataAccount::TokenOwnedEscrow(decoded_account), 
                owner: account.owner, 
                executable: account.executable, 
                rent_epoch: account.rent_epoch, 
            }); 
        } 
         
            if let Some(decoded_account) = token_record::TokenRecord::deserialize(account.data.as_slice()) { 
            return Some(carbon_core::account::DecodedAccount { 
                lamports: account.lamports, 
                data: MplTokenMetadataAccount::TokenRecord(decoded_account), 
                owner: account.owner, 
                executable: account.executable, 
                rent_epoch: account.rent_epoch, 
            }); 
        } 
         
            if let Some(decoded_account) = metadata_delegate_record::MetadataDelegateRecord::deserialize(account.data.as_slice()) { 
            return Some(carbon_core::account::DecodedAccount { 
                lamports: account.lamports, 
                data: MplTokenMetadataAccount::MetadataDelegateRecord(decoded_account), 
                owner: account.owner, 
                executable: account.executable, 
                rent_epoch: account.rent_epoch, 
            }); 
        } 
         
            if let Some(decoded_account) = edition_marker_v2::EditionMarkerV2::deserialize(account.data.as_slice()) { 
            return Some(carbon_core::account::DecodedAccount { 
                lamports: account.lamports, 
                data: MplTokenMetadataAccount::EditionMarkerV2(decoded_account), 
                owner: account.owner, 
                executable: account.executable, 
                rent_epoch: account.rent_epoch, 
            }); 
        } 
         
    None 
    } 
}