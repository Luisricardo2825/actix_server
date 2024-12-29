use std::collections::HashMap;

use chrono::NaiveDateTime;
use derive_more::derive::Debug;
use diesel::{AsChangeset, Insertable};
use serde::{Deserialize, Serialize};
use serde_json::{Number, Value};

use crate::{models::fields_model::Field, routes::utils::reponses::ReturnError};

use super::types::FieldType;

#[derive(Serialize, Deserialize, Insertable, Clone)]
#[diesel(table_name = crate::schema::fields)]
#[serde(rename_all = "camelCase")]
pub struct CreateField {
    pub id: Option<i32>,
    pub name: String,
    pub description: Option<String>,
    pub field_type: FieldType,
    pub table_id: Option<i32>,
    pub is_required: Option<bool>,
    pub is_primary_key: Option<bool>,
    pub is_auto_increment: Option<bool>,
    pub is_generated: Option<bool>,
    pub default_value: Option<String>,
    pub is_unique: Option<bool>,
    pub created_at: Option<NaiveDateTime>,
    pub updated_at: Option<NaiveDateTime>,
    pub custom_expression: Option<String>,
}

impl CreateField {
    pub fn new(
        name: String,
        field_type: String,
        is_required: bool,
        is_unique: bool,
        is_primary_key: bool,
        is_auto_increment: bool,
        is_generated: bool,
        default_value: Option<String>,
        description: Option<String>,
        custom_expression: Option<String>,
    ) -> Self {
        Self {
            id: None,
            table_id: None,
            name,
            field_type: field_type.into(),
            is_required: Some(is_required),
            is_unique: Some(is_unique),
            is_primary_key: Some(is_primary_key),
            is_auto_increment: Some(is_auto_increment),
            is_generated: Some(is_generated),
            default_value,
            description,
            created_at: None,
            updated_at: None,
            custom_expression,
        }
    }

    pub fn to(&self) -> Field {
        Field {
            id: self.id.unwrap(),
            table_id: self.table_id.unwrap(),
            name: self.name.to_owned(),
            field_type: self.field_type.to_owned().into(),
            is_required: self.is_required.unwrap_or(false),
            is_unique: self.is_unique.unwrap_or(false),
            is_primary_key: self.is_primary_key.unwrap_or(false),
            is_auto_increment: self.is_auto_increment.unwrap_or(false),
            is_generated: self.is_generated.unwrap_or(false),
            default_value: self.default_value.to_owned(),
            description: self.description.to_owned(),
            created_at: self.created_at,
            updated_at: self.updated_at,
            custom_expression: self.custom_expression.to_owned(),
        }
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
        let field_type = self.field_type.to_string();

        if field_type.is_empty() {
            return self.this_error("Field type cannot be empty".to_string());
        }
        // Check if primary key is unique
        if self.is_primary_key.is_some_and(|x| x) && !self.is_unique.is_some_and(|x| x) {
            return self.this_error("Primary key must be unique".to_string());
        }
        if self.is_primary_key.is_some_and(|x| x) && self.is_required.is_some_and(|x| x) {
            return self.this_error("Primary key cannot be required".to_string());
        }
        if self.is_unique.is_some_and(|x| x) && self.is_required.is_some_and(|x| x) {
            return self.this_error("Unique key cannot be required".to_string());
        }
        if self.is_auto_increment.is_some_and(|x| x) && !self.is_primary_key.is_some_and(|x| x) {
            return self.this_error("Auto increment can only be set for primary key".to_string());
        }
        if self.is_generated.is_some_and(|x| x) && !self.is_auto_increment.is_some_and(|x| x) {
            return self.this_error("Generated can only be set for auto increment".to_string());
        }
        Ok(())
    }

    pub fn is_pk(&self) -> bool {
        self.is_primary_key.is_some_and(|x| x)
    }
    pub fn is_ai(&self) -> bool {
        self.is_auto_increment.is_some_and(|x| x)
    }
    pub fn is_gn(&self) -> bool {
        self.is_generated.is_some_and(|x| x)
    }
    pub fn is_un(&self) -> bool {
        self.is_unique.is_some_and(|x| x)
    }
    pub fn is_rq(&self) -> bool {
        self.is_required.is_some_and(|x| x)
    }
    pub fn is_fk(&self) -> bool {
        todo!("Not yet implemented")
    }

