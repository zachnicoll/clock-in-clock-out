use rocket::request;
use rocket::request::{Request, FromRequest};
use rocket::http::Status;
use jsonwebtoken::{decode,  Validation, DecodingKey, Algorithm};
use rocket::Outcome;
use crate::misc::Usergroup;
use redis::Commands;
use crate::redis_helpers::{redis_conn};

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
pub struct JWT {
    is_valid: bool,
    claim: Option<Claims>
}

pub struct Auth {
    jwt: JWT,
    valid_session: bool
}

#[derive(Debug)]
pub enum AuthError {
    Missing,
    Invalid,
    CacheFailed,
    BadSession
}

/// Returns true if `key` is a valid API key string.
fn convert_to_jwt(jwt: &str) -> JWT {
    // Extract the string "Bearer " from the header
    let bearer = &jwt[..7];

    // If it doesn't equal exactly "Bearer ", then the headed is malformed
    if bearer != "Bearer " {
        JWT {
            is_valid: false,
            claim: None
        }
    }
    else{
        // Get the token from the Bearer string e.g. "Bearer abcde..."
        let token = &jwt[7..];

        // Decode the token for its data
        let token_data = decode::<Claims>(&token, &DecodingKey::from_secret(JWT_SECRET.as_ref()), &Validation::new(Algorithm::HS256));

        // Return JWT struct if it is correctly decoded
        return match token_data {
            Ok(token_data) => JWT {
                is_valid: true,
                claim: Some(token_data.claims)
            },
            Err(_) => JWT {
                is_valid: false,
                claim: None
            }
        }
    }
}

// Each route with an Auth-typed param will first go through this request guard
impl<'a, 'r> FromRequest<'a, 'r> for Auth{
    type Error = AuthError;

    fn from_request(request: &'a Request<'r>) -> request::Outcome<Self, Self::Error> {
        // Retrieve JWT string from Authorization header
        let jwt: Vec<_> = request.headers().get("Authorization").collect();

        match jwt.len() {
            0 => Outcome::Failure((Status::BadRequest, AuthError::Missing)), // If the vector has length 0, then the header doesn't exists
            1 if convert_to_jwt(jwt[0]).is_valid => {
                // Convert the JWT and extra the user's email
                let token = convert_to_jwt(jwt[0]);
                let user_email = match &token.claim {
                    Some(claim) => String::to_string(&claim.aud),
                    _ => "".to_string()
                };
                let cache_conn = redis_conn(); // Redis connection

                return match cache_conn {
                    Ok(mut conn) => {
                        // Get user's current recorded IP
                        let cached_ip = conn.get::<&String, String>(&user_email);

                        match cached_ip {
                            Ok(ip) => {
                                // If the IPs don't match, reject the request
                                // The user must re-login from the new IP in order to auth successfully
                                if ip != request.client_ip().unwrap().to_string() {
                                    Outcome::Failure((Status::Conflict, AuthError::BadSession))
                                }
                                else {
                                    Outcome::Success(Auth {
                                        jwt: token,
                                        valid_session: true
                                    })
                                }
                            },
                            Err(_) => Outcome::Failure((Status::Unauthorized, AuthError::BadSession)) // Not authed if IP not recorded
                        }
                    },
                    Err(_) => Outcome::Failure((Status::FailedDependency, AuthError::CacheFailed)) // Redis connection failure
                }
            },
            _ => Outcome::Failure((Status::Unauthorized, AuthError::Invalid)), // Invalid JWT
        }
    }
}