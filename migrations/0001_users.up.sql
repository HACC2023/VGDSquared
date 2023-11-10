CREATE TABLE account (
    id SERIAL PRIMARY KEY,
    nickname VARCHAR(50),
    auth_id INT NOT NULL
);
CREATE TABLE auth (
    id SERIAL PRIMARY KEY,
    -- As far as I am aware, the google sub field in the JWT is 21 digits long
    google_sub CHAR(21) UNIQUE
);
ALTER TABLE account
ADD CONSTRAINT fk_account_auth FOREIGN KEY (auth_id) REFERENCES auth (id);