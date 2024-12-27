use anyhow::Result;

use diesel::delete;
use diesel::insert_into;
use diesel::prelude::*;
use diesel::update;

use super::structs::CreateField;
use super::structs::QueryParams;
use super::structs::UpdateField;
use super::utils::set_table_for_vec;
use super::utils::validate_fields;
use crate::controller::tables::table_controller::TableController;
use crate::models::db::connection::establish_connection;
use crate::models::fields_model::Field;

use crate::routes::utils::reponses::ReturnError;
use crate::schema::fields::dsl as fields_dsl;

pub struct FieldController;
// Fields
impl FieldController {
    pub fn create_field(table_id: i32, mut new_field: CreateField) -> Result<Field, ReturnError> {
        let connection = &mut establish_connection();
        new_field.set_table(table_id);

        let field_found = FieldController::find_field_by_name(table_id, &new_field.name);

        if field_found.is_ok() {
            return Err(ReturnError {
                error_msg: format!("Field \"{}\" already exists", new_field.name),
                values: None,
            }
            .into());
        }

        let validation_result = new_field.validate(); // Validate fields
        if validation_result.is_err() {
            return Err(validation_result.unwrap_err());
        }

        println!("{}", serde_json::to_string(&new_field).unwrap());
        let query = insert_into(fields_dsl::fields)
            .values(&new_field.to_create_field_with_type())
            .get_result::<Field>(connection);

        match query {
            Ok(res) => {
                return Ok(res);
            }
            Err(err) => {
                return Err(ReturnError {
                    error_msg: err.to_string(),
                    values: Some(serde_json::to_value(new_field).unwrap()),
                }
                .into()); // if Successful, return the ID of the inserted table
            }
        }
    }

    pub fn create_fields(
        table_id: i32,
        new_fields: Vec<CreateField>,
    ) -> Result<Vec<Field>, ReturnError> {
        let connection = &mut establish_connection();
        let mut fields: Vec<CreateField> = new_fields;

        let set_table_for_vec_result = set_table_for_vec(table_id, &mut fields);

        if set_table_for_vec_result.is_err() {
            return Err(set_table_for_vec_result.unwrap_err());
        }

        let validation_result = validate_fields(&fields); // Validate fields
        if validation_result.is_err() {
            return Err(validation_result.unwrap_err());
        }

        let fields: Vec<Field> = fields.into_iter().map(|field| field.to()).collect();
        let query = insert_into(fields_dsl::fields)
            .values(&fields)
            .get_results::<Field>(connection);

        match query {
            Ok(res) => {
                return Ok(res);
            }
            Err(err) => {
                return Err(ReturnError {
                    error_msg: err.to_string(),
                    values: Some(serde_json::to_value(fields).unwrap()),
                }
                .into()); // if Successful, return the ID of the inserted table
            }
        }
    }

    pub fn find_field(table_id: i32, field_id: i32) -> Result<Field, ReturnError> {
        let connection = &mut establish_connection();
        let mut query = fields_dsl::fields.into_boxed();
        query = query.filter(fields_dsl::table_id.eq(table_id)); // Search for table_id
        query = query.filter(fields_dsl::id.eq(field_id)); // Search for field_id
        match query.first::<Field>(connection) {
            Ok(results) => return Ok(results),
            Err(err) => {
                return Err(ReturnError {
                    error_msg: err.to_string(),
                    values: Some(table_id.into()),
                }
                .into()); // if Successful, return the ID of the inserted table
            }
        }
    }

    pub fn find_field_by_name<S: AsRef<str>>(table_id: i32, name: S) -> Result<Field, ReturnError> {
        let connection = &mut establish_connection();
        let mut query = fields_dsl::fields.into_boxed();
        query = query.filter(fields_dsl::table_id.eq(table_id)); // Search for table_id
        query = query.filter(fields_dsl::name.eq(name.as_ref())); // Search for field_id
        match query.first::<Field>(connection) {
            Ok(results) => return Ok(results),
            Err(err) => {
                return Err(ReturnError {
                    error_msg: err.to_string(),
                    values: Some(table_id.into()),
                }
                .into()); // if Successful, return the ID of the inserted table
            }
        }
    }

