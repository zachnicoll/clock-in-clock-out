#![feature(proc_macro_hygiene, decl_macro)]

#[macro_use]
extern crate diesel;
#[macro_use]
extern crate rocket;
#[macro_use]
extern crate rocket_contrib;
#[macro_use]
extern crate serde_derive;
extern crate serde;
extern crate serde_json;

use serde_derive::{Deserialize, Serialize};

mod schema;

use rocket::http::ContentType;
use rocket::http::Status;
use rocket::request::Request;
use rocket::response;
use rocket::response::{Responder, Response};
use rocket_contrib::databases::{database, diesel::PgConnection};
use rocket_contrib::json::{Json, JsonValue};

use crate::schema::users;
use bcrypt::{hash, verify, DEFAULT_COST};
use diesel::prelude::*;
use diesel::{Insertable, QueryDsl, Queryable};

#[database("postgres")]
struct DbConn(PgConnection);

#[derive(Queryable, Serialize)]
struct FetchUser {
    id: i32,
    email: String,
}

#[derive(Insertable, Queryable, Deserialize)]
#[table_name = "users"]
struct PostUser {
    email: String,
    password: String,
}

#[derive(Debug)]
struct ApiResponse {
    json: JsonValue,
    status: Status,
}

impl<'r> Responder<'r> for ApiResponse {
    fn respond_to(self, req: &Request) -> response::Result<'r> {
        Response::build_from(self.json.respond_to(&req).unwrap())
            .status(self.status)
            .header(ContentType::JSON)
            .ok()
    }
}

#[get("/")]
fn hello() -> &'static str {
    "Hello, world!"
}

#[post("/", format = "json", data = "<new_user>")]
fn create_user(conn: DbConn, new_user: Json<PostUser>) -> ApiResponse {
    let hashed_password = hash(new_user.0.password, DEFAULT_COST);

    return match hashed_password {
        Ok(hash) => {
            let user = PostUser {
                email: new_user.0.email,
                password: hash,
            };

            let result = diesel::insert_into(users::table)
                .values(user)
                .returning((users::id, users::email))
                .get_result::<FetchUser>(&*conn);
            return match result {
                Ok(result) => ApiResponse {
                    json: json!({ "user": result }),
                    status: Status::Ok,
                },
                Err(_) => ApiResponse {
                    json: json!({"error":"Failed to create user.", "message":"Email already exists!"}),
                    status: Status::BadRequest,
                },
            };
        }
        Err(_) => ApiResponse {
            json: json!({"error":"Failed to create user.", "message":"Could not process credentials."}),
            status: Status::BadRequest,
        },
    };
}

#[post("/login", format = "json", data = "<credentials>")]
fn login(conn: DbConn, credentials: Json<PostUser>) -> ApiResponse {
    let user = users::table
        .filter(users::email.eq(&credentials.email))
        .select((users::email, users::password))
        .first::<PostUser>(&*conn);

    return match user {
        Ok(user) => match verify(&credentials.password, &user.password) {
            Ok(verified) => {
                if verified {
                    ApiResponse {
                        json: json!({"message" : "User authenticated."}),
                        status: Status::Ok,
                    }
                } else {
                    ApiResponse {
                        json: json!({"error":"Unauthorized user.","message" : "Incorrect credentials."}),
                        status: Status::Unauthorized,
                    }
                }
            }
            Err(_) => ApiResponse {
                json: json!({"error":"Could not authenticate user.","message" : "Something went wrong verifying the credentials."}),
                status: Status::BadRequest,
            },
        },
        Err(_) => ApiResponse {
            json: json!({"error":"Unauthorized user.","message" : "Incorrect credentials."}),
            status: Status::Unauthorized,
        },
    };
}

#[get("/")]
fn get_users(conn: DbConn) -> ApiResponse {
    let users = users::table
        .select((users::id, users::email))
        .order(users::columns::id.desc())
        .load::<FetchUser>(&*conn);

    return match users {
        Ok(users) => ApiResponse {
            json: json!({ "users": users }),
            status: Status::Ok,
        },
        Err(_) => ApiResponse {
            json: json!({"error":"Could not fetch users!"}),
            status: Status::BadRequest,
        },
    };
}

fn main() {
    rocket::ignite()
        .attach(DbConn::fairing())
        .mount("/api/hello", routes![hello])
        .mount("/api/users", routes![get_users, create_user, login])
        .launch();
}
