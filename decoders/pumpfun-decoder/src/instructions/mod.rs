use crate::PROGRAM_ID;

use super::PumpfunDecoder;
pub mod buy;
pub mod complete_event;
pub mod complete_pump_amm_migration_event;
pub mod create;
pub mod create_event;
pub mod extend_account;
pub mod extend_account_event;
pub mod initialize;
pub mod migrate;
pub mod sell;
pub mod set_params;
pub mod set_params_event;
pub mod trade_event;
pub mod update_global_authority;
pub mod update_global_authority_event;
pub mod withdraw;

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
pub enum PumpfunInstruction {
    Buy(buy::Buy),
    Create(create::Create),
    ExtendAccount(extend_account::ExtendAccount),
    Initialize(initialize::Initialize),
    Migrate(migrate::Migrate),
    Sell(sell::Sell),
    SetParams(set_params::SetParams),
    UpdateGlobalAuthority(update_global_authority::UpdateGlobalAuthority),
    Withdraw(withdraw::Withdraw),
    CompleteEvent(complete_event::CompleteEvent),
    CompletePumpAmmMigrationEvent(complete_pump_amm_migration_event::CompletePumpAmmMigrationEvent),
    CreateEvent(create_event::CreateEvent),
    ExtendAccountEvent(extend_account_event::ExtendAccountEvent),
    SetParamsEvent(set_params_event::SetParamsEvent),
    TradeEvent(trade_event::TradeEvent),
    UpdateGlobalAuthorityEvent(update_global_authority_event::UpdateGlobalAuthorityEvent),
}

impl carbon_core::instruction::InstructionDecoder<'_> for PumpfunDecoder {
    type InstructionType = PumpfunInstruction;

    fn decode_instruction(
        &self,
        instruction: &solana_instruction::Instruction,
    ) -> Option<carbon_core::instruction::DecodedInstruction<Self::InstructionType>> {
        if !instruction.program_id.eq(&PROGRAM_ID) {
            return None;
        }

        carbon_core::try_decode_instructions!(instruction,
            PumpfunInstruction::Buy => buy::Buy,
            PumpfunInstruction::Create => create::Create,
            PumpfunInstruction::ExtendAccount => extend_account::ExtendAccount,
            PumpfunInstruction::Initialize => initialize::Initialize,
            PumpfunInstruction::Migrate => migrate::Migrate,
            PumpfunInstruction::Sell => sell::Sell,
            PumpfunInstruction::SetParams => set_params::SetParams,
            PumpfunInstruction::UpdateGlobalAuthority => update_global_authority::UpdateGlobalAuthority,
            PumpfunInstruction::Withdraw => withdraw::Withdraw,
            PumpfunInstruction::CompleteEvent => complete_event::CompleteEvent,
            PumpfunInstruction::CompletePumpAmmMigrationEvent => complete_pump_amm_migration_event::CompletePumpAmmMigrationEvent,
            PumpfunInstruction::CreateEvent => create_event::CreateEvent,
            PumpfunInstruction::ExtendAccountEvent => extend_account_event::ExtendAccountEvent,
            PumpfunInstruction::SetParamsEvent => set_params_event::SetParamsEvent,
            PumpfunInstruction::TradeEvent => trade_event::TradeEvent,
            PumpfunInstruction::UpdateGlobalAuthorityEvent => update_global_authority_event::UpdateGlobalAuthorityEvent,
        )
    }
}

#[cfg(test)]
mod tests {
    use alloc::{borrow::ToOwned, vec};
    use carbon_core::{deserialize::ArrangeAccounts, instruction::InstructionDecoder};
    use solana_instruction::AccountMeta;
    use solana_pubkey::Pubkey;

    use super::*;

