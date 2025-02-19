use chrono::NaiveDateTime;
use diesel::prelude::*;
use serde::{Deserialize, Serialize};

#[derive(Identifiable, Queryable, PartialEq, Debug, Selectable, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
#[diesel(table_name = crate::schema::tables)]
#[diesel(check_for_backend(diesel::pg::Pg))]
pub struct Table {
    pub id: i32,
    pub name: String,
    pub description: String,
    pub view_sql: Option<String>,
    pub capacity: Option<i32>,
    pub is_view: bool,
    pub is_active: bool,
    pub is_deleted: bool,
    pub auth: bool,
    pub auth_get: bool,
    pub auth_post: bool,
    pub auth_put: bool,
    pub auth_delete: bool,
    pub created_at: Option<NaiveDateTime>,
    pub updated_at: Option<NaiveDateTime>,
}

impl Table {
    pub fn is_public(&self) -> bool {
        !self.auth
    }
    pub fn is_public_get(&self) -> bool {
        !self.auth_get && !self.auth
    }
    pub fn is_public_post(&self) -> bool {
        !self.auth_post && !self.auth
    }
    pub fn is_public_put(&self) -> bool {
        !self.auth_put && !self.auth
    }
    pub fn is_public_delete(&self) -> bool {
        !self.auth_delete && !self.auth
    }

    pub fn check_method(&self, method: &str) -> bool {
        match method {
            "GET" => self.is_public_get(),
            "POST" => self.is_public_post(),
            "PUT" => self.is_public_put(),
            "DELETE" => self.is_public_delete(),
            _ => false,
        }
    }
}
