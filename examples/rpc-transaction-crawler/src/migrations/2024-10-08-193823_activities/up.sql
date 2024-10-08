-- Your SQL goes here
CREATE TABLE activities (
    id SERIAL PRIMARY KEY,     
    activity_type VARCHAR(255) NOT NULL,
    signature VARCHAR(255) NOT NULL      
);