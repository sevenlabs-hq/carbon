use solana_instruction::AccountMeta;

use crate::{
    account::AccountMetadata,
    error::CarbonResult,
    instruction::InstructionMetadata,
    postgres::{
        metadata::{AccountRowMetadata, InstructionRowMetadata},
        operations::{Delete, Insert, LookUp, Upsert},
        primitives::{Pubkey, U32},
    },
};

#[derive(sqlx::FromRow, Debug, Clone)]
pub struct AccountRow<
    T: serde::Serialize + for<'de> serde::Deserialize<'de> + Clone + Send + Sync + Unpin + 'static,
> {
    #[sqlx(flatten)]
    pub metadata: AccountRowMetadata,
    pub data: sqlx::types::Json<T>,
}

impl<
        T: serde::Serialize + for<'de> serde::Deserialize<'de> + Clone + Send + Sync + Unpin + 'static,
    > AccountRow<T>
{
    pub fn from_parts(source: T, metadata: AccountMetadata) -> Self {
        Self {
            metadata: metadata.into(),
            data: sqlx::types::Json(source),
        }
    }
}

#[async_trait::async_trait]
impl<
        T: serde::Serialize + for<'de> serde::Deserialize<'de> + Clone + Send + Sync + Unpin + 'static,
    > Insert for AccountRow<T>
{
    async fn insert(&self, pool: &sqlx::PgPool) -> CarbonResult<()> {
        sqlx::query(r#"INSERT INTO accounts (__pubkey, __slot, data) VALUES ($1, $2, $3)"#)
            .bind(self.metadata.pubkey)
            .bind(self.metadata.slot.clone())
            .bind(self.data.clone())
            .execute(pool)
            .await
            .map_err(|e| crate::error::Error::Custom(e.to_string()))?;
        Ok(())
    }
}

#[async_trait::async_trait]
impl<
        T: serde::Serialize + for<'de> serde::Deserialize<'de> + Clone + Send + Sync + Unpin + 'static,
    > Upsert for AccountRow<T>
{
    async fn upsert(&self, pool: &sqlx::PgPool) -> CarbonResult<()> {
        sqlx::query(r#"INSERT INTO accounts (__pubkey, __slot, data) VALUES ($1, $2, $3) ON CONFLICT (__pubkey) DO UPDATE SET __slot = $2, data = $3"#)
            .bind(self.metadata.pubkey)
            .bind(self.metadata.slot.clone())
            .bind(self.data.clone())
            .execute(pool)
            .await
            .map_err(|e| crate::error::Error::Custom(e.to_string()))?;
        Ok(())
    }
}

#[async_trait::async_trait]
impl<
        T: serde::Serialize + for<'de> serde::Deserialize<'de> + Clone + Send + Sync + Unpin + 'static,
    > Delete for AccountRow<T>
{
    type Key = Pubkey;

    async fn delete(key: Self::Key, pool: &sqlx::PgPool) -> CarbonResult<()> {
        sqlx::query(r#"DELETE FROM accounts WHERE __pubkey = $1"#)
            .bind(key)
            .execute(pool)
            .await
            .map_err(|e| crate::error::Error::Custom(e.to_string()))?;
        Ok(())
    }
}

#[async_trait::async_trait]
impl<
        T: serde::Serialize + for<'de> serde::Deserialize<'de> + Clone + Send + Sync + Unpin + 'static,
    > LookUp for AccountRow<T>
{
    type Key = Pubkey;

    async fn lookup(key: Self::Key, pool: &sqlx::PgPool) -> CarbonResult<Option<Self>> {
        let row = sqlx::query_as(r#"SELECT * FROM accounts WHERE __pubkey = $1"#)
            .bind(key)
            .fetch_optional(pool)
            .await
            .map_err(|e| crate::error::Error::Custom(e.to_string()))?;
        Ok(row)
    }
}

pub struct AccountRowMigrationOperation;

#[async_trait::async_trait]
impl sqlx_migrator::Operation<sqlx::Postgres> for AccountRowMigrationOperation {
    async fn up(
        &self,
        connection: &mut sqlx::PgConnection,
    ) -> Result<(), sqlx_migrator::error::Error> {
        sqlx::query(
            r#"CREATE TABLE IF NOT EXISTS accounts (
            __pubkey BYTEA NOT NULL,
            __slot NUMERIC,
            data JSONB NOT NULL,
            PRIMARY KEY (__pubkey)
        )"#,
        )
        .execute(connection)
        .await?;
        Ok(())
    }

    async fn down(
        &self,
        connection: &mut sqlx::PgConnection,
    ) -> Result<(), sqlx_migrator::error::Error> {
        sqlx::query(r#"DROP TABLE IF EXISTS accounts"#)
            .execute(connection)
            .await?;
        Ok(())
    }
}

pub struct GenericAccountsMigration;

impl sqlx_migrator::Migration<sqlx::Postgres> for GenericAccountsMigration {
    fn app(&self) -> &str {
        "carbon_core"
    }

    fn name(&self) -> &str {
        "generic_accounts"
    }

    fn operations(&self) -> Vec<Box<dyn sqlx_migrator::Operation<sqlx::Postgres>>> {
        vec![Box::new(AccountRowMigrationOperation)]
    }

    fn parents(&self) -> Vec<Box<dyn sqlx_migrator::Migration<sqlx::Postgres>>> {
        vec![]
    }
}

#[derive(sqlx::FromRow, Debug, Clone)]
pub struct InstructionRow<
    T: serde::Serialize + for<'de> serde::Deserialize<'de> + Clone + Send + Sync + Unpin + 'static,
> {
    #[sqlx(flatten)]
    pub metadata: InstructionRowMetadata,
    pub data: sqlx::types::Json<T>,
    pub accounts: sqlx::types::Json<Vec<AccountMeta>>,
}

impl<
        T: serde::Serialize + for<'de> serde::Deserialize<'de> + Clone + Send + Sync + Unpin + 'static,
    > InstructionRow<T>
{
    pub fn from_parts(
        source: T,
        metadata: InstructionMetadata,
        accounts: Vec<AccountMeta>,
    ) -> Self {
        Self {
            metadata: metadata.into(),
            data: sqlx::types::Json(source),
            accounts: sqlx::types::Json(accounts),
        }
    }
}

#[async_trait::async_trait]
impl<
        T: serde::Serialize + for<'de> serde::Deserialize<'de> + Clone + Send + Sync + Unpin + 'static,
    > Insert for InstructionRow<T>
{
    async fn insert(&self, pool: &sqlx::PgPool) -> CarbonResult<()> {
        sqlx::query(r#"INSERT INTO instructions (__signature, __instruction_index, __stack_height, __slot, data, accounts) VALUES ($1, $2, $3, $4, $5, $6)"#)
            .bind(self.metadata.signature.clone())
            .bind(self.metadata.instruction_index)
            .bind(self.metadata.stack_height)
            .bind(self.metadata.slot.clone())
            .bind(self.data.clone())
            .bind(self.accounts.clone())
            .execute(pool)
            .await
            .map_err(|e| crate::error::Error::Custom(e.to_string()))?;
        Ok(())
    }
}

#[async_trait::async_trait]
impl<
        T: serde::Serialize + for<'de> serde::Deserialize<'de> + Clone + Send + Sync + Unpin + 'static,
    > Upsert for InstructionRow<T>
{
    async fn upsert(&self, pool: &sqlx::PgPool) -> CarbonResult<()> {
        sqlx::query(r#"INSERT INTO instructions (__signature, __instruction_index, __stack_height, __slot, data, accounts) VALUES ($1, $2, $3, $4, $5, $6) ON CONFLICT (__signature, __instruction_index, __stack_height) DO UPDATE SET __slot = $4, data = $5, accounts = $6"#)
            .bind(self.metadata.signature.clone())
            .bind(self.metadata.instruction_index)
            .bind(self.metadata.stack_height)
            .bind(self.metadata.slot.clone())
            .bind(self.data.clone())
            .bind(self.accounts.clone())
            .execute(pool)
            .await
            .map_err(|e| crate::error::Error::Custom(e.to_string()))?;
        Ok(())
    }
}

#[async_trait::async_trait]
impl<
        T: serde::Serialize + for<'de> serde::Deserialize<'de> + Clone + Send + Sync + Unpin + 'static,
    > Delete for InstructionRow<T>
{
    type Key = (String, U32, U32);

    async fn delete(key: Self::Key, pool: &sqlx::PgPool) -> CarbonResult<()> {
        sqlx::query(r#"DELETE FROM instructions WHERE __signature = $1 AND __instruction_index = $2 AND __stack_height = $3"#)
            .bind(key.0)
            .bind(key.1)
            .bind(key.2)
            .execute(pool)
            .await
            .map_err(|e| crate::error::Error::Custom(e.to_string()))?;
        Ok(())
    }
}

#[async_trait::async_trait]
impl<
        T: serde::Serialize + for<'de> serde::Deserialize<'de> + Clone + Send + Sync + Unpin + 'static,
    > LookUp for InstructionRow<T>
{
    type Key = (String, U32, U32);

    async fn lookup(key: Self::Key, pool: &sqlx::PgPool) -> CarbonResult<Option<Self>> {
        let row =
            sqlx::query_as(r#"SELECT * FROM instructions WHERE __signature = $1 AND __instruction_index = $2 AND __stack_height = $3"#)
                .bind(key.0)
                .bind(key.1)
                .bind(key.2)
                .fetch_optional(pool)
                .await
                .map_err(|e| crate::error::Error::Custom(e.to_string()))?;
        Ok(row)
    }
}

pub struct InstructionRowMigrationOperation;

#[async_trait::async_trait]
impl sqlx_migrator::Operation<sqlx::Postgres> for InstructionRowMigrationOperation {
    async fn up(
        &self,
        connection: &mut sqlx::PgConnection,
    ) -> Result<(), sqlx_migrator::error::Error> {
        sqlx::query(
            r#"CREATE TABLE IF NOT EXISTS instructions (
            __signature TEXT NOT NULL,
            __instruction_index BIGINT NOT NULL,
            __stack_height BIGINT NOT NULL,
            __slot NUMERIC,
            data JSONB NOT NULL,
            accounts JSONB NOT NULL,
            PRIMARY KEY (__signature, __instruction_index, __stack_height)
        )"#,
        )
        .execute(connection)
        .await?;
        Ok(())
    }

    async fn down(
        &self,
        connection: &mut sqlx::PgConnection,
    ) -> Result<(), sqlx_migrator::error::Error> {
        sqlx::query(r#"DROP TABLE IF EXISTS instructions"#)
            .execute(connection)
            .await?;
        Ok(())
    }
}

pub struct GenericInstructionMigration;

impl sqlx_migrator::Migration<sqlx::Postgres> for GenericInstructionMigration {
    fn app(&self) -> &str {
        "carbon_core"
    }

    fn name(&self) -> &str {
        "generic_instructions"
    }

    fn operations(&self) -> Vec<Box<dyn sqlx_migrator::Operation<sqlx::Postgres>>> {
        vec![Box::new(InstructionRowMigrationOperation)]
    }

    fn parents(&self) -> Vec<Box<dyn sqlx_migrator::Migration<sqlx::Postgres>>> {
        vec![]
    }
}
