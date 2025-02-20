use super::users_model::User;
use diesel::prelude::*;
use serde::{Deserialize, Serialize};

#[derive(
    Identifiable,
    Associations,
    Queryable,
    PartialEq,
    Debug,
    Selectable,
    Serialize,
    Deserialize,
    Insertable,
    Clone,
)]
#[diesel(table_name = crate::schema::users_permissions)]
#[diesel(check_for_backend(diesel::pg::Pg))]
#[serde(rename_all = "camelCase")]
#[diesel(belongs_to(User))]
pub struct UserPermissions {
    pub id: i32,
    pub user_id: i32,
    pub permission: String,
    pub allow: bool,
}
