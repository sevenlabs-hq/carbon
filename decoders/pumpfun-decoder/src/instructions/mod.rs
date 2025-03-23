use crate::PROGRAM_ID;

use super::PumpfunDecoder;
pub mod buy;
pub mod complete_event;
pub mod create;
pub mod create_event;
pub mod initialize;
pub mod sell;
pub mod set_params;
pub mod set_params_event;
pub mod trade_event;
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
    Initialize(initialize::Initialize),
    Sell(sell::Sell),
    SetParams(set_params::SetParams),
    Withdraw(withdraw::Withdraw),
    CompleteEvent(complete_event::CompleteEvent),
    CreateEvent(create_event::CreateEvent),
    SetParamsEvent(set_params_event::SetParamsEvent),
    TradeEvent(trade_event::TradeEvent),
}

impl carbon_core::instruction::InstructionDecoder<'_> for PumpfunDecoder {
    type InstructionType = PumpfunInstruction;

    fn decode_instruction(
        &self,
        instruction: &solana_sdk::instruction::Instruction,
    ) -> Option<carbon_core::instruction::DecodedInstruction<Self::InstructionType>> {
        if !instruction.program_id.eq(&PROGRAM_ID) {
            return None;
        }

        carbon_core::try_decode_instructions!(instruction,
            PumpfunInstruction::Buy => buy::Buy,
            PumpfunInstruction::Create => create::Create,
            PumpfunInstruction::Initialize => initialize::Initialize,
            PumpfunInstruction::Sell => sell::Sell,
            PumpfunInstruction::SetParams => set_params::SetParams,
            PumpfunInstruction::Withdraw => withdraw::Withdraw,
            PumpfunInstruction::CompleteEvent => complete_event::CompleteEvent,
            PumpfunInstruction::CreateEvent => create_event::CreateEvent,
            PumpfunInstruction::SetParamsEvent => set_params_event::SetParamsEvent,
            PumpfunInstruction::TradeEvent => trade_event::TradeEvent,
        )
    }
}

