use async_trait::async_trait;
use carbon_core::{
    datasource::{Datasource, Update, UpdateType},
    error::CarbonResult,
};
use solana_sdk::pubkey::Pubkey;
use std::{collections::HashMap, time::Duration};
use tokio::sync::mpsc::UnboundedSender;
use yellowstone_grpc_proto::geyser::{
    CommitmentLevel, SubscribeRequest, SubscribeRequestFilterAccounts,
    SubscribeRequestFilterTransactions,
};

pub struct YellowstoneGrpcGeyserClient {
    pub accounts: Vec<Pubkey>,
    pub endpoint: String,
    pub x_token: Option<String>,
    pub commitment: Option<CommitmentLevel>,
}

impl YellowstoneGrpcGeyserClient {
    pub fn new(
        accounts: Vec<Pubkey>,
        endpoint: String,
        x_token: Option<String>,
        commitment: Option<CommitmentLevel>,
    ) -> Self {
        YellowstoneGrpcGeyserClient {
            accounts,
            endpoint,
            x_token,
            commitment,
        }
    }
}

#[async_trait]
impl Datasource for YellowstoneGrpcGeyserClient {
    async fn consume(
        &self,
        sender: &UnboundedSender<Update>,
    ) -> CarbonResult<tokio::task::AbortHandle> {
        let accounts = &self.accounts;
        let endpoint = &self.endpoint;
        let x_token = &self.x_token;
        let commitment = self.commitment;

        let abort_handle = tokio::spawn(async move {
            { /*
                 let client = GeyserGrpcClient::build_from_shared(endpoint)?
                      .x_token(x_token)?
                      .connect_timeout(Duration::from_secs(10))
                      .timeout(Duration::from_secs(10))
                      .connect()
                      .await
                      .map_err(|err| {
                          log::error!("Error connecting to Geyser server: {}", err)
                      })?;
                       */
            }
        })
        .abort_handle();

        Ok(abort_handle)
    }

    fn update_types(&self) -> Vec<UpdateType> {
        vec![UpdateType::AccountUpdate]
    }
}

pub fn get_subscribe_request(
    accounts: Vec<Pubkey>,
    commitment: Option<CommitmentLevel>,
    resub: Option<usize>,
) -> Result<(SubscribeRequest, usize), Box<dyn std::error::Error>> {
    let account_filters: HashMap<String, SubscribeRequestFilterAccounts> = accounts
        .iter()
        .map(|account| {
            (
                account.to_string(),
                SubscribeRequestFilterAccounts {
                    owner: vec![account.to_string()],
                    account: vec![account.to_string()],
                    filters: vec![],
                },
            )
        })
        .collect();

    let mut transactions: HashMap<String, SubscribeRequestFilterTransactions> = HashMap::new();

    for account in &accounts {
        transactions.insert(
            account.to_string(),
            SubscribeRequestFilterTransactions {
                vote: None,
                failed: None,
                signature: None,
                account_include: vec![],
                account_exclude: vec![],
                account_required: vec![account.to_string()],
            },
        );
    }

    Ok((
        SubscribeRequest {
            slots: HashMap::new(),
            accounts: account_filters,
            transactions,
            transactions_status: HashMap::new(),
            entry: HashMap::new(),
            blocks: HashMap::new(),
            blocks_meta: HashMap::new(),
            commitment: commitment.map(|x| x as i32),
            accounts_data_slice: vec![],
            ping: None,
        },
        resub.unwrap_or(0),
    ))
}
