use async_trait::async_trait;
use carbon_postgres_client::PgClient;
use solana_pubkey::Pubkey;
use spl_token::state::{Account, Mint};

#[async_trait]
pub trait TokenQueries {
    async fn save_token(&self) -> Result<(), String>;
    async fn get_token_by_pk(&self, pk: &Pubkey) -> Result<Account, String>;
    async fn get_mint_by_pk(&self, pk: &Pubkey) -> Result<Mint, String>;
}

#[async_trait]
impl TokenQueries for PgClient {
    async fn save_token(&self) -> Result<(), String> {
        Ok(())
    }

    async fn get_token_by_pk(&self, pk: &Pubkey) -> Result<Account, String> {
        Ok(Account::default())
    }

    async fn get_mint_by_pk(&self, pk: &Pubkey) -> Result<Mint, String> {
        Ok(Mint::default())
    }
}
