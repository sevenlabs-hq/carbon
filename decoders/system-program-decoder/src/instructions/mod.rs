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
    carbon_core::InstructionType, serde::Serialize, serde::Deserialize, PartialEq, Debug, Clone,
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
        instruction: &solana_sdk::instruction::Instruction,
    ) -> Option<carbon_core::instruction::DecodedInstruction<Self::InstructionType>> {
        if !instruction.program_id.eq(&solana_sdk::system_program::id()) {
            return None;
        }
        println!("Decoding system program instruction: {:?}", instruction);
        println!("data len: {:?}", instruction.data.len());

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
    use carbon_core::{
        deserialize::{ArrangeAccounts, PrefixString},
        instruction::InstructionDecoder,
    };
    use solana_sdk::{instruction::AccountMeta, pubkey};

    use super::*;

    #[test]
    fn test_decode_create_with_seed() {
        // Arrange
        let expected_ix = SystemProgramInstruction::CreateAccountWithSeed(
            create_account_with_seed::CreateAccountWithSeed {
                base: pubkey!("6bBmDxYqXeFbXN8SmtjTpiA3SrEDKsxK8RG6yhPGpa9G"),
                seed: PrefixString("CF9nRGJcFhH57xgcPxaamBs5pHxHexP9".to_string()),
                space: 165,
                amount: 1283531083,
                program_address: pubkey!("TokenkegQfeZyiNwAJbNbGKPFXCWuBvf9Ss623VQ5DA"),
            },
        );
        let expected_accounts = vec![
            AccountMeta::new(
                pubkey!("6bBmDxYqXeFbXN8SmtjTpiA3SrEDKsxK8RG6yhPGpa9G"),
                true,
            ),
            AccountMeta::new(
                pubkey!("3MoeLKJVQHNUtTAXEurLAQtCSXpLGAvairYEHpkqW6CC"),
                false,
            ),
        ];
        let expected_arranged_accounts =
            create_account_with_seed::CreateAccountWithSeedInstructionAccounts {
                payer: pubkey!("6bBmDxYqXeFbXN8SmtjTpiA3SrEDKsxK8RG6yhPGpa9G"),
                new_account: pubkey!("3MoeLKJVQHNUtTAXEurLAQtCSXpLGAvairYEHpkqW6CC"),
                base_account: pubkey!("6bBmDxYqXeFbXN8SmtjTpiA3SrEDKsxK8RG6yhPGpa9G"),
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
        assert_eq!(decoded.program_id, solana_sdk::system_program::id());
        assert_eq!(decoded_arranged_accounts, expected_arranged_accounts);
    }
}