    pub fn set_table(&mut self, table_id: i32) {
        self.table_id = Some(table_id);
    }
}

impl Field {
    pub fn to(self) -> CreateField {
        CreateField {
            id: Some(self.id),
            table_id: Some(self.table_id),
            name: self.name.to_owned(),
            field_type: self.field_type.into(),
            is_required: Some(self.is_required),
            is_unique: Some(self.is_unique),
            is_primary_key: Some(self.is_primary_key),
            is_auto_increment: Some(self.is_auto_increment),
            is_generated: Some(self.is_generated),
            default_value: self.default_value.to_owned(),
            description: self.description.to_owned(),
            created_at: self.created_at,
            updated_at: self.updated_at,
            custom_expression: self.custom_expression.to_owned(),
        }
    }
    pub fn from(field: CreateField) -> Self {
        Self {
            id: field.id.unwrap(),
            table_id: field.table_id.unwrap(),
            name: field.name,
            field_type: field.field_type.into(),
            is_required: field.is_required.unwrap_or(false),
            is_unique: field.is_unique.unwrap_or(false),
            is_primary_key: field.is_primary_key.unwrap_or(false),
            is_auto_increment: field.is_auto_increment.unwrap_or(false),
            is_generated: field.is_generated.unwrap_or(false),
            default_value: field.default_value,
            description: field.description,
            created_at: field.created_at,
            updated_at: field.updated_at,
            custom_expression: field.custom_expression,
        }
    }
}

#[derive(Serialize, Deserialize, AsChangeset, Clone, Debug)]
#[diesel(table_name = crate::schema::fields)]
#[serde(rename_all = "camelCase")]
pub struct UpdateField {
    pub id: Option<i32>,
    pub name: Option<String>,
    pub description: Option<String>,
    pub field_type: Option<FieldType>,
    pub table_id: Option<i32>,
    pub is_required: Option<bool>,
    pub is_primary_key: Option<bool>,
    pub is_auto_increment: Option<bool>,
    pub is_generated: Option<bool>,
    pub default_value: Option<String>,
    pub is_unique: Option<bool>,
    pub created_at: Option<NaiveDateTime>,
    pub updated_at: Option<NaiveDateTime>,
    pub custom_expression: Option<String>,
}

impl UpdateField {
    pub fn is_empty(&self) -> bool {
        self.id.is_none()
            && self.name.is_none()
            && self.description.is_none()
            && self.field_type.is_none()
            && self.table_id.is_none()
            && self.is_required.is_none()
            && self.is_primary_key.is_none()
            && self.is_auto_increment.is_none()
            && self.is_generated.is_none()
            && self.default_value.is_none()
            && self.is_unique.is_none()
            && self.created_at.is_none()
            && self.updated_at.is_none()
            && self.custom_expression.is_none()
    }
}
impl PartialEq<Field> for UpdateField {
    fn eq(&self, other: &Field) -> bool {
        // Check if self has any change compared to old
        if self.id.is_some_and(|x| x != other.id) {
            return false;
        }

        if self.name.as_ref().is_some_and(|x| *x != other.name) {
            return false;
        }
        if self
            .description
            .as_ref()
            .is_some_and(|x| other.description.as_ref().is_some_and(|y| x != y))
        {
            return false;
        }

        if self
            .field_type
            .is_some_and(|x| x.to_string() != other.field_type.to_string())
        {
            return false;
        }
        if self.table_id.is_some_and(|x| x != other.table_id) {
            return false;
        }
        if self.is_required.is_some_and(|x| x != other.is_required) {
            return false;
        }
        if self
            .is_primary_key
            .is_some_and(|x| x != other.is_primary_key)
        {
            return false;
        }

        if self
            .is_auto_increment
            .is_some_and(|x| x != other.is_auto_increment)
        {
            return false;
        }

        if self.is_generated.is_some_and(|x| x != other.is_generated) {
            return false;
        }
        if self
            .default_value
            .as_ref()
            .is_some_and(|x| other.default_value.as_ref().is_some_and(|y| x != y))
        {
            return false;
        }
        if self.is_unique.is_some_and(|x| x != other.is_unique) {
            return false;
        }
        if self
            .created_at
            .is_some_and(|x| other.created_at.is_some_and(|y| x != y))
        {
            return false;
        }

        if self
            .custom_expression
            .as_ref()
            .is_some_and(|x| other.custom_expression.as_ref().is_some_and(|y| x != y))
        {
            return false;
        }

        true
    }
}

