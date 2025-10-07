use carbon_gql_server::types::{pubkey::Pubkey, u64::U64, u8::U8};
use juniper::{GraphQLEnum, GraphQLObject};
use solana_program::program_option::COption;
use spl_token_interface::state::{Account, AccountState, Mint};

#[derive(GraphQLObject)]
pub struct GQLMint {
    pub mint_authority: Option<Pubkey>,
    pub supply: U64,
    pub decimals: U8,
    pub is_initialized: bool,
    pub freeze_authority: Option<Pubkey>,
}

#[derive(GraphQLEnum)]
pub enum GQLAccountState {
    Uninitialized,
    Initialized,
    Frozen,
}

#[derive(GraphQLObject)]
pub struct GQLAccount {
    pub mint: Pubkey,
    pub owner: Pubkey,
    pub amount: U64,
    pub delegate: Option<Pubkey>,
    pub state: GQLAccountState,
    pub is_native: Option<U64>,
    pub delegated_amount: U64,
    pub close_authority: Option<Pubkey>,
}

impl From<Mint> for GQLMint {
    fn from(mint: Mint) -> Self {
        Self {
            mint_authority: match mint.mint_authority {
                COption::Some(mint_authority) => Some(Pubkey(mint_authority)),
                COption::None => None,
            },
            supply: U64(mint.supply),
            decimals: U8(mint.decimals),
            is_initialized: mint.is_initialized,
            freeze_authority: match mint.freeze_authority {
                COption::Some(freeze_authority) => Some(Pubkey(freeze_authority)),
                COption::None => None,
            },
        }
    }
}

impl From<AccountState> for GQLAccountState {
    fn from(account_state: AccountState) -> Self {
        match account_state {
            AccountState::Uninitialized => Self::Uninitialized,
            AccountState::Initialized => Self::Initialized,
            AccountState::Frozen => Self::Frozen,
        }
    }
}

impl From<Account> for GQLAccount {
    fn from(account: Account) -> Self {
        Self {
            mint: Pubkey(account.mint),
            owner: Pubkey(account.owner),
            amount: U64(account.amount),
            delegate: match account.delegate {
                COption::Some(delegate) => Some(Pubkey(delegate)),
                COption::None => None,
            },
            state: account.state.into(),
            is_native: match account.is_native {
                COption::Some(is_native) => Some(U64(is_native)),
                COption::None => None,
            },
            delegated_amount: U64(account.delegated_amount),
            close_authority: match account.close_authority {
                COption::Some(close_authority) => Some(Pubkey(close_authority)),
                COption::None => None,
            },
        }
    }
}
