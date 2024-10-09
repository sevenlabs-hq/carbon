use diesel::Connection;

pub mod models;
pub mod schema;

pub fn get_conn() -> diesel::PgConnection {
    let database_url = std::env::var(
        "postgresql://my_user:my_password@localhost:5432/my_database
",
    )
    .expect("DATABASE_URL must be set");
    diesel::PgConnection::establish(&database_url).unwrap()
}