#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(rename_all(serialize = "camelCase"))]
pub struct QueryParams {
    pub id: Option<i32>,
    #[serde(rename = "perPage")]
    pub per_page: Option<i64>,
    // aditional props
    #[serde(flatten)]
    pub extra: HashMap<String, Value>,
}

impl QueryParams {
    pub fn new(id: Option<i32>, per_page: Option<i64>) -> Self {
        Self {
            id,
            per_page,
            extra: HashMap::new(),
        }
    }
    pub fn from_json(json: &Value) -> Result<Self, ReturnError> {
        let mut params = Self::new(None, None);
        let mut extra = HashMap::new();
        if let Some(id) = json.get("id") {
            params.id = Some(id.as_i64().unwrap() as i32);
        }
        if let Some(per_page) = json.get("perPage") {
            params.per_page = Some(per_page.as_i64().unwrap());
        }
        for (key, value) in json.as_object().unwrap() {
            if key == "id" || key == "perPage" {
                continue;
            }
            extra.insert(key.to_string(), value.clone());
        }
        params.extra = extra;
        Ok(params)
    }
    pub fn to_json(&self) -> Value {
        let mut json = serde_json::to_value(self).unwrap();
        for (key, value) in &self.extra {
            json[key] = value.clone();
        }
        json
    }
    pub fn convert_extra_values(&mut self) {
        for (key, value) in &self.extra.clone() {
            if Self::is_array(key) {
                let value = Self::get_array_value(value);
                println!("{} is array", serde_json::to_string(&value).unwrap());
                let _ = &mut self.extra.insert(key.to_string(), Value::Array(value));
                continue;
            }

            if Self::is_i64(value) {
                let _ = &mut self
                    .extra
                    .insert(key.to_string(), Value::Number(Self::to_i64(value).into()));
                continue;
            }

            if Self::is_f64(value) {
                let _ = &mut self.extra.insert(
                    key.to_string(),
                    Number::from_f64(Self::to_f64(value)).into(),
                );
                continue;
            }
            if Self::is_bool(value) {
                let _ = &mut self
                    .extra
                    .insert(key.to_string(), Value::Bool(Self::to_bool(value)));
                continue;
            }

            let _ = &mut self.extra.insert(key.to_string(), value.clone());
        }
    }

    pub fn is_array(key: &str) -> bool {
        key.ends_with("[]")
    }
    pub fn get_array_key(key: &str) -> String {
        key.trim_end_matches("[]").to_string()
    }
    pub fn get_array_value(value: &Value) -> Vec<Value> {
        let value = value.as_str().unwrap();
        // Remove first and last character
        let value = &value[1..value.len() - 1];
        let value: Vec<Value> = value
            .split(",")
            .map(|x| Value::String(x.to_string()))
            .collect::<Vec<Value>>();
        value
    }
    pub fn is_i64(value: &Value) -> bool {
        value.as_i64().is_some()
    }
    pub fn to_i64(value: &Value) -> i64 {
        value.as_i64().unwrap()
    }
    pub fn to_i32(value: &Value) -> i32 {
        Self::to_i64(value) as i32
    }

    pub fn is_f64(value: &Value) -> bool {
        value.as_f64().is_some()
    }
    pub fn is_i32(value: &Value) -> bool {
        Self::is_f64(value)
    }
    pub fn to_f64(value: &Value) -> f64 {
        value.as_f64().unwrap()
    }
    pub fn to_f32(value: &Value) -> f32 {
        Self::to_f64(value) as f32
    }

    pub fn is_bool(value: &Value) -> bool {
        value.as_bool().is_some()
    }
    pub fn to_bool(value: &Value) -> bool {
        value.as_bool().unwrap()
    }

    pub fn keys_to_lowercase(&mut self) {
        let mut extra = HashMap::new();
        for (key, value) in &self.extra {
            let key = key.to_lowercase();
            extra.insert(key, value.clone());
        }
        self.extra = extra;
    }
}
