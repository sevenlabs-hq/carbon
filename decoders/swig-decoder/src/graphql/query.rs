use juniper::{graphql_object, FieldResult};
use std::str::FromStr;

pub struct QueryRoot;

#[graphql_object(context = crate::graphql::context::GraphQLContext)]
impl QueryRoot {
    // Accounts

    // Instructions (per-instruction list and lookup by signature+index)
}
