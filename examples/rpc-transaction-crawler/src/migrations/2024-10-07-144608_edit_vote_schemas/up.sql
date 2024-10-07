-- Your SQL goes here
DROP TABLE IF EXISTS kv;

CREATE TABLE activities (
    activity_id SERIAL PRIMARY KEY,       
    signature VARCHAR(255) NOT NULL      
);
