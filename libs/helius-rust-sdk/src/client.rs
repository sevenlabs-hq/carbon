use std::{ops::Deref, sync::Arc};

use crate::config::Config;
use crate::error::{HeliusError, Result};
use crate::rpc_client::RpcClient;
use crate::types::Cluster;
use crate::websocket::{EnhancedWebsocket, ENHANCED_WEBSOCKET_URL};

use reqwest::Client;
use solana_client::nonblocking::rpc_client::RpcClient as AsyncSolanaRpcClient;
use solana_client::rpc_client::RpcClient as SolanaRpcClient;

/// The `Helius` struct is the main entry point to interacting with the SDK
///
/// This client is responsible for setting up the network and configuration settins used to interact with the various provided methods.
/// It also provides methods to access RPC client functionalities. The client ensures thread-safe access to the underlying RPC client
pub struct Helius {
    /// The configuration which specifies an `api_key`, `cluster`, and the requisite `endpoints`
    pub config: Arc<Config>,
    /// An HTTP client used for making API requests. The client is reused for all requests made through this instance of `Helius`
    pub client: Client,
    /// A reference-counted RPC client tailored for making requests in a thread-safe manner
    pub rpc_client: Arc<RpcClient>,
    /// An optional asynchronous Solana client for async operations
    pub async_rpc_client: Option<Arc<AsyncSolanaRpcClient>>,
    /// A reference-counted enhanced (geyser) websocket client
    pub ws_client: Option<Arc<EnhancedWebsocket>>,
}

impl Helius {
    /// Creates a new instance of `Helius` configured with a specific API key and a target cluster
    ///
    /// # Arguments
    /// * `api_key` - The API key required for authenticating the requests made
    /// * `cluster` - The Solana cluster (Devnet or MainnetBeta) that defines the given network environment
    ///
    /// # Returns
    /// An instance of `Helius` if successful. A `HeliusError` is returned if an error occurs during configuration or initialization of the HTTP or RPC client
    ///
    /// # Example
    /// ```rust
    /// use helius::client::Helius;
    /// use helius::types::Cluster;
    ///
    /// let helius = Helius::new("your_api_key", Cluster::Devnet).expect("Failed to create a Helius client");
    /// ```
    pub fn new(api_key: &str, cluster: Cluster) -> Result<Self> {
        let config: Arc<Config> = Arc::new(Config::new(api_key, cluster)?);
        let client: Client = Client::builder().build().map_err(HeliusError::ReqwestError)?;
        let rpc_client: Arc<RpcClient> = Arc::new(RpcClient::new(Arc::new(client.clone()), config.clone())?);

        Ok(Helius {
            config,
            client,
            rpc_client,
            async_rpc_client: None,
            ws_client: None,
        })
    }

    /// Creates a new instance of `Helius` with an asynchronous Solana client
    ///
    /// # Arguments
    /// * `api_key` - The API key required for authenticating the requests made
    /// * `cluster` - The Solana cluster (Devnet or MainnetBeta) that defines the given network environment
    ///
    /// # Returns
    /// An instance of `Helius` if successful. A `HeliusError` is returned if an error occurs during configuration or initialization of the HTTP or RPC client
    ///
    /// # Example
    /// ```rust
    /// use helius::Helius;
    /// use helius::types::Cluster;
    ///
    /// let helius = Helius::new_with_async_solana("your_api_key", Cluster::Devnet).expect("Failed to create a Helius client");
    /// ```
    pub fn new_with_async_solana(api_key: &str, cluster: Cluster) -> Result<Self> {
        let config: Arc<Config> = Arc::new(Config::new(api_key, cluster)?);
        let client: Client = Client::builder().build().map_err(HeliusError::ReqwestError)?;
        let url: String = format!("{}/?api-key={}", config.endpoints.rpc, config.api_key);
        let async_solana_client: Arc<AsyncSolanaRpcClient> = Arc::new(AsyncSolanaRpcClient::new(url));

        Ok(Helius {
            config: config.clone(),
            client: client.clone(),
            rpc_client: Arc::new(RpcClient::new(Arc::new(client), config.clone())?),
            async_rpc_client: Some(async_solana_client),
            ws_client: None,
        })
    }

