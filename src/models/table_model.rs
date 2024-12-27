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
    pub is_view: bool,
    pub is_active: bool,
    pub is_deleted: bool,
    pub view_sql: Option<String>,
    // pub view_columns: Option<Vec<Option<String>>>,
    pub capacity: Option<i32>,
    pub created_at: Option<NaiveDateTime>,
    pub updated_at: Option<NaiveDateTime>,
}