    #[test]
    fn test_decode_buy() {
        // Arrange
        let expected_ix = PumpfunInstruction::Buy(buy::Buy {
            amount: 2712969161192,
            max_sol_cost: 204000000,
        });
        let expected_accounts = vec![
            AccountMeta::new_readonly(
                Pubkey::from_str_const("4wTV1YmiEkRvAtNtsSGPtUrqRYQMe5SKy2uB4Jjaxnjf"),
                false,
            ),
            AccountMeta::new(
                Pubkey::from_str_const("62qc2CNXwrYqQScmEdiZFFAnJR262PxWEuNQtxfafNgV"),
                false,
            ),
            AccountMeta::new_readonly(
                Pubkey::from_str_const("9p1PMtto471A7GvnRJVmDcuqUz3xDd1Lhu8vzrmpump"),
                false,
            ),
            AccountMeta::new(
                Pubkey::from_str_const("HWxwYxr4AV5ytUyT8pvjCEiUrXhwpbx365VpvQ6Bd6MZ"),
                false,
            ),
            AccountMeta::new(
                Pubkey::from_str_const("AUfg9aTAix7YarkHXSBMUyQPCTq55Gg1Z2NTe6utwwzG"),
                false,
            ),
            AccountMeta::new(
                Pubkey::from_str_const("4FLYmjhLuUb5ofNBo1PA9enF7HrPUSYUA1t55tUSFYa5"),
                false,
            ),
            AccountMeta::new(
                Pubkey::from_str_const("5ztadiszGPmBeGVcvmtPyqiHRA8SpU8mqNzPV1WeV88F"),
                true,
            ),
            AccountMeta::new_readonly(
                Pubkey::from_str_const("11111111111111111111111111111111"),
                false,
            ),
            AccountMeta::new_readonly(
                Pubkey::from_str_const("TokenkegQfeZyiNwAJbNbGKPFXCWuBvf9Ss623VQ5DA"),
                false,
            ),
            AccountMeta::new_readonly(
                Pubkey::from_str_const("SysvarRent111111111111111111111111111111111"),
                false,
            ),
            AccountMeta::new_readonly(
                Pubkey::from_str_const("Ce6TQqeHC9p8KetsN6JsjHK7UTZk7nasjjnr7XxXp9F1"),
                false,
            ),
            AccountMeta::new_readonly(
                Pubkey::from_str_const("6EF8rrecthR5Dkzon8Nwu78hRvfCKubJ14M5uBEwF6P"),
                false,
            ),
        ];
        let expected_arranged_accounts = buy::BuyInstructionAccounts {
            global: Pubkey::from_str_const("4wTV1YmiEkRvAtNtsSGPtUrqRYQMe5SKy2uB4Jjaxnjf"),
            fee_recipient: Pubkey::from_str_const("62qc2CNXwrYqQScmEdiZFFAnJR262PxWEuNQtxfafNgV"),
            mint: Pubkey::from_str_const("9p1PMtto471A7GvnRJVmDcuqUz3xDd1Lhu8vzrmpump"),
            bonding_curve: Pubkey::from_str_const("HWxwYxr4AV5ytUyT8pvjCEiUrXhwpbx365VpvQ6Bd6MZ"),
            associated_bonding_curve: Pubkey::from_str_const(
                "AUfg9aTAix7YarkHXSBMUyQPCTq55Gg1Z2NTe6utwwzG",
            ),
            associated_user: Pubkey::from_str_const("4FLYmjhLuUb5ofNBo1PA9enF7HrPUSYUA1t55tUSFYa5"),
            user: Pubkey::from_str_const("5ztadiszGPmBeGVcvmtPyqiHRA8SpU8mqNzPV1WeV88F"),
            system_program: Pubkey::from_str_const("11111111111111111111111111111111"),
            token_program: Pubkey::from_str_const("TokenkegQfeZyiNwAJbNbGKPFXCWuBvf9Ss623VQ5DA"),
            rent: Pubkey::from_str_const("SysvarRent111111111111111111111111111111111"),
            event_authority: Pubkey::from_str_const("Ce6TQqeHC9p8KetsN6JsjHK7UTZk7nasjjnr7XxXp9F1"),
            program: Pubkey::from_str_const("6EF8rrecthR5Dkzon8Nwu78hRvfCKubJ14M5uBEwF6P"),
        };

        // Act
        let decoder = PumpfunDecoder;
        let instruction = carbon_test_utils::read_instruction("tests/fixtures/buy_ix.json")
            .expect("read fixture");
        let decoded = decoder
            .decode_instruction(&instruction)
            .expect("decode instruction");
        let decoded_arranged_accounts =
            buy::Buy::arrange_accounts(&instruction.accounts).expect("aranage accounts");

        // Assert
        assert_eq!(decoded.data, expected_ix);
        assert_eq!(decoded.accounts, expected_accounts);
        assert_eq!(decoded.program_id, PROGRAM_ID);
        assert_eq!(decoded_arranged_accounts, expected_arranged_accounts);
    }