#[cfg(test)]
mod tests {
    use carbon_core::{deserialize::ArrangeAccounts, instruction::InstructionDecoder};
    use solana_sdk::{instruction::AccountMeta, pubkey};

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
                pubkey!("4wTV1YmiEkRvAtNtsSGPtUrqRYQMe5SKy2uB4Jjaxnjf"),
                false,
            ),
            AccountMeta::new(
                pubkey!("62qc2CNXwrYqQScmEdiZFFAnJR262PxWEuNQtxfafNgV"),
                false,
            ),
            AccountMeta::new_readonly(
                pubkey!("9p1PMtto471A7GvnRJVmDcuqUz3xDd1Lhu8vzrmpump"),
                false,
            ),
            AccountMeta::new(
                pubkey!("HWxwYxr4AV5ytUyT8pvjCEiUrXhwpbx365VpvQ6Bd6MZ"),
                false,
            ),
            AccountMeta::new(
                pubkey!("AUfg9aTAix7YarkHXSBMUyQPCTq55Gg1Z2NTe6utwwzG"),
                false,
            ),
            AccountMeta::new(
                pubkey!("4FLYmjhLuUb5ofNBo1PA9enF7HrPUSYUA1t55tUSFYa5"),
                false,
            ),
            AccountMeta::new(
                pubkey!("5ztadiszGPmBeGVcvmtPyqiHRA8SpU8mqNzPV1WeV88F"),
                true,
            ),
            AccountMeta::new_readonly(pubkey!("11111111111111111111111111111111"), false),
            AccountMeta::new_readonly(
                pubkey!("TokenkegQfeZyiNwAJbNbGKPFXCWuBvf9Ss623VQ5DA"),
                false,
            ),
            AccountMeta::new_readonly(
                pubkey!("SysvarRent111111111111111111111111111111111"),
                false,
            ),
            AccountMeta::new_readonly(
                pubkey!("Ce6TQqeHC9p8KetsN6JsjHK7UTZk7nasjjnr7XxXp9F1"),
                false,
            ),
            AccountMeta::new_readonly(
                pubkey!("6EF8rrecthR5Dkzon8Nwu78hRvfCKubJ14M5uBEwF6P"),
                false,
            ),
        ];
        let expected_arranged_accounts = buy::BuyInstructionAccounts {
            global: pubkey!("4wTV1YmiEkRvAtNtsSGPtUrqRYQMe5SKy2uB4Jjaxnjf"),
            fee_recipient: pubkey!("62qc2CNXwrYqQScmEdiZFFAnJR262PxWEuNQtxfafNgV"),
            mint: pubkey!("9p1PMtto471A7GvnRJVmDcuqUz3xDd1Lhu8vzrmpump"),
            bonding_curve: pubkey!("HWxwYxr4AV5ytUyT8pvjCEiUrXhwpbx365VpvQ6Bd6MZ"),
            associated_bonding_curve: pubkey!("AUfg9aTAix7YarkHXSBMUyQPCTq55Gg1Z2NTe6utwwzG"),
            associated_user: pubkey!("4FLYmjhLuUb5ofNBo1PA9enF7HrPUSYUA1t55tUSFYa5"),
            user: pubkey!("5ztadiszGPmBeGVcvmtPyqiHRA8SpU8mqNzPV1WeV88F"),
            system_program: pubkey!("11111111111111111111111111111111"),
            token_program: pubkey!("TokenkegQfeZyiNwAJbNbGKPFXCWuBvf9Ss623VQ5DA"),
            rent: pubkey!("SysvarRent111111111111111111111111111111111"),
            event_authority: pubkey!("Ce6TQqeHC9p8KetsN6JsjHK7UTZk7nasjjnr7XxXp9F1"),
            program: pubkey!("6EF8rrecthR5Dkzon8Nwu78hRvfCKubJ14M5uBEwF6P"),
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
                pubkey!("4wTV1YmiEkRvAtNtsSGPtUrqRYQMe5SKy2uB4Jjaxnjf"),
                false,
            ),
            AccountMeta::new(
                pubkey!("62qc2CNXwrYqQScmEdiZFFAnJR262PxWEuNQtxfafNgV"),
                false,
            ),
            AccountMeta::new_readonly(
                pubkey!("HXfFC4G1aJJo17KW56jJ2iaDLFXq6T8XZjPbQfhspump"),
                false,
            ),
            AccountMeta::new(
                pubkey!("8f12Y6z6CkMmcBqduvThRG2V873CP3eu2iBydqKGDX6y"),
                false,
            ),
            AccountMeta::new(
                pubkey!("GkSscwZJBhcFeB6hpWrnfrE73e5SawPmMuT55U1W4uqz"),
                false,
            ),
            AccountMeta::new(
                pubkey!("Bi6H7WPrZoJmqSauP38NuBaEttGraZkceR4p17ekoTwh"),
                false,
            ),
            AccountMeta::new(
                pubkey!("3bApZNQrP3T6Q1GvK1n1nUPHHnpnsbrEmdGyQyYLEbkP"),
                true,
            ),
            AccountMeta::new_readonly(pubkey!("11111111111111111111111111111111"), false),
            AccountMeta::new_readonly(
                pubkey!("ATokenGPvbdGVxr1b2hvZbsiqW5xWH25efTNsLJA8knL"),
                false,
            ),
            AccountMeta::new_readonly(
                pubkey!("TokenkegQfeZyiNwAJbNbGKPFXCWuBvf9Ss623VQ5DA"),
                false,
            ),
            AccountMeta::new_readonly(
                pubkey!("Ce6TQqeHC9p8KetsN6JsjHK7UTZk7nasjjnr7XxXp9F1"),
                false,
            ),
            AccountMeta::new_readonly(
                pubkey!("6EF8rrecthR5Dkzon8Nwu78hRvfCKubJ14M5uBEwF6P"),
                false,
            ),
        ];
        let expected_arranged_accounts = sell::SellInstructionAccounts {
            global: pubkey!("4wTV1YmiEkRvAtNtsSGPtUrqRYQMe5SKy2uB4Jjaxnjf"),
            fee_recipient: pubkey!("62qc2CNXwrYqQScmEdiZFFAnJR262PxWEuNQtxfafNgV"),
            mint: pubkey!("HXfFC4G1aJJo17KW56jJ2iaDLFXq6T8XZjPbQfhspump"),
            bonding_curve: pubkey!("8f12Y6z6CkMmcBqduvThRG2V873CP3eu2iBydqKGDX6y"),
            associated_bonding_curve: pubkey!("GkSscwZJBhcFeB6hpWrnfrE73e5SawPmMuT55U1W4uqz"),
            associated_user: pubkey!("Bi6H7WPrZoJmqSauP38NuBaEttGraZkceR4p17ekoTwh"),
            user: pubkey!("3bApZNQrP3T6Q1GvK1n1nUPHHnpnsbrEmdGyQyYLEbkP"),
            system_program: pubkey!("11111111111111111111111111111111"),
            associated_token_program: pubkey!("ATokenGPvbdGVxr1b2hvZbsiqW5xWH25efTNsLJA8knL"),
            token_program: pubkey!("TokenkegQfeZyiNwAJbNbGKPFXCWuBvf9Ss623VQ5DA"),
            event_authority: pubkey!("Ce6TQqeHC9p8KetsN6JsjHK7UTZk7nasjjnr7XxXp9F1"),
            program: pubkey!("6EF8rrecthR5Dkzon8Nwu78hRvfCKubJ14M5uBEwF6P"),
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
            creator: pubkey!("7a9xQF38YVW58TPeHavvXiVpqynCxY2GcohsZxdUZCX1"),
        });
        let expected_accounts = vec![
            AccountMeta::new(
                pubkey!("5PweXK19JD4PkafHm9BmpgiTaMoQgKq9EXVkDagwpump"),
                true,
            ),
            AccountMeta::new_readonly(
                pubkey!("TSLvdd1pWpHVjahSpsvCXUbgwsL3JAcvokwaKt1eokM"),
                false,
            ),
            AccountMeta::new(
                pubkey!("Chau1rGA8w4L43rAMUAKXGwq8hpPfGjUoHsiZJEyziKz"),
                false,
            ),
            AccountMeta::new(
                pubkey!("7JxriSri8PukwVQ6VQZ6ErpJ3Km1x6eWMnQeJ2Cd2148"),
                false,
            ),
            AccountMeta::new_readonly(
                pubkey!("4wTV1YmiEkRvAtNtsSGPtUrqRYQMe5SKy2uB4Jjaxnjf"),
                false,
            ),
            AccountMeta::new_readonly(
                pubkey!("metaqbxxUerdq28cj1RbAWkYQm3ybzjb6a8bt518x1s"),
                false,
            ),
            AccountMeta::new(
                pubkey!("Dzp1H2K6sNR8VAqXE9Q6eSdZckp7uQSJKR1FKY9SzFoS"),
                false,
            ),
            AccountMeta::new(
                pubkey!("7a9xQF38YVW58TPeHavvXiVpqynCxY2GcohsZxdUZCX1"),
                true,
            ),
            AccountMeta::new_readonly(pubkey!("11111111111111111111111111111111"), false),
            AccountMeta::new_readonly(
                pubkey!("TokenkegQfeZyiNwAJbNbGKPFXCWuBvf9Ss623VQ5DA"),
                false,
            ),
            AccountMeta::new_readonly(
                pubkey!("ATokenGPvbdGVxr1b2hvZbsiqW5xWH25efTNsLJA8knL"),
                false,
            ),
            AccountMeta::new_readonly(
                pubkey!("SysvarRent111111111111111111111111111111111"),
                false,
            ),
            AccountMeta::new_readonly(
                pubkey!("Ce6TQqeHC9p8KetsN6JsjHK7UTZk7nasjjnr7XxXp9F1"),
                false,
            ),
            AccountMeta::new_readonly(
                pubkey!("6EF8rrecthR5Dkzon8Nwu78hRvfCKubJ14M5uBEwF6P"),
                false,
            ),
        ];
        let expected_arranged_accounts = create::CreateInstructionAccounts {
            mint: pubkey!("5PweXK19JD4PkafHm9BmpgiTaMoQgKq9EXVkDagwpump"),
            mint_authority: pubkey!("TSLvdd1pWpHVjahSpsvCXUbgwsL3JAcvokwaKt1eokM"),
            bonding_curve: pubkey!("Chau1rGA8w4L43rAMUAKXGwq8hpPfGjUoHsiZJEyziKz"),
            associated_bonding_curve: pubkey!("7JxriSri8PukwVQ6VQZ6ErpJ3Km1x6eWMnQeJ2Cd2148"),
            global: pubkey!("4wTV1YmiEkRvAtNtsSGPtUrqRYQMe5SKy2uB4Jjaxnjf"),
            mpl_token_metadata: pubkey!("metaqbxxUerdq28cj1RbAWkYQm3ybzjb6a8bt518x1s"),
            metadata: pubkey!("Dzp1H2K6sNR8VAqXE9Q6eSdZckp7uQSJKR1FKY9SzFoS"),
            user: pubkey!("7a9xQF38YVW58TPeHavvXiVpqynCxY2GcohsZxdUZCX1"),
            system_program: pubkey!("11111111111111111111111111111111"),
            token_program: pubkey!("TokenkegQfeZyiNwAJbNbGKPFXCWuBvf9Ss623VQ5DA"),
            associated_token_program: pubkey!("ATokenGPvbdGVxr1b2hvZbsiqW5xWH25efTNsLJA8knL"),
            rent: pubkey!("SysvarRent111111111111111111111111111111111"),
            event_authority: pubkey!("Ce6TQqeHC9p8KetsN6JsjHK7UTZk7nasjjnr7XxXp9F1"),
            program: pubkey!("6EF8rrecthR5Dkzon8Nwu78hRvfCKubJ14M5uBEwF6P"),
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
                pubkey!("4wTV1YmiEkRvAtNtsSGPtUrqRYQMe5SKy2uB4Jjaxnjf"),
                false,
            ),
            AccountMeta::new(
                pubkey!("EGqbBGXmDA9QYd1XJkf3GDFoerQYeFW3FrQZZXRza9JL"),
                false,
            ),
            AccountMeta::new_readonly(
                pubkey!("8f8inBUeF6GCPQvN2qxu95uZMTjidZfS2RbYBrFSpump"),
                false,
            ),
            AccountMeta::new(
                pubkey!("DfcyEVHECKF9U14EzYqxeovufnndbN8qrDurVdJbkUwY"),
                false,
            ),
            AccountMeta::new(
                pubkey!("EQtzJCRiCpbEmKqZndvCQgGAXCFuGtP2FAZ2HYpH8F6F"),
                false,
            ),
            AccountMeta::new(
                pubkey!("7ngNZs9Ax61KJ8MKmQKFao73LB4jRRgrg4SZU3YsAbfY"),
                false,
            ),
            AccountMeta::new(
                pubkey!("39azUYFWPz3VHgKCf3VChUwbpURdCHRxjWVowf5jUJjg"),
                true,
            ),
            AccountMeta::new_readonly(pubkey!("11111111111111111111111111111111"), false),
            AccountMeta::new_readonly(
                pubkey!("TokenkegQfeZyiNwAJbNbGKPFXCWuBvf9Ss623VQ5DA"),
                false,
            ),
            AccountMeta::new_readonly(
                pubkey!("SysvarRent111111111111111111111111111111111"),
                false,
            ),
            AccountMeta::new_readonly(
                pubkey!("Ce6TQqeHC9p8KetsN6JsjHK7UTZk7nasjjnr7XxXp9F1"),
                false,
            ),
            AccountMeta::new_readonly(
                pubkey!("6EF8rrecthR5Dkzon8Nwu78hRvfCKubJ14M5uBEwF6P"),
                false,
            ),
        ];
        let expected_arranged_accounts = withdraw::WithdrawInstructionAccounts {
            global: pubkey!("4wTV1YmiEkRvAtNtsSGPtUrqRYQMe5SKy2uB4Jjaxnjf"),
            last_withdraw: pubkey!("EGqbBGXmDA9QYd1XJkf3GDFoerQYeFW3FrQZZXRza9JL"),
            mint: pubkey!("8f8inBUeF6GCPQvN2qxu95uZMTjidZfS2RbYBrFSpump"),
            bonding_curve: pubkey!("DfcyEVHECKF9U14EzYqxeovufnndbN8qrDurVdJbkUwY"),
            associated_bonding_curve: pubkey!("EQtzJCRiCpbEmKqZndvCQgGAXCFuGtP2FAZ2HYpH8F6F"),
            associated_user: pubkey!("7ngNZs9Ax61KJ8MKmQKFao73LB4jRRgrg4SZU3YsAbfY"),
            user: pubkey!("39azUYFWPz3VHgKCf3VChUwbpURdCHRxjWVowf5jUJjg"),
            system_program: pubkey!("11111111111111111111111111111111"),
            token_program: pubkey!("TokenkegQfeZyiNwAJbNbGKPFXCWuBvf9Ss623VQ5DA"),
            rent: pubkey!("SysvarRent111111111111111111111111111111111"),
            event_authority: pubkey!("Ce6TQqeHC9p8KetsN6JsjHK7UTZk7nasjjnr7XxXp9F1"),
            program: pubkey!("6EF8rrecthR5Dkzon8Nwu78hRvfCKubJ14M5uBEwF6P"),
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
