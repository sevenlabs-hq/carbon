use std::env;
use std::str::FromStr;
use anyhow::{Context, Result}; // Make sure to include this for error handling

#[derive(Debug, Clone)]
pub struct Env {
    // Server
    pub database_url: String,
    pub rpc_url: String,
}

impl Env {
    pub fn new() -> Result<Self> { // Result<Self, anyhow::Error>
        Ok(Self {
            database_url: load_var("DATABASE_URL")?,
            rpc_url: load_var("RPC_URL")?,
        })
    }
}

fn load_var<T: FromStr>(name: &str) -> Result<T>
where
    T::Err: std::error::Error + Send + Sync + 'static, // Make sure error type satisfies anyhow::Context
{
    let value = env::var(name)
        .with_context(|| format!("Environment variable {} is not set.", name))?;
    
    value
        .parse::<T>()
        .with_context(|| format!("Failed to parse {}", name))
}