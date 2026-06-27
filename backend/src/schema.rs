// @generated automatically by Diesel CLI.

diesel::table! {
    users (id) {
        id -> Uuid,
        email -> Varchar,
        password_hash -> Nullable<Varchar>,
        name -> Varchar,
        provider -> Varchar,
        created_at -> Nullable<Timestamptz>,
    }
}
