use carbon_core::deserialize::CarbonDeserialize;
use carbon_core::instruction::InstructionDecoder;

use super::RaydiumAmmDecoder;
pub mod admin_cancel_orders;
pub mod create_config_account;
pub mod deposit;
pub mod init;
pub mod initialize;
pub mod initialize2;
pub mod migrate_to_open_book;
pub mod monitor_step;
pub mod pre_initialize;
pub mod set_params;
pub mod simulate_info;
pub mod swap_base_in;
pub mod swap_base_out;
pub mod update_config_account;
pub mod withdraw;
pub mod withdraw_pnl;
pub mod withdraw_srm;

#[derive(
    carbon_proc_macros::InstructionType,
    serde::Serialize,
    serde::Deserialize,
    PartialEq,
    Eq,
    Debug,
    Clone,
    Hash,
)]
pub enum RaydiumAmmInstruction {
    Initialize(initialize::Initialize),
    Initialize2(initialize2::Initialize2),
    MonitorStep(monitor_step::MonitorStep),
    Deposit(deposit::Deposit),
    Withdraw(withdraw::Withdraw),
    MigrateToOpenBook(migrate_to_open_book::MigrateToOpenBook),
    SetParams(set_params::SetParams),
    WithdrawPnl(withdraw_pnl::WithdrawPnl),
    WithdrawSrm(withdraw_srm::WithdrawSrm),
    SwapBaseIn(swap_base_in::SwapBaseIn),
    PreInitialize(pre_initialize::PreInitialize),
    SwapBaseOut(swap_base_out::SwapBaseOut),
    SimulateInfo(simulate_info::SimulateInfo),
    AdminCancelOrders(admin_cancel_orders::AdminCancelOrders),
    CreateConfigAccount(create_config_account::CreateConfigAccount),
    UpdateConfigAccount(update_config_account::UpdateConfigAccount),
    Init(init::Init),
}

impl InstructionDecoder for RaydiumAmmDecoder {
    type InstructionType = RaydiumAmmInstruction;

