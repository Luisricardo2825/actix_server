use chrono::NaiveDateTime;
use diesel::{AsChangeset, Insertable};
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, AsChangeset, Clone)]
#[diesel(table_name = crate::schema::users)]
#[serde(rename_all = "camelCase")]
pub struct Update {
    pub name: Option<String>,
    pub email: Option<String>,
    pub password: Option<String>,
    pub blocked: Option<bool>,
    pub api_rights: Option<bool>,
    pub admin: Option<bool>,
    pub updated_at: Option<NaiveDateTime>,
}

#[derive(Serialize, Deserialize, Insertable, Clone)]
#[diesel(table_name = crate::schema::users)]
#[serde(rename_all = "camelCase")]
pub struct Create {
    pub id: Option<i32>,
    pub name: String,
    pub email: String,
    pub password: String,
    pub blocked: Option<bool>,
    pub api_rights: Option<bool>,
    pub admin: Option<bool>,
    pub created_at: Option<NaiveDateTime>,
    pub updated_at: Option<NaiveDateTime>,
}
#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(rename_all = "camelCase")]
pub struct QueryParams {
    pub id: Option<i32>,
    pub per_page: Option<i64>,
}
