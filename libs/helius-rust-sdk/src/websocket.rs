use crate::error::{HeliusError, Result};
use crate::types::{RpcTransactionsConfig, TransactionNotification};
use futures_util::{
    future::{ready, BoxFuture, FutureExt},
    sink::SinkExt,
    stream::{BoxStream, StreamExt},
};
use serde::de::DeserializeOwned;
use serde_json::{json, Map, Value};
use solana_account_decoder::UiAccount;
use solana_rpc_client_api::config::RpcAccountInfoConfig;
use solana_rpc_client_api::{error_object::RpcErrorObject, response::Response as RpcResponse};
use solana_sdk::pubkey::Pubkey;
use std::collections::BTreeMap;
use std::fmt::Debug;
use tokio::{
    net::TcpStream,
    sync::{mpsc, oneshot, RwLock},
    task::JoinHandle,
    time::{sleep, Duration},
};
use tokio_stream::wrappers::UnboundedReceiverStream;
use tokio_tungstenite::{
    connect_async,
    tungstenite::{
        protocol::frame::{coding::CloseCode, CloseFrame},
        Message,
    },
    MaybeTlsStream, WebSocketStream,
};
use url::Url;

pub const ENHANCED_WEBSOCKET_URL: &str = "wss://atlas-mainnet.helius-rpc.com?api-key=";
const DEFAULT_PING_DURATION_SECONDS: u64 = 10;

// pub type Result<T = ()> = Result<T, HeliusError>;

type UnsubscribeFn = Box<dyn FnOnce() -> BoxFuture<'static, ()> + Send>;
type SubscribeResponseMsg = Result<(mpsc::UnboundedReceiver<Value>, UnsubscribeFn)>;
type SubscribeRequestMsg = (String, Value, oneshot::Sender<SubscribeResponseMsg>);
type SubscribeResult<'a, T> = Result<(BoxStream<'a, T>, UnsubscribeFn)>;
type RequestMsg = (String, Value, oneshot::Sender<Result<Value>>);

/// A client for subscribing to transaction or account updates from a Helius (geyser) enhanced websocket server.
///
/// Forked from Solana's [`PubsubClient`].
pub struct EnhancedWebsocket {
    subscribe_sender: mpsc::UnboundedSender<SubscribeRequestMsg>,
    shutdown_sender: oneshot::Sender<()>,
    node_version: RwLock<Option<semver::Version>>,
    ws: JoinHandle<Result<()>>,
}

impl EnhancedWebsocket {
    /// Expects enhanced websocket endpoint: wss://atlas-mainnet.helius-rpc.com?api-key=<API_KEY>
    pub async fn new(url: &str) -> Result<Self> {
        let url = Url::parse(url)?;
        let (ws, _response) = connect_async(url).await.map_err(HeliusError::Tungstenite)?;

        let (subscribe_sender, subscribe_receiver) = mpsc::unbounded_channel();
        let (_request_sender, request_receiver) = mpsc::unbounded_channel();
        let (shutdown_sender, shutdown_receiver) = oneshot::channel();

        Ok(Self {
            subscribe_sender,
            shutdown_sender,
            node_version: RwLock::new(None),
            ws: tokio::spawn(EnhancedWebsocket::run_ws(
                ws,
                subscribe_receiver,
                request_receiver,
                shutdown_receiver,
                DEFAULT_PING_DURATION_SECONDS,
            )),
        })
    }

    pub async fn shutdown(self) -> Result<()> {
        let _ = self.shutdown_sender.send(());
        self.ws.await.unwrap() // WS future should not be cancelled or panicked
    }

    pub async fn set_node_version(&self, version: semver::Version) -> Result<()> {
        let mut w_node_version = self.node_version.write().await;
        *w_node_version = Some(version);
        Ok(())
    }

    async fn subscribe<'a, T: DeserializeOwned + Send + Debug + 'a>(
        &self,
        operation: &str,
        params: Value,
    ) -> SubscribeResult<'a, T> {
        let (response_sender, response_receiver) = oneshot::channel();
        self.subscribe_sender
            .send((operation.to_string(), params, response_sender))
            .map_err(|err| HeliusError::WebsocketClosed(err.to_string()))?;

