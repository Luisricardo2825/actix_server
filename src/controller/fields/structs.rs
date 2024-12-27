use std::fmt;

use chrono::NaiveDateTime;
use derive_more::derive::Debug;
use diesel::{AsChangeset, Expression, Insertable};
use serde::{
    de::{self, Visitor},
    Deserialize, Deserializer, Serialize,
};

use crate::{models::fields_model::Field, routes::utils::reponses::ReturnError};

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

#[derive(Serialize, Deserialize, Insertable, Clone)]
#[diesel(table_name = crate::schema::fields)]
#[serde(rename_all = "camelCase")]
pub struct CreateFieldWithType {
    pub id: Option<i32>,
    pub name: String,
    pub description: Option<String>,
    pub field_type: String,
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

impl CreateFieldWithType {
    pub fn to_create_field(&self) -> Result<CreateField, ReturnError> {
        let field_type = match self.field_type.as_str() {
            "string" => FieldType::String,
            "integer" => FieldType::Integer,
            "float" => FieldType::Float,
            "boolean" => FieldType::Boolean,
            "date" => FieldType::Date,
            "time" => FieldType::Time,
            "datetime" => FieldType::DateTime,
            "text" => FieldType::Text,
            "json" => FieldType::Json,
            "binary" => FieldType::Binary,
            e => return Err(ReturnError::new("Invalid field type".to_string(), e)),
        };
        Ok(CreateField {
            id: self.id,
            name: self.name.clone(),
            description: self.description.clone(),
            field_type,
            table_id: self.table_id,
            is_required: self.is_required,
            is_primary_key: self.is_primary_key,
            is_auto_increment: self.is_auto_increment,
            is_generated: self.is_generated,
            default_value: self.default_value.clone(),
            is_unique: self.is_unique,
            created_at: self.created_at,
            updated_at: self.updated_at,
            custom_expression: self.custom_expression.clone(),
        })
    }
}

#[derive(Serialize, Clone)]
#[serde(untagged)]
pub enum FieldType {
    String = 0,
    Integer = 1,
    Float = 2,
    Boolean = 3,
    Date = 4,
    Time = 5,
    DateTime = 6,
    Text = 7,
    Json = 8,
    Binary = 9,
}

impl<'de> Deserialize<'de> for FieldType {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        struct FieldTypeVisitor;

        impl<'de> Visitor<'de> for FieldTypeVisitor {
            type Value = FieldType;

            fn expecting(&self, formatter: &mut fmt::Formatter) -> fmt::Result {
                formatter.write_str("a valid FieldType variant")
            }

            fn visit_str<E>(self, value: &str) -> Result<Self::Value, E>
            where
                E: de::Error,
            {
                match value {
                    "String" => Ok(FieldType::String),
                    "Integer" => Ok(FieldType::Integer),
                    "Float" => Ok(FieldType::Float),
                    "Boolean" => Ok(FieldType::Boolean),
                    "Date" => Ok(FieldType::Date),
                    "Time" => Ok(FieldType::Time),
                    "DateTime" => Ok(FieldType::DateTime),
                    "Text" => Ok(FieldType::Text),
                    "Json" => Ok(FieldType::Json),
                    "Binary" => Ok(FieldType::Binary),
                    _ => Err(de::Error::unknown_variant(
                        value,
                        &[
                            "String", "Integer", "Float", "Boolean", "Date", "Time", "DateTime",
                            "Text", "Json", "Binary",
                        ],
                    )),
                }
            }
        }

        deserializer.deserialize_any(FieldTypeVisitor)
    }
}

impl FieldType {
    pub fn to_string(&self) -> Result<String, ReturnError> {
        match self {
            FieldType::String => Ok("string".to_string()),
            FieldType::Integer => Ok("integer".to_string()),
            FieldType::Float => Ok("float".to_string()),
            FieldType::Boolean => Ok("boolean".to_string()),
            FieldType::Date => Ok("date".to_string()),
            FieldType::Time => Ok("time".to_string()),
            FieldType::DateTime => Ok("datetime".to_string()),
            FieldType::Text => Ok("text".to_string()),
            FieldType::Json => Ok("json".to_string()),
            FieldType::Binary => Ok("binary".to_string()),
        }
    }
    pub fn from_string(s: &str) -> Result<Self, ReturnError> {
        match s {
            "string" => Ok(FieldType::String),
            "integer" => Ok(FieldType::Integer),
            "float" => Ok(FieldType::Float),
            "boolean" => Ok(FieldType::Boolean),
            "date" => Ok(FieldType::Date),
            "time" => Ok(FieldType::Time),
            "datetime" => Ok(FieldType::DateTime),
            "text" => Ok(FieldType::Text),
            "json" => Ok(FieldType::Json),
            "binary" => Ok(FieldType::Binary),
            e => Err(ReturnError::new(
                "Invalid field type".to_string(),
                format!("{}", e).as_str(),
            )),
        }
    }

