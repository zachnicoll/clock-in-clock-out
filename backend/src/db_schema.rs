/* 
    Diesel overwrites every schema generation, wiping out Enum definitions.
    This is the safe file and contains imports that Diesel cannot imply iteself.
    --> THIS IS THE SCHEMA IMPORTED IN THE PROJECT <--
*/

table! {
    tags (id) {
        id -> Uuid,
        owner_id -> Uuid,
        label -> Text,
        is_generic -> Bool,
    }
}

table! {
    task_tag (id) {
        id -> Int4,
        task_id -> Nullable<Uuid>,
        tag_id -> Nullable<Uuid>,
    }
}

table! {
    tasks (id) {
        id -> Uuid,
        owner_id -> Uuid,
        duration -> Int4,
        start -> Timestamp,
        label -> Nullable<Text>,
    }
}

table! {
    use diesel::sql_types::Text;
    use diesel::sql_types::Uuid;
    use crate::misc_helpers::UsergroupMapping;
    users (id) {
        id -> Uuid,
        email -> Text,
        password -> Text,
        user_group -> UsergroupMapping,
    }
}

joinable!(tags -> users (owner_id));
joinable!(task_tag -> tags (tag_id));
joinable!(task_tag -> tasks (task_id));
joinable!(tasks -> users (owner_id));

allow_tables_to_appear_in_same_query!(
    tags,
    task_tag,
    tasks,
    users,
);
