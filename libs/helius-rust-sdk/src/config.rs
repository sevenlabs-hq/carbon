use crate::error::{HeliusError, Result};
use crate::types::{Cluster, HeliusEndpoints};

/// Configuration settings for the Helius client
///
/// `Config` contains all the necessary parameters needed to configure and authenticate the `Helius` client to interact with a specific Solana cluster
#[derive(Clone)]
pub struct Config {
    /// The API key used for authenticating requests
    pub api_key: String,
    /// The target Solana cluster the client will interact with
    pub cluster: Cluster,
    /// The endpoints associated with the specified `cluster`. Note these endpoints are automatically determined based on the cluster to ensure requests
    /// are made to the correct cluster
    pub endpoints: HeliusEndpoints,
}

impl Config {
    /// Creates a new configuration for the `Helius` client
    ///
    /// # Arguments
    /// * `api_key` - A string slice that holds the API key necessary for authenticating the client
    /// * `cluster` - The Solana cluster to interact with
    ///
    /// # Returns
    /// An instance of `Config` if successful
    ///
    /// # Errors
    /// Returns `HeliusError::InvalidInput` if the `api_key` is empty
    pub fn new(api_key: &str, cluster: Cluster) -> Result<Self> {
        if api_key.is_empty() {
            return Err(HeliusError::InvalidInput("API key cannot be empty".to_string()));
        }

        let endpoints: HeliusEndpoints = HeliusEndpoints::for_cluster(&cluster);

        Ok(Config {
            api_key: api_key.to_string(),
            cluster,
            endpoints,
        })
    }
}