    pub fn check_if_exists(field_type: &String) -> bool {
        match field_type.as_str() {
            "string" => true,
            "integer" => true,
            "float" => true,
            "boolean" => true,
            "date" => true,
            "time" => true,
            "datetime" => true,
            "text" => true,
            "json" => true,
            "binary" => true,
            _ => false,
        }
    }
    pub fn check_value(&self, value: &str) -> Result<(), ReturnError> {
        match self {
            FieldType::String => Ok(()),
            FieldType::Integer => {
                if value.parse::<i32>().is_err() {
                    return Err(ReturnError::new(
                        "Invalid field type".to_string(),
                        "Value is not an integer".to_string().as_str(),
                    ));
                }
                Ok(())
            }
            FieldType::Float => {
                if value.parse::<f32>().is_err() {
                    return Err(ReturnError::new(
                        "Invalid field type".to_string(),
                        "Value is not a float".to_string().as_str(),
                    ));
                }
                Ok(())
            }
            FieldType::Boolean => {
                if value.parse::<bool>().is_err() {
                    return Err(ReturnError::new(
                        "Invalid field type".to_string(),
                        "Value is not a boolean".to_string().as_str(),
                    ));
                }
                Ok(())
            }
            FieldType::Date => {
                if value.parse::<NaiveDateTime>().is_err() {
                    return Err(ReturnError::new(
                        "Invalid field type".to_string(),
                        "Value is not a date".to_string().as_str(),
                    ));
                }
                Ok(())
            }
            FieldType::Time => {
                if value.parse::<NaiveDateTime>().is_err() {
                    return Err(ReturnError::new(
                        "Invalid field type".to_string(),
                        "Value is not a time".to_string().as_str(),
                    ));
                }
                Ok(())
            }
            FieldType::DateTime => {
                if value.parse::<NaiveDateTime>().is_err() {
                    return Err(ReturnError::new(
                        "Invalid field type".to_string(),
                        "Value is not a datetime".to_string().as_str(),
                    ));
                }
                Ok(())
            }
            FieldType::Text => Ok(()),
            FieldType::Json => {
                if serde_json::from_str::<serde_json::Value>(value).is_err() {
                    return Err(ReturnError::new(
                        "Invalid field type".to_string(),
                        "Value is not a json".to_string().as_str(),
                    ));
                }
                Ok(())
            }
            FieldType::Binary => Ok(()),
        }
    }
}

impl Expression for FieldType {
    type SqlType = diesel::sql_types::Text;
}

impl Into<String> for FieldType {
    fn into(self) -> String {
        match self {
            FieldType::String => "string".to_string(),
            FieldType::Integer => "integer".to_string(),
            FieldType::Float => "float".to_string(),
            FieldType::Boolean => "boolean".to_string(),
            FieldType::Date => "date".to_string(),
            FieldType::Time => "time".to_string(),
            FieldType::DateTime => "datetime".to_string(),
            FieldType::Text => "text".to_string(),
            FieldType::Json => "json".to_string(),
            FieldType::Binary => "binary".to_string(),
        }
    }
}

impl Into<FieldType> for String {
    fn into(self) -> FieldType {
        match self.as_str() {
            "string" => FieldType::String,
            "integer" => FieldType::Integer,
            "float" => FieldType::Float,
            "boolean" => FieldType::Boolean,
            "date" => FieldType::Date,
            "time" => FieldType::Time,
            "datetime" => FieldType::DateTime,
            "text" => FieldType::Text,
            "json" => FieldType::Json,
            "binary" => FieldType::Binary,
            _ => panic!("Invalid type"),
        }
    }
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

    pub fn to_create_field_with_type(&self) -> CreateFieldWithType {
        CreateFieldWithType {
            id: None,
            table_id: self.table_id,
            name: self.name.to_owned(),
            field_type: self.field_type.to_string().unwrap(),
            is_required: self.is_required,
            is_unique: self.is_unique,
            is_primary_key: self.is_primary_key,
            is_auto_increment: self.is_auto_increment,
            is_generated: self.is_generated,
            default_value: self.default_value.to_owned(),
            description: self.description.to_owned(),
            created_at: None,
            updated_at: None,
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

        if field_type.is_err() {
            return Err(field_type.unwrap_err());
        }
        let field_type = field_type.unwrap();
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
    pub id: i32,
    pub name: Option<String>,
    pub description: Option<String>,
    pub field_type: Option<String>,
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

#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(rename_all = "camelCase")]
pub struct QueryParams {
    pub id: Option<i32>,
    pub per_page: Option<i64>,
}
