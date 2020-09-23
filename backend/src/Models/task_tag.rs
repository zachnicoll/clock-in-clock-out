use crate::db_schema::task_tag;
use uuid::Uuid;

#[derive(Debug, Insertable, Queryable, Deserialize, Serialize)]
#[table_name = "task_tag"]
pub struct TaskTag {
    pub id: u32,
    pub task_id: Option<Uuid>,
    pub tag_id: Option<Uuid>
}