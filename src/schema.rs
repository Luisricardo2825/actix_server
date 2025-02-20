// @generated automatically by Diesel CLI.

diesel::table! {
    customizations (id) {
        id -> Int4,
        path -> Text,
        table_id -> Int4,
        #[max_length = 50]
        run_on -> Varchar,
        created_at -> Timestamp,
        updated_at -> Timestamp,
        created_by -> Int4,
        updated_by -> Int4,
    }
}

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
        created_at -> Nullable<Timestamp>,
        updated_at -> Nullable<Timestamp>,
    }
}

diesel::table! {
    tables_permissions (id) {
        id -> Int4,
        table_id -> Int4,
        #[max_length = 9]
        permission -> Varchar,
        allow -> Bool,
    }
}

diesel::table! {
    users (id) {
        id -> Int4,
        #[max_length = 255]
        name -> Varchar,
        #[max_length = 255]
        email -> Varchar,
        #[max_length = 255]
        password -> Varchar,
        blocked -> Bool,
        api_rights -> Bool,
        admin -> Bool,
        created_at -> Nullable<Timestamp>,
        updated_at -> Nullable<Timestamp>,
        picture -> Nullable<Text>,
    }
}

diesel::table! {
    users_permissions (id) {
        id -> Int4,
        user_id -> Int4,
        #[max_length = 9]
        permission -> Varchar,
        allow -> Bool,
    }
}

diesel::joinable!(customizations -> tables (table_id));
diesel::joinable!(fields -> tables (table_id));
diesel::joinable!(tables_permissions -> tables (table_id));
diesel::joinable!(users_permissions -> users (user_id));

diesel::allow_tables_to_appear_in_same_query!(
    customizations,
    fields,
    posts,
    tables,
    tables_permissions,
    users,
    users_permissions,
);
