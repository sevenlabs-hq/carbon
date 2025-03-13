use {
    super::SolayerRestakingProgramDecoder,
    carbon_core::{account::AccountDecoder, deserialize::CarbonDeserialize},
    crates::PROGRAM_ID,
};
pub mod restaking_pool;

pub enum SolayerRestakingProgramAccount {
    RestakingPool(restaking_pool::RestakingPool),
}

impl<'a> AccountDecoder<'a> for SolayerRestakingProgramDecoder {
    type AccountType = SolayerRestakingProgramAccount;
    fn decode_account(
        &self,
        account: &solana_sdk::account::Account,
    ) -> Option<carbon_core::account::DecodedAccount<Self::AccountType>> {
        if !account.owner.eq(&PROGRAM_ID) {
            return None;
        }
        if let Some(decoded_account) =
            restaking_pool::RestakingPool::deserialize(account.data.as_slice())
        {
            return Some(carbon_core::account::DecodedAccount {
                lamports: account.lamports,
                data: SolayerRestakingProgramAccount::RestakingPool(decoded_account),
                owner: account.owner,
                executable: account.executable,
                rent_epoch: account.rent_epoch,
            });
        }

        None
    }
}
