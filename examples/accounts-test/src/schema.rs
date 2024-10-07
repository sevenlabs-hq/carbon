// @generated automatically by Diesel CLI.

diesel::table! {
    bonding_curve (id) {
        id -> Int4,
        pubkey -> Varchar,
        virtual_token_reserves -> Int8,
        virtual_sol_reserves -> Int8,
        real_token_reserves -> Int8,
        real_sol_reserves -> Int8,
        token_total_supply -> Int8,
        complete -> Bool,
    }
}

diesel::table! {
    global_account (id) {
        id -> Int4,
        pubkey -> Varchar,
        initialized -> Bool,
        #[max_length = 44]
        authority -> Varchar,
        #[max_length = 44]
        fee_recipient -> Varchar,
        initial_virtual_token_reserves -> Int8,
        initial_virtual_sol_reserves -> Int8,
        initial_real_token_reserves -> Int8,
        token_total_supply -> Int8,
        fee_basis_points -> Int8,
    }
}

diesel::allow_tables_to_appear_in_same_query!(
    bonding_curve,
    global_account,
);
