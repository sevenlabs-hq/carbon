use super::SystemProgramDecoder;
pub mod advance_nonce_account;
pub mod allocate;
pub mod allocate_with_seed;
pub mod assign;
pub mod assign_with_seed;
pub mod authorize_nonce_account;
pub mod create_account;
pub mod create_account_with_seed;
pub mod initialize_nonce_account;
pub mod transfer_sol;
pub mod transfer_sol_with_seed;
pub mod upgrade_nonce_account;
pub mod withdraw_nonce_account;

#[derive(
    carbon_core::InstructionType,
    serde::Serialize,
    serde::Deserialize,
    PartialEq,
    Debug,
    Clone,
    Eq,
    Hash,
)]
pub enum SystemProgramInstruction {
    CreateAccount(create_account::CreateAccount),
    Assign(assign::Assign),
    TransferSol(transfer_sol::TransferSol),
    CreateAccountWithSeed(create_account_with_seed::CreateAccountWithSeed),
    AdvanceNonceAccount(advance_nonce_account::AdvanceNonceAccount),
    WithdrawNonceAccount(withdraw_nonce_account::WithdrawNonceAccount),
    InitializeNonceAccount(initialize_nonce_account::InitializeNonceAccount),
    AuthorizeNonceAccount(authorize_nonce_account::AuthorizeNonceAccount),
    Allocate(allocate::Allocate),
    AllocateWithSeed(allocate_with_seed::AllocateWithSeed),
    AssignWithSeed(assign_with_seed::AssignWithSeed),
    TransferSolWithSeed(transfer_sol_with_seed::TransferSolWithSeed),
    UpgradeNonceAccount(upgrade_nonce_account::UpgradeNonceAccount),
}

impl carbon_core::instruction::InstructionDecoder<'_> for SystemProgramDecoder {
    type InstructionType = SystemProgramInstruction;

    fn decode_instruction(
        &self,
        instruction: &solana_instruction::Instruction,
    ) -> Option<carbon_core::instruction::DecodedInstruction<Self::InstructionType>> {
        if !instruction
            .program_id
            .eq(&solana_system_interface::program::id())
        {
            return None;
        }

        carbon_core::try_decode_instructions!(instruction,
            SystemProgramInstruction::CreateAccount => create_account::CreateAccount,
            SystemProgramInstruction::Assign => assign::Assign,
            SystemProgramInstruction::TransferSol => transfer_sol::TransferSol,
            SystemProgramInstruction::CreateAccountWithSeed => create_account_with_seed::CreateAccountWithSeed,
            SystemProgramInstruction::AdvanceNonceAccount => advance_nonce_account::AdvanceNonceAccount,
            SystemProgramInstruction::WithdrawNonceAccount => withdraw_nonce_account::WithdrawNonceAccount,
            SystemProgramInstruction::InitializeNonceAccount => initialize_nonce_account::InitializeNonceAccount,
            SystemProgramInstruction::AuthorizeNonceAccount => authorize_nonce_account::AuthorizeNonceAccount,
            SystemProgramInstruction::Allocate => allocate::Allocate,
            SystemProgramInstruction::AllocateWithSeed => allocate_with_seed::AllocateWithSeed,
            SystemProgramInstruction::AssignWithSeed => assign_with_seed::AssignWithSeed,
            SystemProgramInstruction::TransferSolWithSeed => transfer_sol_with_seed::TransferSolWithSeed,
            SystemProgramInstruction::UpgradeNonceAccount => upgrade_nonce_account::UpgradeNonceAccount,
        )
    }
}

#[cfg(test)]
mod tests {
    use alloc::{string::ToString, vec};
    use carbon_core::{
        deserialize::{ArrangeAccounts, U64PrefixString},
        instruction::InstructionDecoder,
    };
    use solana_instruction::AccountMeta;

    use super::*;

    #[test]
    fn test_decode_create_with_seed() {
        // Arrange
        let expected_ix = SystemProgramInstruction::CreateAccountWithSeed(
            create_account_with_seed::CreateAccountWithSeed {
                base: solana_pubkey::Pubkey::from_str_const(
                    "6bBmDxYqXeFbXN8SmtjTpiA3SrEDKsxK8RG6yhPGpa9G",
                ),
                seed: U64PrefixString("CF9nRGJcFhH57xgcPxaamBs5pHxHexP9".to_string()),
                space: 165,
                amount: 1283531083,
                program_address: solana_pubkey::Pubkey::from_str_const(
                    "TokenkegQfeZyiNwAJbNbGKPFXCWuBvf9Ss623VQ5DA",
                ),
            },
        );
        let expected_accounts = vec![
            AccountMeta::new(
                solana_pubkey::Pubkey::from_str_const(
                    "6bBmDxYqXeFbXN8SmtjTpiA3SrEDKsxK8RG6yhPGpa9G",
                ),
                true,
            ),
            AccountMeta::new(
                solana_pubkey::Pubkey::from_str_const(
                    "3MoeLKJVQHNUtTAXEurLAQtCSXpLGAvairYEHpkqW6CC",
                ),
                false,
            ),
        ];
        let expected_arranged_accounts =
            create_account_with_seed::CreateAccountWithSeedInstructionAccounts {
                payer: solana_pubkey::Pubkey::from_str_const(
                    "6bBmDxYqXeFbXN8SmtjTpiA3SrEDKsxK8RG6yhPGpa9G",
                ),
                new_account: solana_pubkey::Pubkey::from_str_const(
                    "3MoeLKJVQHNUtTAXEurLAQtCSXpLGAvairYEHpkqW6CC",
                ),
                base_account: solana_pubkey::Pubkey::from_str_const(
                    "6bBmDxYqXeFbXN8SmtjTpiA3SrEDKsxK8RG6yhPGpa9G",
                ),
            };

        // Act
        let decoder = SystemProgramDecoder;
        let instruction =
            carbon_test_utils::read_instruction("tests/fixtures/create_with_seed_ix.json")
                .expect("read fixture");
        let decoded = decoder
            .decode_instruction(&instruction)
            .expect("decode instruction");
        let decoded_arranged_accounts =
            create_account_with_seed::CreateAccountWithSeed::arrange_accounts(
                &instruction.accounts,
            )
            .expect("aranage accounts");

        // Assert
        assert_eq!(decoded.data, expected_ix);
        assert_eq!(decoded.accounts, expected_accounts);
        assert_eq!(decoded.program_id, solana_system_interface::program::id());
        assert_eq!(decoded_arranged_accounts, expected_arranged_accounts);
    }
}