    #[test]
    fn test_decode_sell() {
        // Arrange
        let expected_ix = PumpfunInstruction::Sell(sell::Sell {
            amount: 26705394300,
            min_sol_output: 724522,
        });
        let expected_accounts = vec![
            AccountMeta::new_readonly(
                Pubkey::from_str_const("4wTV1YmiEkRvAtNtsSGPtUrqRYQMe5SKy2uB4Jjaxnjf"),
                false,
            ),
            AccountMeta::new(
                Pubkey::from_str_const("62qc2CNXwrYqQScmEdiZFFAnJR262PxWEuNQtxfafNgV"),
                false,
            ),
            AccountMeta::new_readonly(
                Pubkey::from_str_const("HXfFC4G1aJJo17KW56jJ2iaDLFXq6T8XZjPbQfhspump"),
                false,
            ),
            AccountMeta::new(
                Pubkey::from_str_const("8f12Y6z6CkMmcBqduvThRG2V873CP3eu2iBydqKGDX6y"),
                false,
            ),
            AccountMeta::new(
                Pubkey::from_str_const("GkSscwZJBhcFeB6hpWrnfrE73e5SawPmMuT55U1W4uqz"),
                false,
            ),
            AccountMeta::new(
                Pubkey::from_str_const("Bi6H7WPrZoJmqSauP38NuBaEttGraZkceR4p17ekoTwh"),
                false,
            ),
            AccountMeta::new(
                Pubkey::from_str_const("3bApZNQrP3T6Q1GvK1n1nUPHHnpnsbrEmdGyQyYLEbkP"),
                true,
            ),
            AccountMeta::new_readonly(
                Pubkey::from_str_const("11111111111111111111111111111111"),
                false,
            ),
            AccountMeta::new_readonly(
                Pubkey::from_str_const("ATokenGPvbdGVxr1b2hvZbsiqW5xWH25efTNsLJA8knL"),
                false,
            ),
            AccountMeta::new_readonly(
                Pubkey::from_str_const("TokenkegQfeZyiNwAJbNbGKPFXCWuBvf9Ss623VQ5DA"),
                false,
            ),
            AccountMeta::new_readonly(
                Pubkey::from_str_const("Ce6TQqeHC9p8KetsN6JsjHK7UTZk7nasjjnr7XxXp9F1"),
                false,
            ),
            AccountMeta::new_readonly(
                Pubkey::from_str_const("6EF8rrecthR5Dkzon8Nwu78hRvfCKubJ14M5uBEwF6P"),
                false,
            ),
        ];
        let expected_arranged_accounts = sell::SellInstructionAccounts {
            global: Pubkey::from_str_const("4wTV1YmiEkRvAtNtsSGPtUrqRYQMe5SKy2uB4Jjaxnjf"),
            fee_recipient: Pubkey::from_str_const("62qc2CNXwrYqQScmEdiZFFAnJR262PxWEuNQtxfafNgV"),
            mint: Pubkey::from_str_const("HXfFC4G1aJJo17KW56jJ2iaDLFXq6T8XZjPbQfhspump"),
            bonding_curve: Pubkey::from_str_const("8f12Y6z6CkMmcBqduvThRG2V873CP3eu2iBydqKGDX6y"),
            associated_bonding_curve: Pubkey::from_str_const(
                "GkSscwZJBhcFeB6hpWrnfrE73e5SawPmMuT55U1W4uqz",
            ),
            associated_user: Pubkey::from_str_const("Bi6H7WPrZoJmqSauP38NuBaEttGraZkceR4p17ekoTwh"),
            user: Pubkey::from_str_const("3bApZNQrP3T6Q1GvK1n1nUPHHnpnsbrEmdGyQyYLEbkP"),
            system_program: Pubkey::from_str_const("11111111111111111111111111111111"),
            associated_token_program: Pubkey::from_str_const(
                "ATokenGPvbdGVxr1b2hvZbsiqW5xWH25efTNsLJA8knL",
            ),
            token_program: Pubkey::from_str_const("TokenkegQfeZyiNwAJbNbGKPFXCWuBvf9Ss623VQ5DA"),
            event_authority: Pubkey::from_str_const("Ce6TQqeHC9p8KetsN6JsjHK7UTZk7nasjjnr7XxXp9F1"),
            program: Pubkey::from_str_const("6EF8rrecthR5Dkzon8Nwu78hRvfCKubJ14M5uBEwF6P"),
        };

        // Act
        let decoder = PumpfunDecoder;
        let instruction = carbon_test_utils::read_instruction("tests/fixtures/sell_ix.json")
            .expect("read fixture");
        let decoded = decoder
            .decode_instruction(&instruction)
            .expect("decode instruction");
        let decoded_arranged_accounts =
            sell::Sell::arrange_accounts(&instruction.accounts).expect("aranage accounts");

        // Assert
        assert_eq!(decoded.data, expected_ix);
        assert_eq!(decoded.accounts, expected_accounts);
        assert_eq!(decoded.program_id, PROGRAM_ID);
        assert_eq!(decoded_arranged_accounts, expected_arranged_accounts);
    }

