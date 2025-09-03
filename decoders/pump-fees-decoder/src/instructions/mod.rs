use crate::PROGRAM_ID;

use super::PumpFeesDecoder;

pub mod get_fees;
pub mod initialize_fee_config;
pub mod initialize_fee_config_event;
pub mod update_admin;
pub mod update_admin_event;
pub mod update_fee_config;
pub mod update_fee_config_event;
pub mod upsert_fee_tiers;
pub mod upsert_fee_tiers_event;

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
pub enum PumpFeesInstruction {
    GetFees(get_fees::GetFees),
    InitializeFeeConfig(initialize_fee_config::InitializeFeeConfig),
    UpdateAdmin(update_admin::UpdateAdmin),
    UpdateFeeConfig(update_fee_config::UpdateFeeConfig),
    UpsertFeeTiers(upsert_fee_tiers::UpsertFeeTiers),
    InitializeFeeConfigEvent(initialize_fee_config_event::InitializeFeeConfigEvent),
    UpdateAdminEvent(update_admin_event::UpdateAdminEvent),
    UpdateFeeConfigEvent(update_fee_config_event::UpdateFeeConfigEvent),
    UpsertFeeTiersEvent(upsert_fee_tiers_event::UpsertFeeTiersEvent),
}

impl carbon_core::instruction::InstructionDecoder<'_> for PumpFeesDecoder {
    type InstructionType = PumpFeesInstruction;

    fn decode_instruction(
        &self,
        instruction: &solana_instruction::Instruction,
    ) -> Option<carbon_core::instruction::DecodedInstruction<Self::InstructionType>> {
        if !instruction.program_id.eq(&PROGRAM_ID) {
            return None;
        }

        carbon_core::try_decode_instructions!(instruction,
            PumpFeesInstruction::GetFees => get_fees::GetFees,
            PumpFeesInstruction::InitializeFeeConfig => initialize_fee_config::InitializeFeeConfig,
            PumpFeesInstruction::UpdateAdmin => update_admin::UpdateAdmin,
            PumpFeesInstruction::UpdateFeeConfig => update_fee_config::UpdateFeeConfig,
            PumpFeesInstruction::UpsertFeeTiers => upsert_fee_tiers::UpsertFeeTiers,
            PumpFeesInstruction::InitializeFeeConfigEvent => initialize_fee_config_event::InitializeFeeConfigEvent,
            PumpFeesInstruction::UpdateAdminEvent => update_admin_event::UpdateAdminEvent,
            PumpFeesInstruction::UpdateFeeConfigEvent => update_fee_config_event::UpdateFeeConfigEvent,
            PumpFeesInstruction::UpsertFeeTiersEvent => upsert_fee_tiers_event::UpsertFeeTiersEvent,
        )
    }
}
