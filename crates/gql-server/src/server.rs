use {
    crate::pg_graphql,
    axum::{
        routing::{get, on, MethodFilter},
        Extension, Router,
    },
    carbon_postgres_client::PgClient,
    juniper_axum::{graphiql, playground},
    juniper_graphql_ws::Schema,
    std::net::SocketAddr,
    tokio::net::TcpListener,
};

pub async fn run<S>(addr: SocketAddr, pg_client: PgClient, schema: S)
where
    S: Schema,
    S::Context: From<PgClient>,
{
    let app = Router::new()
        .route(
            "/graphql",
            on(MethodFilter::GET.or(MethodFilter::POST), pg_graphql::<S>),
        )
        .route("/graphiql", get(graphiql("/graphql", None)))
        .route("/playground", get(playground("/graphql", None)))
        .layer(Extension(schema))
        .layer(Extension(pg_client));

    let listener = TcpListener::bind(addr)
        .await
        .unwrap_or_else(|e| panic!("failed to listen on {addr}: {e}"));

    println!("listening on {addr}");

    axum::serve(listener, app)
        .await
        .unwrap_or_else(|e| panic!("failed to run `axum::serve`: {e}"));
}
