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

type ExtensionSchema<Q, C> =
    Extension<Arc<RootNode<'static, Q, DefaultMutation<C>, DefaultSubscription<C>>>>;

pub fn graphql_router<Q, C>(
    schema: Arc<RootNode<'static, Q, DefaultMutation<C>, DefaultSubscription<C>>>,
    context: C,
) -> Router
where
    Q: juniper::GraphQLTypeAsync<Context = C, TypeInfo = ()>
        + juniper::GraphQLValueAsync<Context = C, TypeInfo = ()>
        + Send
        + Sync
        + 'static,
    C: juniper::Context + Clone + Send + Sync + 'static,
{
    async fn handler<Q, C>(
        Extension(schema): ExtensionSchema<Q, C>,
        Extension(ctx): Extension<C>,
        juniper_axum::extract::JuniperRequest(req): JuniperRequest<juniper::DefaultScalarValue>,
    ) -> JuniperResponse<juniper::DefaultScalarValue>
    where
        Q: juniper::GraphQLTypeAsync<Context = C, TypeInfo = ()> + Send + Sync + 'static,
        C: juniper::Context + Clone + Send + Sync + 'static,
    {
        JuniperResponse(req.execute(&schema, &ctx).await)
    }

    Router::new()
        .route(
            "/graphql",
            on(MethodFilter::GET.or(MethodFilter::POST), handler::<Q, C>),
        )
        .route("/graphiql", get(graphiql("/graphql", None)))
        .layer(Extension(schema))
        .layer(Extension(context))
}
