// @generated automatically by Diesel CLI.

diesel::table! {
    contents (id) {
        id -> Int8,
        body -> Bytea,
    }
}

diesel::table! {
    revisions (id) {
        id -> Int8,
        parent_id -> Nullable<Int8>,
        content_id -> Int8,
        page_id -> Int8,
        user_id -> Int8,
        created_on -> Timestamptz,
    }
}

diesel::table! {
    user_sessions (token) {
        #[max_length = 24]
        token -> Varchar,
        user_id -> Int8,
        expire_on -> Nullable<Timestamptz>,
    }
}

diesel::table! {
    users (id) {
        id -> Int8,
        #[max_length = 255]
        name -> Varchar,
        #[max_length = 320]
        email -> Varchar,
        password -> Bytea,
        created_on -> Timestamptz,
        updated_on -> Timestamptz,
    }
}

diesel::joinable!(revisions -> contents (content_id));
diesel::joinable!(revisions -> users (user_id));
diesel::joinable!(user_sessions -> users (user_id));

diesel::allow_tables_to_appear_in_same_query!(
    contents,
    revisions,
    user_sessions,
    users,
);
