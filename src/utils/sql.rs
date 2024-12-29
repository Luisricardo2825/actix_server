use std::vec;

use crate::controller::{
    fields::{
        structs::{CreateField, UpdateField},
        types::FieldType,
    },
    tables::structs::Create,
};

use super::string_utils::to_snake_case;

pub struct TableQueryBuilder {
    pub table: Create,
    pub fields: Vec<CreateField>,
}

pub struct FieldQueryBuilder {
    pub table: String,
    pub fields: Vec<CreateField>,
}

impl TableQueryBuilder {
    pub fn from_create(table: Create, fields: Vec<CreateField>) -> Self {
        Self { table, fields }
    }

    pub fn from_table(table: Create) -> Self {
        Self {
            table: table,
            fields: vec![],
        }
    }
    pub fn build_create_table(&self) -> String {
        let mut str_table = String::new();

        str_table.push_str(&format!("CREATE TABLE {}", self.table.name));
        str_table.push_str("(\n");
        str_table.push_str(&self.build_fields());
        str_table.push_str(");");
        str_table
    }
    pub fn build_update_table(&self) -> String {
        let mut str_table = String::new();

        str_table.push_str(&format!("ALTER TABLE {}", self.table.name));
        str_table.push_str("\n");
        str_table.push_str(&self.build_fields());
        str_table.push_str(";");
        str_table
    }
    pub fn build_drop_table(&self) -> String {
        format!("DROP TABLE IF EXISTS {};", self.table.name)
    }

    pub fn build_fields(&self) -> String {
        let mut str_fields = String::new();

        for field in &self.fields {
            let type_name = if field.is_primary_key.is_some_and(|x| x) && field.is_ai() {
                "SERIAL"
            } else {
                &field.field_type.to_pg_type().to_uppercase()
            };
            str_fields.push_str(&format!(
                "\t{} {} {},",
                field.name,
                type_name,
                &FieldQueryBuilder::get_field_constraints(field)
            ));
        }
        // Remove last comma
        str_fields.pop();
        str_fields
    }
}

impl FieldQueryBuilder {
    pub fn new<S: AsRef<str>>(table: S, field: &CreateField) -> Self {
        Self {
            table: table.as_ref().to_string(),
            fields: vec![field.clone()],
        }
    }
    pub fn from_vec<S: AsRef<str>, V: AsRef<CreateField>>(table: S, fields: Vec<V>) -> Self {
        let fields = fields
            .iter()
            .map(|field| field.as_ref().clone())
            .collect::<Vec<CreateField>>();
        Self {
            table: table.as_ref().to_string(),
            fields,
        }
    }

    pub fn build_add(&self) -> String {
        let mut str_field = String::new();

        str_field.push_str(&format!("ALTER TABLE {}", self.table));
        for (idx, field) in self.fields.iter().enumerate() {
            let constraints = FieldQueryBuilder::get_field_constraints(field);
            if idx == 0 {
                str_field.push_str("\nADD COLUMN ");
            } else {
                str_field.push_str(",\nADD COLUMN ");
            }
            if field.is_ai() {
                str_field.push_str(&format!(
                    "{} {} {}",
                    to_snake_case(&field.name),
                    "SERIAL",
                    constraints
                ));
                continue;
            }
            str_field.push_str(&format!(
                "{} {} {}",
                to_snake_case(&field.name),
                field.field_type.to_pg_type().to_uppercase(),
                constraints
            ));
            if field.is_unique.is_some_and(|x| x) {
                str_field.push_str(" UNIQUE");
            }
            if field.is_primary_key.is_some_and(|x| x) {
                str_field.push_str(" PRIMARY KEY");
            }
            if field.default_value.is_some() {
                str_field.push_str(&format!(
                    " DEFAULT {}",
                    field.default_value.clone().unwrap()
                ));
            }
        }
        str_field.push_str(";");

        str_field
    }

    pub fn build_drop(&self) -> String {
        let mut str_field = String::new();

        str_field.push_str(&format!("ALTER TABLE {}", self.table));
        for (idx, field) in self.fields.iter().enumerate() {
            if idx == 0 {
                str_field.push_str("\nDROP COLUMN ");
            } else {
                str_field.push_str(", \nDROP COLUMN ");
            }
            str_field.push_str(&format!("{}", to_snake_case(&field.name)));
        }
        str_field.push_str(";");

        str_field
    }

