use {
    carbon_core::{
        error::{CarbonResult, Error as CarbonError},
        postgres::metadata::InstructionRowMetadata,
    },
    carbon_jupiter_swap_decoder::types::{RoutePlanStep, RoutePlanStepV2, Swap},
    serde_json::Value,
    solana_instruction::AccountMeta,
};

#[derive(Debug, Clone)]
pub struct AccountMetaRecord {
    pub position: i32,
    pub pubkey: String,
    pub is_signer: bool,
    pub is_writable: bool,
}

#[derive(Debug, Clone)]
pub struct NormalizedRoutePlanStep {
    pub step_index: i32,
    pub swap_variant: String,
    pub swap_json: Value,
    pub weight_percent: Option<i32>,
    pub weight_bps: Option<i32>,
    pub input_index: i32,
    pub output_index: i32,
}

#[derive(Debug, Clone)]
pub struct RouteInstructionRecord {
    pub metadata: InstructionRowMetadata,
    pub variant: String,
    pub shared_accounts_id: Option<u8>,
    pub in_amount: Option<u64>,
    pub out_amount: Option<u64>,
    pub quoted_in_amount: Option<u64>,
    pub quoted_out_amount: Option<u64>,
    pub slippage_bps: Option<u16>,
    pub platform_fee_bps: Option<u16>,
    pub positive_slippage_bps: Option<u16>,
    pub source_mint: Option<String>,
    pub destination_mint: Option<String>,
    pub route_plan: Value,
    pub route_plan_version: i16,
    pub accounts: Vec<AccountMetaRecord>,
    pub steps: Vec<NormalizedRoutePlanStep>,
}

#[derive(Debug, Clone)]
pub enum SwapEventType {
    SwapEvent,
    SwapsEvent,
}

impl SwapEventType {
    pub fn as_str(&self) -> &'static str {
        match self {
            Self::SwapEvent => "SwapEvent",
            Self::SwapsEvent => "SwapsEvent",
        }
    }
}

#[derive(Debug, Clone)]
pub struct SwapEventRecord {
    pub metadata: InstructionRowMetadata,
    pub event_type: SwapEventType,
    pub event_index: i32,
    pub batch_size: Option<i32>,
    pub input_mint: String,
    pub output_mint: String,
    pub input_amount: u64,
    pub output_amount: u64,
    pub amm: Option<String>,
    pub raw_event: Value,
}

pub fn capture_account_metas(accounts: &[AccountMeta]) -> Vec<AccountMetaRecord> {
    accounts
        .iter()
        .enumerate()
        .map(|(index, meta)| AccountMetaRecord {
            position: index as i32,
            pubkey: meta.pubkey.to_string(),
            is_signer: meta.is_signer,
            is_writable: meta.is_writable,
        })
        .collect()
}

pub fn normalize_route_plan_v1(
    steps: &[RoutePlanStep],
) -> CarbonResult<Vec<NormalizedRoutePlanStep>> {
    steps
        .iter()
        .enumerate()
        .map(|(idx, step)| {
            let swap_json = serde_json::to_value(&step.swap)
                .map_err(|err| CarbonError::Custom(err.to_string()))?;
            Ok(NormalizedRoutePlanStep {
                step_index: idx as i32,
                swap_variant: swap_variant_name(&step.swap),
                swap_json,
                weight_percent: Some(step.percent as i32),
                weight_bps: Some((step.percent as i32) * 100),
                input_index: step.input_index as i32,
                output_index: step.output_index as i32,
            })
        })
        .collect()
}

pub fn normalize_route_plan_v2(
    steps: &[RoutePlanStepV2],
) -> CarbonResult<Vec<NormalizedRoutePlanStep>> {
    steps
        .iter()
        .enumerate()
        .map(|(idx, step)| {
            let swap_json = serde_json::to_value(&step.swap)
                .map_err(|err| CarbonError::Custom(err.to_string()))?;
            Ok(NormalizedRoutePlanStep {
                step_index: idx as i32,
                swap_variant: swap_variant_name(&step.swap),
                swap_json,
                weight_percent: None,
                weight_bps: Some(step.bps as i32),
                input_index: step.input_index as i32,
                output_index: step.output_index as i32,
            })
        })
        .collect()
}

pub fn route_plan_json_v1(steps: &[RoutePlanStep]) -> CarbonResult<Value> {
    serde_json::to_value(steps).map_err(|err| CarbonError::Custom(err.to_string()))
}

pub fn route_plan_json_v2(steps: &[RoutePlanStepV2]) -> CarbonResult<Value> {
    serde_json::to_value(steps).map_err(|err| CarbonError::Custom(err.to_string()))
}

pub fn swap_variant_name(swap: &Swap) -> String {
    match serde_json::to_value(swap) {
        Ok(Value::String(name)) => name,
        Ok(Value::Object(map)) => map
            .keys()
            .next()
            .cloned()
            .unwrap_or_else(|| format!("{:?}", swap)),
        _ => format!("{:?}", swap),
    }
}
