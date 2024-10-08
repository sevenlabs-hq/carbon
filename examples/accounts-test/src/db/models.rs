use crate::db::schema::bonding_curve;
use crate::db::schema::global_account;
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
    pub fn new(
        id: i32,
        pubkey: String,
        initialized: bool,
        authority: String,
        fee_recipient: String,
        initial_virtual_token_reserves: i64,
        initial_virtual_sol_reserves: i64,
        initial_real_token_reserves: i64,
        token_total_supply: i64,
        fee_basis_points: i64,
    ) -> Self {
        Self {
            id,
            pubkey,
            initialized,
            authority,
            fee_recipient,
            initial_virtual_token_reserves,
            initial_virtual_sol_reserves,
            initial_real_token_reserves,
            token_total_supply,
            fee_basis_points,
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
    pub fn new(
        id: i32,
        pubkey: String,
        virtual_token_reserves: i64,
        virtual_sol_reserves: i64,
        real_token_reserves: i64,
        real_sol_reserves: i64,
        token_total_supply: i64,
        complete: bool,
    ) -> Self {
        Self {
            id,
            pubkey,
            virtual_token_reserves,
            virtual_sol_reserves,
            real_token_reserves,
            real_sol_reserves,
            token_total_supply,
            complete,
        }
    }
}
