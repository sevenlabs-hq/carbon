use sqlx_migrator::error::Error;
use sqlx_migrator::operation::Operation;

pub(crate) struct InitOperation;

#[async_trait::async_trait]
impl Operation<sqlx::Postgres> for InitOperation {
    // Up function runs apply migration
    async fn up(&self, connection: &mut sqlx::PgConnection) -> Result<(), Error> {
        sqlx::query(
            "CREATE TYPE account_state AS ENUM (
                        'Uninitialized',
                        'Initialized',
                        'Frozen'
                    )",
        )
        .execute(&mut *connection)
        .await?;
        sqlx::query(
            "CREATE TABLE IF NOT EXISTS  tokens (
                        mint BYTEA PRIMARY KEY,
                        owner BYTEA NOT NULL,
                        amount NUMERIC NOT NULL,
                        delegate BYTEA,
                        state account_state,
                        is_native NUMERIC,
                        delegated_amount NUMERIC,
                        close_authority BYTEA
                    )",
        )
        .execute(&mut *connection)
        .await?;
        sqlx::query(
            "CREATE TABLE IF NOT EXISTS mints (
                            mint BYTEA PRIMARY KEY,
                            mint_authority BYTEA,
                            supply NUMERIC NOT NULL,
                            decimals SMALLINT NOT NULL,
                            is_initialized BOOLEAN NOT NULL,
                            freeze_authority BYTEA
                        )",
        )
        .execute(&mut *connection)
        .await?;
        Ok(())
    }

    // down migration runs down migration
    async fn down(&self, connection: &mut sqlx::PgConnection) -> Result<(), Error> {
        sqlx::query("DROP TABLE token")
            .execute(&mut *connection)
            .await?;
        sqlx::query("DROP TABLE mint")
            .execute(&mut *connection)
            .await?;
        sqlx::query("DROP TYPE \"AccountState\"")
            .execute(&mut *connection)
            .await?;
        Ok(())
    }
}
