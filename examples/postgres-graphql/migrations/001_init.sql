CREATE TABLE IF NOT EXISTS accounts (
    __pubkey BYTEA PRIMARY KEY,
    __slot   BIGINT,
    data     JSONB
);

CREATE INDEX IF NOT EXISTS accounts_slot_idx ON accounts (__slot);
CREATE INDEX IF NOT EXISTS accounts_data_gin_idx ON accounts USING GIN (data);
