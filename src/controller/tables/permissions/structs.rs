use derive_more::derive::Debug;

use diesel::{AsChangeset, Insertable};
use serde::{Deserialize, Serialize};

use crate::models::cms::permission_model::PermissionType;

#[derive(Serialize, Deserialize, Insertable, Clone, Debug)]
#[diesel(table_name = crate::schema::tables_permissions)]
#[diesel(check_for_backend(diesel::pg::Pg))]
#[serde(rename_all = "camelCase")]
pub struct CreateTablePermission {
    pub id: Option<i32>,
    pub table_id: Option<i32>,
    pub permission: PermissionType,
    pub allow: bool,
}

impl CreateTablePermission {
    pub fn new(table_id: i32, permission: PermissionType, allow: bool) -> Self {
        Self {
            id: None,
            table_id: Some(table_id),
            permission,
            allow,
        }
    }
}

#[derive(Serialize, Deserialize, AsChangeset, Clone, Debug)]
#[diesel(table_name = crate::schema::tables_permissions)]
#[serde(rename_all = "camelCase")]
pub struct UpdateTablePermission {
    pub id: i32,
    pub table_id: Option<i32>,
    pub permission: Option<String>,
    pub allow: Option<bool>,
}
