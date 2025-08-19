use {super::PerpetualsDecoder, crate::PROGRAM_ID};
pub mod add_custody;
pub mod add_liquidity2;
pub mod add_liquidity_event;
pub mod add_pool;
pub mod close_position_request;
pub mod close_position_request_event;
pub mod create_decrease_position_market_request;
pub mod create_decrease_position_request2;
pub mod create_increase_position_market_request;
pub mod create_position_request_event;
pub mod create_token_ledger;
pub mod create_token_metadata;
pub mod decrease_position4;
pub mod decrease_position_event;
pub mod decrease_position_post_swap_event;
pub mod decrease_position_with_internal_swap;
pub mod get_add_liquidity_amount_and_fee2;
pub mod get_assets_under_management2;
pub mod get_remove_liquidity_amount_and_fee2;
pub mod increase_position4;
pub mod increase_position_event;
pub mod increase_position_pre_swap;
pub mod increase_position_pre_swap_event;
pub mod increase_position_with_internal_swap;
pub mod init;
pub mod instant_create_limit_order;
pub mod instant_create_limit_order_event;
pub mod instant_create_tpsl;
pub mod instant_create_tpsl_event;
pub mod instant_decrease_position;
pub mod instant_decrease_position_event;
pub mod instant_increase_position;
pub mod instant_increase_position_event;
pub mod instant_update_limit_order;
pub mod instant_update_tpsl;
pub mod instant_update_tpsl_event;
pub mod liquidate_full_position4;
pub mod liquidate_full_position_event;
pub mod operator_set_custody_config;
pub mod operator_set_pool_config;
pub mod pool_swap_event;
pub mod pool_swap_exact_out_event;
pub mod refresh_assets_under_management;
pub mod remove_liquidity2;
pub mod remove_liquidity_event;
pub mod set_custody_config;
pub mod set_perpetuals_config;
pub mod set_pool_config;
pub mod set_test_time;
pub mod set_token_ledger;
pub mod swap2;
pub mod test_init;
pub mod transfer_admin;
pub mod update_decrease_position_request2;
pub mod withdraw_fees2;

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
pub enum PerpetualsInstruction {
    Init(init::Init),
    AddPool(add_pool::AddPool),
    AddCustody(add_custody::AddCustody),
    SetCustodyConfig(set_custody_config::SetCustodyConfig),
    SetPoolConfig(set_pool_config::SetPoolConfig),
    SetPerpetualsConfig(set_perpetuals_config::SetPerpetualsConfig),
    TransferAdmin(transfer_admin::TransferAdmin),
    WithdrawFees2(withdraw_fees2::WithdrawFees2),
    CreateTokenMetadata(create_token_metadata::CreateTokenMetadata),
    CreateTokenLedger(create_token_ledger::CreateTokenLedger),
    OperatorSetCustodyConfig(operator_set_custody_config::OperatorSetCustodyConfig),
    OperatorSetPoolConfig(operator_set_pool_config::OperatorSetPoolConfig),
    TestInit(test_init::TestInit),
    SetTestTime(set_test_time::SetTestTime),
    SetTokenLedger(set_token_ledger::SetTokenLedger),
    Swap2(swap2::Swap2),
    AddLiquidity2(add_liquidity2::AddLiquidity2),
    RemoveLiquidity2(remove_liquidity2::RemoveLiquidity2),
    CreateIncreasePositionMarketRequest(
        create_increase_position_market_request::CreateIncreasePositionMarketRequest,
    ),
    CreateDecreasePositionRequest2(
        create_decrease_position_request2::CreateDecreasePositionRequest2,
    ),
    CreateDecreasePositionMarketRequest(
        create_decrease_position_market_request::CreateDecreasePositionMarketRequest,
    ),
    UpdateDecreasePositionRequest2(
        update_decrease_position_request2::UpdateDecreasePositionRequest2,
    ),
    ClosePositionRequest(close_position_request::ClosePositionRequest),
    IncreasePosition4(increase_position4::IncreasePosition4),
    IncreasePositionPreSwap(increase_position_pre_swap::IncreasePositionPreSwap),
    IncreasePositionWithInternalSwap(
        increase_position_with_internal_swap::IncreasePositionWithInternalSwap,
    ),
    DecreasePosition4(decrease_position4::DecreasePosition4),
    DecreasePositionWithInternalSwap(
        decrease_position_with_internal_swap::DecreasePositionWithInternalSwap,
    ),
    LiquidateFullPosition4(liquidate_full_position4::LiquidateFullPosition4),
    RefreshAssetsUnderManagement(refresh_assets_under_management::RefreshAssetsUnderManagement),
    InstantCreateTpsl(instant_create_tpsl::InstantCreateTpsl),
    InstantCreateLimitOrder(instant_create_limit_order::InstantCreateLimitOrder),
    InstantIncreasePosition(instant_increase_position::InstantIncreasePosition),
    InstantDecreasePosition(instant_decrease_position::InstantDecreasePosition),
    InstantUpdateLimitOrder(instant_update_limit_order::InstantUpdateLimitOrder),
    InstantUpdateTpsl(instant_update_tpsl::InstantUpdateTpsl),
    GetAddLiquidityAmountAndFee2(get_add_liquidity_amount_and_fee2::GetAddLiquidityAmountAndFee2),
    GetRemoveLiquidityAmountAndFee2(
        get_remove_liquidity_amount_and_fee2::GetRemoveLiquidityAmountAndFee2,
    ),
    GetAssetsUnderManagement2(get_assets_under_management2::GetAssetsUnderManagement2),
    CreatePositionRequestEvent(create_position_request_event::CreatePositionRequestEvent),
    InstantCreateTpslEvent(instant_create_tpsl_event::InstantCreateTpslEvent),
    InstantUpdateTpslEvent(instant_update_tpsl_event::InstantUpdateTpslEvent),
    ClosePositionRequestEvent(close_position_request_event::ClosePositionRequestEvent),
    IncreasePositionEvent(increase_position_event::IncreasePositionEvent),
    IncreasePositionPreSwapEvent(increase_position_pre_swap_event::IncreasePositionPreSwapEvent),
    DecreasePositionEvent(decrease_position_event::DecreasePositionEvent),
    DecreasePositionPostSwapEvent(decrease_position_post_swap_event::DecreasePositionPostSwapEvent),
    LiquidateFullPositionEvent(liquidate_full_position_event::LiquidateFullPositionEvent),
    PoolSwapEvent(pool_swap_event::PoolSwapEvent),
    PoolSwapExactOutEvent(pool_swap_exact_out_event::PoolSwapExactOutEvent),
    AddLiquidityEvent(add_liquidity_event::AddLiquidityEvent),
    RemoveLiquidityEvent(remove_liquidity_event::RemoveLiquidityEvent),
    InstantCreateLimitOrderEvent(instant_create_limit_order_event::InstantCreateLimitOrderEvent),
    InstantIncreasePositionEvent(instant_increase_position_event::InstantIncreasePositionEvent),
    InstantDecreasePositionEvent(instant_decrease_position_event::InstantDecreasePositionEvent),
}

