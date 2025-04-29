use sqlx_migrator::error::Error;
use sqlx_migrator::operation::Operation;

pub(crate) struct InitOperation;

#[async_trait::async_trait]
impl Operation<sqlx::Postgres> for InitOperation {
    // Up function runs apply migration
    async fn up(&self, connection: &mut sqlx::PgConnection) -> Result<(), Error> {
        sqlx::query(
            "CREATE TYPE IF NOT EXISTS \"AccountState\" AS ENUM (
                        'Uninitialized',
                        'Initialized',
                        'Frozen'
                    )",
        )
        .execute(&mut *connection)
        .await?;
        sqlx::query(
            "CREATE TABLE IF NOT EXISTS  token (
                        mint BYTEA PRIMARY KEY,
                        owner BYTEA NOT NULL,
                        amount BIGINT NOT NULL,
                        delegate BYTEA,
                        state AccountState,
                        is_native BIGINT,
                        delegated_amount BIGINT,
                        close_authority BYTEA
                    )",
        )
        .execute(&mut *connection)
        .await?;
        sqlx::query(
            "CREATE TABLE IF NOT EXISTS mint (
                            mint_authority BYTEA,
                            supply BIGINT NOT NULL,
                            decimals INT NOT NULL,
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
