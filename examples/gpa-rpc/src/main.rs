mod variants;

use {
    carbon_core::{
        account::AccountProcessorInputType, error::CarbonResult, pipeline::Pipeline,
        processor::Processor,
    },
    carbon_log_metrics::LogMetrics,
    carbon_marginfi_v2_decoder::{
        accounts::MarginfiV2Account, MarginfiV2Decoder, PROGRAM_ID as MARGINFI_PROGRAM_ID,
    },
    std::sync::Arc,
};

#[tokio::main]
pub async fn main() -> CarbonResult<()> {
    dotenv::dotenv().ok();
    env_logger::init();

    let datasource = variants::rpc(MARGINFI_PROGRAM_ID);
    // alt: let datasource = variants::helius_gpa_v2(MARGINFI_PROGRAM_ID);

    Pipeline::builder()
        .datasource(datasource)
        .metrics(Arc::new(LogMetrics::new()))
        .account(MarginfiV2Decoder, MarginfiAccountProcessor)
        .shutdown_strategy(carbon_core::pipeline::ShutdownStrategy::ProcessPending)
        .build()?
        .run()
        .await?;

    log::info!("snapshot complete");
    Ok(())
}

pub struct MarginfiAccountProcessor;

impl Processor<AccountProcessorInputType<'_, MarginfiV2Account>> for MarginfiAccountProcessor {
    async fn process(
        &mut self,
        input: &AccountProcessorInputType<'_, MarginfiV2Account>,
    ) -> CarbonResult<()> {
        let pubkey = input.metadata.pubkey;

        match &input.decoded_account.data {
            MarginfiV2Account::Bank(bank) => {
                log::info!(
                    "bank pubkey={pubkey} mint={} group={}",
                    bank.mint,
                    bank.group
                );
            }
            MarginfiV2Account::MarginfiGroup(group) => {
                log::info!("group pubkey={pubkey} admin={}", group.admin);
            }
            MarginfiV2Account::MarginfiAccount(account) => {
                log::debug!(
                    "user_account pubkey={pubkey} group={} authority={}",
                    account.group,
                    account.authority
                );
            }
            other => {
                log::debug!("other pubkey={pubkey} kind={:?}", account_kind(other));
            }
        }

        Ok(())
    }
}

fn account_kind(account: &MarginfiV2Account) -> &'static str {
    match account {
        MarginfiV2Account::Bank(_) => "Bank",
        MarginfiV2Account::BankMetadata(_) => "BankMetadata",
        MarginfiV2Account::FeeState(_) => "FeeState",
        MarginfiV2Account::MarginfiAccount(_) => "MarginfiAccount",
        MarginfiV2Account::MarginfiGroup(_) => "MarginfiGroup",
        _ => "other",
    }
}
