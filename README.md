# Dependencies:
- [PostgreSQL](https://www.postgresql.org/download/) - Required to run
- [Rust](https://www.rust-lang.org/tools/install) - For modifying backend
- [Docker](https://docs.docker.com/get-docker/) - For deploying

# How to
## Run
todo (Will be using docker)

## Modify back end or build from source
0. Install PostgreSQL and Rust
1. Follow steps for Modify Front End
2. Create a database (You can use pgAdmin4)
3. Create a `.env` file at the root, if you are a member of VGDSquared, ask on teams for the file 
   1. Add the `DATABASE_URL` field with your postgres database connection string
   2. Add all the `*_CLIENT_ID` and `*_CLIENT_SECRET` fields for Oauth. So far we have `GOOGLE` and `MICROSOFT` authentication
4. Run the migration using `sqlx migrate run` (use `cargo install sqlx-cli` if you don't have it)
5. Use `cargo run` at the root of the project to run it
6. Go to https://localhost:3000

## Package for docker
todo (Using docker)