        let (notifications, unsubscribe) = response_receiver
            .await
            .map_err(|err| HeliusError::WebsocketClosed(err.to_string()))??;
        Ok((
            UnboundedReceiverStream::new(notifications)
                .filter_map(|value| match serde_json::from_value::<T>(value.clone()) {
                    Err(e) => {
                        eprintln!(
                            "Failed to parse websocket notification: {:#?} for value: {:#?}",
                            e, value
                        );
                        ready(None)
                    }
                    Ok(res) => ready(Some(res)),
                })
                .boxed(),
            unsubscribe,
        ))
    }

    /// Stream transactions with numerous configurations and filters to choose from.
    ///
    /// # Example
    /// ```rust
    /// use helius::Helius;
    /// use helius::error::Result;
    /// use helius::types::{Cluster, RpcTransactionsConfig, TransactionSubscribeFilter, TransactionSubscribeOptions};
    /// use solana_sdk::pubkey;
    /// use tokio_stream::StreamExt;
    ///
    /// #[tokio::main]
    /// async fn main() -> Result<()> {
    ///   let helius = Helius::new("your_api_key", Cluster::MainnetBeta).expect("Failed to create a Helius client");
    ///   // you may monitor transactions for any pubkey, this is just an example.
    ///   let key = pubkey!("BtsmiEEvnSuUnKxqXj2PZRYpPJAc7C34mGz8gtJ1DAaH");
    ///   let config = RpcTransactionsConfig {
    ///     filter: TransactionSubscribeFilter::standard(&key),
    ///     options: TransactionSubscribeOptions::default(),
    ///   };
    ///   if let Some(ws) = helius.ws() {
    ///     let (mut stream, _unsub) = ws.transaction_subscribe(config).await?;
    ///     while let Some(event) = stream.next().await {
    ///       println!("{:#?}", event);
    ///     }
    ///   }
    ///   Ok(())
    /// }
    /// ```
    pub async fn transaction_subscribe(
        &self,
        config: RpcTransactionsConfig,
    ) -> SubscribeResult<'_, TransactionNotification> {
        let params = json!([config.filter, config.options]);
        self.subscribe("transaction", params).await
    }

    /// Stream accounts with numerous configurations and filters to choose from.
    ///
    /// # Example
    /// ```rust
    /// use helius::Helius;
    /// use helius::error::Result;
    /// use helius::types::{Cluster, RpcTransactionsConfig, TransactionSubscribeFilter, TransactionSubscribeOptions};
    /// use solana_sdk::pubkey;
    /// use tokio_stream::StreamExt;
    ///
    /// #[tokio::main]
    /// async fn main() -> Result<()> {
    ///   let helius = Helius::new("your_api_key", Cluster::MainnetBeta).expect("Failed to create a Helius client");
    ///   // you may monitor updates for any account pubkey, this is just an example.
    ///   let key = pubkey!("BtsmiEEvnSuUnKxqXj2PZRYpPJAc7C34mGz8gtJ1DAaH");
    ///   if let Some(ws) = helius.ws() {
    ///     let (mut stream, _unsub) = ws.account_subscribe(&key, None).await?;
    ///     while let Some(event) = stream.next().await {
    ///       println!("{:#?}", event);
    ///     }
    ///   }
    ///   Ok(())
    /// }
    /// ```
    pub async fn account_subscribe(
        &self,
        pubkey: &Pubkey,
        config: Option<RpcAccountInfoConfig>,
    ) -> SubscribeResult<'_, RpcResponse<UiAccount>> {
        let params = json!([pubkey.to_string(), config]);
        self.subscribe("account", params).await
    }

    async fn run_ws(
        mut ws: WebSocketStream<MaybeTlsStream<TcpStream>>,
        mut subscribe_receiver: mpsc::UnboundedReceiver<SubscribeRequestMsg>,
        mut request_receiver: mpsc::UnboundedReceiver<RequestMsg>,
        mut shutdown_receiver: oneshot::Receiver<()>,
        ping_duration_seconds: u64,
    ) -> Result<()> {
        let mut request_id: u64 = 0;

        let mut requests_subscribe = BTreeMap::new();
        let mut requests_unsubscribe = BTreeMap::<u64, oneshot::Sender<()>>::new();
        let mut other_requests = BTreeMap::new();
        let mut subscriptions = BTreeMap::new();
        let (unsubscribe_sender, mut unsubscribe_receiver) = mpsc::unbounded_channel();

        loop {
            tokio::select! {
              // Send close on shutdown signal
              _ = &mut shutdown_receiver => {
                let frame = CloseFrame { code: CloseCode::Normal, reason: "".into() };
                ws.send(Message::Close(Some(frame))).await?;
                ws.flush().await?;
                break;
              },
              // Send `Message::Ping` each 10s if no any other communication
              () = sleep(Duration::from_secs(ping_duration_seconds)) => {
                ws.send(Message::Ping(Vec::new())).await?;
              },
              // Read message for subscribe
              Some((operation, params, response_sender)) = subscribe_receiver.recv() => {
                request_id += 1;
                let method = format!("{operation}Subscribe");
                let body = json!({"jsonrpc":"2.0","id":request_id,"method":method,"params":params});
                println!("subscription: {:#}", body);
                ws.send(Message::Text(body.to_string())).await?;
                requests_subscribe.insert(request_id, (operation, response_sender));
              },
              // Read message for unsubscribe
              Some((operation, sid, response_sender)) = unsubscribe_receiver.recv() => {
                subscriptions.remove(&sid);
                request_id += 1;
                let method = format!("{operation}Unsubscribe");
                let text = json!({"jsonrpc":"2.0","id":request_id,"method":method,"params":[sid]}).to_string();
                ws.send(Message::Text(text)).await?;
                requests_unsubscribe.insert(request_id, response_sender);
              },
              // Read message for other requests
              Some((method, params, response_sender)) = request_receiver.recv() => {
                request_id += 1;
                let text = json!({"jsonrpc":"2.0","id":request_id,"method":method,"params":params}).to_string();
                ws.send(Message::Text(text)).await?;
                other_requests.insert(request_id, response_sender);
              }
              // Read incoming WebSocket message
              next_msg = ws.next() => {
                let msg = match next_msg {
                  Some(msg) => msg?,
                  None => break,
                };

                // Get text from the message
                let text = match msg {
                  Message::Text(text) => text,
                  Message::Binary(_data) => continue, // Ignore
                  Message::Ping(data) => {
                      ws.send(Message::Pong(data)).await?;
                      continue
                  },
                  Message::Pong(_data) => continue,
                  Message::Close(_frame) => break,
                  Message::Frame(_frame) => continue,
                };

                let mut json: Map<String, Value> = serde_json::from_str(&text)?;

                // Subscribe/Unsubscribe response, example:
                // `{"jsonrpc":"2.0","result":5308752,"id":1}`
                if let Some(id) = json.get("id") {
                  let id = id.as_u64().ok_or_else(|| {
                      HeliusError::EnhancedWebsocket { reason: "invalid `id` field".into(), message: text.clone() }
                  })?;

                  let err = json.get("error").map(|error_object| {
                      match serde_json::from_value::<RpcErrorObject>(error_object.clone()) {
                          Ok(rpc_error_object) => {
                              format!("{} ({})",  rpc_error_object.message, rpc_error_object.code)
                          }
                          Err(err) => format!(
                              "Failed to deserialize RPC error response: {} [{}]",
                              serde_json::to_string(error_object).unwrap(),
                              err
                          )
                      }
                  });

                  if let Some(response_sender) = other_requests.remove(&id) {
                    match err {
                      Some(reason) => {
                        let _ = response_sender.send(Err(HeliusError::EnhancedWebsocket { reason, message: text.clone()}));
                      },
                      None => {
                        let json_result = json.get("result").ok_or_else(|| {
                            HeliusError::EnhancedWebsocket { reason: "missing `result` field".into(), message: text.clone() }
                        })?;
                        if response_sender.send(Ok(json_result.clone())).is_err() {
                            break;
                        }
                      }
                    }
                  } else if let Some(response_sender) = requests_unsubscribe.remove(&id) {
                    let _ = response_sender.send(()); // do not care if receiver is closed
                  } else if let Some((operation, response_sender)) = requests_subscribe.remove(&id) {
                    match err {
                      Some(reason) => {
                        let _ = response_sender.send(Err(HeliusError::EnhancedWebsocket { reason, message: text.clone()}));
                      },
                      None => {
                        // Subscribe Id
                        let sid = json.get("result").and_then(Value::as_u64).ok_or_else(|| {
                          HeliusError::EnhancedWebsocket { reason: "invalid `result` field".into(), message: text.clone() }
                        })?;

                        // Create notifications channel and unsubscribe function
                        let (notifications_sender, notifications_receiver) = mpsc::unbounded_channel();
                        let unsubscribe_sender = unsubscribe_sender.clone();
                        let unsubscribe = Box::new(move || async move {
                          let (response_sender, response_receiver) = oneshot::channel();
                          // do nothing if ws already closed
                          if unsubscribe_sender.send((operation, sid, response_sender)).is_ok() {
                            let _ = response_receiver.await; // channel can be closed only if ws is closed
                          }
                        }.boxed());

                        if response_sender.send(Ok((notifications_receiver, unsubscribe))).is_err() {
                            break;
                        }
                        subscriptions.insert(sid, notifications_sender);
                      }
                    }
                  } else {
                      eprintln!("Unknown request id: {}", id);
                      break;
                  }
                  continue;
                }

                // Notification, example:
                // `{"jsonrpc":"2.0","method":"logsNotification","params":{"result":{...},"subscription":3114862}}`
                if let Some(Value::Object(params)) = json.get_mut("params") {
                  if let Some(sid) = params.get("subscription").and_then(Value::as_u64) {
                    let mut unsubscribe_required = false;

                    if let Some(notifications_sender) = subscriptions.get(&sid) {
                      if let Some(result) = params.remove("result") {
                        if notifications_sender.send(result).is_err() {
                          unsubscribe_required = true;
                        }
                      }
                    } else {
                      unsubscribe_required = true;
                    }

                    if unsubscribe_required {
                      if let Some(Value::String(method)) = json.remove("method") {
                        if let Some(operation) = method.strip_suffix("Notification") {
                          let (response_sender, _response_receiver) = oneshot::channel();
                          let _ = unsubscribe_sender.send((operation.to_string(), sid, response_sender));
                        }
                      }
                    }
                  }
                }
              }
            }
        }

        Ok(())
    }
}
