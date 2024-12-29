use anyhow::Result;

use diesel::delete;
use diesel::insert_into;
use diesel::prelude::*;
use diesel::sql_query;
use diesel::update;

use super::structs::CreateField;
use super::structs::QueryParams;
use super::structs::UpdateField;
use super::utils::set_table_for_vec;
use crate::controller::tables::table_controller::TableController;
use crate::controller::Controller;
use crate::models::db::connection::establish_connection;
use crate::models::fields_model::Field;

use crate::routes::utils::reponses::ReturnError;
use crate::schema::fields::dsl as fields_dsl;
use crate::utils::sql::FieldQueryBuilder;

pub struct FieldController;

define_sql_function!(fn lower(x: diesel::sql_types::Text) -> diesel::sql_types::Text);

// Fields
impl FieldController {
    pub fn create_fields<S: AsRef<str>>(
        table_name: S,
        new_fields: Vec<CreateField>,
    ) -> Result<Vec<Field>, ReturnError> {
        let connection = &mut establish_connection();

        let table = TableController::find_by_name(table_name.as_ref());
        let table_id = match table {
            Ok(table) => table.id,
            Err(err) => {
                return Err(err);
            }
        };

        let mut fields: Vec<CreateField> = new_fields;

        let set_table_for_vec_result = set_table_for_vec(table_id, &mut fields);

        if set_table_for_vec_result.is_err() {
            return Err(set_table_for_vec_result.unwrap_err());
        }

        let transaction: std::result::Result<Vec<Field>, ReturnError> =
            connection.transaction(|conn| {
                let query = insert_into(fields_dsl::fields)
                    .values(&fields)
                    .get_results::<Field>(conn);

                match query {
                    Ok(res) => {
                        let add_field = FieldQueryBuilder::from_vec(table_name, fields.clone());
                        let query = add_field.build_add();
                        let create_field = sql_query(query).execute(conn);
                        match create_field {
                            Ok(_) => return Ok(res),
                            Err(err) => {
                                return Err(ReturnError {
                                    error_msg: err.to_string(),
                                    values: Some(serde_json::to_value(fields).unwrap()),
                                }
                                .into());
                            }
                        }
                    }
                    Err(err) => {
                        return Err(ReturnError {
                            error_msg: err.to_string(),
                            values: Some(serde_json::to_value(fields).unwrap()),
                        }
                        .into()); // if Successful, return the ID of the inserted table
                    }
                }
            });
        transaction
    }

    pub fn find(field_id: i32) -> Result<Field, ReturnError> {
        let connection = &mut establish_connection();

        let mut query = fields_dsl::fields.into_boxed();
        query = query.filter(fields_dsl::id.eq(field_id)); // Search for field_id
        match query.first::<Field>(connection) {
            Ok(results) => return Ok(results),
            Err(err) => {
                return Err(ReturnError {
                    error_msg: err.to_string(),
                    values: Some(field_id.into()),
                }
                .into()); // if Successful, return the ID of the inserted table
            }
        }
    }

