use carbon_core::account::AccountDecoder;
use carbon_core::deserialize::CarbonDeserialize;

use super::SharkyDecoder;
pub mod escrow_pda;
pub mod loan;
pub mod nft_list;
pub mod order_book;
pub mod program_version;

pub enum SharkyAccount {
    OrderBook(order_book::OrderBook),
    Loan(loan::Loan),
    NftList(nft_list::NftList),
    EscrowPda(escrow_pda::EscrowPda),
    ProgramVersion(program_version::ProgramVersion),
}

impl<'a> AccountDecoder<'a> for SharkyDecoder {
    type AccountType = SharkyAccount;
    fn decode_account(
        &self,
        account: &solana_sdk::account::Account,
    ) -> Option<carbon_core::account::DecodedAccount<Self::AccountType>> {
        if let Some(decoded_account) = order_book::OrderBook::deserialize(account.data.as_slice()) {
            return Some(carbon_core::account::DecodedAccount {
                lamports: account.lamports,
                data: SharkyAccount::OrderBook(decoded_account),
                owner: account.owner,
                executable: account.executable,
                rent_epoch: account.rent_epoch,
            });
        }

        if let Some(decoded_account) = loan::Loan::deserialize(account.data.as_slice()) {
            return Some(carbon_core::account::DecodedAccount {
                lamports: account.lamports,
                data: SharkyAccount::Loan(decoded_account),
                owner: account.owner,
                executable: account.executable,
                rent_epoch: account.rent_epoch,
            });
        }

        if let Some(decoded_account) = nft_list::NftList::deserialize(account.data.as_slice()) {
            return Some(carbon_core::account::DecodedAccount {
                lamports: account.lamports,
                data: SharkyAccount::NftList(decoded_account),
                owner: account.owner,
                executable: account.executable,
                rent_epoch: account.rent_epoch,
            });
        }

        if let Some(decoded_account) = escrow_pda::EscrowPda::deserialize(account.data.as_slice()) {
            return Some(carbon_core::account::DecodedAccount {
                lamports: account.lamports,
                data: SharkyAccount::EscrowPda(decoded_account),
                owner: account.owner,
                executable: account.executable,
                rent_epoch: account.rent_epoch,
            });
        }

        if let Some(decoded_account) =
            program_version::ProgramVersion::deserialize(account.data.as_slice())
        {
            return Some(carbon_core::account::DecodedAccount {
                lamports: account.lamports,
                data: SharkyAccount::ProgramVersion(decoded_account),
                owner: account.owner,
                executable: account.executable,
                rent_epoch: account.rent_epoch,
            });
        }

        None
    }
}
