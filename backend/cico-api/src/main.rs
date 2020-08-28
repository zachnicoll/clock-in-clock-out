#![feature(proc_macro_hygiene, decl_macro)]

#[macro_use] 
extern crate diesel;

extern crate bcrypt;

mod schema;

use rocket::{self, get, routes, post, put};
use crate::schema::users;
use serde_derive::{Serialize, Deserialize};
use rocket_contrib::json::Json;
use rocket_contrib::databases::{database, diesel::PgConnection};
use diesel::{Insertable, Queryable, QueryDsl, select};
use diesel::prelude::*;
use bcrypt::{DEFAULT_COST, hash, verify};


#[database("postgres")]
struct DbConn(PgConnection);

#[derive(Queryable, Serialize)]
struct FetchUser {
    id: i32,
    email: String,
}

#[derive(Insertable, Deserialize)]
#[table_name="users"]
struct PostUser {
    email: String,
    password: String
}


#[get("/")]
fn hello() -> &'static str {
    "Hello, world!"
}

#[post("/", format="json", data = "<new_user>")]
fn create_user(conn:DbConn, new_user:Json<PostUser>) -> Json<FetchUser> {
    let hashed_password = hash(new_user.0.password, DEFAULT_COST).unwrap();

    let user = PostUser {
        email: new_user.0.email,
        password: hashed_password
    };
    
    let result = diesel::insert_into(users::table)
    .values(user)
    .returning((users::id, users::email))
    .get_result::<FetchUser>(&*conn)
    .unwrap();

    let secure_result = FetchUser {
        id: result.id,
        email: result.email
    };

    Json(secure_result)
}

#[get("/")]
fn get_users(conn:DbConn) -> Json<Vec<FetchUser>> {
    let users = users::table
    .select((users::id, users::email))
    .order(users::columns::id.desc())
    .load::<FetchUser>(&*conn)
    .unwrap();

    Json(users)
}

fn main() {
    rocket::ignite()
    .attach(DbConn::fairing())
    .mount("/hello", routes![hello])
    .mount("/users", routes![get_users, create_user])
    .launch();
}