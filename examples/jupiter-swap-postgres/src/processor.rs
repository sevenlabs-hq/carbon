use {
    crate::{
        db::JupiterSwapRepository,
        models::{
            capture_account_metas, normalize_route_plan_v1, normalize_route_plan_v2,
            route_plan_json_v1, route_plan_json_v2, AccountMetaRecord, NormalizedRoutePlanStep,
            RouteInstructionRecord, SwapEventRecord, SwapEventType,
        },
    },
    carbon_core::{
        deserialize::ArrangeAccounts,
        error::{CarbonResult, Error as CarbonError},
        instruction::{InstructionMetadata, InstructionProcessorInputType},
        metrics::MetricsCollection,
        postgres::metadata::InstructionRowMetadata,
        processor::Processor,
    },
    carbon_jupiter_swap_decoder::instructions::{
        exact_out_route, exact_out_route_v2, route, route_v2, route_with_token_ledger,
        shared_accounts_exact_out_route, shared_accounts_exact_out_route_v2, shared_accounts_route,
        shared_accounts_route_v2, shared_accounts_route_with_token_ledger, JupiterSwapInstruction,
    },
    solana_instruction::AccountMeta,
    solana_pubkey::Pubkey,
    sqlx::PgPool,
    std::{sync::Arc, time::Instant},
};

pub struct JupiterSwapProcessor {
    repository: JupiterSwapRepository,
}

impl JupiterSwapProcessor {
    pub fn new(pool: PgPool) -> Self {
        Self {
            repository: JupiterSwapRepository::new(pool),
        }
    }

    fn instruction_metadata(metadata: &InstructionMetadata) -> InstructionRowMetadata {
        metadata.clone().into()
    }

    fn account_records(accounts: &[AccountMeta]) -> Vec<AccountMetaRecord> {
        capture_account_metas(accounts)
    }

    fn base_route_record(
        metadata: InstructionRowMetadata,
        variant: &str,
        shared_accounts_id: Option<u8>,
        in_amount: Option<u64>,
        out_amount: Option<u64>,
        quoted_in_amount: Option<u64>,
        quoted_out_amount: Option<u64>,
        slippage_bps: Option<u16>,
        platform_fee_bps: Option<u16>,
        positive_slippage_bps: Option<u16>,
        source_mint: Option<String>,
        destination_mint: Option<String>,
        route_plan: serde_json::Value,
        route_plan_version: i16,
        accounts: Vec<AccountMetaRecord>,
        steps: Vec<NormalizedRoutePlanStep>,
    ) -> RouteInstructionRecord {
        RouteInstructionRecord {
            metadata,
            variant: variant.to_string(),
            shared_accounts_id,
            in_amount,
            out_amount,
            quoted_in_amount,
            quoted_out_amount,
            slippage_bps,
            platform_fee_bps,
            positive_slippage_bps,
            source_mint,
            destination_mint,
            route_plan,
            route_plan_version,
            accounts,
            steps,
        }
    }

    fn pubkey_to_string(pubkey: Pubkey) -> String {
        pubkey.to_string()
    }
}

#[async_trait::async_trait]
impl Processor for JupiterSwapProcessor {
    type InputType = InstructionProcessorInputType<JupiterSwapInstruction>;

