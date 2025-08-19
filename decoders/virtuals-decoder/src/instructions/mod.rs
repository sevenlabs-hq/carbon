use {super::VirtualsDecoder, crate::PROGRAM_ID};
pub mod buy;
pub mod buy_event;
pub mod claim_fees;
pub mod create_meteora_pool;
pub mod graduation_event;
pub mod initialize;
pub mod initialize_meteora_accounts;
pub mod launch;
pub mod launch_event;
pub mod sell;
pub mod sell_event;
pub mod update_pool_creator;

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
pub enum VirtualsInstruction {
    Buy(buy::Buy),
    ClaimFees(claim_fees::ClaimFees),
    CreateMeteoraPool(create_meteora_pool::CreateMeteoraPool),
    Initialize(initialize::Initialize),
    InitializeMeteoraAccounts(initialize_meteora_accounts::InitializeMeteoraAccounts),
    Launch(launch::Launch),
    Sell(sell::Sell),
    UpdatePoolCreator(update_pool_creator::UpdatePoolCreator),
    BuyEvent(buy_event::BuyEvent),
    GraduationEvent(graduation_event::GraduationEvent),
    LaunchEvent(launch_event::LaunchEvent),
    SellEvent(sell_event::SellEvent),
}

impl carbon_core::instruction::InstructionDecoder<'_> for VirtualsDecoder {
    type InstructionType = VirtualsInstruction;

    fn decode_instruction(
        &self,
        instruction: &solana_instruction::Instruction,
    ) -> Option<carbon_core::instruction::DecodedInstruction<Self::InstructionType>> {
        if !instruction.program_id.eq(&PROGRAM_ID) {
            return None;
        }

        carbon_core::try_decode_instructions!(instruction,
            VirtualsInstruction::Buy => buy::Buy,
            VirtualsInstruction::ClaimFees => claim_fees::ClaimFees,
            VirtualsInstruction::CreateMeteoraPool => create_meteora_pool::CreateMeteoraPool,
            VirtualsInstruction::Initialize => initialize::Initialize,
            VirtualsInstruction::InitializeMeteoraAccounts => initialize_meteora_accounts::InitializeMeteoraAccounts,
            VirtualsInstruction::Launch => launch::Launch,
            VirtualsInstruction::Sell => sell::Sell,
            VirtualsInstruction::UpdatePoolCreator => update_pool_creator::UpdatePoolCreator,
            VirtualsInstruction::BuyEvent => buy_event::BuyEvent,
            VirtualsInstruction::GraduationEvent => graduation_event::GraduationEvent,
            VirtualsInstruction::LaunchEvent => launch_event::LaunchEvent,
            VirtualsInstruction::SellEvent => sell_event::SellEvent,
        )
    }
}