    pub fn find_field_by_table_id_and_name<S: AsRef<str>>(
        table_id: i32,
        name: S,
    ) -> Result<Field, ReturnError> {
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

    pub fn find_field_by_name<S: AsRef<str>>(table_name: S, name: S) -> Result<Field, ReturnError> {
        let connection = &mut establish_connection();

        let table = TableController::find_by_name(table_name.as_ref());
        let table_id = match table {
            Ok(table) => table.id,
            Err(err) => {
                return Err(err);
            }
        };

        let mut query = fields_dsl::fields.into_boxed();
        query = query.filter(fields_dsl::table_id.eq(table_id)); // Search for table_id
        query = query.filter(lower(fields_dsl::name).eq(name.as_ref().to_lowercase())); // Search for field_id
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

    pub fn find_all_fields<S: AsRef<str>>(
        table_name: S,
        mut query_params: QueryParams,
    ) -> Result<Vec<Field>, ReturnError> {
        let connection = &mut establish_connection();

        let table = TableController::find_by_name(table_name.as_ref());
        let table_id = match table {
            Ok(table) => table.id,
            Err(err) => {
                return Err(err);
            }
        };
        let mut query = fields_dsl::fields.into_boxed();

        query_params.convert_extra_values();
        query_params.keys_to_lowercase();

        let name = query_params.extra.get("name");

        if name.is_some() {
            let name = name.unwrap();
            let name = name.as_str().unwrap();
            query = query.filter(lower(fields_dsl::name).eq(name.to_lowercase()));
        }

        let names = query_params.extra.get("name[]");

        if names.is_some() {
            let names = names.unwrap();
            let names: Vec<String> = names
                .as_array()
                .unwrap()
                .iter()
                .map(|x| x.as_str().unwrap().to_lowercase())
                .collect();
            query = query.filter(lower(fields_dsl::name).eq_any(names));
        }

        query = query.filter(fields_dsl::table_id.eq(table_id)); // Search for a unique table

        let name = query_params.extra.get("name");

        if name.is_some() {
            let name = name.unwrap();
            let name = name.as_str().unwrap();
            query = query.filter(fields_dsl::name.eq(name));
        }

        let ids = query_params.extra.get("id[]");

        if ids.is_some() {
            let ids = ids.unwrap();
            let ids: Vec<i32> = ids
                .as_array()
                .unwrap()
                .iter()
                .map(|x| {
                    x.as_str()
                        .unwrap()
                        .parse::<i32>()
                        .unwrap_or(f32::NEG_INFINITY as i32)
                })
                .collect();
            query = query.filter(fields_dsl::id.eq_any(ids));
        }

        if let Some(id_query) = query_params.id {
            query = query.filter(fields_dsl::id.eq(id_query)); // Search for a unique table
        };
        if let Some(per_page) = query_params.per_page {
            query = query.limit(per_page); // Define user tables per page
        } else {
            let per_page = query_params.extra.get("perpage");
            if per_page.is_some() {
                let per_page = per_page.unwrap();
                let per_page = per_page.as_str().unwrap();
                let per_page = per_page.parse::<i64>().unwrap();
                query = query.limit(per_page); // Define user tables per page
            } else {
                query = query.limit(100) // Default limit to 100
            }
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

    pub fn update_field(id: i32, mut new_field: UpdateField) -> Result<Field, ReturnError> {
        if new_field.is_empty() {
            return Err(ReturnError {
                error_msg: "Invalid json send at least one field".to_string(),
                values: Some(serde_json::to_value(new_field).unwrap()),
            }
            .into());
        }
        new_field.id = Some(id);
        let old = Self::find(id).unwrap();
        if new_field.eq(&old) {
            return Err(ReturnError {
                error_msg: "No changes".to_string(),
                values: Some(serde_json::to_value(new_field).unwrap()),
            }
            .into());
        }
        let connection = &mut establish_connection();

        let transaction: std::result::Result<Field, ReturnError> = connection.transaction(|conn| {
            new_field.updated_at = Some(chrono::Utc::now().naive_utc()); // update the updated_at field with the current time
            match update(fields_dsl::fields)
                .set(&new_field)
                .filter(fields_dsl::id.eq(id))
                .get_result::<Field>(conn)
            {
                Ok(res) => {
                    let table_name = TableController::find(res.table_id).unwrap().name;
                    let field_query_builder =
                        FieldQueryBuilder::from_vec(table_name, vec![res.clone().to()]);
                    let query = field_query_builder.build_update(&old.name, new_field.clone());
                    // todo!("NOt yet implemented {}", query);

                    // Split query in vec of strings and add ";" at end
                    let query = query.split(";").collect::<Vec<&str>>();
                    // Remove last element of vec
                    let query = query[..query.len() - 1].to_vec();
                    let update_field = || {
                        for query in query {
                            if query.is_empty() {
                                continue;
                            }
                            let update_query = sql_query(query).execute(conn);
                            if update_query.is_err() {
                                return Err(ReturnError {
                                    error_msg: update_query.unwrap_err().to_string(),
                                    values: Some(serde_json::to_value(new_field.clone()).unwrap()),
                                });
                            }
                        }
                        Ok(())
                    };
                    match update_field() {
                        Ok(_) => return Ok(res),
                        Err(err) => {
                            return Err(ReturnError {
                                error_msg: err.to_string(),
                                values: Some(serde_json::to_value(new_field).unwrap()),
                            }
                            .into());
                        }
                    }
                }
                Err(err) => {
                    return Err(ReturnError {
                        error_msg: err.to_string(),
                        values: Some(serde_json::to_value(new_field).unwrap()),
                    }
                    .into()); // if Successful, return the updated field
                }
            }
        });
        transaction
    }
}
