use chrono::NaiveDateTime;
use derive_more::derive::Debug;
use diesel::{prelude::Identifiable, AsChangeset, Insertable};
use serde::{Deserialize, Serialize};

use crate::{
    controller::fields::structs::CreateField, models::table_model::Table,
    routes::utils::reponses::ReturnError,
};

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

#[derive(Serialize, Deserialize, Clone, Debug)]
#[serde(rename_all = "camelCase")]
pub struct CreateTableRequest {
    pub id: Option<i32>,
    pub name: String,
    pub description: String,
    pub is_view: Option<bool>,
    pub is_active: Option<bool>,
    pub is_deleted: Option<bool>,
    pub view_sql: Option<String>,
    pub auth: Option<bool>,
    pub auth_get: Option<bool>,
    pub auth_post: Option<bool>,
    pub auth_put: Option<bool>,
    pub auth_delete: Option<bool>,
    pub capacity: Option<i32>,
    pub created_at: Option<NaiveDateTime>,
    pub updated_at: Option<NaiveDateTime>,
    pub fields: Option<Vec<CreateField>>,
}

#[derive(Serialize, Deserialize, Insertable, Clone, Debug, Identifiable, PartialEq)]
#[diesel(table_name = crate::schema::tables)]
#[diesel(check_for_backend(diesel::pg::Pg))]
#[serde(rename_all = "camelCase")]
pub struct Create {
    pub id: Option<i32>,
    pub name: String,
    pub description: String,
    pub is_view: Option<bool>,
    pub is_active: Option<bool>,
    pub is_deleted: Option<bool>,
    pub view_sql: Option<String>,
    pub auth: Option<bool>,
    pub auth_get: Option<bool>,
    pub auth_post: Option<bool>,
    pub auth_put: Option<bool>,
    pub auth_delete: Option<bool>,
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
        auth: Option<bool>,
        auth_get: Option<bool>,
        auth_post: Option<bool>,
        auth_put: Option<bool>,
        auth_delete: Option<bool>,
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
            auth,
            auth_get,
            auth_post,
            auth_put,
            auth_delete,
            created_at: None,
            updated_at: None,
        }
    }

    pub fn to(&self) -> Table {
        Table {
            id: self.id.unwrap(),
            name: self.name.clone(),
            description: self.description.clone(),
            is_view: self.is_view.unwrap(),
            is_active: self.is_active.unwrap_or(true),
            is_deleted: self.is_deleted.unwrap_or(false),
            view_sql: self.view_sql.clone(),
            capacity: self.capacity,
            auth: self.auth.unwrap_or(true),
            auth_get: self.auth_get.unwrap_or(false),
            auth_post: self.auth_post.unwrap_or(false),
            auth_put: self.auth_put.unwrap_or(false),
            auth_delete: self.auth_delete.unwrap_or(false),
            created_at: self.created_at,
            updated_at: self.updated_at,
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
            table_request.auth,
            table_request.auth_get,
            table_request.auth_post,
            table_request.auth_put,
            table_request.auth_delete,
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

#[derive(Serialize, Deserialize, Insertable, Clone, Debug)]
#[diesel(table_name = crate::schema::tables)]
#[serde(rename_all = "camelCase")]
pub struct Delete {
    pub id: i32,
}
