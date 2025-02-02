// @generated automatically by Diesel CLI.

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

diesel::joinable!(user_sessions -> users (user_id));

diesel::allow_tables_to_appear_in_same_query!(
    user_sessions,
    users,
);
