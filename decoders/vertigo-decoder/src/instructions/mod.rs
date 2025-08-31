use super::VertigoDecoder;
pub mod buy;
pub mod buy_event;
pub mod claim;
pub mod create;
pub mod pool_created_event;
pub mod quote_buy;
pub mod quote_sell;
pub mod sell;
pub mod sell_event;

#[derive(
    carbon_core::InstructionType, serde::Serialize, serde::Deserialize, PartialEq, Debug, Clone,
)]
pub enum VertigoInstruction {
    Buy(buy::Buy),
    Claim(claim::Claim),
    Create(create::Create),
    QuoteBuy(quote_buy::QuoteBuy),
    QuoteSell(quote_sell::QuoteSell),
    Sell(sell::Sell),
    BuyEvent(buy_event::BuyEvent),
    PoolCreatedEvent(pool_created_event::PoolCreatedEvent),
    SellEvent(sell_event::SellEvent),
}

impl carbon_core::instruction::InstructionDecoder<'_> for VertigoDecoder {
    type InstructionType = VertigoInstruction;

    fn decode_instruction(
        &self,
        instruction: &solana_instruction::Instruction,
    ) -> Option<carbon_core::instruction::DecodedInstruction<Self::InstructionType>> {
        carbon_core::try_decode_instructions!(instruction,
            VertigoInstruction::Buy => buy::Buy,
            VertigoInstruction::Claim => claim::Claim,
            VertigoInstruction::Create => create::Create,
            VertigoInstruction::QuoteBuy => quote_buy::QuoteBuy,
            VertigoInstruction::QuoteSell => quote_sell::QuoteSell,
            VertigoInstruction::Sell => sell::Sell,
            VertigoInstruction::BuyEvent => buy_event::BuyEvent,
            VertigoInstruction::PoolCreatedEvent => pool_created_event::PoolCreatedEvent,
            VertigoInstruction::SellEvent => sell_event::SellEvent,
        )
    }
}
