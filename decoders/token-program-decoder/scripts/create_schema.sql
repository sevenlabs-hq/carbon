/**
 * Script for creating the schema for PostgreSQL used for the Carbon Token Decoder.
 */

CREATE TYPE "AccountState" AS ENUM (
    'Uninitialized',
    'Initialized',
    'Frozen'
);

CREATE TABLE token (
    mint BYTEA PRIMARY KEY,
    owner BYTEA NOT NULL,
    amount BIGINT NOT NULL,
    delegate BYTEA,
    state AccountState,
    is_native BIGINT,
    delegated_amount BIGINT,
    close_authority BYTEA
);

CREATE TABLE mint (
    mint_authority BYTEA,
    supply BIGINT NOT NULL,
    decimals INT NOT NULL,
    is_initialized BOOLEAN NOT NULL,
    freeze_authority BYTEA
);
