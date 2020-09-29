# Clock In/Clock Out

http://clockinout.net

## Description
 An application targetted at business owners, team leads, project mangers and corporations that operate via a ticketing systems, like Jira, and need to keep track of their employee's tasks throughout the day. This tool is useful for centralising time tracking on tickets and gaining statistics on employees like utilisation and averages in certain categories of work.

## Setting Up

### Environment Variables
This project uses `docker-compose` for development and deployment. Before starting the containers, you'll need some environment variables to make them work.

In the root directory, create a file named `docker.env` containing the following values:
 ```
POSTGRES_PASSWORD=password                  // Change this to desired password
POSTGRES_USER=user                          // Change this to desired username
POSTGRES_DB=cico                            // LEAVE THIS
PGADMIN_DEFAULT_EMAIL=admin@example.com     // Change this to desired email (can be fake)
PGADMIN_DEFAULT_PASSWORD=pgadminpassword    // Change to desired password
 ```

 In the `backend` directory, create a file named `.env` containing the following values:
```
DATABASE_URL=postgres://user:password@postgres-dev/cico
JWT_SECRET=thisIsAnExampleJWTsecret_UwU

```
Make sure that `user`and `password` are the same as the `POSTGRES_USER` and `POSTGRES_PASSWORD` defined in `docker.env`. The rest of `DATABASE_URL` should be left the same.

### Rust Tools
Firstly, install Rust, `rust-up` and `cargo` with:

`curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh`
`export PATH=PATH:~/.cargo/bin`

Make sure you selected `nightly` rust with `modify PATH vairable: yes`:
```
    default host triple: x86_64-unknown-linux-gnu
    default toolchain: nightly
    profile: default
    modify PATH variable: yes
```
## Development

### docker-compose
The development environment can be run with:

`docker-compose up -d` (you may need to use `sudo`)

Changes in both the `frontend` and `backend` folders will hot-reload, so you don't need to restart the containers every change.

### Database Migrations
You will need to apply database migrations through `diesel-cli`. This is an ORM tool written in Rust and used to handle the database connections and queries in the Rust API (`backend`). Install diesel with:

`cargo install diesel_cli --no-default-features --features postgres`

To apply migrations to the `postgres-dev` container, `cd backend` and run the following command:

`diesel --database-url postgres://user:password@localhost:5432/cico migration run`

Revert the last migration with:

`diesel --database-url postgres://user:password@localhost:5432/cico migration revert`

And create a new migration with:

`diesel migration generate migration_name`

## Deployment
The site is currently deployed at http://clockinout.net, running on a Linode server. To run the "production" build of the app, use:

`docker-compose -f docker-compose-prod.yml up -d --build`

This is not necessary for normal development.
