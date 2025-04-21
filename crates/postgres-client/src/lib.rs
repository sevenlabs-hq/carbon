use sqlx::{
    migrate::Migrator,
    postgres::{PgConnectOptions, PgPoolOptions},
    ConnectOptions, Error, Executor, PgPool, Postgres, QueryBuilder, Row, Transaction,
};

#[derive(Clone)]
pub struct PgClient {
    pub pool: PgPool,
}

impl PgClient {
    pub async fn new(url: &str, min_connections: u32, max_connections: u32) -> Result<Self, Error> {
        let pool = PgPoolOptions::new()
            .min_connections(min_connections)
            .max_connections(max_connections)
            .connect(url)
            .await?;

        Ok(Self { pool })
    }
}
