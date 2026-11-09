use {
    carbon_core::instruction::InstructionDecoder,
    carbon_token_2022_decoder::{instructions::Token2022Instruction, types, Token2022Decoder},
    solana_instruction::Instruction,
    solana_pubkey::Pubkey,
    spl_token_2022::{
        extension::{
            default_account_state::instruction::update_default_account_state,
            interest_bearing_mint::instruction::update_rate,
            permissioned_burn::instruction::burn_checked,
            scaled_ui_amount::instruction::update_multiplier,
            transfer_fee::instruction::{
                set_transfer_fee, transfer_checked_with_fee, withdraw_withheld_tokens_from_accounts,
            },
        },
        state::AccountState,
    },
    std::fmt::Debug,
};

const PROGRAM: Pubkey = spl_token_2022::ID;

fn key(seed: u8) -> Pubkey {
    Pubkey::new_from_array([seed; 32])
}

fn decode(instruction: Result<Instruction, impl Debug>) -> Token2022Instruction {
    Token2022Decoder
        .decode_instruction(&instruction.unwrap())
        .expect("the decoder should recognize the instruction")
}

#[test]
fn transfer_checked_with_fee_reads_its_amount_and_fee() {
    let Token2022Instruction::TransferCheckedWithFee { data, .. } =
        decode(transfer_checked_with_fee(
            &PROGRAM,
            &key(1),
            &key(2),
            &key(3),
            &key(4),
            &[],
            1_000_000,
            6,
            2_500,
        ))
    else {
        panic!("decoded as another instruction");
    };
    assert_eq!(
        (data.amount, data.decimals, data.fee),
        (1_000_000, 6, 2_500)
    );
}

#[test]
fn set_transfer_fee_reads_its_fee_settings() {
    let Token2022Instruction::SetTransferFee { data, .. } = decode(set_transfer_fee(
        &PROGRAM,
        &key(1),
        &key(2),
        &[],
        150,
        5_000_000,
    )) else {
        panic!("decoded as another instruction");
    };
    assert_eq!(
        (data.transfer_fee_basis_points, data.maximum_fee),
        (150, 5_000_000)
    );
}

#[test]
fn withdraw_withheld_tokens_from_accounts_reads_its_account_count() {
    let Token2022Instruction::WithdrawWithheldTokensFromAccounts { data, .. } =
        decode(withdraw_withheld_tokens_from_accounts(
            &PROGRAM,
            &key(1),
            &key(2),
            &key(3),
            &[],
            &[&key(4), &key(5)],
        ))
    else {
        panic!("decoded as another instruction");
    };
    assert_eq!(data.num_token_accounts, 2);
}

#[test]
fn update_default_account_state_reads_its_state() {
    let Token2022Instruction::UpdateDefaultAccountState { data, .. } = decode(
        update_default_account_state(&PROGRAM, &key(1), &key(2), &[], &AccountState::Frozen),
    ) else {
        panic!("decoded as another instruction");
    };
    assert_eq!(data.state, types::AccountState::Frozen);
}

#[test]
fn update_rate_reads_its_rate() {
    let Token2022Instruction::UpdateRateInterestBearingMint { data, .. } =
        decode(update_rate(&PROGRAM, &key(1), &key(2), &[], -250))
    else {
        panic!("decoded as another instruction");
    };
    assert_eq!(data.rate, -250);
}

#[test]
fn update_multiplier_reads_its_multiplier_and_timestamp() {
    let Token2022Instruction::UpdateMultiplierScaledUiMint { data, .. } = decode(
        update_multiplier(&PROGRAM, &key(1), &key(2), &[], 1.5, 1_800_000_000),
    ) else {
        panic!("decoded as another instruction");
    };
    assert_eq!(
        (data.multiplier, data.effective_timestamp),
        (1.5, 1_800_000_000)
    );
}

#[test]
fn permissioned_burn_checked_reads_its_amount_and_decimals() {
    let Token2022Instruction::PermissionedBurnChecked { data, .. } = decode(burn_checked(
        &PROGRAM,
        &key(1),
        &key(2),
        &key(3),
        &key(4),
        &[],
        42_000,
        9,
    )) else {
        panic!("decoded as another instruction");
    };
    assert_eq!((data.amount, data.decimals), (42_000, 9));
}
