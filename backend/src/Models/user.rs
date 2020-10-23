use crate::db_schema::users;
use uuid::Uuid;
use crate::misc::Usergroup;

#[derive(Debug, Queryable, Serialize)]
pub struct FetchUser {
    pub id: Uuid,
    pub email: String,
}

#[derive(Debug, Queryable, Deserialize)]
pub struct PostUser {
    pub email: String,
    pub password: String,
}

#[derive(Debug, Insertable, Queryable, Deserialize)]
#[table_name = "users"]
pub struct User {
    pub id: Uuid,
    pub email: String,
    pub password: String,
    pub user_group: Usergroup
}