    /// The enhanced websocket is optional, and this method is used to create a new instance of `Helius` with an enhanced websocket client.
    /// Upon calling this method, the websocket will connect hence the asynchronous function definition omission from the default `new` method.
    ///
    /// # Arguments
    /// * `api_key` - The API key required for authenticating requests made
    /// * `cluster` - The Solana cluster (Devnet or MainnetBeta) that defines the given network environment
    /// # Returns
    /// An instance of `Helius` if successful. A `HeliusError` is returned if an error occurs during configuration or initialization of the HTTP, RPC, or WS client
    pub async fn new_with_ws(api_key: &str, cluster: Cluster) -> Result<Self> {
        let config: Arc<Config> = Arc::new(Config::new(api_key, cluster)?);
        let client: Client = Client::builder().build().map_err(HeliusError::ReqwestError)?;
        let rpc_client: Arc<RpcClient> = Arc::new(RpcClient::new(Arc::new(client.clone()), config.clone())?);
        let wss: String = format!("{}{}", ENHANCED_WEBSOCKET_URL, api_key);
        let ws_client: Arc<EnhancedWebsocket> = Arc::new(EnhancedWebsocket::new(&wss).await?);

        Ok(Helius {
            config,
            client,
            rpc_client,
            async_rpc_client: None,
            ws_client: Some(ws_client),
        })
    }

    /// Provides a thread-safe way to access RPC functionalities
    ///
    /// # Returns
    /// A cloned `Arc<RpcClient>` that can be safely shared across threads
    pub fn rpc(&self) -> Arc<RpcClient> {
        self.rpc_client.clone()
    }

    /// Provides a thread-safe way to access asynchronous Solana client functionalities
    ///
    /// # Returns
    /// A `Result` containing a `HeliusAsyncSolanaClient` if an `async_rpc_client` exists, otherwise a `HeliusError`
    pub fn async_connection(&self) -> Result<HeliusAsyncSolanaClient> {
        match &self.async_rpc_client {
            Some(client) => Ok(HeliusAsyncSolanaClient::new(client.clone())),
            None => Err(HeliusError::ClientNotInitialized {
                text: "An asynchronous Solana RPC client must be initialized before trying to access async_connection"
                    .to_string(),
            }),
        }
    }

    /// Provides a thread-safe way to access synchronous Solana client functionalities
    ///
    /// # Returns
    /// A cloned `Arc<SolanaRpcClient>` that can be safely shared across threads
    pub fn connection(&self) -> Arc<SolanaRpcClient> {
        self.rpc_client.solana_client.clone()
    }

    pub fn ws(&self) -> Option<Arc<EnhancedWebsocket>> {
        self.ws_client.clone()
    }
}

/// A wrapper around the asynchronous Solana RPC client that provides thread-safe access
pub struct HeliusAsyncSolanaClient {
    client: Arc<AsyncSolanaRpcClient>,
}

impl HeliusAsyncSolanaClient {
    /// Creates a new instance of `HeliusAsyncSolanaClient`
    ///
    /// # Arguments
    /// * `client` - The asynchronous Solana RPC client to wrap
    ///
    /// # Returns
    /// An instance of `HeliusAsyncSolanaClient`
    pub fn new(client: Arc<AsyncSolanaRpcClient>) -> Self {
        Self { client }
    }
}

impl Deref for HeliusAsyncSolanaClient {
    type Target = AsyncSolanaRpcClient;

    /// Dereferences the wrapper to provide access to the underlying asynchronous Solana RPC client
    fn deref(&self) -> &Self::Target {
        &self.client
    }
}