    pub fn build_update<S: AsRef<str>>(&self, name: S, field: UpdateField) -> String {
        let mut str_field = String::new();
        let mut str_query = String::new();

        // internal functions
        if field.field_type.is_some() {
            str_field.push_str(&format!("ALTER TABLE {}", self.table));
            str_field.push_str("\nALTER COLUMN ");
            str_field.push_str(&format!(
                "{} TYPE {}",
                to_snake_case(name.as_ref()),
                if Self::is_timestamp(field.field_type.unwrap()) {
                    format!(
                        "TIMESTAMP without time zone USING {}::TIMESTAMP",
                        to_snake_case(name.as_ref())
                    )
                } else {
                    field.field_type.unwrap().to_pg_type().to_uppercase()
                }
            ));
            str_field.push_str(";");
        }
        if field.is_auto_increment.is_some() {
            if field.is_auto_increment.is_some_and(|x| !x) {
                str_query.push_str(&self.build_drop_ai());
            } else {
                str_query.push_str(&self.build_add_ai());
            }
        }

        if field.is_primary_key.is_some() {
            if field.is_primary_key.is_some_and(|x| !x) {
                str_query.push_str(&self.build_drop_pk());
            } else {
                str_query.push_str(&self.build_add_pk());
            }
        }
        if field.is_unique.is_some() {
            if field.is_unique.is_some_and(|x| !x) {
                str_query.push_str(&self.build_drop_unique());
            } else {
                str_query.push_str(&self.build_add_unique());
            }
        }
        if field.name.is_some() {
            str_query.push_str(&self.build_rename(
                to_snake_case(name.as_ref()),
                to_snake_case(field.name.unwrap().as_str()),
            ));
        }
        if field.default_value.is_some() {
            str_query.push_str(&self.build_add_default());
        }

        str_field.push_str(&str_query);

        str_field
    }

    pub fn is_timestamp(field_type: FieldType) -> bool {
        match field_type {
            FieldType::Timestamp => true,
            _ => false,
        }
    }
    pub fn build_rename(&self, old_name: String, new_name: String) -> String {
        let mut str_field = String::new();

        str_field.push_str(&format!("ALTER TABLE {}", self.table));
        str_field.push_str("\nRENAME COLUMN ");
        str_field.push_str(&format!("{} TO {}", old_name, new_name));
        str_field.push_str(";");

        str_field
    }

    pub fn build_drop_pk(&self) -> String {
        let mut str_field = String::new();

        str_field.push_str(&format!("ALTER TABLE {}", self.table));
        for (idx, field) in self.fields.iter().enumerate() {
            if idx == 0 {
                str_field.push_str("\nDROP CONSTRAINT IF EXISTS ");
            } else {
                str_field.push_str(", \nDROP CONSTRAINT IF EXISTS ");
            }
            str_field.push_str(&format!("pk_{}", to_snake_case(&field.name)));
        }
        str_field.push_str(";");

        str_field
    }

    pub fn build_add_pk(&self) -> String {
        let mut str_field = String::new();

        str_field.push_str(&format!("ALTER TABLE {}", self.table));
        for (idx, field) in self.fields.iter().enumerate() {
            if idx == 0 {
                str_field.push_str("\nADD CONSTRAINT ");
            } else {
                str_field.push_str(", \nADD CONSTRAINT ");
            }
            str_field.push_str(&format!(
                "pk_{} PRIMARY KEY ({})",
                to_snake_case(&field.name),
                to_snake_case(&field.name)
            ));
        }
        str_field.push_str(";");

        str_field
    }

