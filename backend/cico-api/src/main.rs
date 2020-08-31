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
use std::io::{self, Write};

#[database("postgres")]
struct DbConn(PgConnection);

#[derive(Queryable, Serialize)]
struct FetchUser {
    id: i32,
    email: String,
}

#[derive(Insertable, Queryable, Deserialize)]
#[table_name="users"]
struct PostUser {
    email: String,
    password: String
}

#[derive(Serialize)]
struct Response {
    status: String,
    message: String
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

#[post("/login", format="json", data="<credentials>")]
fn login(conn:DbConn, credentials:Json<PostUser>) -> Json<Response> {
    let user = users::table
    .filter(users::email.eq(&credentials.email))
    .select((users::email, users::password))
    .first::<PostUser>(&*conn);

    let mut status = "400";
    let mut message = "Incorrect credentials";

    let user = match user{
        Ok(user) => user,
        Err(error) => PostUser {email:"".to_string(), password:"".to_string()}
    };

    if user.email != "" {
        let verified : bool = verify(&credentials.password, &user.password).unwrap();
    
        if verified {
            message = "User authenticated";
            status = "200";
        }
    }

    let response = Response {
        status: String::from(status),
        message: String::from(message)
    };

    Json(response)
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
    .mount("/api/hello", routes![hello])
    .mount("/api/users", routes![get_users, create_user, login])
    .launch();
}