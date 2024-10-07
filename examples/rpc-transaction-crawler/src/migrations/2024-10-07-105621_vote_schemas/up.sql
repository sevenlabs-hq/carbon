-- Your SQL goes here
CREATE TABLE votes (
    vote_id VARCHAR(44) PRIMARY KEY,
    authority VARCHAR(44) NOT NULL,
    yes INTEGER DEFAULT 0,
    no INTEGER DEFAULT 0
);


CREATE TABLE vote_entries (
    voter_id VARCHAR(44) NOT NULL,
    vote_id VARCHAR(44) NOT NULL,
    choice BOOLEAN NOT NULL,
    PRIMARY KEY (voter_id, vote_id),
    FOREIGN KEY (vote_id) REFERENCES votes(vote_id) ON DELETE CASCADE
);

CREATE TABLE kv (
    key VARCHAR PRIMARY KEY,
    value VARCHAR NOT NULL
);