impl carbon_core::instruction::InstructionDecoder<'_> for PerpetualsDecoder {
    type InstructionType = PerpetualsInstruction;

    fn decode_instruction(
        &self,
        instruction: &solana_instruction::Instruction,
    ) -> Option<carbon_core::instruction::DecodedInstruction<Self::InstructionType>> {
        if !instruction.program_id.eq(&PROGRAM_ID) {
            return None;
        }

        carbon_core::try_decode_instructions!(instruction,
            PerpetualsInstruction::Init => init::Init,
            PerpetualsInstruction::AddPool => add_pool::AddPool,
            PerpetualsInstruction::AddCustody => add_custody::AddCustody,
            PerpetualsInstruction::SetCustodyConfig => set_custody_config::SetCustodyConfig,
            PerpetualsInstruction::SetPoolConfig => set_pool_config::SetPoolConfig,
            PerpetualsInstruction::SetPerpetualsConfig => set_perpetuals_config::SetPerpetualsConfig,
            PerpetualsInstruction::TransferAdmin => transfer_admin::TransferAdmin,
            PerpetualsInstruction::WithdrawFees2 => withdraw_fees2::WithdrawFees2,
            PerpetualsInstruction::CreateTokenMetadata => create_token_metadata::CreateTokenMetadata,
            PerpetualsInstruction::CreateTokenLedger => create_token_ledger::CreateTokenLedger,
            PerpetualsInstruction::OperatorSetCustodyConfig => operator_set_custody_config::OperatorSetCustodyConfig,
            PerpetualsInstruction::OperatorSetPoolConfig => operator_set_pool_config::OperatorSetPoolConfig,
            PerpetualsInstruction::TestInit => test_init::TestInit,
            PerpetualsInstruction::SetTestTime => set_test_time::SetTestTime,
            PerpetualsInstruction::SetTokenLedger => set_token_ledger::SetTokenLedger,
            PerpetualsInstruction::Swap2 => swap2::Swap2,
            PerpetualsInstruction::AddLiquidity2 => add_liquidity2::AddLiquidity2,
            PerpetualsInstruction::RemoveLiquidity2 => remove_liquidity2::RemoveLiquidity2,
            PerpetualsInstruction::CreateIncreasePositionMarketRequest => create_increase_position_market_request::CreateIncreasePositionMarketRequest,
            PerpetualsInstruction::CreateDecreasePositionRequest2 => create_decrease_position_request2::CreateDecreasePositionRequest2,
            PerpetualsInstruction::CreateDecreasePositionMarketRequest => create_decrease_position_market_request::CreateDecreasePositionMarketRequest,
            PerpetualsInstruction::UpdateDecreasePositionRequest2 => update_decrease_position_request2::UpdateDecreasePositionRequest2,
            PerpetualsInstruction::ClosePositionRequest => close_position_request::ClosePositionRequest,
            PerpetualsInstruction::IncreasePosition4 => increase_position4::IncreasePosition4,
            PerpetualsInstruction::IncreasePositionPreSwap => increase_position_pre_swap::IncreasePositionPreSwap,
            PerpetualsInstruction::IncreasePositionWithInternalSwap => increase_position_with_internal_swap::IncreasePositionWithInternalSwap,
            PerpetualsInstruction::DecreasePosition4 => decrease_position4::DecreasePosition4,
            PerpetualsInstruction::DecreasePositionWithInternalSwap => decrease_position_with_internal_swap::DecreasePositionWithInternalSwap,
            PerpetualsInstruction::LiquidateFullPosition4 => liquidate_full_position4::LiquidateFullPosition4,
            PerpetualsInstruction::RefreshAssetsUnderManagement => refresh_assets_under_management::RefreshAssetsUnderManagement,
            PerpetualsInstruction::InstantCreateTpsl => instant_create_tpsl::InstantCreateTpsl,
            PerpetualsInstruction::InstantCreateLimitOrder => instant_create_limit_order::InstantCreateLimitOrder,
            PerpetualsInstruction::InstantIncreasePosition => instant_increase_position::InstantIncreasePosition,
            PerpetualsInstruction::InstantDecreasePosition => instant_decrease_position::InstantDecreasePosition,
            PerpetualsInstruction::InstantUpdateLimitOrder => instant_update_limit_order::InstantUpdateLimitOrder,
            PerpetualsInstruction::InstantUpdateTpsl => instant_update_tpsl::InstantUpdateTpsl,
            PerpetualsInstruction::GetAddLiquidityAmountAndFee2 => get_add_liquidity_amount_and_fee2::GetAddLiquidityAmountAndFee2,
            PerpetualsInstruction::GetRemoveLiquidityAmountAndFee2 => get_remove_liquidity_amount_and_fee2::GetRemoveLiquidityAmountAndFee2,
            PerpetualsInstruction::GetAssetsUnderManagement2 => get_assets_under_management2::GetAssetsUnderManagement2,
            PerpetualsInstruction::CreatePositionRequestEvent => create_position_request_event::CreatePositionRequestEvent,
            PerpetualsInstruction::InstantCreateTpslEvent => instant_create_tpsl_event::InstantCreateTpslEvent,
            PerpetualsInstruction::InstantUpdateTpslEvent => instant_update_tpsl_event::InstantUpdateTpslEvent,
            PerpetualsInstruction::ClosePositionRequestEvent => close_position_request_event::ClosePositionRequestEvent,
            PerpetualsInstruction::IncreasePositionEvent => increase_position_event::IncreasePositionEvent,
            PerpetualsInstruction::IncreasePositionPreSwapEvent => increase_position_pre_swap_event::IncreasePositionPreSwapEvent,
            PerpetualsInstruction::DecreasePositionEvent => decrease_position_event::DecreasePositionEvent,
            PerpetualsInstruction::DecreasePositionPostSwapEvent => decrease_position_post_swap_event::DecreasePositionPostSwapEvent,
            PerpetualsInstruction::LiquidateFullPositionEvent => liquidate_full_position_event::LiquidateFullPositionEvent,
            PerpetualsInstruction::PoolSwapEvent => pool_swap_event::PoolSwapEvent,
            PerpetualsInstruction::PoolSwapExactOutEvent => pool_swap_exact_out_event::PoolSwapExactOutEvent,
            PerpetualsInstruction::AddLiquidityEvent => add_liquidity_event::AddLiquidityEvent,
            PerpetualsInstruction::RemoveLiquidityEvent => remove_liquidity_event::RemoveLiquidityEvent,
            PerpetualsInstruction::InstantCreateLimitOrderEvent => instant_create_limit_order_event::InstantCreateLimitOrderEvent,
            PerpetualsInstruction::InstantIncreasePositionEvent => instant_increase_position_event::InstantIncreasePositionEvent,
            PerpetualsInstruction::InstantDecreasePositionEvent => instant_decrease_position_event::InstantDecreasePositionEvent,
        )
    }
}
