CREATE TABLE global_account (
    id SERIAL PRIMARY KEY,
    pubkey VARCHAR NOT NULL,
    initialized BOOLEAN NOT NULL,
    authority VARCHAR(44) NOT NULL,
    fee_recipient VARCHAR(44) NOT NULL,
    initial_virtual_token_reserves BIGINT NOT NULL,
    initial_virtual_sol_reserves BIGINT NOT NULL,
    initial_real_token_reserves BIGINT NOT NULL,
    token_total_supply BIGINT NOT NULL,
    fee_basis_points BIGINT NOT NULL
);


CREATE TABLE bonding_curve (
    id SERIAL PRIMARY KEY,
    pubkey VARCHAR NOT NULL,
    virtual_token_reserves BIGINT NOT NULL,
    virtual_sol_reserves BIGINT NOT NULL,
    real_token_reserves BIGINT NOT NULL,
    real_sol_reserves BIGINT NOT NULL,
    token_total_supply BIGINT NOT NULL,
    complete BOOLEAN NOT NULL
);
