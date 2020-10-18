use rocket::request;
use rocket::request::{Request, FromRequest};
use rocket::http::Status;
use jsonwebtoken::{decode,  Validation, DecodingKey, Algorithm};
use rocket::Outcome;
use crate::misc_helpers::Usergroup;
pub const JWT_SECRET: &str = dotenv!("JWT_SECRET");

// Claim struct for JWT token creation
#[derive(Debug, Serialize, Deserialize)]
pub struct Claims {
    pub aud: String,
    pub iss: String,
    pub exp: i64,
    pub grp: Usergroup
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