//! GraphQL feature: pre-wired Juniper schema + Axum router and
//! GraphQL-shaped scalars for Solana primitives.
//!
//! - [`primitives`] — `GraphQLScalar`-derived newtypes (`Pubkey`, `I64`/`I128`,
//!   `U8`/`U32`/`U64`/`U128`, `Json`).
//! - [`server`] (feature-gated) — `build_schema` + `graphql_router` helpers
//!   that mount `/graphql` and `/graphiql` on an Axum `Router`.

pub mod primitives;
#[cfg(feature = "graphql")]
pub mod server;
