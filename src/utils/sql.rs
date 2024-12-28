use crate::controller::{fields::structs::CreateField, tables::structs::Create};

use super::string_utils::to_snake_case;

pub struct TableBuilder {
    pub table: Create,
    pub fields: Vec<CreateField>,
}

pub struct AddField {
    pub table: String,
    pub field: CreateField,
}

impl TableBuilder {
    pub fn from_create(table: Create, fields: Vec<CreateField>) -> Self {
        Self { table, fields }
    }
    pub fn build(&self) -> String {
        let mut str_table = String::new();

        str_table.push_str(&format!("CREATE TABLE {}", self.table.name));
        str_table.push_str("(\n");
        str_table.push_str(&self.build_fields());
        str_table.push_str(");");
        str_table
    }

    pub fn build_fields(&self) -> String {
        let mut str_fields = String::new();

        for field in &self.fields {
            let type_name = if field.is_primary_key.is_some_and(|x| x) && field.is_ai() {
                "SERIAL"
            } else {
                &field.field_type.to_string().unwrap().to_uppercase()
            };
            str_fields.push_str(&format!(
                "\t{} {} {},",
                field.name,
                type_name,
                &TableBuilder::get_field_constraints(field)
            ));
        }
        // Remove last comma
        str_fields.pop();
        str_fields
    }

    pub fn get_field_constraints(field: &CreateField) -> String {
        let mut str_constraints = String::new();

        if field.is_primary_key.is_some_and(|x| x) {
            str_constraints.push_str(" PRIMARY KEY");
        }
        if field.is_unique.is_some_and(|x| x) {
            str_constraints.push_str(" UNIQUE");
        }

        if !field.is_required.is_some_and(|x| x) && !field.is_pk() {
            if field.default_value.is_some() {
                str_constraints.push_str(&format!(
                    " DEFAULT {}",
                    field.default_value.clone().unwrap()
                ));
            } else {
                str_constraints.push_str(" DEFAULT NULL");
            }
        }

        str_constraints
    }
}

impl AddField {
    pub fn new<S: AsRef<str>>(table: S, field: &CreateField) -> Self {
        Self {
            table: table.as_ref().to_string(),
            field: field.clone(),
        }
    }
    pub fn build(&self) -> String {
        let mut str_field = String::new();

        str_field.push_str(&format!("ALTER TABLE {}", self.table));
        str_field.push_str(&format!(
            " ADD COLUMN {} {} {}",
            to_snake_case(&self.field.name),
            self.field.field_type.to_string().unwrap().to_uppercase(),
            TableBuilder::get_field_constraints(&self.field)
        ));
        str_field.push_str(";");

        str_field
    }
}