    pub fn find_all_fields(
        table_id: i32,
        query_params: QueryParams,
    ) -> Result<Vec<Field>, ReturnError> {
        let connection = &mut establish_connection();
        let mut query = fields_dsl::fields.into_boxed();

        query = query.filter(fields_dsl::table_id.eq(table_id)); // Search for a unique table

        if let Some(id_query) = query_params.id {
            query = query.filter(fields_dsl::id.eq(id_query)); // Search for a unique table
        };
        if let Some(per_page) = query_params.per_page {
            query = query.limit(per_page); // Define user tables per page
        } else {
            query = query.limit(100) // Default limit to 100
        }

        match query.load::<Field>(connection) {
            Ok(results) => return Ok(results),
            Err(err) => {
                return Err(ReturnError {
                    error_msg: err.to_string(),
                    values: Some(serde_json::to_value(query_params).unwrap()),
                }
                .into()); // if Successful, return the ID of the inserted table
            }
        }
    }

    pub fn find_all(table_id: i32) -> Result<Vec<Field>, ReturnError> {
        let connection = &mut establish_connection();
        let mut query = fields_dsl::fields.into_boxed();

        query = query.filter(fields_dsl::table_id.eq(table_id)); // Search for a unique table

        match query.load::<Field>(connection) {
            Ok(results) => return Ok(results),
            Err(err) => {
                return Err(ReturnError {
                    error_msg: err.to_string(),
                    values: Some(serde_json::to_value(table_id).unwrap()),
                }
                .into()); // if Successful, return the ID of the inserted table
            }
        }
    }

    pub fn find_all_by_table_name<S: AsRef<str>>(table_name: S) -> Result<Vec<Field>, ReturnError> {
        let connection = &mut establish_connection();
        let mut query = fields_dsl::fields.into_boxed();
        let table = TableController::find_by_name(table_name);
        if table.is_err() {
            return Err(table.unwrap_err());
        }
        let table_id = table.unwrap().id;
        query = query.filter(fields_dsl::table_id.eq(table_id)); // Search for a unique table

        match query.load::<Field>(connection) {
            Ok(results) => return Ok(results),
            Err(err) => {
                return Err(ReturnError {
                    error_msg: err.to_string(),
                    values: Some(serde_json::to_value(table_id).unwrap()),
                }
                .into()); // if Successful, return the ID of the inserted table
            }
        }
    }
    pub fn find_pk<S: AsRef<str>>(table_name: S) -> Result<Field, ReturnError> {
        let connection = &mut establish_connection();
        let mut query = fields_dsl::fields.into_boxed();
        let table = TableController::find_by_name(table_name);
        if table.is_err() {
            return Err(table.unwrap_err());
        }
        let table_id = table.unwrap().id;
        query = query.filter(fields_dsl::table_id.eq(table_id)); // Search for a unique table
        query = query.filter(fields_dsl::is_primary_key.eq(true));

        match query.get_result::<Field>(connection) {
            Ok(results) => return Ok(results),
            Err(err) => {
                return Err(ReturnError {
                    error_msg: err.to_string(),
                    values: Some(serde_json::to_value(table_id).unwrap()),
                }
                .into()); // if Successful, return the ID of the inserted table
            }
        }
    }

    pub fn delete_field(table_id: i32, id: i32) -> Result<Field, ReturnError> {
        let connection = &mut establish_connection();
        match delete(fields_dsl::fields)
            .filter(fields_dsl::id.eq(&id))
            .filter(fields_dsl::table_id.eq(table_id))
            .get_result::<Field>(connection)
        {
            Ok(res) => {
                return Ok(res); // if Successful, return the deleted data
            }
            Err(err) => {
                return Err(ReturnError {
                    error_msg: err.to_string(),
                    values: Some(id.into()),
                }
                .into());
            }
        }
    }

    pub fn delete_field_by_name<S: AsRef<str>>(
        table_id: i32,
        name: S,
    ) -> Result<Field, ReturnError> {
        let connection = &mut establish_connection();
        match delete(fields_dsl::fields)
            .filter(fields_dsl::name.eq(name.as_ref()))
            .filter(fields_dsl::table_id.eq(table_id))
            .get_result::<Field>(connection)
        {
            Ok(res) => {
                return Ok(res); // if Successful, return the deleted data
            }
            Err(err) => {
                return Err(ReturnError {
                    error_msg: err.to_string(),
                    values: Some(name.as_ref().into()),
                }
                .into());
            }
        }
    }

    pub fn update_field(id: i32, new_field: UpdateField) -> Result<Field, ReturnError> {
        let connection = &mut establish_connection();
        match update(fields_dsl::fields)
            .set(&new_field)
            .filter(fields_dsl::id.eq(id))
            .get_result::<Field>(connection)
        {
            Ok(res) => {
                return Ok(res); // if Successful, return the updated field
            }
            Err(err) => {
                return Err(ReturnError {
                    error_msg: err.to_string(),
                    values: Some(serde_json::to_value(new_field).unwrap()),
                }
                .into()); // if Successful, return the updated field
            }
        }
    }
}
