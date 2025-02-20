use chrono::NaiveDateTime;
use derive_more::derive::Debug;
use diesel::{AsChangeset, Insertable};
use serde::{Deserialize, Serialize};

use crate::{controller::fields::structs::CreateField, routes::utils::reponses::ReturnError};

#[derive(Serialize, Deserialize, AsChangeset, Clone, Debug)]
#[diesel(table_name = crate::schema::tables)]
#[serde(rename_all = "camelCase")]
pub struct Update {
    pub id: i32,
    pub name: Option<String>,
    pub description: Option<String>,
    pub is_view: Option<bool>,
    pub is_active: Option<bool>,
    pub is_deleted: Option<bool>,
    pub view_sql: Option<String>,
    pub capacity: Option<i32>,
    pub created_at: Option<NaiveDateTime>,
    pub updated_at: Option<NaiveDateTime>,
}

#[derive(Serialize, Deserialize, Clone)]
#[serde(rename_all = "camelCase")]
pub struct CreateTableRequest {
    pub id: Option<i32>,
    pub name: String,
    pub description: String,
    pub is_view: Option<bool>,
    pub is_active: Option<bool>,
    pub is_deleted: Option<bool>,
    pub view_sql: Option<String>,
    // pub view_columns: Option<Vec<Option<String>>>,
    pub capacity: Option<i32>,
    pub created_at: Option<NaiveDateTime>,
    pub updated_at: Option<NaiveDateTime>,
    pub fields: Option<Vec<CreateField>>,
}

#[derive(Serialize, Deserialize, Insertable, Clone, Debug)]
#[diesel(table_name = crate::schema::tables)]
#[serde(rename_all = "camelCase")]
pub struct Create {
    pub id: Option<i32>,
    pub name: String,
    pub description: String,
    pub is_view: Option<bool>,
    pub is_active: Option<bool>,
    pub is_deleted: Option<bool>,
    pub view_sql: Option<String>,
    pub capacity: Option<i32>,
    pub created_at: Option<NaiveDateTime>,
    pub updated_at: Option<NaiveDateTime>,
}

impl Create {
    pub fn new(
        name: String,
        description: String,
        is_view: Option<bool>,
        is_active: Option<bool>,
        is_deleted: Option<bool>,
        view_sql: Option<String>,
        capacity: Option<i32>,
    ) -> Self {
        Self {
            id: None,
            name,
            description,
            is_view,
            is_active,
            is_deleted,
            view_sql,
            capacity,
            created_at: None,
            updated_at: None,
        }
    }

    pub fn from(table_request: CreateTableRequest) -> (Create, Vec<CreateField>) {
        let fields = table_request.fields.unwrap_or_default();
        let table = Create::new(
            table_request.name,
            table_request.description,
            table_request.is_view,
            table_request.is_active,
            table_request.is_deleted,
            table_request.view_sql,
            table_request.capacity,
        );
        (table, fields)
    }

    fn this_error(&self, error: String) -> Result<(), ReturnError> {
        Err(ReturnError::new(
            error,
            serde_json::to_value(&self).unwrap(),
        ))
    }

    pub fn validate(&self) -> Result<(), ReturnError> {
        // Check name is not empty
        if self.name.is_empty() {
            return self.this_error("Name cannot be empty".to_string());
        }
        // Check name is not longer than 50 characters
        if self.name.len() > 50 {
            return self.this_error("Name cannot be longer than 50 characters".to_string());
        }
        // Chack if name contains only alphanumeric and underline
        if !self.name.chars().all(|c| c.is_alphanumeric() || c == '_') {
            return self.this_error("Name can only contain alphanumeric and underline".to_string());
        }
        if self.description.is_empty() {
            return self.this_error("Description cannot be empty".to_string());
        }
        if self.capacity.is_some_and(|x| x < 0) {
            return self.this_error("Capacity cannot be negative".to_string());
        }

        if (self.is_view.is_some_and(|x| x) && self.view_sql.is_none())
            || self
                .is_view
                .is_some_and(|x| x && self.view_sql.as_ref().is_some_and(|x| x.is_empty()))
        {
            return self.this_error("View SQL cannot be empty".to_string());
        }

        Ok(())
    }

    pub fn normalize_name(&mut self) {
        self.name = self.name.trim().to_string();
        self.name = self.name.to_lowercase();
        self.name = self.name.replace(" ", "_");
    }
}


// #[derive(AsExpression, Debug, Deserialize, Serialize, FromSqlRow)]
// #[sql_type = "Text"]
// pub struct MyJsonType(serde_json::Value);

// impl FromSql<Text, diesel::pg::Pg> for MyJsonType {
//     fn from_sql(bytes: PgValue<'_>) -> deserialize::Result<Self> {
//         let t = <String as FromSql<Text, Pg>>::from_sql(bytes)?;
//         Ok(Self(serde_json::from_str(&t)?))
//     }
// }

// impl ToSql<Text, diesel::pg::Pg> for MyJsonType
// where
//     String: ToSql<Text, diesel::pg::Pg>,
// {
//     fn to_sql<'b>(&'b self, out: &mut Output<'b, '_, diesel::pg::Pg>) -> serialize::Result {
//         let v = serde_json::to_string(&self.0)?;
//         <String as ToSql<Text, diesel::pg::Pg>>::to_sql(&v, &mut out.reborrow())
//     }
// }
