use {
    carbon_core::{account::AccountProcessorInputType, error::CarbonResult, processor::Processor},
    carbon_log_metrics::LogMetrics,
    carbon_pumpfun_decoder::{
        accounts::PumpfunAccount, PumpfunDecoder, PROGRAM_ID as PUMPFUN_PROGRAM_ID,
    },
    carbon_rpc_program_subscribe_datasource::{Filters, RpcProgramSubscribe},
    solana_account_decoder::UiAccountEncoding,
    solana_client::rpc_config::{RpcAccountInfoConfig, RpcProgramAccountsConfig},
    std::{env, sync::Arc},
};

pub struct PumpfunAccountProcessor;

impl Processor<AccountProcessorInputType<'_, PumpfunAccount>> for PumpfunAccountProcessor {
    async fn process(
        &mut self,
        input: &AccountProcessorInputType<'_, PumpfunAccount>,
    ) -> CarbonResult<()> {
        match &input.decoded_account.data {
            PumpfunAccount::BondingCurve(bonding_curve) => {
                log::info!(
                    "BondingCurve account updated: pubkey={}, virtual_token_reserves={}, virtual_sol_reserves={}, real_token_reserves={}, real_sol_reserves={}, token_total_supply={}, complete={}, slot={}",
                    input.metadata.pubkey,
                    bonding_curve.virtual_token_reserves,
                    bonding_curve.virtual_sol_reserves,
                    bonding_curve.real_token_reserves,
                    bonding_curve.real_sol_reserves,
                    bonding_curve.token_total_supply,
                    bonding_curve.complete,
                    input.metadata.slot
                );
            }
            PumpfunAccount::Global(global) => {
                log::info!(
                    "Global account updated: pubkey={}, initialized={}, authority={}, fee_recipient={}, initial_virtual_token_reserves={}, initial_virtual_sol_reserves={}, initial_real_token_reserves={}, token_total_supply={}, fee_basis_points={}, slot={}",
                    input.metadata.pubkey,
                    global.initialized,
                    global.authority,
                    global.fee_recipient,
                    global.initial_virtual_token_reserves,
                    global.initial_virtual_sol_reserves,
                    global.initial_real_token_reserves,
                    global.token_total_supply,
                    global.fee_basis_points,
                    input.metadata.slot
                );
            }
            PumpfunAccount::FeeConfig(fee_config) => {
                log::info!(
                    "FeeConfig account updated: pubkey={}, admin={}, flat_fees={:?}, fee_tiers={:?}, slot={}",
                    input.metadata.pubkey,
                    fee_config.admin,
                    fee_config.flat_fees,
                    fee_config.fee_tiers,
                    input.metadata.slot
                );
            }
            PumpfunAccount::GlobalVolumeAccumulator(volume_acc) => {
                log::info!(
                    "GlobalVolumeAccumulator account updated: pubkey={}, mint={}, start_time={}, end_time={}, total_sol_volume={}, slot={}",
                    input.metadata.pubkey,
                    volume_acc.mint,
                    volume_acc.start_time,
                    volume_acc.end_time,
                    volume_acc.sol_volumes.iter().sum::<u64>(),
                    input.metadata.slot
                );
            }
            PumpfunAccount::UserVolumeAccumulator(user_volume) => {
                log::info!(
                    "UserVolumeAccumulator account updated: pubkey={}, user={}, current_sol_volume={}, total_unclaimed_tokens={}, total_claimed_tokens={}, slot={}",
                    input.metadata.pubkey,
                    user_volume.user,
                    user_volume.current_sol_volume,
                    user_volume.total_unclaimed_tokens,
                    user_volume.total_claimed_tokens,
                    input.metadata.slot
                );
            }
            PumpfunAccount::SharingConfig(sharing_config) => {
                log::info!(
                    "SharingConfig account updated: pubkey={}, mint={}, admin={}, status={:?}, shareholders_count={}, slot={}",
                    input.metadata.pubkey,
                    sharing_config.mint,
                    sharing_config.admin,
                    sharing_config.status,
                    sharing_config.shareholders.len(),
                    input.metadata.slot
                );
            }
        }

        Ok(())
    }
}

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    dotenv::dotenv().ok();
    env_logger::init();

    carbon_core::pipeline::Pipeline::builder()
        .datasource(RpcProgramSubscribe::new(
            // Websocket RPC url, usually starts with "wss://"
            env::var("RPC_WS_URL").unwrap_or_default(),
            Filters::new(
                PUMPFUN_PROGRAM_ID,
                Some(RpcProgramAccountsConfig {
                    filters: None,
                    account_config: RpcAccountInfoConfig {
                        encoding: Some(UiAccountEncoding::Base64),
                        ..Default::default()
                    },
                    ..Default::default()
                }),
            ),
        ))
        .account(PumpfunDecoder, PumpfunAccountProcessor)
        .metrics(Arc::new(LogMetrics::new()))
        .build()?
        .run()
        .await?;

    Ok(())
}
