extern crate diesel_derive_enum;
use rocket_contrib::json::{JsonValue};
use rocket::request::{Request, FromParam, FromFormValue};
use rocket::response;
use rocket::response::{Responder, Response};
use rocket::http::{ContentType, Status, RawStr};
use diesel_derive_enum::*;
use uuid::Uuid;
use std::ops::Deref;

use chrono::{DateTime, Utc, NaiveDateTime, NaiveDate};

#[derive(Debug, Serialize, Deserialize, DbEnum)]
pub enum Usergroup {
    User,
    Admin
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

pub struct NaiveDateForm(NaiveDateTime);

impl<'v> FromFormValue<'v> for NaiveDateForm {
    type Error = &'v RawStr;

    fn from_form_value(form_value: &'v RawStr) -> Result<NaiveDateForm, &'v RawStr> {
        let date = NaiveDateTime::parse_from_str(form_value, "%Y-%m-%dT%H:%M:%S");

        return match date {
            Ok(date) => Ok(NaiveDateForm(date)),
            _ => Err(form_value)
        }
    }
}

impl Deref for NaiveDateForm{
    type Target = NaiveDateTime;

    fn deref(&self)-> &Self::Target{
        &self.0
    }
}

pub mod json_time {
	use super::*;
    use serde::{Serialize, Serializer, Deserialize, Deserializer, de::Error};
    
    fn time_to_json(t: NaiveDateTime) -> String {
        DateTime::<Utc>::from_utc(t, Utc).to_rfc3339()
    }

	pub fn serialize<S: Serializer>(time: &NaiveDateTime, serializer: S) -> Result<S::Ok, S::Error> {
		time_to_json(time.clone()).serialize(serializer)
	}

    pub fn deserialize<'de, D: Deserializer<'de>>(deserializer: D) -> Result<NaiveDateTime, D::Error> {
		let time: String = Deserialize::deserialize(deserializer)?;
		Ok(DateTime::parse_from_rfc3339(&time).map_err(D::Error::custom)?.naive_utc())
	}
}