    async fn process(
        &mut self,
        input: Self::InputType,
        metrics: Arc<MetricsCollection>,
    ) -> CarbonResult<()> {
        let (metadata, decoded_instruction, _nested, _raw) = input;
        let mut handled = false;
        let start = Instant::now();

        let slot_result = match decoded_instruction.data {
            JupiterSwapInstruction::Route(data) => {
                handled = true;
                let row_metadata = Self::instruction_metadata(&metadata);
                let accounts = Self::account_records(&decoded_instruction.accounts);
                let plan_json = route_plan_json_v1(&data.route_plan)?;
                let steps = normalize_route_plan_v1(&data.route_plan)?;
                let arranged = route::Route::arrange_accounts(&decoded_instruction.accounts);
                let destination_mint = arranged
                    .as_ref()
                    .map(|accounts| Self::pubkey_to_string(accounts.destination_mint));
                let record = Self::base_route_record(
                    row_metadata,
                    "Route",
                    None,
                    Some(data.in_amount),
                    None,
                    None,
                    Some(data.quoted_out_amount),
                    Some(data.slippage_bps),
                    Some(data.platform_fee_bps as u16),
                    None,
                    None,
                    destination_mint,
                    plan_json,
                    1,
                    accounts,
                    steps,
                );
                self.repository.upsert_route_instruction(record).await
            }
            JupiterSwapInstruction::RouteWithTokenLedger(data) => {
                handled = true;
                let row_metadata = Self::instruction_metadata(&metadata);
                let accounts = Self::account_records(&decoded_instruction.accounts);
                let plan_json = route_plan_json_v1(&data.route_plan)?;
                let steps = normalize_route_plan_v1(&data.route_plan)?;
                let arranged = route_with_token_ledger::RouteWithTokenLedger::arrange_accounts(
                    &decoded_instruction.accounts,
                );
                let destination_mint = arranged
                    .as_ref()
                    .map(|accounts| Self::pubkey_to_string(accounts.destination_mint));
                let record = Self::base_route_record(
                    row_metadata,
                    "RouteWithTokenLedger",
                    None,
                    None,
                    None,
                    None,
                    Some(data.quoted_out_amount),
                    Some(data.slippage_bps),
                    Some(data.platform_fee_bps as u16),
                    None,
                    None,
                    destination_mint,
                    plan_json,
                    1,
                    accounts,
                    steps,
                );
                self.repository.upsert_route_instruction(record).await
            }
            JupiterSwapInstruction::RouteV2(data) => {
                handled = true;
                let row_metadata = Self::instruction_metadata(&metadata);
                let accounts = Self::account_records(&decoded_instruction.accounts);
                let plan_json = route_plan_json_v2(&data.route_plan)?;
                let steps = normalize_route_plan_v2(&data.route_plan)?;
                let arranged = route_v2::RouteV2::arrange_accounts(&decoded_instruction.accounts);
                let source_mint = arranged
                    .as_ref()
                    .map(|accounts| Self::pubkey_to_string(accounts.source_mint));
                let destination_mint = arranged
                    .as_ref()
                    .map(|accounts| Self::pubkey_to_string(accounts.destination_mint));
                let record = Self::base_route_record(
                    row_metadata,
                    "RouteV2",
                    None,
                    Some(data.in_amount),
                    None,
                    None,
                    Some(data.quoted_out_amount),
                    Some(data.slippage_bps),
                    Some(data.platform_fee_bps),
                    Some(data.positive_slippage_bps),
                    source_mint,
                    destination_mint,
                    plan_json,
                    2,
                    accounts,
                    steps,
                );
                self.repository.upsert_route_instruction(record).await
            }
            JupiterSwapInstruction::SharedAccountsRoute(data) => {
                handled = true;
                let row_metadata = Self::instruction_metadata(&metadata);
                let accounts = Self::account_records(&decoded_instruction.accounts);
                let plan_json = route_plan_json_v1(&data.route_plan)?;
                let steps = normalize_route_plan_v1(&data.route_plan)?;
                let arranged = shared_accounts_route::SharedAccountsRoute::arrange_accounts(
                    &decoded_instruction.accounts,
                );
                let source_mint = arranged
                    .as_ref()
                    .map(|accounts| Self::pubkey_to_string(accounts.source_mint));
                let destination_mint = arranged
                    .as_ref()
                    .map(|accounts| Self::pubkey_to_string(accounts.destination_mint));
                let record = Self::base_route_record(
                    row_metadata,
                    "SharedAccountsRoute",
                    Some(data.id),
                    Some(data.in_amount),
                    None,
                    None,
                    Some(data.quoted_out_amount),
                    Some(data.slippage_bps),
                    Some(data.platform_fee_bps as u16),
                    None,
                    source_mint,
                    destination_mint,
                    plan_json,
                    1,
                    accounts,
                    steps,
                );
                self.repository.upsert_route_instruction(record).await
            }
            JupiterSwapInstruction::SharedAccountsRouteWithTokenLedger(data) => {
                handled = true;
                let row_metadata = Self::instruction_metadata(&metadata);
                let accounts = Self::account_records(&decoded_instruction.accounts);
                let plan_json = route_plan_json_v1(&data.route_plan)?;
                let steps = normalize_route_plan_v1(&data.route_plan)?;
                let arranged =
                    shared_accounts_route_with_token_ledger::SharedAccountsRouteWithTokenLedger::arrange_accounts(
                        &decoded_instruction.accounts,
                    );
                let source_mint = arranged
                    .as_ref()
                    .map(|accounts| Self::pubkey_to_string(accounts.source_mint));
                let destination_mint = arranged
                    .as_ref()
                    .map(|accounts| Self::pubkey_to_string(accounts.destination_mint));
                let record = Self::base_route_record(
                    row_metadata,
                    "SharedAccountsRouteWithTokenLedger",
                    Some(data.id),
                    None,
                    None,
                    None,
                    Some(data.quoted_out_amount),
                    Some(data.slippage_bps),
                    Some(data.platform_fee_bps as u16),
                    None,
                    source_mint,
                    destination_mint,
                    plan_json,
                    1,
                    accounts,
                    steps,
                );
                self.repository.upsert_route_instruction(record).await
            }
            JupiterSwapInstruction::SharedAccountsRouteV2(data) => {
                handled = true;
                let row_metadata = Self::instruction_metadata(&metadata);
                let accounts = Self::account_records(&decoded_instruction.accounts);
                let plan_json = route_plan_json_v2(&data.route_plan)?;
                let steps = normalize_route_plan_v2(&data.route_plan)?;
                let arranged = shared_accounts_route_v2::SharedAccountsRouteV2::arrange_accounts(
                    &decoded_instruction.accounts,
                );
                let source_mint = arranged
                    .as_ref()
                    .map(|accounts| Self::pubkey_to_string(accounts.source_mint));
                let destination_mint = arranged
                    .as_ref()
                    .map(|accounts| Self::pubkey_to_string(accounts.destination_mint));
                let record = Self::base_route_record(
                    row_metadata,
                    "SharedAccountsRouteV2",
                    Some(data.id),
                    Some(data.in_amount),
                    None,
                    None,
                    Some(data.quoted_out_amount),
                    Some(data.slippage_bps),
                    Some(data.platform_fee_bps),
                    Some(data.positive_slippage_bps),
                    source_mint,
                    destination_mint,
                    plan_json,
                    2,
                    accounts,
                    steps,
                );
                self.repository.upsert_route_instruction(record).await
            }
            JupiterSwapInstruction::ExactOutRoute(data) => {
                handled = true;
                let row_metadata = Self::instruction_metadata(&metadata);
                let accounts = Self::account_records(&decoded_instruction.accounts);
                let plan_json = route_plan_json_v1(&data.route_plan)?;
                let steps = normalize_route_plan_v1(&data.route_plan)?;
                let arranged =
                    exact_out_route::ExactOutRoute::arrange_accounts(&decoded_instruction.accounts);
                let source_mint = arranged
                    .as_ref()
                    .map(|accounts| Self::pubkey_to_string(accounts.source_mint));
                let destination_mint = arranged
                    .as_ref()
                    .map(|accounts| Self::pubkey_to_string(accounts.destination_mint));
                let record = Self::base_route_record(
                    row_metadata,
                    "ExactOutRoute",
                    None,
                    None,
                    Some(data.out_amount),
                    Some(data.quoted_in_amount),
                    None,
                    Some(data.slippage_bps),
                    Some(data.platform_fee_bps as u16),
                    None,
                    source_mint,
                    destination_mint,
                    plan_json,
                    1,
                    accounts,
                    steps,
                );
                self.repository.upsert_route_instruction(record).await
            }
            JupiterSwapInstruction::ExactOutRouteV2(data) => {
                handled = true;
                let row_metadata = Self::instruction_metadata(&metadata);
                let accounts = Self::account_records(&decoded_instruction.accounts);
                let plan_json = route_plan_json_v2(&data.route_plan)?;
                let steps = normalize_route_plan_v2(&data.route_plan)?;
                let arranged = exact_out_route_v2::ExactOutRouteV2::arrange_accounts(
                    &decoded_instruction.accounts,
                );
                let source_mint = arranged
                    .as_ref()
                    .map(|accounts| Self::pubkey_to_string(accounts.source_mint));
                let destination_mint = arranged
                    .as_ref()
                    .map(|accounts| Self::pubkey_to_string(accounts.destination_mint));
                let record = Self::base_route_record(
                    row_metadata,
                    "ExactOutRouteV2",
                    None,
                    None,
                    Some(data.out_amount),
                    Some(data.quoted_in_amount),
                    None,
                    Some(data.slippage_bps),
                    Some(data.platform_fee_bps),
                    Some(data.positive_slippage_bps),
                    source_mint,
                    destination_mint,
                    plan_json,
                    2,
                    accounts,
                    steps,
                );
                self.repository.upsert_route_instruction(record).await
            }
            JupiterSwapInstruction::SharedAccountsExactOutRoute(data) => {
                handled = true;
                let row_metadata = Self::instruction_metadata(&metadata);
                let accounts = Self::account_records(&decoded_instruction.accounts);
                let plan_json = route_plan_json_v1(&data.route_plan)?;
                let steps = normalize_route_plan_v1(&data.route_plan)?;
                let arranged =
                    shared_accounts_exact_out_route::SharedAccountsExactOutRoute::arrange_accounts(
                        &decoded_instruction.accounts,
                    );
                let source_mint = arranged
                    .as_ref()
                    .map(|accounts| Self::pubkey_to_string(accounts.source_mint));
                let destination_mint = arranged
                    .as_ref()
                    .map(|accounts| Self::pubkey_to_string(accounts.destination_mint));
                let record = Self::base_route_record(
                    row_metadata,
                    "SharedAccountsExactOutRoute",
                    Some(data.id),
                    None,
                    Some(data.out_amount),
                    Some(data.quoted_in_amount),
                    None,
                    Some(data.slippage_bps),
                    Some(data.platform_fee_bps as u16),
                    None,
                    source_mint,
                    destination_mint,
                    plan_json,
                    1,
                    accounts,
                    steps,
                );
                self.repository.upsert_route_instruction(record).await
            }
            JupiterSwapInstruction::SharedAccountsExactOutRouteV2(data) => {
                handled = true;
                let row_metadata = Self::instruction_metadata(&metadata);
                let accounts = Self::account_records(&decoded_instruction.accounts);
                let plan_json = route_plan_json_v2(&data.route_plan)?;
                let steps = normalize_route_plan_v2(&data.route_plan)?;
                let arranged = shared_accounts_exact_out_route_v2::SharedAccountsExactOutRouteV2::arrange_accounts(
                    &decoded_instruction.accounts,
                );
                let source_mint = arranged
                    .as_ref()
                    .map(|accounts| Self::pubkey_to_string(accounts.source_mint));
                let destination_mint = arranged
                    .as_ref()
                    .map(|accounts| Self::pubkey_to_string(accounts.destination_mint));
                let record = Self::base_route_record(
                    row_metadata,
                    "SharedAccountsExactOutRouteV2",
                    Some(data.id),
                    None,
                    Some(data.out_amount),
                    Some(data.quoted_in_amount),
                    None,
                    Some(data.slippage_bps),
                    Some(data.platform_fee_bps),
                    Some(data.positive_slippage_bps),
                    source_mint,
                    destination_mint,
                    plan_json,
                    2,
                    accounts,
                    steps,
                );
                self.repository.upsert_route_instruction(record).await
            }
            JupiterSwapInstruction::SwapEvent(event) => {
                handled = true;
                let row_metadata = Self::instruction_metadata(&metadata);
                let raw_event = serde_json::to_value(&event)
                    .map_err(|err| CarbonError::Custom(err.to_string()))?;
                let record = SwapEventRecord {
                    metadata: row_metadata,
                    event_type: SwapEventType::SwapEvent,
                    event_index: 0,
                    batch_size: Some(1),
                    input_mint: event.input_mint.to_string(),
                    output_mint: event.output_mint.to_string(),
                    input_amount: event.input_amount,
                    output_amount: event.output_amount,
                    amm: Some(event.amm.to_string()),
                    raw_event,
                };
                self.repository.persist_swap_event(record).await
            }
            JupiterSwapInstruction::SwapsEvent(event) => {
                handled = true;
                if event.swap_events.is_empty() {
                    Ok(None)
                } else {
                    let mut last_slot = None;
                    for (index, swap) in event.swap_events.iter().enumerate() {
                        let row_metadata = Self::instruction_metadata(&metadata);
                        let raw_event = serde_json::to_value(swap)
                            .map_err(|err| CarbonError::Custom(err.to_string()))?;
                        let record = SwapEventRecord {
                            metadata: row_metadata.clone(),
                            event_type: SwapEventType::SwapsEvent,
                            event_index: index as i32,
                            batch_size: Some(event.swap_events.len() as i32),
                            input_mint: swap.input_mint.to_string(),
                            output_mint: swap.output_mint.to_string(),
                            input_amount: swap.input_amount,
                            output_amount: swap.output_amount,
                            amm: None,
                            raw_event,
                        };
                        let slot = self.repository.persist_swap_event(record).await?;
                        if slot.is_some() {
                            last_slot = slot;
                        }
                    }
                    Ok(last_slot)
                }
            }
            _ => Ok(None),
        };

        match slot_result {
            Ok(last_slot) => {
                if handled {
                    metrics
                        .increment_counter("postgres.instructions.upsert.upserted", 1)
                        .await?;
                    metrics
                        .record_histogram(
                            "postgres.instructions.upsert.duration_milliseconds",
                            start.elapsed().as_millis() as f64,
                        )
                        .await?;
                    if let Some(slot) = last_slot {
                        metrics
                            .update_gauge(
                                "postgres.instructions.last_processed_slot",
                                slot as f64,
                            )
                            .await?;
                    }
                }
                Ok(())
            }
            Err(err) => {
                if handled {
                    metrics
                        .increment_counter("postgres.instructions.upsert.failed", 1)
                        .await?;
                }
                Err(err)
            }
        }
    }
}
