use carbon_core::deserialize::CarbonDeserialize;
use carbon_core::instruction::InstructionDecoder;

use super::JupiterDecoder;
pub mod aldrin_swap;
pub mod aldrin_v2_swap;
pub mod balansol_swap;
pub mod create_open_orders;
pub mod create_program_open_orders;
pub mod create_token_ledger;
pub mod crema_swap;
pub mod cropper_swap;
pub mod cykura_swap;
pub mod deltafi_swap;
pub mod dradex_swap;
pub mod fee_event;
pub mod goosefx_swap;
pub mod goosefx_v2_swap;
pub mod helium_treasury_management_redeem_v0;
pub mod invariant_swap;
pub mod lifinity_swap;
pub mod lifinity_v2_swap;
pub mod marco_polo_swap;
pub mod marinade_deposit;
pub mod marinade_unstake;
pub mod mercurial_swap;
pub mod meteora_dlmm_swap;
pub mod meteora_swap;
pub mod perps_add_liquidity;
pub mod perps_remove_liquidity;
pub mod perps_swap;
pub mod phoenix_swap;
pub mod raydium_clmm_swap;
pub mod raydium_swap;
pub mod route;
pub mod route_with_token_ledger;
pub mod saber_add_decimals;
pub mod saber_swap;
pub mod sencha_swap;
pub mod serum_swap;
pub mod set_token_ledger;
pub mod shared_accounts_exact_out_route;
pub mod shared_accounts_route;
pub mod shared_accounts_route_with_token_ledger;
pub mod step_swap;
pub mod swap_event;
pub mod symmetry_swap;
pub mod token_swap;
pub mod token_swap_v2;
pub mod whirlpool_swap;

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
pub enum JupiterInstruction {
    Route(route::Route),
    RouteWithTokenLedger(route_with_token_ledger::RouteWithTokenLedger),
    SharedAccountsRoute(shared_accounts_route::SharedAccountsRoute),
    SharedAccountsRouteWithTokenLedger(
        shared_accounts_route_with_token_ledger::SharedAccountsRouteWithTokenLedger,
    ),
    SharedAccountsExactOutRoute(shared_accounts_exact_out_route::SharedAccountsExactOutRoute),
    SetTokenLedger(set_token_ledger::SetTokenLedger),
    CreateOpenOrders(create_open_orders::CreateOpenOrders),
    CreateProgramOpenOrders(create_program_open_orders::CreateProgramOpenOrders),
    CreateTokenLedger(create_token_ledger::CreateTokenLedger),
    MercurialSwap(mercurial_swap::MercurialSwap),
    CykuraSwap(cykura_swap::CykuraSwap),
    SerumSwap(serum_swap::SerumSwap),
    SaberSwap(saber_swap::SaberSwap),
    SaberAddDecimals(saber_add_decimals::SaberAddDecimals),
    TokenSwap(token_swap::TokenSwap),
    TokenSwapV2(token_swap_v2::TokenSwapV2),
    SenchaSwap(sencha_swap::SenchaSwap),
    StepSwap(step_swap::StepSwap),
    CropperSwap(cropper_swap::CropperSwap),
    RaydiumSwap(raydium_swap::RaydiumSwap),
    CremaSwap(crema_swap::CremaSwap),
    LifinitySwap(lifinity_swap::LifinitySwap),
    MarinadeDeposit(marinade_deposit::MarinadeDeposit),
    MarinadeUnstake(marinade_unstake::MarinadeUnstake),
    AldrinSwap(aldrin_swap::AldrinSwap),
    AldrinV2Swap(aldrin_v2_swap::AldrinV2Swap),
    WhirlpoolSwap(whirlpool_swap::WhirlpoolSwap),
    InvariantSwap(invariant_swap::InvariantSwap),
    MeteoraSwap(meteora_swap::MeteoraSwap),
    GoosefxSwap(goosefx_swap::GoosefxSwap),
    DeltafiSwap(deltafi_swap::DeltafiSwap),
    BalansolSwap(balansol_swap::BalansolSwap),
    MarcoPoloSwap(marco_polo_swap::MarcoPoloSwap),
    DradexSwap(dradex_swap::DradexSwap),
    LifinityV2Swap(lifinity_v2_swap::LifinityV2Swap),
    RaydiumClmmSwap(raydium_clmm_swap::RaydiumClmmSwap),
    PhoenixSwap(phoenix_swap::PhoenixSwap),
    SymmetrySwap(symmetry_swap::SymmetrySwap),
    HeliumTreasuryManagementRedeemV0(
        helium_treasury_management_redeem_v0::HeliumTreasuryManagementRedeemV0,
    ),
    GoosefxV2Swap(goosefx_v2_swap::GoosefxV2Swap),
    PerpsSwap(perps_swap::PerpsSwap),
    PerpsAddLiquidity(perps_add_liquidity::PerpsAddLiquidity),
    PerpsRemoveLiquidity(perps_remove_liquidity::PerpsRemoveLiquidity),
    MeteoraDlmmSwap(meteora_dlmm_swap::MeteoraDlmmSwap),
    SwapEvent(swap_event::SwapEvent),
    FeeEvent(fee_event::FeeEvent),
}

