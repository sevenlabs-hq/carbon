use {
    super::JupiterDcaDecoder,
    crate::PROGRAM_ID,
    carbon_core::{account::AccountDecoder, deserialize::CarbonDeserialize},
};
pub mod dca;

pub enum JupiterDcaAccount {
    Dca(dca::Dca),
}

impl AccountDecoder<'_> for JupiterDcaDecoder {
    type AccountType = JupiterDcaAccount;
    fn decode_account(
        &self,
        account: &solana_account::Account,
    ) -> Option<carbon_core::account::DecodedAccount<Self::AccountType>> {
        if !account.owner.eq(&PROGRAM_ID) {
            return None;
        }

        if let Some(decoded_account) = dca::Dca::deserialize(account.data.as_slice()) {
            return Some(carbon_core::account::DecodedAccount {
                lamports: account.lamports,
                data: JupiterDcaAccount::Dca(decoded_account),
                owner: account.owner,
                executable: account.executable,
                rent_epoch: account.rent_epoch,
            });
        }

        None
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_decode_dca_account() {
        let expected_dca_account = dca::Dca {
            user: solana_pubkey::Pubkey::from_str_const(
                "EjJLRurptQpJCPQhC62V7D3CmEV5bvMo2tZG1B8x1Td8",
            ),
            input_mint: solana_pubkey::Pubkey::from_str_const(
                "JxxWsvm9jHt4ah7DT9NuLyVLYZcZLUdPD93PcPQ71Ka",
            ),
            output_mint: solana_pubkey::Pubkey::from_str_const(
                "UxxvnMnUGoXJig9wc8phE1mYJZMFq3hpfr259zMWcq4",
            ),
            idx: 1705434692,
            next_cycle_at: 1705434698,
            in_deposited: 100000000000,
            in_withdrawn: 0,
            out_withdrawn: 0,
            in_used: 0,
            out_received: 0,
            in_amount_per_cycle: 16666666666,
            cycle_frequency: 300,
            next_cycle_amount_left: 16666666666,
            in_account: solana_pubkey::Pubkey::from_str_const(
                "DKFfnUUVWfXEnUFgNq7Rrrv6mY96nmugBGz4tpw7VDnw",
            ),
            out_account: solana_pubkey::Pubkey::from_str_const(
                "4aurjtonLaxwaAuScXu3VZCHh6UnZcyxgPWEwy4tXQyS",
            ),
            min_out_amount: 0,
            max_out_amount: 8333333,
            keeper_in_balance_before_borrow: 0,
            dca_out_balance_before_swap: 0,
            created_at: 1705434698,
            bump: 249,
        };

        let decoder = JupiterDcaDecoder;
        let account = carbon_test_utils::read_account("tests/fixtures/dca_account.json")
            .expect("read fixture");
        let decoded_account = decoder.decode_account(&account).expect("decode fixture");

        match decoded_account.data {
            JupiterDcaAccount::Dca(dca_account) => {
                assert_eq!(expected_dca_account.user, dca_account.user);
                assert_eq!(expected_dca_account.input_mint, dca_account.input_mint);
                assert_eq!(expected_dca_account.output_mint, dca_account.output_mint);
                assert_eq!(expected_dca_account.idx, dca_account.idx);
                assert_eq!(
                    expected_dca_account.next_cycle_at,
                    dca_account.next_cycle_at
                );
                assert_eq!(expected_dca_account.in_deposited, dca_account.in_deposited);
                assert_eq!(expected_dca_account.in_withdrawn, dca_account.in_withdrawn);
                assert_eq!(
                    expected_dca_account.out_withdrawn,
                    dca_account.out_withdrawn
                );
                assert_eq!(expected_dca_account.in_used, dca_account.in_used);
                assert_eq!(expected_dca_account.out_received, dca_account.out_received);
                assert_eq!(
                    expected_dca_account.in_amount_per_cycle,
                    dca_account.in_amount_per_cycle
                );
                assert_eq!(
                    expected_dca_account.cycle_frequency,
                    dca_account.cycle_frequency
                );
                assert_eq!(
                    expected_dca_account.next_cycle_amount_left,
                    dca_account.next_cycle_amount_left
                );
                assert_eq!(expected_dca_account.in_account, dca_account.in_account);
                assert_eq!(expected_dca_account.out_account, dca_account.out_account);
                assert_eq!(
                    expected_dca_account.min_out_amount,
                    dca_account.min_out_amount
                );
                assert_eq!(
                    expected_dca_account.max_out_amount,
                    dca_account.max_out_amount
                );
                assert_eq!(
                    expected_dca_account.keeper_in_balance_before_borrow,
                    dca_account.keeper_in_balance_before_borrow
                );
                assert_eq!(
                    expected_dca_account.dca_out_balance_before_swap,
                    dca_account.dca_out_balance_before_swap
                );
                assert_eq!(expected_dca_account.created_at, dca_account.created_at);
                assert_eq!(expected_dca_account.bump, dca_account.bump);
            }
        }
    }
}
