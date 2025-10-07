use async_trait::async_trait;
use carbon_postgres_client::PgClient;
use solana_pubkey::Pubkey;
use spl_token_interface::state::{Account, Mint};
use sqlx::Postgres;

use super::converters::{DBMint, DBTokenAccount};

#[async_trait]
pub trait TokenQueries {
    async fn save_token(&self, token: Account) -> Result<(), String>;
    async fn save_mint(&self, mint: Mint, pk: &Pubkey) -> Result<(), String>;
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

    async fn save_mint(&self, mint: Mint, pk: &Pubkey) -> Result<(), String> {
        let query = sqlx::query(
            "INSERT INTO mints (mint, mint_authority, supply, decimals, is_initialized, freeze_authority)
                          VALUES ($1, $2, $3, $4, $5, $6)
                          ON CONFLICT (mint) DO UPDATE SET 
                            supply=excluded.supply, 
                            decimals=excluded.decimals,
                            is_initialized=excluded.is_initialized,
                            freeze_authority=excluded.freeze_authority",
        );

        let db_mint: DBMint = mint.into();

        query
            .bind(pk.to_bytes().to_vec())
            .bind(db_mint.mint_authority)
            .bind(db_mint.supply)
            .bind(db_mint.decimals)
            .bind(db_mint.is_initialized)
            .bind(db_mint.freeze_authority)
            .execute(&self.pool)
            .await
            .map_err(|e| format!("save mint: {}", e))?;

        Ok(())
    }

    async fn get_token_by_pk(&self, pk: &Pubkey) -> Result<Account, String> {
        let query = sqlx::query_as::<Postgres, DBTokenAccount>(
            "SELECT mint, owner, amount, delegate, state, is_native, delegated_amount, close_authority
               FROM tokens
               WHERE mint = $1
               LIMIT 1",
        );
        let row = query
            .bind(pk.to_bytes().to_vec())
            .fetch_one(&self.pool)
            .await
            .map_err(|e| format!("query token: {}", e))?;

        Ok(row.try_into().map_err(|e| format!("parse token: {}", e))?)
    }

    async fn get_mint_by_pk(&self, pk: &Pubkey) -> Result<Mint, String> {
        let query = sqlx::query_as::<Postgres, DBMint>(
            "SELECT mint, mint_authority, supply, decimals, is_initialized, freeze_authority
               FROM mints
               WHERE mint = $1
               LIMIT 1",
        );
        let row = query
            .bind(pk.to_bytes().to_vec())
            .fetch_one(&self.pool)
            .await
            .map_err(|e| format!("query mint: {}", e))?;

        Ok(row.try_into().map_err(|e| format!("parse mint: {}", e))?)
    }
}