impl InstructionDecoder for JupiterDecoder {
    type InstructionType = JupiterInstruction;

    fn decode_instruction(
        &self,
        instruction: solana_sdk::instruction::Instruction,
    ) -> Option<carbon_core::instruction::DecodedInstruction<Self::InstructionType>> {
        if let Some(decoded_instruction) = route::Route::deserialize(instruction.data.as_slice()) {
            return Some(carbon_core::instruction::DecodedInstruction {
                program_id: instruction.program_id,
                accounts: instruction.accounts,
                data: JupiterInstruction::Route(decoded_instruction),
            });
        }
        if let Some(decoded_instruction) =
            route_with_token_ledger::RouteWithTokenLedger::deserialize(instruction.data.as_slice())
        {
            return Some(carbon_core::instruction::DecodedInstruction {
                program_id: instruction.program_id,
                accounts: instruction.accounts,
                data: JupiterInstruction::RouteWithTokenLedger(decoded_instruction),
            });
        }
        if let Some(decoded_instruction) =
            shared_accounts_route::SharedAccountsRoute::deserialize(instruction.data.as_slice())
        {
            return Some(carbon_core::instruction::DecodedInstruction {
                program_id: instruction.program_id,
                accounts: instruction.accounts,
                data: JupiterInstruction::SharedAccountsRoute(decoded_instruction),
            });
        }
        if let Some(decoded_instruction) =
            shared_accounts_route_with_token_ledger::SharedAccountsRouteWithTokenLedger::deserialize(
                instruction.data.as_slice(),
            )
        {
            return Some(carbon_core::instruction::DecodedInstruction {
                program_id: instruction.program_id,
                accounts: instruction.accounts,
                data: JupiterInstruction::SharedAccountsRouteWithTokenLedger(decoded_instruction),
            });
        }
        if let Some(decoded_instruction) =
            shared_accounts_exact_out_route::SharedAccountsExactOutRoute::deserialize(
                instruction.data.as_slice(),
            )
        {
            return Some(carbon_core::instruction::DecodedInstruction {
                program_id: instruction.program_id,
                accounts: instruction.accounts,
                data: JupiterInstruction::SharedAccountsExactOutRoute(decoded_instruction),
            });
        }
        if let Some(decoded_instruction) =
            set_token_ledger::SetTokenLedger::deserialize(instruction.data.as_slice())
        {
            return Some(carbon_core::instruction::DecodedInstruction {
                program_id: instruction.program_id,
                accounts: instruction.accounts,
                data: JupiterInstruction::SetTokenLedger(decoded_instruction),
            });
        }
        if let Some(decoded_instruction) =
            create_open_orders::CreateOpenOrders::deserialize(instruction.data.as_slice())
        {
            return Some(carbon_core::instruction::DecodedInstruction {
                program_id: instruction.program_id,
                accounts: instruction.accounts,
                data: JupiterInstruction::CreateOpenOrders(decoded_instruction),
            });
        }
        if let Some(decoded_instruction) =
            create_program_open_orders::CreateProgramOpenOrders::deserialize(
                instruction.data.as_slice(),
            )
        {
            return Some(carbon_core::instruction::DecodedInstruction {
                program_id: instruction.program_id,
                accounts: instruction.accounts,
                data: JupiterInstruction::CreateProgramOpenOrders(decoded_instruction),
            });
        }
        if let Some(decoded_instruction) =
            create_token_ledger::CreateTokenLedger::deserialize(instruction.data.as_slice())
        {
            return Some(carbon_core::instruction::DecodedInstruction {
                program_id: instruction.program_id,
                accounts: instruction.accounts,
                data: JupiterInstruction::CreateTokenLedger(decoded_instruction),
            });
        }
        if let Some(decoded_instruction) =
            mercurial_swap::MercurialSwap::deserialize(instruction.data.as_slice())
        {
            return Some(carbon_core::instruction::DecodedInstruction {
                program_id: instruction.program_id,
                accounts: instruction.accounts,
                data: JupiterInstruction::MercurialSwap(decoded_instruction),
            });
        }
        if let Some(decoded_instruction) =
            cykura_swap::CykuraSwap::deserialize(instruction.data.as_slice())
        {
            return Some(carbon_core::instruction::DecodedInstruction {
                program_id: instruction.program_id,
                accounts: instruction.accounts,
                data: JupiterInstruction::CykuraSwap(decoded_instruction),
            });
        }
        if let Some(decoded_instruction) =
            serum_swap::SerumSwap::deserialize(instruction.data.as_slice())
        {
            return Some(carbon_core::instruction::DecodedInstruction {
                program_id: instruction.program_id,
                accounts: instruction.accounts,
                data: JupiterInstruction::SerumSwap(decoded_instruction),
            });
        }
        if let Some(decoded_instruction) =
            saber_swap::SaberSwap::deserialize(instruction.data.as_slice())
        {
            return Some(carbon_core::instruction::DecodedInstruction {
                program_id: instruction.program_id,
                accounts: instruction.accounts,
                data: JupiterInstruction::SaberSwap(decoded_instruction),
            });
        }
        if let Some(decoded_instruction) =
            saber_add_decimals::SaberAddDecimals::deserialize(instruction.data.as_slice())
        {
            return Some(carbon_core::instruction::DecodedInstruction {
                program_id: instruction.program_id,
                accounts: instruction.accounts,
                data: JupiterInstruction::SaberAddDecimals(decoded_instruction),
            });
        }
        if let Some(decoded_instruction) =
            token_swap::TokenSwap::deserialize(instruction.data.as_slice())
        {
            return Some(carbon_core::instruction::DecodedInstruction {
                program_id: instruction.program_id,
                accounts: instruction.accounts,
                data: JupiterInstruction::TokenSwap(decoded_instruction),
            });
        }
        if let Some(decoded_instruction) =
            token_swap_v2::TokenSwapV2::deserialize(instruction.data.as_slice())
        {
            return Some(carbon_core::instruction::DecodedInstruction {
                program_id: instruction.program_id,
                accounts: instruction.accounts,
                data: JupiterInstruction::TokenSwapV2(decoded_instruction),
            });
        }
        if let Some(decoded_instruction) =
            sencha_swap::SenchaSwap::deserialize(instruction.data.as_slice())
        {
            return Some(carbon_core::instruction::DecodedInstruction {
                program_id: instruction.program_id,
                accounts: instruction.accounts,
                data: JupiterInstruction::SenchaSwap(decoded_instruction),
            });
        }
        if let Some(decoded_instruction) =
            step_swap::StepSwap::deserialize(instruction.data.as_slice())
        {
            return Some(carbon_core::instruction::DecodedInstruction {
                program_id: instruction.program_id,
                accounts: instruction.accounts,
                data: JupiterInstruction::StepSwap(decoded_instruction),
            });
        }
        if let Some(decoded_instruction) =
            cropper_swap::CropperSwap::deserialize(instruction.data.as_slice())
        {
            return Some(carbon_core::instruction::DecodedInstruction {
                program_id: instruction.program_id,
                accounts: instruction.accounts,
                data: JupiterInstruction::CropperSwap(decoded_instruction),
            });
        }
        if let Some(decoded_instruction) =
            raydium_swap::RaydiumSwap::deserialize(instruction.data.as_slice())
        {
            return Some(carbon_core::instruction::DecodedInstruction {
                program_id: instruction.program_id,
                accounts: instruction.accounts,
                data: JupiterInstruction::RaydiumSwap(decoded_instruction),
            });
        }
        if let Some(decoded_instruction) =
            crema_swap::CremaSwap::deserialize(instruction.data.as_slice())
        {
            return Some(carbon_core::instruction::DecodedInstruction {
                program_id: instruction.program_id,
                accounts: instruction.accounts,
                data: JupiterInstruction::CremaSwap(decoded_instruction),
            });
        }
        if let Some(decoded_instruction) =
            lifinity_swap::LifinitySwap::deserialize(instruction.data.as_slice())
        {
            return Some(carbon_core::instruction::DecodedInstruction {
                program_id: instruction.program_id,
                accounts: instruction.accounts,
                data: JupiterInstruction::LifinitySwap(decoded_instruction),
            });
        }
        if let Some(decoded_instruction) =
            marinade_deposit::MarinadeDeposit::deserialize(instruction.data.as_slice())
        {
            return Some(carbon_core::instruction::DecodedInstruction {
                program_id: instruction.program_id,
                accounts: instruction.accounts,
                data: JupiterInstruction::MarinadeDeposit(decoded_instruction),
            });
        }
        if let Some(decoded_instruction) =
            marinade_unstake::MarinadeUnstake::deserialize(instruction.data.as_slice())
        {
            return Some(carbon_core::instruction::DecodedInstruction {
                program_id: instruction.program_id,
                accounts: instruction.accounts,
                data: JupiterInstruction::MarinadeUnstake(decoded_instruction),
            });
        }
        if let Some(decoded_instruction) =
            aldrin_swap::AldrinSwap::deserialize(instruction.data.as_slice())
        {
            return Some(carbon_core::instruction::DecodedInstruction {
                program_id: instruction.program_id,
                accounts: instruction.accounts,
                data: JupiterInstruction::AldrinSwap(decoded_instruction),
            });
        }
        if let Some(decoded_instruction) =
            aldrin_v2_swap::AldrinV2Swap::deserialize(instruction.data.as_slice())
        {
            return Some(carbon_core::instruction::DecodedInstruction {
                program_id: instruction.program_id,
                accounts: instruction.accounts,
                data: JupiterInstruction::AldrinV2Swap(decoded_instruction),
            });
        }
        if let Some(decoded_instruction) =
            whirlpool_swap::WhirlpoolSwap::deserialize(instruction.data.as_slice())
        {
            return Some(carbon_core::instruction::DecodedInstruction {
                program_id: instruction.program_id,
                accounts: instruction.accounts,
                data: JupiterInstruction::WhirlpoolSwap(decoded_instruction),
            });
        }
        if let Some(decoded_instruction) =
            invariant_swap::InvariantSwap::deserialize(instruction.data.as_slice())
        {
            return Some(carbon_core::instruction::DecodedInstruction {
                program_id: instruction.program_id,
                accounts: instruction.accounts,
                data: JupiterInstruction::InvariantSwap(decoded_instruction),
            });
        }
        if let Some(decoded_instruction) =
            meteora_swap::MeteoraSwap::deserialize(instruction.data.as_slice())
        {
            return Some(carbon_core::instruction::DecodedInstruction {
                program_id: instruction.program_id,
                accounts: instruction.accounts,
                data: JupiterInstruction::MeteoraSwap(decoded_instruction),
            });
        }
        if let Some(decoded_instruction) =
            goosefx_swap::GoosefxSwap::deserialize(instruction.data.as_slice())
        {
            return Some(carbon_core::instruction::DecodedInstruction {
                program_id: instruction.program_id,
                accounts: instruction.accounts,
                data: JupiterInstruction::GoosefxSwap(decoded_instruction),
            });
        }
        if let Some(decoded_instruction) =
            deltafi_swap::DeltafiSwap::deserialize(instruction.data.as_slice())
        {
            return Some(carbon_core::instruction::DecodedInstruction {
                program_id: instruction.program_id,
                accounts: instruction.accounts,
                data: JupiterInstruction::DeltafiSwap(decoded_instruction),
            });
        }
        if let Some(decoded_instruction) =
            balansol_swap::BalansolSwap::deserialize(instruction.data.as_slice())
        {
            return Some(carbon_core::instruction::DecodedInstruction {
                program_id: instruction.program_id,
                accounts: instruction.accounts,
                data: JupiterInstruction::BalansolSwap(decoded_instruction),
            });
        }
        if let Some(decoded_instruction) =
            marco_polo_swap::MarcoPoloSwap::deserialize(instruction.data.as_slice())
        {
            return Some(carbon_core::instruction::DecodedInstruction {
                program_id: instruction.program_id,
                accounts: instruction.accounts,
                data: JupiterInstruction::MarcoPoloSwap(decoded_instruction),
            });
        }
        if let Some(decoded_instruction) =
            dradex_swap::DradexSwap::deserialize(instruction.data.as_slice())
        {
            return Some(carbon_core::instruction::DecodedInstruction {
                program_id: instruction.program_id,
                accounts: instruction.accounts,
                data: JupiterInstruction::DradexSwap(decoded_instruction),
            });
        }
        if let Some(decoded_instruction) =
            lifinity_v2_swap::LifinityV2Swap::deserialize(instruction.data.as_slice())
        {
            return Some(carbon_core::instruction::DecodedInstruction {
                program_id: instruction.program_id,
                accounts: instruction.accounts,
                data: JupiterInstruction::LifinityV2Swap(decoded_instruction),
            });
        }
        if let Some(decoded_instruction) =
            raydium_clmm_swap::RaydiumClmmSwap::deserialize(instruction.data.as_slice())
        {
            return Some(carbon_core::instruction::DecodedInstruction {
                program_id: instruction.program_id,
                accounts: instruction.accounts,
                data: JupiterInstruction::RaydiumClmmSwap(decoded_instruction),
            });
        }
        if let Some(decoded_instruction) =
            phoenix_swap::PhoenixSwap::deserialize(instruction.data.as_slice())
        {
            return Some(carbon_core::instruction::DecodedInstruction {
                program_id: instruction.program_id,
                accounts: instruction.accounts,
                data: JupiterInstruction::PhoenixSwap(decoded_instruction),
            });
        }
        if let Some(decoded_instruction) =
            symmetry_swap::SymmetrySwap::deserialize(instruction.data.as_slice())
        {
            return Some(carbon_core::instruction::DecodedInstruction {
                program_id: instruction.program_id,
                accounts: instruction.accounts,
                data: JupiterInstruction::SymmetrySwap(decoded_instruction),
            });
        }
        if let Some(decoded_instruction) =
            helium_treasury_management_redeem_v0::HeliumTreasuryManagementRedeemV0::deserialize(
                instruction.data.as_slice(),
            )
        {
            return Some(carbon_core::instruction::DecodedInstruction {
                program_id: instruction.program_id,
                accounts: instruction.accounts,
                data: JupiterInstruction::HeliumTreasuryManagementRedeemV0(decoded_instruction),
            });
        }
        if let Some(decoded_instruction) =
            goosefx_v2_swap::GoosefxV2Swap::deserialize(instruction.data.as_slice())
        {
            return Some(carbon_core::instruction::DecodedInstruction {
                program_id: instruction.program_id,
                accounts: instruction.accounts,
                data: JupiterInstruction::GoosefxV2Swap(decoded_instruction),
            });
        }
        if let Some(decoded_instruction) =
            perps_swap::PerpsSwap::deserialize(instruction.data.as_slice())
        {
            return Some(carbon_core::instruction::DecodedInstruction {
                program_id: instruction.program_id,
                accounts: instruction.accounts,
                data: JupiterInstruction::PerpsSwap(decoded_instruction),
            });
        }
        if let Some(decoded_instruction) =
            perps_add_liquidity::PerpsAddLiquidity::deserialize(instruction.data.as_slice())
        {
            return Some(carbon_core::instruction::DecodedInstruction {
                program_id: instruction.program_id,
                accounts: instruction.accounts,
                data: JupiterInstruction::PerpsAddLiquidity(decoded_instruction),
            });
        }
        if let Some(decoded_instruction) =
            perps_remove_liquidity::PerpsRemoveLiquidity::deserialize(instruction.data.as_slice())
        {
            return Some(carbon_core::instruction::DecodedInstruction {
                program_id: instruction.program_id,
                accounts: instruction.accounts,
                data: JupiterInstruction::PerpsRemoveLiquidity(decoded_instruction),
            });
        }
        if let Some(decoded_instruction) =
            meteora_dlmm_swap::MeteoraDlmmSwap::deserialize(instruction.data.as_slice())
        {
            return Some(carbon_core::instruction::DecodedInstruction {
                program_id: instruction.program_id,
                accounts: instruction.accounts,
                data: JupiterInstruction::MeteoraDlmmSwap(decoded_instruction),
            });
        }
        if let Some(decoded_instruction) =
            swap_event::SwapEvent::deserialize(instruction.data.as_slice())
        {
            return Some(carbon_core::instruction::DecodedInstruction {
                program_id: instruction.program_id,
                accounts: instruction.accounts,
                data: JupiterInstruction::SwapEvent(decoded_instruction),
            });
        }
        if let Some(decoded_instruction) =
            fee_event::FeeEvent::deserialize(instruction.data.as_slice())
        {
            return Some(carbon_core::instruction::DecodedInstruction {
                program_id: instruction.program_id,
                accounts: instruction.accounts,
                data: JupiterInstruction::FeeEvent(decoded_instruction),
            });
        }

        None
    }
}
