use crate::db::schema::bonding_curve;
use crate::db::schema::global_account;
use crate::pump_decoder::accounts::bonding_curve::BondingCurve as BondingCurveAccount;
use crate::pump_decoder::accounts::global::Global;
use diesel::prelude::*;

#[derive(Debug, Queryable, Insertable, AsChangeset)]
#[table_name = "global_account"]
pub struct GlobalAccount {
    pub id: i32,
    pub pubkey: String,
    pub initialized: bool,
    pub authority: String,
    pub fee_recipient: String,
    pub initial_virtual_token_reserves: i64,
    pub initial_virtual_sol_reserves: i64,
    pub initial_real_token_reserves: i64,
    pub token_total_supply: i64,
    pub fee_basis_points: i64,
}

impl GlobalAccount {
    pub fn from_account(account: Global, pubkey: solana_sdk::pubkey::Pubkey) -> Self {
        Self {
            id: 0,
            pubkey: pubkey.to_string(),
            initialized: account.initialized,
            authority: account.authority.to_string(),
            fee_recipient: account.fee_recipient.to_string(),
            initial_virtual_token_reserves: account.initial_virtual_token_reserves as i64,
            initial_virtual_sol_reserves: account.initial_virtual_sol_reserves as i64,
            initial_real_token_reserves: account.initial_real_token_reserves as i64,
            token_total_supply: account.token_total_supply as i64,
            fee_basis_points: account.fee_basis_points as i64,
        }
    }
}

#[derive(Debug, Queryable, Insertable, AsChangeset)]
#[table_name = "bonding_curve"]
pub struct BondingCurve {
    pub id: i32,
    pub pubkey: String,
    pub virtual_token_reserves: i64,
    pub virtual_sol_reserves: i64,
    pub real_token_reserves: i64,
    pub real_sol_reserves: i64,
    pub token_total_supply: i64,
    pub complete: bool,
}

impl BondingCurve {
    pub fn from_account(account: BondingCurveAccount, pubkey: solana_sdk::pubkey::Pubkey) -> Self {
        Self {
            id: 0,
            pubkey: pubkey.to_string(),
            virtual_token_reserves: account.virtual_token_reserves as i64,
            virtual_sol_reserves: account.virtual_sol_reserves as i64,
            real_token_reserves: account.real_token_reserves as i64,
            real_sol_reserves: account.real_sol_reserves as i64,
            token_total_supply: account.token_total_supply as i64,
            complete: account.complete,
        }
    }
}
