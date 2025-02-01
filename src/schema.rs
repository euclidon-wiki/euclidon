// @generated automatically by Diesel CLI.

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
