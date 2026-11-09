//! Row types + CRUD impls for the bundled `accounts` and `instructions`
//! Postgres tables.
//!
//! `AccountRow<T>` and `InstructionRow<T>` store the decoded body as
//! `Json<T>`. The accompanying `*Migration` types apply
//! `CREATE TABLE IF NOT EXISTS` via `sqlx_migrator`, and
//! `GenericInstructionAbsolutePathMigration` then keys `instructions` by
//! absolute path.

use crate::{
    account::AccountMetadata,
    error::CarbonResult,
    instruction::InstructionMetadata,
    postgres::{
        metadata::{AccountRowMetadata, InstructionRowMetadata},
        operations::{Delete, Insert, Lookup, Upsert},
        primitives::Pubkey,
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
    > Lookup for AccountRow<T>
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
    pub decoded_instruction: sqlx::types::Json<T>,
}

impl<
        T: serde::Serialize + for<'de> serde::Deserialize<'de> + Clone + Send + Sync + Unpin + 'static,
    > InstructionRow<T>
{
    pub fn from_parts(decoded_instruction: T, metadata: InstructionMetadata) -> Self {
        Self {
            metadata: metadata.into(),
            decoded_instruction: sqlx::types::Json(decoded_instruction),
        }
    }
}

#[async_trait::async_trait]
impl<
        T: serde::Serialize + for<'de> serde::Deserialize<'de> + Clone + Send + Sync + Unpin + 'static,
    > Insert for InstructionRow<T>
{
    async fn insert(&self, pool: &sqlx::PgPool) -> CarbonResult<()> {
        sqlx::query(r#"INSERT INTO instructions (__signature, __instruction_index, __stack_height, __absolute_path, __slot, decoded_instruction) VALUES ($1, $2, $3, $4, $5, $6)"#)
            .bind(self.metadata.signature.clone())
            .bind(self.metadata.instruction_index)
            .bind(self.metadata.stack_height)
            .bind(self.metadata.absolute_path.clone())
            .bind(self.metadata.slot.clone())
            .bind(self.decoded_instruction.clone())
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
        sqlx::query(r#"INSERT INTO instructions (__signature, __instruction_index, __stack_height, __absolute_path, __slot, decoded_instruction) VALUES ($1, $2, $3, $4, $5, $6) ON CONFLICT (__signature, __absolute_path) DO UPDATE SET __slot = $5, decoded_instruction = $6"#)
            .bind(self.metadata.signature.clone())
            .bind(self.metadata.instruction_index)
            .bind(self.metadata.stack_height)
            .bind(self.metadata.absolute_path.clone())
            .bind(self.metadata.slot.clone())
            .bind(self.decoded_instruction.clone())
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
    type Key = (String, Vec<i16>);

    async fn delete(key: Self::Key, pool: &sqlx::PgPool) -> CarbonResult<()> {
        sqlx::query(r#"DELETE FROM instructions WHERE __signature = $1 AND __absolute_path = $2"#)
            .bind(key.0)
            .bind(key.1)
            .execute(pool)
            .await
            .map_err(|e| crate::error::Error::Custom(e.to_string()))?;
        Ok(())
    }
}

#[async_trait::async_trait]
impl<
        T: serde::Serialize + for<'de> serde::Deserialize<'de> + Clone + Send + Sync + Unpin + 'static,
    > Lookup for InstructionRow<T>
{
    type Key = (String, Vec<i16>);

    async fn lookup(key: Self::Key, pool: &sqlx::PgPool) -> CarbonResult<Option<Self>> {
        let row = sqlx::query_as(
            r#"SELECT * FROM instructions WHERE __signature = $1 AND __absolute_path = $2"#,
        )
        .bind(key.0)
        .bind(key.1)
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
            decoded_instruction JSONB NOT NULL,
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

    fn run_before(&self) -> Vec<Box<dyn sqlx_migrator::Migration<sqlx::Postgres>>> {
        vec![Box::new(GenericInstructionAbsolutePathMigration)]
    }
}

/// Re-keys `instructions` by absolute path. Registered through
/// `GenericInstructionMigration::run_before`, so applications that already
/// add that migration get this one too.
pub struct GenericInstructionAbsolutePathMigration;

impl sqlx_migrator::Migration<sqlx::Postgres> for GenericInstructionAbsolutePathMigration {
    fn app(&self) -> &str {
        "carbon_core"
    }

    fn name(&self) -> &str {
        "generic_instructions_absolute_path"
    }

    fn operations(&self) -> Vec<Box<dyn sqlx_migrator::Operation<sqlx::Postgres>>> {
        vec![Box::new(AbsolutePathKeyMigrationOperation("instructions"))]
    }

    fn parents(&self) -> Vec<Box<dyn sqlx_migrator::Migration<sqlx::Postgres>>> {
        vec![Box::new(GenericInstructionMigration)]
    }
}

/// Keys an instruction table by `(__signature, __absolute_path)`.
///
/// The old key, `(__signature, __instruction_index, __stack_height)`, holds
/// one row per depth under a top-level instruction, so a second CPI at the
/// same depth overwrote the first. The path is unique per instruction.
///
/// Rows written under the old key keep the part of their path that is known:
/// a top-level instruction gets `[index]`, and a nested one `[index, -1, ...]`
/// with one `-1` per level below it, because the old key doesn't say which
/// sibling it was. Reverting keeps only the first of the siblings stored at
/// each old key, since the old key can't hold the rest.
///
/// Decoders share tables, `cpi_events` among them, and each decoder's
/// migration lists its own, so a table that already has the column was
/// re-keyed by another decoder and is left as it is, and reverting skips a
/// table that no longer has it.
pub struct AbsolutePathKeyMigrationOperation(pub &'static str);

#[async_trait::async_trait]
impl sqlx_migrator::Operation<sqlx::Postgres> for AbsolutePathKeyMigrationOperation {
    async fn up(
        &self,
        connection: &mut sqlx::PgConnection,
    ) -> Result<(), sqlx_migrator::error::Error> {
        let table = self.0;
        if has_absolute_path(connection, table).await? {
            return Ok(());
        }
        for statement in [
            format!("ALTER TABLE {table} ADD COLUMN __absolute_path SMALLINT[]"),
            format!(
                "UPDATE {table} SET __absolute_path = ARRAY[__instruction_index]::SMALLINT[] \
                 || array_fill((-1)::SMALLINT, ARRAY[(__stack_height - 1)::INT])"
            ),
            format!("ALTER TABLE {table} ALTER COLUMN __absolute_path SET NOT NULL"),
            drop_primary_key(table),
            format!("ALTER TABLE {table} ADD PRIMARY KEY (__signature, __absolute_path)"),
        ] {
            sqlx::query(&statement).execute(&mut *connection).await?;
        }
        Ok(())
    }

    async fn down(
        &self,
        connection: &mut sqlx::PgConnection,
    ) -> Result<(), sqlx_migrator::error::Error> {
        let table = self.0;
        if !has_absolute_path(connection, table).await? {
            return Ok(());
        }
        for statement in [
            format!(
                "DELETE FROM {table} later USING {table} first \
                 WHERE later.__signature = first.__signature \
                 AND later.__instruction_index = first.__instruction_index \
                 AND later.__stack_height = first.__stack_height \
                 AND later.__absolute_path > first.__absolute_path"
            ),
            drop_primary_key(table),
            format!(
                "ALTER TABLE {table} ADD PRIMARY KEY (__signature, __instruction_index, __stack_height)"
            ),
            format!("ALTER TABLE {table} DROP COLUMN __absolute_path"),
        ] {
            sqlx::query(&statement).execute(&mut *connection).await?;
        }
        Ok(())
    }
}

async fn has_absolute_path(
    connection: &mut sqlx::PgConnection,
    table: &str,
) -> Result<bool, sqlx_migrator::error::Error> {
    let column: Option<i32> = sqlx::query_scalar(
        "SELECT 1 FROM pg_attribute \
         WHERE attrelid = $1::regclass AND attname = '__absolute_path' AND NOT attisdropped",
    )
    .bind(table)
    .fetch_optional(&mut *connection)
    .await?;
    Ok(column.is_some())
}

/// Postgres names an inline primary key `<table>_pkey`, shortened for long
/// table names, so the name is looked up rather than rebuilt.
fn drop_primary_key(table: &str) -> String {
    format!(
        "DO $$ DECLARE key_name TEXT; BEGIN \
         SELECT conname INTO key_name FROM pg_constraint \
         WHERE conrelid = '{table}'::regclass AND contype = 'p'; \
         IF key_name IS NOT NULL THEN \
         EXECUTE format('ALTER TABLE {table} DROP CONSTRAINT %I', key_name); \
         END IF; END $$"
    )
}
