use super::*;

use carbon_core::{borsh, CarbonDeserialize};

#[derive(
    CarbonDeserialize, Debug, serde::Serialize, serde::Deserialize, PartialEq, Eq, Clone, Hash,
)]
pub enum PlasmaEvent {
    Swap {
        header: PlasmaEventHeader,
        event: SwapEvent,
    },
    AddLiquidity {
        header: PlasmaEventHeader,
        event: AddLiquidityEvent,
    },
    RemoveLiquidity {
        header: PlasmaEventHeader,
        event: RemoveLiquidityEvent,
    },
    RenounceLiquidity {
        header: PlasmaEventHeader,
        event: RenounceLiquidityEvent,
    },
    WithdrawLpFees {
        header: PlasmaEventHeader,
        event: WithdrawLpFeesEvent,
    },
    InitializeLpPosition {
        header: PlasmaEventHeader,
        event: InitializeLpPositionEvent,
    },
    InitializePool {
        header: PlasmaEventHeader,
        event: InitializePoolEvent,
    },
    WithdrawProtocolFees {
        header: PlasmaEventHeader,
        event: WithdrawProtocolFeesEvent,
    },
}
