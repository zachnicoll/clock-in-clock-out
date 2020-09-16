use rocket_contrib::json::{JsonValue};
use rocket::request::{Request,FromParam};
use rocket::response;
use rocket::response::{Responder, Response};
use rocket::http::{ContentType, Status, RawStr};
use uuid::Uuid;


// Claim struct for JWT token creation
#[derive(Debug, Serialize, Deserialize)]
pub struct Claims {
    pub sub: String,
    pub iss: String,
    pub exp: i64,
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