    #[test]
    fn test_decode_create() {
        // Arrange
        let expected_ix = PumpfunInstruction::Create(create::Create {
            name: "Super Elon Bros ".to_owned(),
            symbol: "SEB".to_owned(),
            uri: "https://ipfs.io/ipfs/QmVnjMrWqhMBsmeFnaje87XVxMKY9y7BRL2DtFYJazTGM5".to_owned(),
            creator: Pubkey::from_str_const("7a9xQF38YVW58TPeHavvXiVpqynCxY2GcohsZxdUZCX1"),
        });
        let expected_accounts = vec![
            AccountMeta::new(
                Pubkey::from_str_const("5PweXK19JD4PkafHm9BmpgiTaMoQgKq9EXVkDagwpump"),
                true,
            ),
            AccountMeta::new_readonly(
                Pubkey::from_str_const("TSLvdd1pWpHVjahSpsvCXUbgwsL3JAcvokwaKt1eokM"),
                false,
            ),
            AccountMeta::new(
                Pubkey::from_str_const("Chau1rGA8w4L43rAMUAKXGwq8hpPfGjUoHsiZJEyziKz"),
                false,
            ),
            AccountMeta::new(
                Pubkey::from_str_const("7JxriSri8PukwVQ6VQZ6ErpJ3Km1x6eWMnQeJ2Cd2148"),
                false,
            ),
            AccountMeta::new_readonly(
                Pubkey::from_str_const("4wTV1YmiEkRvAtNtsSGPtUrqRYQMe5SKy2uB4Jjaxnjf"),
                false,
            ),
            AccountMeta::new_readonly(
                Pubkey::from_str_const("metaqbxxUerdq28cj1RbAWkYQm3ybzjb6a8bt518x1s"),
                false,
            ),
            AccountMeta::new(
                Pubkey::from_str_const("Dzp1H2K6sNR8VAqXE9Q6eSdZckp7uQSJKR1FKY9SzFoS"),
                false,
            ),
            AccountMeta::new(
                Pubkey::from_str_const("7a9xQF38YVW58TPeHavvXiVpqynCxY2GcohsZxdUZCX1"),
                true,
            ),
            AccountMeta::new_readonly(
                Pubkey::from_str_const("11111111111111111111111111111111"),
                false,
            ),
            AccountMeta::new_readonly(
                Pubkey::from_str_const("TokenkegQfeZyiNwAJbNbGKPFXCWuBvf9Ss623VQ5DA"),
                false,
            ),
            AccountMeta::new_readonly(
                Pubkey::from_str_const("ATokenGPvbdGVxr1b2hvZbsiqW5xWH25efTNsLJA8knL"),
                false,
            ),
            AccountMeta::new_readonly(
                Pubkey::from_str_const("SysvarRent111111111111111111111111111111111"),
                false,
            ),
            AccountMeta::new_readonly(
                Pubkey::from_str_const("Ce6TQqeHC9p8KetsN6JsjHK7UTZk7nasjjnr7XxXp9F1"),
                false,
            ),
            AccountMeta::new_readonly(
                Pubkey::from_str_const("6EF8rrecthR5Dkzon8Nwu78hRvfCKubJ14M5uBEwF6P"),
                false,
            ),
        ];
        let expected_arranged_accounts = create::CreateInstructionAccounts {
            mint: Pubkey::from_str_const("5PweXK19JD4PkafHm9BmpgiTaMoQgKq9EXVkDagwpump"),
            mint_authority: Pubkey::from_str_const("TSLvdd1pWpHVjahSpsvCXUbgwsL3JAcvokwaKt1eokM"),
            bonding_curve: Pubkey::from_str_const("Chau1rGA8w4L43rAMUAKXGwq8hpPfGjUoHsiZJEyziKz"),
            associated_bonding_curve: Pubkey::from_str_const(
                "7JxriSri8PukwVQ6VQZ6ErpJ3Km1x6eWMnQeJ2Cd2148",
            ),
            global: Pubkey::from_str_const("4wTV1YmiEkRvAtNtsSGPtUrqRYQMe5SKy2uB4Jjaxnjf"),
            mpl_token_metadata: Pubkey::from_str_const(
                "metaqbxxUerdq28cj1RbAWkYQm3ybzjb6a8bt518x1s",
            ),
            metadata: Pubkey::from_str_const("Dzp1H2K6sNR8VAqXE9Q6eSdZckp7uQSJKR1FKY9SzFoS"),
            user: Pubkey::from_str_const("7a9xQF38YVW58TPeHavvXiVpqynCxY2GcohsZxdUZCX1"),
            system_program: Pubkey::from_str_const("11111111111111111111111111111111"),
            token_program: Pubkey::from_str_const("TokenkegQfeZyiNwAJbNbGKPFXCWuBvf9Ss623VQ5DA"),
            associated_token_program: Pubkey::from_str_const(
                "ATokenGPvbdGVxr1b2hvZbsiqW5xWH25efTNsLJA8knL",
            ),
            rent: Pubkey::from_str_const("SysvarRent111111111111111111111111111111111"),
            event_authority: Pubkey::from_str_const("Ce6TQqeHC9p8KetsN6JsjHK7UTZk7nasjjnr7XxXp9F1"),
            program: Pubkey::from_str_const("6EF8rrecthR5Dkzon8Nwu78hRvfCKubJ14M5uBEwF6P"),
        };

        // Act
        let decoder = PumpfunDecoder;
        let instruction = carbon_test_utils::read_instruction("tests/fixtures/create_ix.json")
            .expect("read fixture");
        let decoded = decoder
            .decode_instruction(&instruction)
            .expect("decode instruction");
        let decoded_arranged_accounts =
            create::Create::arrange_accounts(&instruction.accounts).expect("aranage accounts");

        // Assert
        assert_eq!(decoded.data, expected_ix);
        assert_eq!(decoded.accounts, expected_accounts);
        assert_eq!(decoded.program_id, PROGRAM_ID);
        assert_eq!(decoded_arranged_accounts, expected_arranged_accounts);
    }

