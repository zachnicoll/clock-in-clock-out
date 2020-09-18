table! {
    use diesel::types::Varchar;
    use diesel::sql_types::Uuid;
    use crate::helpers::UsergroupMapping;
    users (id) {
        id -> Uuid,
        email -> Varchar,
        password -> Varchar,
        user_group -> UsergroupMapping,
    }
}
