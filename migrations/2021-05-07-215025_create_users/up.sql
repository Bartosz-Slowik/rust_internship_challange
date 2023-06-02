CREATE EXTENSION IF NOT EXISTS "uuid-ossp";

-- Do not change anything in this file

CREATE TABLE users (
    id uuid PRIMARY KEY DEFAULT uuid_generate_v4() NOT NULL,
    email text UNIQUE NOT NULL,
    api_key text NOT NULL
);
