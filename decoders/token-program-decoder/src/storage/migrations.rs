use sqlx_migrator::migration::Migration;
use sqlx_migrator::operation::Operation;

use crate::storage::operations::InitOperation;

pub struct InitMigration;

impl Migration<sqlx::Postgres> for InitMigration {
    fn app(&self) -> &str {
        "main"
    }

    fn name(&self) -> &str {
        "init_token_storage"
    }

    fn parents(&self) -> Vec<Box<dyn Migration<sqlx::Postgres>>> {
        vec![]
    }

    fn operations(&self) -> Vec<Box<dyn Operation<sqlx::Postgres>>> {
        vec![Box::new(InitOperation)]
    }
}
