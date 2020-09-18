extern crate diesel_derive_enum;
use rocket_contrib::json::{JsonValue};
use rocket::request::{Request,FromParam, FromRequest};
use rocket::request;
use rocket::response;
use rocket::response::{Responder, Response};
use rocket::http::{ContentType, Status, RawStr};
use rocket::Outcome;
use diesel_derive_enum::*;

use jsonwebtoken::{decode,  Validation, DecodingKey, Algorithm};

use uuid::Uuid;

pub const JWT_SECRET: &str = dotenv!("JWT_SECRET");

#[derive(Debug, Serialize, Deserialize, DbEnum)]
pub enum Usergroup {
    User,
    Leader,
    Admin
}

// Claim struct for JWT token creation
#[derive(Debug, Serialize, Deserialize)]
pub struct Claims {
    pub aud: String,
    pub iss: String,
    pub exp: i64,
    pub grp: Usergroup
}

// Generic struct for API Responses
// Contains json in the body and status in the header
#[derive(Debug)]
pub struct ApiResponse {
    pub json: JsonValue,
    pub status: Status,
}

// Implement Responder trait for auto-construction of proper ApiResponse
// i.e. insert header status and body content
impl<'r> Responder<'r> for ApiResponse {
    fn respond_to(self, req: &Request) -> response::Result<'r> {
        Response::build_from(self.json.respond_to(&req).unwrap())
            .status(self.status)
            .header(ContentType::JSON)
            .ok()
    }
}

// Struct for implementing rocket's FromParam for Uuids
pub struct UuidParam {
    pub uuid: Uuid,
}

// Implement FromParam for Uuids so a Uuid can be used to query endpoints
impl<'r> FromParam<'r> for UuidParam{
    type Error = &'r RawStr;

    fn from_param(param: &'r RawStr) -> Result<Self, Self::Error>{
        let id = Uuid::parse_str(param);

        return match id {
            Ok(id) => Ok(UuidParam {
                uuid: id
            }),
            Err(_) => return Err(param)
        }
    }
}

// Tuple to hold JWT Token in signed form
pub struct JWT(String);

/// Returns true if `key` is a valid API key string.
fn is_valid(jwt: &str) -> bool {
    let bearer = &jwt[..7];

    if bearer != "Bearer " {
        false
    }
    else{
        let token = &jwt[7..];
        let token_message = decode::<Claims>(&token, &DecodingKey::from_secret(JWT_SECRET.as_ref()), &Validation::new(Algorithm::HS256));

        println!("Decoded JWT: {:?}", token_message);

        return match token_message {
            Ok(_msg) => true,
            Err(_) => false
        }
    }
}

#[derive(Debug)]
pub enum JWTError {
    Missing,
    Invalid,
}

impl<'a, 'r> FromRequest<'a, 'r> for JWT{
    type Error = JWTError;

    fn from_request(request: &'a Request<'r>) -> request::Outcome<Self, Self::Error> {
        let jwt: Vec<_> = request.headers().get("Authorization").collect();
        match jwt.len() {
            0 => Outcome::Failure((Status::BadRequest, JWTError::Missing)),
            1 if is_valid(jwt[0]) => Outcome::Success(JWT(jwt[0].to_string())),
            _ => Outcome::Failure((Status::Unauthorized, JWTError::Invalid)),
        }
    }
}