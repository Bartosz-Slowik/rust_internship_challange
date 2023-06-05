-- Your SQL goes here
CREATE TABLE search_requests (
    id UUID PRIMARY KEY,
    api_key TEXT NOT NULL,
    search_string TEXT NOT NULL,
    successful BOOL NOT NULL
);