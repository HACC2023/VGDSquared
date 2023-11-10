ALTER TABLE account
DROP CONSTRAINT IF EXISTS fk_account_auth;
DROP TABLE auth;
DROP TABLE account;
