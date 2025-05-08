use crate::storage::queries::TokenQueries;
use carbon_gql_server::types::pubkey::Pubkey;
use carbon_postgres_client::PgClient;
use converters::{GQLAccount, GQLMint};
use juniper::{graphql_object, EmptyMutation, EmptySubscription, RootNode};

mod converters;

#[derive(Clone, Copy, Debug)]
pub struct TokenQuery;

#[graphql_object]
#[graphql(context = PgClient)]
impl TokenQuery {
    async fn mint_by_pubkey(#[graphql(context)] pg: &PgClient, pk: Pubkey) -> Option<GQLMint> {
        pg.get_mint_by_pk(&pk.0).await.ok().map(Into::into)
    }

    async fn token_by_pubkey(#[graphql(context)] pg: &PgClient, pk: Pubkey) -> Option<GQLAccount> {
        pg.get_token_by_pk(&pk.0).await.ok().map(Into::into)
    }
}

pub type TokenProgramSchema =
    RootNode<'static, TokenQuery, EmptyMutation<PgClient>, EmptySubscription<PgClient>>;
