use std::sync::Arc;

use axum::Extension;

use carbon_postgres_client::PgClient;
use juniper_axum::{extract::JuniperRequest, response::JuniperResponse};
use juniper_graphql_ws::Schema;

pub mod server;
pub mod types;

// async fn pg_graphql(
//     Extension(schema): Extension<Arc<Schema>>,
//     Extension(pg_client): Extension<PgClient>,
//     JuniperRequest(request): JuniperRequest,
// ) -> JuniperResponse {
//     JuniperResponse(request.execute(&*schema, &pg_client).await)
// }

pub async fn pg_graphql<S>(
    Extension(schema): Extension<S>,
    Extension(pg_client): Extension<PgClient>,
    JuniperRequest(req): JuniperRequest<S::ScalarValue>,
) -> JuniperResponse<S::ScalarValue>
where
    S: Schema,
    S::Context: Default,
{
    JuniperResponse(req.execute(schema.root_node(), &pg_client).await)
}
