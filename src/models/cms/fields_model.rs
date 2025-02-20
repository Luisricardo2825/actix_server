use super::table_model::Table;
use chrono::NaiveDateTime;
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
#[serde(rename_all = "camelCase")]
#[diesel(table_name = crate::schema::fields)]
#[diesel(check_for_backend(diesel::pg::Pg))]
#[diesel(belongs_to(Table))]
pub struct Field {
    pub id: i32,
    pub name: String,
    pub description: Option<String>,
    pub field_type: String,
    pub table_id: i32,
    pub is_required: bool,
    pub is_primary_key: bool,
    pub is_auto_increment: bool,
    pub is_generated: bool,
    pub default_value: Option<String>,
    pub custom_expression: Option<String>,
    pub is_unique: bool,
    pub created_at: Option<NaiveDateTime>,
    pub updated_at: Option<NaiveDateTime>,
}
