#![feature(proc_macro_hygiene, decl_macro)]

#[macro_use]
extern crate diesel;
#[macro_use]
extern crate rocket;
#[macro_use]
extern crate rocket_contrib;
#[macro_use]
extern crate serde_derive;
extern crate dotenv;
extern crate redis;
extern crate serde;
extern crate serde_json;
#[macro_use]
extern crate dotenv_codegen;

use dotenv::dotenv;

use diesel::r2d2::{ConnectionManager, Pool, PooledConnection};
use diesel::PgConnection;

use rocket::http::{Method, Status};
use rocket::request::{self, FromRequest};
use rocket::{Outcome, Request, State};
use std::ops::Deref;

use rocket_cors::{
    AllowedHeaders,
    AllowedOrigins,
    Cors,
    CorsOptions,
};

// If we are running dev mode or not
pub static IS_DEV: &'static str = dotenv!("DEV");

// An alias to the type for a pool of Diesel Pg connections.
type PgPool = Pool<ConnectionManager<PgConnection>>;

/// Initializes a database pool.
fn init_pool() -> PgPool {
    println!("Currently running with IS_DEV: {}", IS_DEV);

    let db_url = match IS_DEV {
        "1" => format!(
            "postgres://{}:{}@{}/{}",
            dotenv!("POSTGRES_USER_DEV"),
            dotenv!("POSTGRES_PASSWORD_DEV"),
            dotenv!("DATABASE_IP"),
            dotenv!("POSTGRES_DB_DEV")
        ),
        _ => format!(
            "postgres://{}:{}@{}/{}",
            dotenv!("POSTGRES_USER_PROD"),
            dotenv!("POSTGRES_PASSWORD_PROD"),
            dotenv!("DATABASE_IP"),
            dotenv!("POSTGRES_DB_PROD")
        ),
    };

    let manager = ConnectionManager::<PgConnection>::new(db_url);
    Pool::new(manager).expect("db pool")
}

// Connection request guard type: a wrapper around an r2d2 pooled connection.
pub struct DbConn(pub PooledConnection<ConnectionManager<PgConnection>>);

/// Attempts to retrieve a single connection from the managed database pool. If
/// no pool is currently managed, fails with an `InternalServerError` status. If
/// no connections are available, fails with a `ServiceUnavailable` status.
impl<'a, 'r> FromRequest<'a, 'r> for DbConn {
    type Error = ();

    fn from_request(request: &'a Request<'r>) -> request::Outcome<Self, Self::Error> {
        let pool = request.guard::<State<PgPool>>()?;
        match pool.get() {
            Ok(conn) => Outcome::Success(DbConn(conn)),
            Err(_) => Outcome::Failure((Status::ServiceUnavailable, ())),
        }
    }
}

// For the convenience of using an &DbConn as an &PgConnection.
impl Deref for DbConn {
    type Target = PgConnection;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

#[path = "./Routes/api_tasks.rs"]
mod api_tasks;
#[path = "./Routes/api_users.rs"]
mod api_users;
#[path = "Helpers/auth.rs"]
mod auth;
mod db_schema;
#[path = "./Helpers/misc.rs"]
mod misc;
#[path = "Helpers/redis_helpers.rs"]
mod redis_helpers;

use api_tasks::*;
use api_users::*;

fn make_cors() -> Cors {
    let allowed_origins = AllowedOrigins::some_exact(&[
        "http://localhost:3000",
        "https://localhost:3000",
        "http://localhost:8000",
        "https://localhost:8000",
        "http://localhost:80",
        "https://localhost:443",
        "http://clockinout.net",
        "https://clockinout.net",
    ]);

    CorsOptions {
        // 5.
        allowed_origins,
        allowed_methods: vec![
            Method::Get,
            Method::Post,
            Method::Put,
            Method::Delete,
            Method::Options,
        ]
        .into_iter()
        .map(From::from)
        .collect(), // 1.
        allowed_headers: AllowedHeaders::some(&[
            "Authorization",
            "Accept",
            "Access-Control-Allow-Origin",
            "Content-Type",
            "DNT",
            "Referer",
            "User-Agent",
        ]),
        allow_credentials: true,
        ..Default::default()
    }
    .to_cors()
    .expect("error while building CORS")
}

// Using this endpoint as the "check" to see if the server is running
#[get("/")]
fn hello() -> &'static str {
    "Hello, world!"
}

fn main() {
    dotenv().ok();

    rocket::ignite()
        .mount("/api/hello", routes![hello])
        .mount("/api/users", routes![login, get_user, create_user])
        .mount("/api/tasks", routes![create_task, get_task, get_task_date])
        .manage(init_pool())
        .attach(make_cors())
        .launch();
}
