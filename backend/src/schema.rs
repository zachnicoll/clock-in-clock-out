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
    users (id) {
        id -> Uuid,
        email -> Text,
        password -> Text,
        user_group -> USERGROUP,
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
