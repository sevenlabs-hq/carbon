 
use carbon_core::account::AccountDecoder; 
use carbon_core::deserialize::CarbonDeserialize;
 

use super::MplCoreDecoder; 
pub mod uninitialized; 
pub mod asset_v1; 
pub mod hashed_asset_v1; 
pub mod plugin_header_v1; 
pub mod plugin_registry_v1; 
pub mod collection_v1; 

pub enum MplCoreAccount { 
        Uninitialized(uninitialized::Uninitialized), 
        AssetV1(asset_v1::AssetV1), 
        HashedAssetV1(hashed_asset_v1::HashedAssetV1), 
        PluginHeaderV1(plugin_header_v1::PluginHeaderV1), 
        PluginRegistryV1(plugin_registry_v1::PluginRegistryV1), 
        CollectionV1(collection_v1::CollectionV1), 
}


impl<'a> AccountDecoder<'a> for MplCoreDecoder { 
    type AccountType = MplCoreAccount;
     fn decode_account( &self, account: &solana_sdk::account::Account, ) -> Option<carbon_core::account::DecodedAccount<Self::AccountType>> { 
         
            if let Some(decoded_account) = uninitialized::Uninitialized::deserialize(account.data.as_slice()) { 
            return Some(carbon_core::account::DecodedAccount { 
                lamports: account.lamports, 
                data: MplCoreAccount::Uninitialized(decoded_account), 
                owner: account.owner, 
                executable: account.executable, 
                rent_epoch: account.rent_epoch, 
            }); 
        } 
         
            if let Some(decoded_account) = asset_v1::AssetV1::deserialize(account.data.as_slice()) { 
            return Some(carbon_core::account::DecodedAccount { 
                lamports: account.lamports, 
                data: MplCoreAccount::AssetV1(decoded_account), 
                owner: account.owner, 
                executable: account.executable, 
                rent_epoch: account.rent_epoch, 
            }); 
        } 
         
            if let Some(decoded_account) = hashed_asset_v1::HashedAssetV1::deserialize(account.data.as_slice()) { 
            return Some(carbon_core::account::DecodedAccount { 
                lamports: account.lamports, 
                data: MplCoreAccount::HashedAssetV1(decoded_account), 
                owner: account.owner, 
                executable: account.executable, 
                rent_epoch: account.rent_epoch, 
            }); 
        } 
         
            if let Some(decoded_account) = plugin_header_v1::PluginHeaderV1::deserialize(account.data.as_slice()) { 
            return Some(carbon_core::account::DecodedAccount { 
                lamports: account.lamports, 
                data: MplCoreAccount::PluginHeaderV1(decoded_account), 
                owner: account.owner, 
                executable: account.executable, 
                rent_epoch: account.rent_epoch, 
            }); 
        } 
         
            if let Some(decoded_account) = plugin_registry_v1::PluginRegistryV1::deserialize(account.data.as_slice()) { 
            return Some(carbon_core::account::DecodedAccount { 
                lamports: account.lamports, 
                data: MplCoreAccount::PluginRegistryV1(decoded_account), 
                owner: account.owner, 
                executable: account.executable, 
                rent_epoch: account.rent_epoch, 
            }); 
        } 
         
            if let Some(decoded_account) = collection_v1::CollectionV1::deserialize(account.data.as_slice()) { 
            return Some(carbon_core::account::DecodedAccount { 
                lamports: account.lamports, 
                data: MplCoreAccount::CollectionV1(decoded_account), 
                owner: account.owner, 
                executable: account.executable, 
                rent_epoch: account.rent_epoch, 
            }); 
        } 
         
    None 
    } 
}
