use super::JupiterSwapDecoder;
pub mod claim;
pub mod claim_token;
pub mod close_token;
pub mod create_token_account;
pub mod create_token_ledger;
pub mod exact_out_route;
pub mod exact_out_route_v2;
pub mod fee_event;
pub mod route;
pub mod route_v2;
pub mod route_with_token_ledger;
pub mod set_token_ledger;
pub mod shared_accounts_exact_out_route;
pub mod shared_accounts_exact_out_route_v2;
pub mod shared_accounts_route;
pub mod shared_accounts_route_v2;
pub mod shared_accounts_route_with_token_ledger;
pub mod swap_event;
pub mod swaps_event;

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
pub enum JupiterSwapInstruction {
    Claim(claim::Claim),
    ClaimToken(claim_token::ClaimToken),
    CloseToken(close_token::CloseToken),
    CreateTokenLedger(create_token_ledger::CreateTokenLedger),
    CreateTokenAccount(create_token_account::CreateTokenAccount),
    ExactOutRoute(exact_out_route::ExactOutRoute),
    Route(route::Route),
    RouteWithTokenLedger(route_with_token_ledger::RouteWithTokenLedger),
    SetTokenLedger(set_token_ledger::SetTokenLedger),
    SharedAccountsExactOutRoute(shared_accounts_exact_out_route::SharedAccountsExactOutRoute),
    SharedAccountsRoute(shared_accounts_route::SharedAccountsRoute),
    SharedAccountsRouteWithTokenLedger(
        shared_accounts_route_with_token_ledger::SharedAccountsRouteWithTokenLedger,
    ),
    ExactOutRouteV2(exact_out_route_v2::ExactOutRouteV2),
    RouteV2(route_v2::RouteV2),
    SharedAccountsExactOutRouteV2(
        shared_accounts_exact_out_route_v2::SharedAccountsExactOutRouteV2,
    ),
    SharedAccountsRouteV2(shared_accounts_route_v2::SharedAccountsRouteV2),
    FeeEvent(fee_event::FeeEvent),
    SwapEvent(swap_event::SwapEvent),
    SwapsEvent(swaps_event::SwapsEvent),
}

impl<'a> carbon_core::instruction::InstructionDecoder<'a> for JupiterSwapDecoder {
    type InstructionType = JupiterSwapInstruction;

    fn decode_instruction(
        &self,
        instruction: &solana_instruction::Instruction,
    ) -> Option<carbon_core::instruction::DecodedInstruction<Self::InstructionType>> {
        carbon_core::try_decode_instructions!(instruction,
            JupiterSwapInstruction::Claim => claim::Claim,
            JupiterSwapInstruction::ClaimToken => claim_token::ClaimToken,
            JupiterSwapInstruction::CloseToken => close_token::CloseToken,
            JupiterSwapInstruction::CreateTokenLedger => create_token_ledger::CreateTokenLedger,
            JupiterSwapInstruction::CreateTokenAccount => create_token_account::CreateTokenAccount,
            JupiterSwapInstruction::ExactOutRoute => exact_out_route::ExactOutRoute,
            JupiterSwapInstruction::Route => route::Route,
            JupiterSwapInstruction::RouteWithTokenLedger => route_with_token_ledger::RouteWithTokenLedger,
            JupiterSwapInstruction::SetTokenLedger => set_token_ledger::SetTokenLedger,
            JupiterSwapInstruction::SharedAccountsExactOutRoute => shared_accounts_exact_out_route::SharedAccountsExactOutRoute,
            JupiterSwapInstruction::SharedAccountsRoute => shared_accounts_route::SharedAccountsRoute,
            JupiterSwapInstruction::SharedAccountsRouteWithTokenLedger => shared_accounts_route_with_token_ledger::SharedAccountsRouteWithTokenLedger,
            JupiterSwapInstruction::ExactOutRouteV2 => exact_out_route_v2::ExactOutRouteV2,
            JupiterSwapInstruction::RouteV2 => route_v2::RouteV2,
            JupiterSwapInstruction::SharedAccountsExactOutRouteV2 => shared_accounts_exact_out_route_v2::SharedAccountsExactOutRouteV2,
            JupiterSwapInstruction::SharedAccountsRouteV2 => shared_accounts_route_v2::SharedAccountsRouteV2,
            JupiterSwapInstruction::FeeEvent => fee_event::FeeEvent,
            JupiterSwapInstruction::SwapEvent => swap_event::SwapEvent,
            JupiterSwapInstruction::SwapsEvent => swaps_event::SwapsEvent,
        )
    }
}
