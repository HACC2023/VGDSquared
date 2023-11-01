# Dependencies:
(PostgreSQL)[https://www.postgresql.org/download/] - Required to run
(Rust)[https://www.rust-lang.org/tools/install] - For modifying backend
Docker - For deploying

# How to
## Run
todo (Will be using docker)

## Modify Front End (Windows only)
0. Install PostgreSQL
1. Create a database with the name `HACC` using pgadmin4 (TODO Make this easier to do)
   1. Have the database name to be `postgres` and `spartechs` for the password if you don't want to modify the `.env` file
2. Run the executable (at the root of the project)
3. Go to https://localhost:3000

## Modify back end or build from source
0. Install PostgreSQL and Rust
1. Follow steps for Modify Front End
2. Use `cargo run` at the root of the project

## Package for docker
todo (Using docker)
