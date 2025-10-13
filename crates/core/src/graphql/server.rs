use axum::{
    routing::{get, on, MethodFilter},
    Extension, Router,
};
use juniper::{EmptyMutation, EmptySubscription, RootNode};
use juniper_axum::{extract::JuniperRequest, graphiql, response::JuniperResponse};
use std::sync::Arc;

pub type DefaultMutation<C> = EmptyMutation<C>;
pub type DefaultSubscription<C> = EmptySubscription<C>;

pub fn build_schema<Q, C>(
    query_root: Q,
) -> Arc<RootNode<'static, Q, DefaultMutation<C>, DefaultSubscription<C>>>
where
    Q: juniper::GraphQLType<Context = C, TypeInfo = ()> + Send + Sync + 'static,
    C: juniper::Context + Send + Sync + 'static,
{
    Arc::new(RootNode::new(
        query_root,
        DefaultMutation::new(),
        DefaultSubscription::new(),
    ))
}

pub fn graphql_router<Q, C>(
    schema: Arc<RootNode<'static, Q, DefaultMutation<C>, DefaultSubscription<C>>>,
    make_ctx: impl Clone + Send + Sync + 'static + Fn() -> C,
) -> Router
where
    Q: juniper::GraphQLType<Context = C, TypeInfo = ()> + Send + Sync + 'static,
    C: juniper::Context + Send + Sync + 'static,
{
    async fn handler<Q, C>(
        Extension(schema): Extension<
            Arc<RootNode<'static, Q, DefaultMutation<C>, DefaultSubscription<C>>>,
        >,
        Extension(make_ctx): Extension<Arc<dyn Send + Sync + Fn() -> C>>,
        juniper_axum::extract::JuniperRequest(req): JuniperRequest<juniper::DefaultScalarValue>,
    ) -> JuniperResponse<juniper::DefaultScalarValue>
    where
        Q: juniper::GraphQLType<Context = C, TypeInfo = ()> + Send + Sync + 'static,
        C: juniper::Context + Send + Sync + 'static,
    {
        let ctx = make_ctx();
        JuniperResponse(req.execute_sync(&schema, &ctx))
    }

    let make_ctx = Arc::new(make_ctx);
    Router::new()
        .route(
            "/graphql",
            on(MethodFilter::GET.or(MethodFilter::POST), handler::<Q, C>),
        )
        .route("/graphiql", get(graphiql("/graphql", None)))
        .layer(Extension(schema))
        .layer(Extension(make_ctx))
}
