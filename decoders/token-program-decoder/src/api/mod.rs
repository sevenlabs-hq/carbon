use carbon_gql_server::types::{amount::Amount, decimals::Decimals, pubkey::Pubkey};
use carbon_postgres_client::PgClient;
use juniper::{
    graphql_object, EmptyMutation, EmptySubscription, GraphQLEnum, GraphQLObject, RootNode,
};

#[derive(GraphQLObject)]
pub struct Mint {
    pub mint_authority: Option<Pubkey>,
    pub supply: Amount,
    pub decimals: Decimals,
    pub is_initialized: bool,
    pub freeze_authority: Option<Pubkey>,
}

#[derive(GraphQLEnum)]
pub enum AccountState {
    Uninitialized,
    Initialized,
    Frozen,
}

#[derive(GraphQLObject)]
pub struct Account {
    pub mint: Pubkey,
    pub owner: Pubkey,
    pub amount: Amount,
    pub delegate: Option<Pubkey>,
    pub state: AccountState,
    pub is_native: Option<Amount>,
    pub delegated_amount: Amount,
    pub close_authority: Option<Pubkey>,
}

#[derive(Clone, Copy, Debug)]
pub struct TokenQuery;

#[graphql_object]
#[graphql(context = PgClient)] 
impl TokenQuery {
    fn mint_by_pubkey<'pg>(pk: Pubkey, context: &'pg PgClient) -> Mint {
        Mint {
            mint_authority: None,
            supply: Amount(0),
            decimals: Decimals(0),
            is_initialized: false,
            freeze_authority: None,
        }
    }
    fn token_by_pubkey(pk: Pubkey) -> Account {
        Account {
            mint: pk.clone(),
            owner: pk,
            amount: Amount(0),
            delegate: None,
            state: AccountState::Uninitialized,
            is_native: None,
            delegated_amount: Amount(0),
            close_authority: None,
        }
    }
}

pub type TokenProgramSchema = RootNode<'static, TokenQuery, EmptyMutation, EmptySubscription>;