    #[test]
    fn test_decode_withdraw() {
        // Arrange
        let expected_ix = PumpfunInstruction::Withdraw(withdraw::Withdraw {});
        let expected_accounts = vec![
            AccountMeta::new_readonly(
                Pubkey::from_str_const("4wTV1YmiEkRvAtNtsSGPtUrqRYQMe5SKy2uB4Jjaxnjf"),
                false,
            ),
            AccountMeta::new(
                Pubkey::from_str_const("EGqbBGXmDA9QYd1XJkf3GDFoerQYeFW3FrQZZXRza9JL"),
                false,
            ),
            AccountMeta::new_readonly(
                Pubkey::from_str_const("8f8inBUeF6GCPQvN2qxu95uZMTjidZfS2RbYBrFSpump"),
                false,
            ),
            AccountMeta::new(
                Pubkey::from_str_const("DfcyEVHECKF9U14EzYqxeovufnndbN8qrDurVdJbkUwY"),
                false,
            ),
            AccountMeta::new(
                Pubkey::from_str_const("EQtzJCRiCpbEmKqZndvCQgGAXCFuGtP2FAZ2HYpH8F6F"),
                false,
            ),
            AccountMeta::new(
                Pubkey::from_str_const("7ngNZs9Ax61KJ8MKmQKFao73LB4jRRgrg4SZU3YsAbfY"),
                false,
            ),
            AccountMeta::new(
                Pubkey::from_str_const("39azUYFWPz3VHgKCf3VChUwbpURdCHRxjWVowf5jUJjg"),
                true,
            ),
            AccountMeta::new_readonly(
                Pubkey::from_str_const("11111111111111111111111111111111"),
                false,
            ),
            AccountMeta::new_readonly(
                Pubkey::from_str_const("TokenkegQfeZyiNwAJbNbGKPFXCWuBvf9Ss623VQ5DA"),
                false,
            ),
            AccountMeta::new_readonly(
                Pubkey::from_str_const("SysvarRent111111111111111111111111111111111"),
                false,
            ),
            AccountMeta::new_readonly(
                Pubkey::from_str_const("Ce6TQqeHC9p8KetsN6JsjHK7UTZk7nasjjnr7XxXp9F1"),
                false,
            ),
            AccountMeta::new_readonly(
                Pubkey::from_str_const("6EF8rrecthR5Dkzon8Nwu78hRvfCKubJ14M5uBEwF6P"),
                false,
            ),
        ];
        let expected_arranged_accounts = withdraw::WithdrawInstructionAccounts {
            global: Pubkey::from_str_const("4wTV1YmiEkRvAtNtsSGPtUrqRYQMe5SKy2uB4Jjaxnjf"),
            last_withdraw: Pubkey::from_str_const("EGqbBGXmDA9QYd1XJkf3GDFoerQYeFW3FrQZZXRza9JL"),
            mint: Pubkey::from_str_const("8f8inBUeF6GCPQvN2qxu95uZMTjidZfS2RbYBrFSpump"),
            bonding_curve: Pubkey::from_str_const("DfcyEVHECKF9U14EzYqxeovufnndbN8qrDurVdJbkUwY"),
            associated_bonding_curve: Pubkey::from_str_const(
                "EQtzJCRiCpbEmKqZndvCQgGAXCFuGtP2FAZ2HYpH8F6F",
            ),
            associated_user: Pubkey::from_str_const("7ngNZs9Ax61KJ8MKmQKFao73LB4jRRgrg4SZU3YsAbfY"),
            user: Pubkey::from_str_const("39azUYFWPz3VHgKCf3VChUwbpURdCHRxjWVowf5jUJjg"),
            system_program: Pubkey::from_str_const("11111111111111111111111111111111"),
            token_program: Pubkey::from_str_const("TokenkegQfeZyiNwAJbNbGKPFXCWuBvf9Ss623VQ5DA"),
            rent: Pubkey::from_str_const("SysvarRent111111111111111111111111111111111"),
            event_authority: Pubkey::from_str_const("Ce6TQqeHC9p8KetsN6JsjHK7UTZk7nasjjnr7XxXp9F1"),
            program: Pubkey::from_str_const("6EF8rrecthR5Dkzon8Nwu78hRvfCKubJ14M5uBEwF6P"),
        };

        // Act
        let decoder = PumpfunDecoder;
        let instruction = carbon_test_utils::read_instruction("tests/fixtures/withdraw_ix.json")
            .expect("read fixture");
        let decoded = decoder
            .decode_instruction(&instruction)
            .expect("decode instruction");
        let decoded_arranged_accounts =
            withdraw::Withdraw::arrange_accounts(&instruction.accounts).expect("aranage accounts");

        // Assert
        assert_eq!(decoded.data, expected_ix);
        assert_eq!(decoded.accounts, expected_accounts);
        assert_eq!(decoded.program_id, PROGRAM_ID);
        assert_eq!(decoded_arranged_accounts, expected_arranged_accounts);
    }
}
