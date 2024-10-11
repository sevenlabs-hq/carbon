use std::sync::Arc;

use crate::client::Helius;
use crate::config::Config;
use crate::error::Result;
use crate::rpc_client::RpcClient;
use crate::types::Cluster;

use reqwest::Client;

/// A factory for creating instances of `Helius`
///
/// This factory allows for a centralized configuration and creation of `Helius` clients so work can be done across multiple clusters at the same time.
/// Using a factory simplifies client code and enhances maintainability by ensuring that all `Helius` clients are configured consistently.
pub struct HeliusFactory {
    api_key: String,
    client: Option<Client>,
}

impl HeliusFactory {
    /// Creates a new `HeliusFactor` capable of producing `Helius` clients
    ///
    /// # Arguments
    /// * `api_key` - The API key used for authenticating requests made by the `Helius` clients
    ///
    /// # Example
    /// ```rust
    /// use helius::HeliusFactory;
    /// let factory = HeliusFactory::new("your_api_key_here");
    /// ```
    pub fn new(api_key: &str) -> Self {
        HeliusFactory {
            api_key: api_key.to_string(),
            client: None,
        }
    }

    /// Use your own reqwest client
    ///
    /// # Arguments
    /// * `client` - a [`reqwest::Client`]
    ///
    /// # Example
    /// ```rust
    /// use helius::HeliusFactory;
    /// use helius::types::Cluster;
    /// let mut factory = HeliusFactory::new("your_api_key_here");
    /// factory.with_client(reqwest::Client::new()).create(Cluster::Devnet).unwrap();
    /// ```
    pub fn with_client(&mut self, client: Client) -> &mut Self {
        self.client = Some(client);
        self
    }

    /// Provides a way to create multiple `Helius` clients in a thread-safe manner
    ///
    /// # Arguments
    /// * `cluster` - The Solana cluster for which the `Helius` client will be configured
    ///
    /// # Returns
    /// A `Result` wrapping a `Helius` client if successful
    ///
    /// # Errors
    /// This method returns a `HeliusError` if the configuration or client instantiation fails, typically due to issues with network settings or the API key provided
    ///
    /// # Example
    /// ```rust
    /// use helius::types::*;
    /// use helius::client::Helius;
    /// use helius::factory::HeliusFactory;
    ///
    /// #[tokio::main]
    /// async fn main() {
    ///     let factory = HeliusFactory::new("your_api_key_here");
    ///
    ///     let helius_dev = factory.create(Cluster::Devnet).unwrap();
    ///     // Perform devnet request: helius_dev.request_name().await;
    ///     
    ///     let helius_main = factory.create(Cluster::MainnetBeta).unwrap();
    ///     // Perform mainnet request: helius_main.request_name().await;
    /// }
    /// ```
    pub fn create(&self, cluster: Cluster) -> Result<Helius> {
        let config: Arc<Config> = Arc::new(Config::new(&self.api_key, cluster)?);
        let client: Client = self.client.clone().unwrap_or_default();
        let rpc_client: Arc<RpcClient> = Arc::new(RpcClient::new(Arc::new(client.clone()), config.clone())?);

        Ok(Helius {
            config,
            client,
            rpc_client,
            async_rpc_client: None,
            ws_client: None,
        })
    }
}
