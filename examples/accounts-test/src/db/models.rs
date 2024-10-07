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
