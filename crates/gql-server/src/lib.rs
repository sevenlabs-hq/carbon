use axum::Extension;

use carbon_postgres_client::PgClient;
use juniper_axum::{extract::JuniperRequest, response::JuniperResponse};
use juniper_graphql_ws::Schema;

pub mod server;
pub mod types;

pub async fn pg_graphql<S>(
    Extension(schema): Extension<S>,
    Extension(pg_client): Extension<PgClient>,
    JuniperRequest(req): JuniperRequest<S::ScalarValue>,
) -> JuniperResponse<S::ScalarValue>
where
    S: Schema,
    S::Context: From<PgClient>,
{
    JuniperResponse(req.execute(schema.root_node(), &pg_client.into()).await)
}
