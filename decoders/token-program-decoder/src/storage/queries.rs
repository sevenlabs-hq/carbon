use async_trait::async_trait;
use carbon_postgres_client::PgClient;
use solana_pubkey::Pubkey;
use spl_token::state::{Account, Mint};

use super::converters::DBTokenAccount;

#[async_trait]
pub trait TokenQueries {
    async fn save_token(&self, token: Account) -> Result<(), String>;
    async fn save_mint(&self, mint: Mint) -> Result<(), String>;
    async fn get_token_by_pk(&self, pk: &Pubkey) -> Result<Account, String>;
    async fn get_mint_by_pk(&self, pk: &Pubkey) -> Result<Mint, String>;
}

#[async_trait]
impl TokenQueries for PgClient {
    async fn save_token(&self, token: Account) -> Result<(), String> {
        let query = sqlx::query(
            "INSERT INTO tokens (mint, owner, amount, delegate, state, is_native, delegated_amount, close_authority)
                VALUES ($1, $2, $3, $4, $5, $6, $7, $8)
                ON CONFLICT (mint) DO UPDATE SET 
                            owner=excluded.owner, amount=excluded.amount, delegate=excluded.delegate, state=excluded.state, is_native=excluded.is_native, delegated_amount=excluded.delegated_amount, close_authority=excluded.close_authority"
            );

        let db_token: DBTokenAccount = token.into();

        query
            .bind(db_token.mint)
            .bind(db_token.owner)
            .bind(db_token.amount)
            .bind(db_token.delegate)
            .bind(db_token.state)
            .bind(db_token.is_native)
            .bind(db_token.delegated_amount)
            .bind(db_token.close_authority)
            .execute(&self.pool)
            .await
            .map_err(|e| format!("save token: {}", e))?;

        Ok(())
    }

    async fn save_mint(&self, mint: Mint) -> Result<(), String> {
        let query = sqlx::query(
            "INSERT INTO mints ()
                          VALUES ($1)
                          ON CONFLICT () DO UPDATE SET "
            );

        if let Err(e) = query.bind().execute(&self.pool).await {
            return Err(e.to_string());
        }

        Ok(())
    }

    async fn get_token_by_pk(&self, pk: &Pubkey) -> Result<Account, String> {
        Ok(Account::default())
    }

    async fn get_mint_by_pk(&self, pk: &Pubkey) -> Result<Mint, String> {
        Ok(Mint::default())
    }
}
