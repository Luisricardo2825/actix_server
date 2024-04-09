use chrono::NaiveDateTime;
use diesel::{AsChangeset, Insertable};
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, AsChangeset, Clone)]
#[diesel(table_name = crate::schema::posts)]
#[serde(rename_all = "camelCase")]
pub struct Update {
    pub title: Option<String>,
    pub body: Option<String>,
    pub published: Option<bool>,
    pub updated_at: Option<NaiveDateTime>,
}

#[derive(Serialize, Deserialize, Insertable, Clone)]
#[diesel(table_name = crate::schema::posts)]
#[serde(rename_all = "camelCase")]
pub struct Create {
    pub id: Option<i32>,
    pub title: Option<String>,
    pub body: Option<String>,
    pub published: Option<bool>,
    pub created_at: Option<NaiveDateTime>,
    pub updated_at: Option<NaiveDateTime>,
}

#[derive(Serialize, Deserialize, Insertable, Clone)]
#[diesel(table_name = crate::schema::posts)]
#[serde(rename_all = "camelCase")]
pub struct Delete {
    pub id: i32,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(rename_all = "camelCase")]
pub struct QueryParams {
    pub id: Option<i32>,
    pub per_page: Option<i64>,
}
