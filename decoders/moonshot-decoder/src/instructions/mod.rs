use crate::PROGRAM_ID;

use super::MoonshotDecoder;
pub mod buy;
pub mod config_init;
pub mod config_update;
pub mod migrate_funds;
pub mod migration_event;
pub mod sell;
pub mod token_mint;
pub mod trade_event;

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
pub enum MoonshotInstruction {
    TokenMint(token_mint::TokenMint),
    Buy(buy::Buy),
    Sell(sell::Sell),
    MigrateFunds(migrate_funds::MigrateFunds),
    ConfigInit(config_init::ConfigInit),
    ConfigUpdate(config_update::ConfigUpdate),
    TradeEvent(trade_event::TradeEvent),
    MigrationEvent(migration_event::MigrationEvent),
}

impl carbon_core::instruction::InstructionDecoder<'_> for MoonshotDecoder {
    type InstructionType = MoonshotInstruction;

    fn decode_instruction(
        &self,
        instruction: &solana_sdk::instruction::Instruction,
    ) -> Option<carbon_core::instruction::DecodedInstruction<Self::InstructionType>> {
        if !instruction.program_id.eq(&PROGRAM_ID) {
            return None;
        }

        carbon_core::try_decode_instructions!(instruction,
            MoonshotInstruction::TokenMint => token_mint::TokenMint,
            MoonshotInstruction::Buy => buy::Buy,
            MoonshotInstruction::Sell => sell::Sell,
            MoonshotInstruction::MigrateFunds => migrate_funds::MigrateFunds,
            MoonshotInstruction::ConfigInit => config_init::ConfigInit,
            MoonshotInstruction::ConfigUpdate => config_update::ConfigUpdate,
            MoonshotInstruction::TradeEvent => trade_event::TradeEvent,
            MoonshotInstruction::MigrationEvent => migration_event::MigrationEvent,
        )
    }
}

#[cfg(test)]
mod tests {
    use carbon_core::instruction::InstructionDecoder;

    use crate::types::{TokenMintParams, TradeParams};

    use super::*;

    #[test]
    fn test_decode_token_mint() {
        let decoder = MoonshotDecoder;
        let instruction =
            carbon_test_utils::read_instruction("../../tests/fixtures/moonshot/token_mint.json")
                .expect("read fixture");
        let decoded = decoder.decode_instruction(&instruction);

        let expected = MoonshotInstruction::TokenMint(token_mint::TokenMint {
            mint_params: TokenMintParams {
                name: "Gamestop".to_string(),
                symbol: "$GME".to_string(),
                uri: "https://cdn.dexscreener.com/cms/tokens/metadata/UgnNvayhAu8K97aoT3B8"
                    .to_string(),
                decimals: 9,
                collateral_currency: 0,
                amount: 1000000000000000000,
                curve_type: 1,
                migration_target: 0,
            },
        });

        assert!(matches!(decoded, Some(expected)));
    }

    #[test]
    fn test_decode_buy() {
        let decoder = MoonshotDecoder;
        let instruction =
            carbon_test_utils::read_instruction("../../tests/fixtures/moonshot/buy.json")
                .expect("read fixture");
        let decoded = decoder.decode_instruction(&instruction);

        let expected = MoonshotInstruction::Buy(buy::Buy {
            data: TradeParams {
                token_amount: 5430576418647,
                collateral_amount: 1640000,
                fixed_side: 1,
                slippage_bps: 9999,
            },
        });

        assert!(matches!(decoded, Some(expected)));
    }

    #[test]
    fn test_decode_sell() {
        let decoder = MoonshotDecoder;
        let instruction =
            carbon_test_utils::read_instruction("../../tests/fixtures/moonshot/sell.json")
                .expect("read fixture");
        let decoded = decoder.decode_instruction(&instruction);

        let expected = MoonshotInstruction::Sell(sell::Sell {
            data: TradeParams {
                token_amount: 157227000000000,
                collateral_amount: 20990579,
                fixed_side: 0,
                slippage_bps: 100,
            },
        });

        assert!(matches!(decoded, Some(expected)));
    }

    // #[test]
    // fn test_decode_migrate_funds() {
    //     let decoder = MoonshotDecoder;
    //     let instruction_data = vec![/* appropriate data for MigrateFunds */];
    //     let instruction = create_instruction(instruction_data);
    //     let decoded = decoder.decode_instruction(&instruction);
    //     assert!(matches!(
    //         decoded,
    //         Some(carbon_core::instruction::DecodedInstruction::MigrateFunds(
    //             _
    //         ))
    //     ));
    // }

    // #[test]
    // fn test_decode_config_init() {
    //     let decoder = MoonshotDecoder;
    //     let instruction_data = vec![/* appropriate data for ConfigInit */];
    //     let instruction = create_instruction(instruction_data);
    //     let decoded = decoder.decode_instruction(&instruction);
    //     assert!(matches!(
    //         decoded,
    //         Some(carbon_core::instruction::DecodedInstruction::ConfigInit(_))
    //     ));
    // }

    // #[test]
    // fn test_decode_config_update() {
    //     let decoder = MoonshotDecoder;
    //     let instruction_data = vec![/* appropriate data for ConfigUpdate */];
    //     let instruction = create_instruction(instruction_data);
    //     let decoded = decoder.decode_instruction(&instruction);
    //     assert!(matches!(
    //         decoded,
    //         Some(carbon_core::instruction::DecodedInstruction::ConfigUpdate(
    //             _
    //         ))
    //     ));
    // }

    // #[test]
    // fn test_decode_trade_event() {
    //     let decoder = MoonshotDecoder;
    //     let instruction_data = vec![/* appropriate data for TradeEvent */];
    //     let instruction = create_instruction(instruction_data);
    //     let decoded = decoder.decode_instruction(&instruction);
    //     assert!(matches!(
    //         decoded,
    //         Some(carbon_core::instruction::DecodedInstruction::TradeEvent(_))
    //     ));
    // }

    // #[test]
    // fn test_decode_migration_event() {
    //     let decoder = MoonshotDecoder;
    //     let instruction_data = vec![/* appropriate data for MigrationEvent */];
    //     let instruction = create_instruction(instruction_data);
    //     let decoded = decoder.decode_instruction(&instruction);
    //     assert!(matches!(
    //         decoded,
    //         Some(carbon_core::instruction::DecodedInstruction::MigrationEvent(_))
    //     ));
    // }
}
