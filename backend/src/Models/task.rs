use crate::db_schema::tasks;
use uuid::Uuid;
use crate::misc_helpers::json_time;
use chrono::NaiveDateTime;


#[derive(Debug, Insertable, Queryable, Deserialize, Serialize)]
#[table_name = "tasks"]
pub struct Task {
    pub id: Uuid,
    pub owner_id: Uuid,
    pub duration: i32,
    #[serde(with = "json_time")]
    pub start: NaiveDateTime,
    pub label: Option<String>
}

#[derive(Debug, Queryable, Deserialize)]
pub struct PostTask {
    pub owner_id: Uuid,
    pub duration: i32,
    #[serde(with = "json_time")]
    pub start: NaiveDateTime,
    pub label: Option<String>
}