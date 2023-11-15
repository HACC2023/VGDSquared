-- NOTE: If this app actually deploys we are going to be using redis, but for now PostgreSQL is good enough
CREATE TABLE refresh_token (
    id SERIAL PRIMARY KEY,
    token VARCHAR(255) NOT NULL
);
