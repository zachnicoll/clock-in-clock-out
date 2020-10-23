#![feature(proc_macro_hygiene, decl_macro)]

#[macro_use] extern crate diesel;
#[macro_use] extern crate rocket;
#[macro_use] extern crate rocket_contrib;
#[macro_use] extern crate serde_derive;
extern crate serde;
extern crate serde_json;
extern crate redis;
extern crate dotenv;
use dotenv::dotenv;
#[macro_use]
extern crate dotenv_codegen;

use diesel::PgConnection;
use diesel::r2d2::{ConnectionManager, Pool, PooledConnection};

use std::ops::Deref;
use rocket::http::Status;
use rocket::request::{self, FromRequest};
use rocket::{Request, State, Outcome};

// An alias to the type for a pool of Diesel Pg connections.
type PgPool = Pool<ConnectionManager<PgConnection>>;

// The URL to the database, set via the `DATABASE_URL` environment variable.
static DATABASE_URL: &'static str = dotenv!("DATABASE_URL");

/// Initializes a database pool.
fn init_pool() -> PgPool {
    let manager = ConnectionManager::<PgConnection>::new(DATABASE_URL);
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
            Err(_) => Outcome::Failure((Status::ServiceUnavailable, ()))
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

mod db_schema;
#[path = "./Helpers/misc.rs"]
mod misc;
#[path = "./Helpers/jwt.rs"]
mod jwt;
#[path = "Helpers/redis_helpers.rs"]
mod redis_helpers;
#[path = "./Routes/api_users.rs"]
mod api_users;
#[path = "./Routes/api_tasks.rs"]
mod api_tasks;

use api_users::*;
use api_tasks::*;

// Using this endpoint as the "check" to see if the server is running
#[get("/")]
fn hello() -> &'static str {
    "Hello, world!"
}

fn main() {
    dotenv().ok();

    rocket::ignite()
        .mount("/api/hello", routes![hello])
        .mount("/api/users", routes![get_user, create_user, login])
        .mount("/api/tasks", routes![create_task, get_task, get_task_date])
        .manage(init_pool())
        .launch();
}
