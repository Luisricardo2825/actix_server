use chrono::NaiveDateTime;
use derive_more::derive::Debug;
use diesel::{AsChangeset, Insertable};
use serde::{Deserialize, Serialize};

use crate::{models::cms::fields_model::Field, routes::utils::reponses::ReturnError};

use super::types::FieldType;

#[derive(Serialize, Deserialize, Insertable, Clone, Debug)]
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
