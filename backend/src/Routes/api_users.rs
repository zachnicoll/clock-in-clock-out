#[path = "../Models/user.rs"]
mod user;

use rocket_contrib::json::Json;
use rocket::http::Status;
use crate::DbConn;
use crate::misc_helpers::*;
use crate::jwt_helpers::*;
use crate::db_schema::users;
use user::{PostUser, FetchUser, User};
use bcrypt::{hash, verify, DEFAULT_COST};
use diesel::prelude::*;
use chrono::{Utc, Duration};
use jsonwebtoken::{encode,  Header, EncodingKey};

use uuid::Uuid;

/*
    Route:      /api/users/<id>
    Method:     GET
    Info:       Gets a user based on <id> (Uuid) param
    Authorized: True
*/
#[get("/<id>")]
pub fn get_user(conn: DbConn, _jwt: JWT, id: UuidParam) -> ApiResponse {
    let user = users::table
        .filter(users::id.eq(id.uuid))
        .select((users::id, users::email))
        .first::<FetchUser>(&*conn);

    return match user {
        Ok(user) => ApiResponse {
            json: json!({ "user": user }),
            status: Status::Ok,
        },
        Err(_) => ApiResponse {
            json: json!({"error" : "true", "message" : "User does not exist!"}),
            status: Status::BadRequest,
        },
    };
}

/*
    Route:      /api/users
    Method:     POST
    Info:       Creates a user with given ID and password. Password is hashed before storage
    Authorized: False
*/
#[post("/", format = "json", data = "<new_user>")]
pub fn create_user(conn: DbConn, new_user: Json<PostUser>) -> ApiResponse {
    let hashed_password = hash(new_user.0.password, DEFAULT_COST);

    return match hashed_password {
        Ok(hash) => {
            // Generate UUID before storing in DB
            let user = User {
                id: Uuid::new_v4(),
                email: new_user.0.email,
                password: hash,
                user_group: Usergroup::User
            };

            println!("Attempting to insert user: {:?}", user);

            let result = diesel::insert_into(users::table)
                .values(user)
                .returning((users::id, users::email))
                .get_result::<FetchUser>(&*conn);

            return match result {
                Ok(result) => ApiResponse {
                    json: json!({ "user" : result }),
                    status: Status::Ok,
                },
                Err(e) => {
                    println!("Creating user failed: {:?}", e);
                    ApiResponse {
                        json: json!({
                            "error" : "true",
                            "message" : "Email already exists!"
                        }),
                        status: Status::BadRequest,
                    }
                },
            };
        }
        Err(_) => ApiResponse {
            json: json!({
                "error" : "true",
                "message" : "Could not process credentials."
            }),
            status: Status::BadRequest,
        },
    };
}

/*
    Route:      /api/users/login
    Method:     POST
    Info:       Provides user with JWT token if their credentials are correct
    Authorized: False
*/
#[post("/login", format = "json", data = "<credentials>")]
pub fn login(conn: DbConn, credentials: Json<PostUser>) -> ApiResponse {
    let user = users::table
        .filter(users::email.eq(&credentials.email))
        .select((users::id, users::email, users::password, users::user_group))
        .first::<User>(&*conn);

    return match user {
        Ok(user) => match verify(&credentials.password, &user.password) {
            Ok(verified) => {
                if verified {
                    let expiry = Utc::now() + Duration::days(1);
                    let claim = Claims {
                        aud: credentials.0.email,
                        iss: String::from("www.cico.com.au"),
                        exp: expiry.timestamp(),
                        grp: Usergroup::User
                    };

                    let token = encode(&Header::default(), &claim, &EncodingKey::from_secret(JWT_SECRET.as_ref()));
                    let user_info = FetchUser {
                        id: user.id,
                        email: user.email
                    };

                    ApiResponse {
                        json: json!({
                            "token" : token.unwrap(),
                            "user" : user_info
                        }),
                        status: Status::Ok,
                    }
                } else {
                    ApiResponse {
                        json: json!({
                            "error" : "true",
                            "message" : "Incorrect credentials."
                        }),
                        status: Status::Unauthorized,
                    }
                }
            }
            Err(_) => ApiResponse {
                json: json!({
                    "error" : "true",
                    "message" : "Something went wrong verifying the credentials."
                }),
                status: Status::BadRequest,
            },
        },
        Err(_) => ApiResponse {
            json: json!({
                "error" : "true",
                "message" : "Incorrect credentials."
            }),
            status: Status::Unauthorized,
        },
    };
}