#[path = "../Models/task.rs"]
mod task;

use crate::auth::*;
use crate::db_schema::tasks;
use crate::misc::*;
use crate::DbConn;
use chrono::{NaiveDateTime, Utc};
use diesel::prelude::*;
use rocket::http::Status;
use rocket_contrib::json::Json;
use task::{PostTask, Task};
use uuid::Uuid;

/*
    Route:      /api/tasks
    Method:     POST
    Info:       Create a Task given owner_id, duration, start, and optional label attribute.
    Authorized: True
*/
#[post("/", format = "json", data = "<new_task>")]
pub fn create_task(conn: DbConn, _authed: Auth, new_task: Json<PostTask>) -> ApiResponse {
    let task = Task {
        id: Uuid::new_v4(),
        owner_id: new_task.0.owner_id,
        duration: new_task.0.duration,
        start: new_task.0.start,
        label: new_task.0.label,
    };

    let result = diesel::insert_into(tasks::table)
        .values(task)
        .returning((
            tasks::id,
            tasks::owner_id,
            tasks::duration,
            tasks::start,
            tasks::label,
        ))
        .get_result::<Task>(&*conn);

    return match result {
        Ok(result) => ApiResponse {
            json: json!({ "task": result }),
            status: Status::Ok,
        },
        Err(e) => {
            return match e {
                diesel::result::Error::DatabaseError(_, _) => ApiResponse {
                    json: json!({
                        "error" : "true",
                        "message" : "Malformed task object, make sure all attributes are correct!"
                    }),
                    status: Status::BadRequest,
                },
                _ => ApiResponse {
                    json: json!({
                        "error" : "true",
                        "message" : "Failed to create task!"
                    }),
                    status: Status::BadRequest,
                },
            };
        }
    };
}

/*
    Route:      /api/tasks/<user_id>?start_date=2020-05-23T00:00:00&end_date=2020-06-23T00:00:00
    Method:     GET
    Info:       Get all tasks for a given user_id between a start_date and end_date.
                start_date OR end_date must be specified, OR both.
                If no start_date is specified, all tasks up until end_date are returned.
                If no end_date is specified, all tasks from start_date are returned.
                Dates must be formated like "YYYY-MM-DDTHH:ss:ss" and should be in UTC time.
    Authorized: True
*/
#[get("/<user_id>?<start_date>&<end_date>")]
pub fn get_task_date(
    conn: DbConn,
    _authed: Auth,
    user_id: UuidParam,
    start_date: Option<Result<NaiveDateForm, &rocket::http::RawStr>>,
    end_date: Option<Result<NaiveDateForm, &rocket::http::RawStr>>,
) -> ApiResponse {
    let mut valid_dates = (true, true);
    let mut malformed = false;
    let mut task_query = tasks::table
        .select((
            tasks::id,
            tasks::owner_id,
            tasks::duration,
            tasks::start,
            tasks::label,
        ))
        .filter(tasks::owner_id.eq(user_id.uuid))
        .into_boxed();

    // Set the default start datetime to a very early arbitrary date
    // Set the default end datetime to right now
    let mut start: NaiveDateTime =
        NaiveDateTime::parse_from_str("2000-01-01T00:00:00", "%Y-%m-%dT%H:%M:%S").unwrap();
    let mut end: NaiveDateTime = Utc::now().naive_utc();

    // If a start_date exists, set the start variable to it
    // Otherwise, invalidate the first of the valid_dates tuple
    match start_date {
        Some(start_date) => {
            match start_date {
                Ok(start_date) => start = *start_date,
                Err(_) => malformed = true, // Date must be malformed if resulting in Err
            }
        }
        None => valid_dates.0 = false,
    };

    // If an end_start exists, set the end variable to it
    // Otherwise, invalidate the second of the valid_dates tuple
    match end_date {
        Some(end_date) => {
            match end_date {
                Ok(end_date) => end = *end_date,
                Err(_) => malformed = true, // Date must be malformed if resulting in Err
            }
        }
        None => valid_dates.1 = false,
    };

    // If neither of the dates exists, return an error
    if malformed || valid_dates == (false, false) {
        return ApiResponse {
            json: json!({
                "error"     : "true",
                "message"   : "Missing start_date OR end_date, or dates are not in \'YYYY-MM-DDTHH:mm:ss\' format."
            }),
            status: Status::BadRequest,
        };
    }

    // Get all tasks between start and end date
    task_query = task_query.filter(tasks::start.ge(start));
    task_query = task_query.filter(tasks::start.le(end));

    // Order by earliest first
    let task_arr = task_query.order(tasks::start).load::<Task>(&*conn);

    return match task_arr {
        Ok(task_arr) => ApiResponse {
            json: json!({ "tasks": task_arr }),
            status: Status::Ok,
        },
        Err(_) => ApiResponse {
            json: json!({
                "error" : "true", 
                "message" : "Could not fetch tasks!"}),
            status: Status::BadRequest,
        },
    };
}

/*
    Route:      /api/tasks/<user_id>/<task_id>
    Method:     GET
    Info:       Get a Task for a given user_id, with ID task_id.
    Authorized: True
*/
#[get("/<user_id>/<task_id>")]
pub fn get_task(
    conn: DbConn,
    _authed: Auth,
    user_id: UuidParam,
    task_id: UuidParam,
) -> ApiResponse {
    let task_query = tasks::table
        .select((
            tasks::id,
            tasks::owner_id,
            tasks::duration,
            tasks::start,
            tasks::label,
        ))
        .filter(tasks::owner_id.eq(user_id.uuid))
        .filter(tasks::id.eq(task_id.uuid))
        .first::<Task>(&*conn);

    match task_query {
        Ok(task_query) => ApiResponse {
            json: json!({ "task": task_query }),
            status: Status::Ok,
        },
        Err(_) => ApiResponse {
            json: json!({
                "error"     : "true",
                "message"   : "Could not fetch task."
            }),
            status: Status::Ok,
        },
    }
}