    fn decode_instruction(
        &self,
        instruction: solana_sdk::instruction::Instruction,
    ) -> Option<carbon_core::instruction::DecodedInstruction<Self::InstructionType>> {
        if let Some(decoded_instruction) =
            initialize::Initialize::deserialize(instruction.data.as_slice())
        {
            return Some(carbon_core::instruction::DecodedInstruction {
                program_id: instruction.program_id,
                data: RaydiumAmmInstruction::Initialize(decoded_instruction),
            });
        }
        if let Some(decoded_instruction) =
            initialize2::Initialize2::deserialize(instruction.data.as_slice())
        {
            return Some(carbon_core::instruction::DecodedInstruction {
                program_id: instruction.program_id,
                data: RaydiumAmmInstruction::Initialize2(decoded_instruction),
            });
        }
        if let Some(decoded_instruction) =
            monitor_step::MonitorStep::deserialize(instruction.data.as_slice())
        {
            return Some(carbon_core::instruction::DecodedInstruction {
                program_id: instruction.program_id,
                data: RaydiumAmmInstruction::MonitorStep(decoded_instruction),
            });
        }
        if let Some(decoded_instruction) =
            deposit::Deposit::deserialize(instruction.data.as_slice())
        {
            return Some(carbon_core::instruction::DecodedInstruction {
                program_id: instruction.program_id,
                data: RaydiumAmmInstruction::Deposit(decoded_instruction),
            });
        }
        if let Some(decoded_instruction) =
            withdraw::Withdraw::deserialize(instruction.data.as_slice())
        {
            return Some(carbon_core::instruction::DecodedInstruction {
                program_id: instruction.program_id,
                data: RaydiumAmmInstruction::Withdraw(decoded_instruction),
            });
        }
        if let Some(decoded_instruction) =
            migrate_to_open_book::MigrateToOpenBook::deserialize(instruction.data.as_slice())
        {
            return Some(carbon_core::instruction::DecodedInstruction {
                program_id: instruction.program_id,
                data: RaydiumAmmInstruction::MigrateToOpenBook(decoded_instruction),
            });
        }
        if let Some(decoded_instruction) =
            set_params::SetParams::deserialize(instruction.data.as_slice())
        {
            return Some(carbon_core::instruction::DecodedInstruction {
                program_id: instruction.program_id,
                data: RaydiumAmmInstruction::SetParams(decoded_instruction),
            });
        }
        if let Some(decoded_instruction) =
            withdraw_pnl::WithdrawPnl::deserialize(instruction.data.as_slice())
        {
            return Some(carbon_core::instruction::DecodedInstruction {
                program_id: instruction.program_id,
                data: RaydiumAmmInstruction::WithdrawPnl(decoded_instruction),
            });
        }
        if let Some(decoded_instruction) =
            withdraw_srm::WithdrawSrm::deserialize(instruction.data.as_slice())
        {
            return Some(carbon_core::instruction::DecodedInstruction {
                program_id: instruction.program_id,
                data: RaydiumAmmInstruction::WithdrawSrm(decoded_instruction),
            });
        }
        if let Some(decoded_instruction) =
            swap_base_in::SwapBaseIn::deserialize(instruction.data.as_slice())
        {
            return Some(carbon_core::instruction::DecodedInstruction {
                program_id: instruction.program_id,
                data: RaydiumAmmInstruction::SwapBaseIn(decoded_instruction),
            });
        }
        if let Some(decoded_instruction) =
            pre_initialize::PreInitialize::deserialize(instruction.data.as_slice())
        {
            return Some(carbon_core::instruction::DecodedInstruction {
                program_id: instruction.program_id,
                data: RaydiumAmmInstruction::PreInitialize(decoded_instruction),
            });
        }
        if let Some(decoded_instruction) =
            swap_base_out::SwapBaseOut::deserialize(instruction.data.as_slice())
        {
            return Some(carbon_core::instruction::DecodedInstruction {
                program_id: instruction.program_id,
                data: RaydiumAmmInstruction::SwapBaseOut(decoded_instruction),
            });
        }
        if let Some(decoded_instruction) =
            simulate_info::SimulateInfo::deserialize(instruction.data.as_slice())
        {
            return Some(carbon_core::instruction::DecodedInstruction {
                program_id: instruction.program_id,
                data: RaydiumAmmInstruction::SimulateInfo(decoded_instruction),
            });
        }
        if let Some(decoded_instruction) =
            admin_cancel_orders::AdminCancelOrders::deserialize(instruction.data.as_slice())
        {
            return Some(carbon_core::instruction::DecodedInstruction {
                program_id: instruction.program_id,
                data: RaydiumAmmInstruction::AdminCancelOrders(decoded_instruction),
            });
        }
        if let Some(decoded_instruction) =
            create_config_account::CreateConfigAccount::deserialize(instruction.data.as_slice())
        {
            return Some(carbon_core::instruction::DecodedInstruction {
                program_id: instruction.program_id,
                data: RaydiumAmmInstruction::CreateConfigAccount(decoded_instruction),
            });
        }
        if let Some(decoded_instruction) =
            update_config_account::UpdateConfigAccount::deserialize(instruction.data.as_slice())
        {
            return Some(carbon_core::instruction::DecodedInstruction {
                program_id: instruction.program_id,
                data: RaydiumAmmInstruction::UpdateConfigAccount(decoded_instruction),
            });
        }
        if let Some(decoded_instruction) = init::Init::deserialize(instruction.data.as_slice()) {
            return Some(carbon_core::instruction::DecodedInstruction {
                program_id: instruction.program_id,
                data: RaydiumAmmInstruction::Init(decoded_instruction),
            });
        }
        if let Some(decoded_instruction) =
            deposit::Deposit::deserialize(instruction.data.as_slice())
        {
            return Some(carbon_core::instruction::DecodedInstruction {
                program_id: instruction.program_id,
                data: RaydiumAmmInstruction::Deposit(decoded_instruction),
            });
        }
        if let Some(decoded_instruction) =
            withdraw::Withdraw::deserialize(instruction.data.as_slice())
        {
            return Some(carbon_core::instruction::DecodedInstruction {
                program_id: instruction.program_id,
                data: RaydiumAmmInstruction::Withdraw(decoded_instruction),
            });
        }
        if let Some(decoded_instruction) =
            swap_base_in::SwapBaseIn::deserialize(instruction.data.as_slice())
        {
            return Some(carbon_core::instruction::DecodedInstruction {
                program_id: instruction.program_id,
                data: RaydiumAmmInstruction::SwapBaseIn(decoded_instruction),
            });
        }
        if let Some(decoded_instruction) =
            swap_base_out::SwapBaseOut::deserialize(instruction.data.as_slice())
        {
            return Some(carbon_core::instruction::DecodedInstruction {
                program_id: instruction.program_id,
                data: RaydiumAmmInstruction::SwapBaseOut(decoded_instruction),
            });
        }

        None
    }
}
