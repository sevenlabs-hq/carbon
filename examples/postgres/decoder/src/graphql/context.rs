pub struct GraphQLContext {
    pub pool: std::sync::Arc<sqlx::PgPool>,
}

impl juniper::Context for GraphQLContext {}

