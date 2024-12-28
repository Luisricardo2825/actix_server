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
    postagens (idpost) {
        idpost -> Int4,
        titulo -> Nullable<Text>,
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
        is_view -> Bool,
        is_active -> Bool,
        is_deleted -> Bool,
        view_sql -> Nullable<Text>,
        capacity -> Nullable<Int4>,
        created_at -> Nullable<Timestamp>,
        updated_at -> Nullable<Timestamp>,
    }
}

diesel::table! {
    todo (id) {
        id -> Int4,
        titulo -> Nullable<Text>,
        isrequired2 -> Nullable<Text>,
        isrequired22 -> Nullable<Text>,
        is_required23 -> Nullable<Text>,
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
    postagens,
    posts,
    tables,
    todo,
    users,
);
