// @generated automatically by Diesel CLI.

diesel::table! {
    fields (id) {
        id -> Int4,
        #[max_length = 255]
        name -> Varchar,
        description -> Nullable<Text>,
        #[max_length = 255]
        field_type -> Varchar,
        table_id -> Int4,
        is_required -> Bool,
        is_primary_key -> Bool,
        is_auto_increment -> Bool,
        is_generated -> Bool,
        default_value -> Nullable<Varchar>,
        custom_expression -> Nullable<Text>,
        is_unique -> Bool,
        created_at -> Nullable<Timestamp>,
        updated_at -> Nullable<Timestamp>,
    }
}

diesel::table! {
    posts (id) {
        id -> Int4,
        title -> Varchar,
        body -> Text,
        published -> Bool,
        created_at -> Nullable<Timestamp>,
        updated_at -> Nullable<Timestamp>,
    }
}

diesel::table! {
    tables (id) {
        id -> Int4,
        #[max_length = 255]
        name -> Varchar,
        description -> Text,
        view_sql -> Nullable<Text>,
        capacity -> Nullable<Int4>,
        is_view -> Bool,
        is_active -> Bool,
        is_deleted -> Bool,
        auth -> Bool,
        auth_get -> Bool,
        auth_post -> Bool,
        auth_put -> Bool,
        auth_delete -> Bool,
        created_at -> Nullable<Timestamp>,
        updated_at -> Nullable<Timestamp>,
    }
}

diesel::table! {
    users (id) {
        id -> Int4,
        name -> Varchar,
        email -> Text,
        password -> Text,
        blocked -> Bool,
        api_rights -> Bool,
        admin -> Bool,
        created_at -> Nullable<Timestamp>,
        updated_at -> Nullable<Timestamp>,
    }
}

diesel::joinable!(fields -> tables (table_id));

diesel::allow_tables_to_appear_in_same_query!(
    fields,
    posts,
    tables,
    users,
);