    pub fn build_drop_unique(&self) -> String {
        let mut str_field = String::new();

        str_field.push_str(&format!("ALTER TABLE {}", self.table));
        for (idx, field) in self.fields.iter().enumerate() {
            if idx == 0 {
                str_field.push_str("\nDROP CONSTRAINT ");
            } else {
                str_field.push_str(", \nDROP CONSTRAINT ");
            }
            str_field.push_str(&format!("uq_{}", to_snake_case(&field.name)));
        }
        str_field.push_str(";");

        str_field
    }
    pub fn build_add_unique(&self) -> String {
        let mut str_field = String::new();

        str_field.push_str(&format!("ALTER TABLE {}", self.table));
        for (idx, field) in self.fields.iter().enumerate() {
            if idx == 0 {
                str_field.push_str("\nADD CONSTRAINT ");
            } else {
                str_field.push_str(", \nADD CONSTRAINT ");
            }
            str_field.push_str(&format!(
                "uq_{} UNIQUE ({})",
                to_snake_case(&field.name),
                to_snake_case(&field.name)
            ));
        }
        str_field.push_str(";");

        str_field
    }
    pub fn build_drop_default(&self) -> String {
        let mut str_field = String::new();

        str_field.push_str(&format!("ALTER TABLE {}", self.table));
        for (idx, field) in self.fields.iter().enumerate() {
            if idx == 0 {
                str_field.push_str("\nALTER COLUMN ");
            } else {
                str_field.push_str(", \nALTER COLUMN ");
            }
            str_field.push_str(&format!("{} DROP DEFAULT", to_snake_case(&field.name)));
        }
        str_field.push_str(";");

        str_field
    }
    pub fn build_add_default(&self) -> String {
        let mut str_field = String::new();

        str_field.push_str(&format!("ALTER TABLE {}", self.table));
        for (idx, field) in self.fields.iter().enumerate() {
            if idx == 0 {
                str_field.push_str("\nALTER COLUMN ");
            } else {
                str_field.push_str(", \nALTER COLUMN ");
            }
            str_field.push_str(&format!(
                "{} SET DEFAULT {}",
                to_snake_case(&field.name),
                field.default_value.clone().unwrap()
            ));
        }
        str_field.push_str(";");

        str_field
    }
    pub fn build_drop_not_null(&self) -> String {
        let mut str_field = String::new();

        str_field.push_str(&format!("ALTER TABLE {}", self.table));
        for (idx, field) in self.fields.iter().enumerate() {
            if idx == 0 {
                str_field.push_str("\nALTER COLUMN ");
            } else {
                str_field.push_str(", \nALTER COLUMN ");
            }
            str_field.push_str(&format!("{} DROP NOT NULL", to_snake_case(&field.name)));
        }
        str_field.push_str(";");

        str_field
    }
    pub fn build_add_not_null(&self) -> String {
        let mut str_field = String::new();

        str_field.push_str(&format!("ALTER TABLE {}", self.table));
        for (idx, field) in self.fields.iter().enumerate() {
            if idx == 0 {
                str_field.push_str("\nALTER COLUMN ");
            } else {
                str_field.push_str(", \nALTER COLUMN ");
            }
            str_field.push_str(&format!("{} SET NOT NULL", to_snake_case(&field.name)));
        }
        str_field.push_str(";");

        str_field
    }
    pub fn build_drop_ai(&self) -> String {
        let mut str_field = String::new();

        str_field.push_str(&format!("ALTER TABLE {}", self.table));
        for (idx, field) in self.fields.iter().enumerate() {
            if idx == 0 {
                str_field.push_str("\nDROP COLUMN ");
            } else {
                str_field.push_str(", \nDROP COLUMN ");
            }
            str_field.push_str(&format!("{}_seq", to_snake_case(&field.name)));
        }
        str_field.push_str(";");

        str_field
    }
    pub fn build_add_ai(&self) -> String {
        let mut str_field = String::new();

        str_field.push_str(&format!("ALTER TABLE {}", self.table));
        for (idx, field) in self.fields.iter().enumerate() {
            if idx == 0 {
                str_field.push_str("\nADD COLUMN ");
            } else {
                str_field.push_str(", \nADD COLUMN ");
            }
            str_field.push_str(&format!(
                "{} {} {}",
                to_snake_case(&field.name),
                "SERIAL",
                Self::get_field_constraints(field)
            ));
        }
        str_field.push_str(";");

        str_field
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

impl AsRef<CreateField> for CreateField {
    fn as_ref(&self) -> &Self {
        self
    }
}
