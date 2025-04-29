use {
    async_trait::async_trait,
    carbon_core::{
        error::CarbonResult,
        instruction::{
            DecodedInstruction, InstructionMetadata, InstructionProcessorInputType,
            NestedInstructions,
        },
        metrics::MetricsCollection,
        processor::Processor,
    },
    carbon_log_metrics::LogMetrics,
    carbon_rpc_block_subscribe_datasource::{Filters, RpcBlockSubscribe},
    carbon_token_program_decoder::{
        instructions::TokenProgramInstruction, storage::migrations::FirstMigration,
        TokenProgramDecoder,
    },
    solana_client::rpc_config::{RpcBlockSubscribeConfig, RpcBlockSubscribeFilter},
    std::{env, sync::Arc},
};

#[tokio::main]
pub async fn main() -> CarbonResult<()> {
    env_logger::init();
    dotenv::dotenv().ok();

    let db_uri = std::env::var("DATABASE_URL").expect("DATABASE_URL must be set");
    let pg_client = carbon_postgres_client::PgClient::new(&db_uri, 1, 10)
        .await
        .expect("Failed to create Postgres client");


    let filters = Filters::new(
        RpcBlockSubscribeFilter::MentionsAccountOrProgram(spl_token::id().to_string()),
        Some(RpcBlockSubscribeConfig {
            max_supported_transaction_version: Some(0),
            ..RpcBlockSubscribeConfig::default()
        }),
    );

    let rpc_ws_url =
        env::var("RPC_WS_URL").unwrap_or("wss://api.mainnet-beta.solana.com/".to_string());

    log::info!("Starting with RPC: {}", rpc_ws_url);
    let block_subscribe = RpcBlockSubscribe::new(rpc_ws_url, filters);

    carbon_core::pipeline::Pipeline::builder()
        .datasource(block_subscribe)
        .metrics(Arc::new(LogMetrics::new()))
        .metrics_flush_interval(3)
        .instruction(TokenProgramDecoder, TokenProgramInstructionProcessor)
        .shutdown_strategy(carbon_core::pipeline::ShutdownStrategy::Immediate)
        .build()?
        .run()
        .await?;

    Ok(())
}

pub struct TokenProgramInstructionProcessor;

#[async_trait]
impl Processor for TokenProgramInstructionProcessor {
    type InputType = InstructionProcessorInputType<TokenProgramInstruction>;

    async fn process(
        &mut self,
        data: Self::InputType,
        _metrics: Arc<MetricsCollection>,
    ) -> CarbonResult<()> {
        let token_program_instruction: TokenProgramInstruction = data.1.data;

        match token_program_instruction {
            TokenProgramInstruction::AmountToUiAmount(amount_to_ui_amount) => {
                log::info!("Amount to UI Amount: {:#?}", amount_to_ui_amount);
            }
            TokenProgramInstruction::ApproveChecked(approve_checked) => {
                log::info!("Approve Checked: {:#?}", approve_checked);
            }
            TokenProgramInstruction::Approve(approve) => {
                log::info!("Approve: {:#?}", approve);
            }
            TokenProgramInstruction::BurnChecked(burn_checked) => {
                log::info!("Burn Checked: {:#?}", burn_checked);
            }
            TokenProgramInstruction::Burn(burn) => {
                log::info!("Burn: {:#?}", burn);
            }
            TokenProgramInstruction::CloseAccount(close_account) => {
                log::info!("Close Account: {:#?}", close_account);
            }
            TokenProgramInstruction::FreezeAccount(freeze_account) => {
                log::info!("Freeze Account: {:#?}", freeze_account);
            }
            TokenProgramInstruction::GetAccountDataSize(get_account_data_size) => {
                log::info!("Get Account Data Size: {:#?}", get_account_data_size);
            }
            TokenProgramInstruction::InitializeAccount(initialize_account) => {
                log::info!("Initialize Account: {:#?}", initialize_account);
            }
            TokenProgramInstruction::InitializeAccount2(initialize_account2) => {
                log::info!("Initialize Account 2: {:#?}", initialize_account2);
            }
            TokenProgramInstruction::InitializeAccount3(initialize_account3) => {
                log::info!("Initialize Account 3: {:#?}", initialize_account3);
            }
            TokenProgramInstruction::InitializeImmutableOwner(initialize_immutable_owner) => {
                log::info!(
                    "Initialize Immutable Owner: {:#?}",
                    initialize_immutable_owner
                );
            }
            TokenProgramInstruction::InitializeMint(initialize_mint) => {
                log::info!("Initialize Mint: {:#?}", initialize_mint);
            }
            TokenProgramInstruction::InitializeMint2(initialize_mint2) => {
                log::info!("Initialize Mint 2: {:#?}", initialize_mint2);
            }
            TokenProgramInstruction::InitializeMultisig(initialize_multisig) => {
                log::info!("Initialize Multisig: {:#?}", initialize_multisig);
            }
            TokenProgramInstruction::InitializeMultisig2(initialize_multisig2) => {
                log::info!("Initialize Multisig 2: {:#?}", initialize_multisig2);
            }
            TokenProgramInstruction::MintToChecked(mint_to_checked) => {
                log::info!("Mint To Checked: {:#?}", mint_to_checked);
            }
            TokenProgramInstruction::MintTo(mint_to) => {
                log::info!("Mint To: {:#?}", mint_to);
            }
            TokenProgramInstruction::Revoke(revoke) => {
                log::info!("Revoke: {:#?}", revoke);
            }
            TokenProgramInstruction::SetAuthority(set_authority) => {
                log::info!("Set Authority: {:#?}", set_authority);
            }
            TokenProgramInstruction::SyncNative(sync_native) => {
                log::info!("Sync Native: {:#?}", sync_native);
            }
            TokenProgramInstruction::ThawAccount(thaw_account) => {
                log::info!("Thaw Account: {:#?}", thaw_account);
            }
            TokenProgramInstruction::Transfer(transfer) => {
                log::info!("Transfer: {:#?}", transfer);
            }
            TokenProgramInstruction::TransferChecked(transfer_checked) => {
                log::info!("Transfer Checked: {:#?}", transfer_checked);
            }
            TokenProgramInstruction::UiAmountToAmount(ui_amount_to_amount) => {
                log::info!("UI Amount to Amount: {:#?}", ui_amount_to_amount);
            }
        };

        Ok(())
    }
}
