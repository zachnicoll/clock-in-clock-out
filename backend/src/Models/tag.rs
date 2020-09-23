use crate::db_schema::tags;
use uuid::Uuid;

#[derive(Debug, Insertable, Queryable, Deserialize, Serialize)]
#[table_name = "tags"]
pub struct Tag {
    pub id: Uuid,
    pub owner_id: Uuid,
    pub label: Opion<String>,
    pub is_generic: Bool
}