CREATE TABLE auth (
    id SERIAL PRIMARY KEY,
    provider INT NOT NULL, -- Handle the type in the code
    subject VARCHAR(64) UNIQUE,
    account_id SERIAL REFERENCES account (id)
);