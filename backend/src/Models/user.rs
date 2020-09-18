use crate::schema::users;
use uuid::Uuid;
use crate::helpers::Usergroup;

#[derive(Queryable, Serialize)]
pub struct FetchUser {
    pub id: Uuid,
    pub email: String,
}

#[derive(Queryable, Deserialize)]
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