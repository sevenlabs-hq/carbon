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
