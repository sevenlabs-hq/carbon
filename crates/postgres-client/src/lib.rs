use sqlx::{postgres::PgPoolOptions, Error, PgPool, Postgres};
use sqlx_migrator::{
    migrator::{Info, Migrate, Migrator},
    Migration, Plan,
};

#[derive(Clone)]
pub struct PgClient {
    pub pool: PgPool,
}

impl juniper::Context for PgClient {}

impl PgClient {
    pub async fn new(url: &str, min_connections: u32, max_connections: u32) -> Result<Self, Error> {
        let pool = PgPoolOptions::new()
            .min_connections(min_connections)
            .max_connections(max_connections)
            .connect(url)
            .await?;

        Ok(Self { pool })
    }

    pub async fn migrate(
        &self,
        migrations: Vec<Box<dyn Migration<Postgres>>>,
    ) -> Result<(), Error> {
        let mut migrator = Migrator::<Postgres>::default();
        migrator
            .add_migrations(migrations)
            .expect("Failed to add migrations");

        let mut conn = self.pool.acquire().await?;
        migrator
            .run(&mut *conn, &Plan::apply_all())
            .await
            .expect("Failed to run migrations");

        Ok(())
    }
}
