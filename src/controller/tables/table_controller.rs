use anyhow::Result;

use diesel::delete;
use diesel::insert_into;
use diesel::prelude::*;
use diesel::sql_query;
use diesel::update;

use super::structs::Create;
use super::structs::CreateTableRequest;
use super::structs::QueryParams;
use super::structs::Update;
use crate::controller::fields::structs::CreateField;
use crate::controller::fields::utils::validate_fields;
use crate::controller::Controller;
use crate::models::db::connection::establish_connection;

use crate::models::table_model::Table;
use crate::routes::utils::reponses::ReturnError;
use crate::schema::fields::dsl as fields_dsl;
use crate::schema::tables::dsl as tables_dsl;
use crate::utils::sql::TableQueryBuilder;

pub struct TableController;

impl Controller<Table, QueryParams, CreateTableRequest, Update> for TableController {
    fn delete(id: i32) -> Result<Table, ReturnError> {
        let connection = &mut establish_connection();
        match delete(tables_dsl::tables)
            .filter(tables_dsl::id.eq(&id))
            .get_result::<Table>(connection)
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
    fn create(new_table: CreateTableRequest) -> Result<Table, ReturnError> {
        let (mut table, fields) = Create::from(new_table);

        if fields.is_empty() {
            return Err(ReturnError {
                error_msg: "No fields provided".to_string(),
                values: None,
            }
            .into());
        }

        table.normalize_name();

        let validation_result = validate_fields(&fields); // Validate fields
        if validation_result.is_err() {
            return Err(validation_result.unwrap_err());
        }

        let table_found = Self::find_by_name(&table.name);

        match table_found {
            Ok(_) => {
                return Err(ReturnError {
                    error_msg: format!("Table \"{}\" already exists", table.name),
                    values: None,
                }
                .into());
            }
            Err(_) => {}
        }
        let validation_result = table.validate(); // Validate table

        if validation_result.is_err() {
            return Err(validation_result.unwrap_err());
        }

        let connection = &mut establish_connection();

        let transaction: std::result::Result<Table, ReturnError> = connection.transaction(|conn| {
            let query = insert_into(tables_dsl::tables).values(&table);
            match query.get_result::<Table>(conn) {
                Ok(res_table) => {
                    let fields: Vec<CreateField> = fields
                        .into_iter()
                        .map(|mut field| {
                            field.table_id = Some(res_table.id);
                            return field;
                        })
                        .collect();

                    let query = insert_into(fields_dsl::fields).values(&fields).execute(conn);

                    match query {
                        Ok(_) => {
                            let builder = TableQueryBuilder::from_create(table.clone(), fields);
                            let query_table = builder.build_create_table();
                            let create_table = sql_query(query_table).execute(conn);
                            match create_table {
                                Ok(_) => {
                                    return Ok(res_table);
                                }
                                Err(err) => {
                                    return Err(ReturnError {
                                        error_msg: err.to_string(),
                                        values: Some(serde_json::to_value(table).unwrap()),
                                    }
                                    .into());
                                }
                            }
                        }
                        Err(err) => {
                            return Err(ReturnError {
                                error_msg: err.to_string(),
                                values: Some(serde_json::to_value(table).unwrap()),
                            }
                            .into());
                        }
                    }
                    // if Successful, return the ID of the inserted table
                }
                Err(err) => {
                    return Err(ReturnError {
                        error_msg: err.to_string(),
                        values: Some(serde_json::to_value(table).unwrap()),
                    }
                    .into());
                }
            }
        });

        transaction
    }
    fn update(table_id: i32, new_table: Update) -> Result<Table, ReturnError> {
        let connection = &mut establish_connection();
        match update(tables_dsl::tables)
            .set(&new_table)
            .filter(tables_dsl::id.eq(table_id))
            .get_result::<Table>(connection)
        {
            Ok(res) => {
                return Ok(res); // if Successful, return the ID of the inserted table
            }
            Err(err) => {
                return Err(ReturnError {
                    error_msg: err.to_string(),
                    values: Some(serde_json::to_value(new_table).unwrap()),
                }
                .into()); // if Successful, return the ID of the inserted table
            }
        }
    }
    fn find_all(query_params: QueryParams) -> Result<Vec<Table>, ReturnError> {
        let connection = &mut establish_connection();
        let mut query = tables_dsl::tables.into_boxed();

        if let Some(id_query) = query_params.id {
            query = query.filter(tables_dsl::id.eq(id_query)); // Search for a unique table
        };
        if let Some(per_page) = query_params.per_page {
            query = query.limit(per_page); // Define user tables per page
        } else {
            query = query.limit(100) // Default limit to 100
        }

        match query.load::<Table>(connection) {
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
    fn find(id: i32) -> Result<Table, ReturnError> {
        let connection: &mut PgConnection = &mut establish_connection();
        let mut query = tables_dsl::tables.into_boxed();
        query = query.filter(tables_dsl::id.eq(id)); // Search for a unique table
        match query.first::<Table>(connection) {
            Ok(results) => return Ok(results),
            Err(err) => {
                return Err(ReturnError {
                    error_msg: err.to_string(),
                    values: Some(id.into()),
                }
                .into()); // if Successful, return the ID of the inserted table
            }
        }
    }
}

// Table aditionals
impl TableController {
    pub fn find_by_name<S: AsRef<str>>(name: S) -> Result<Table, ReturnError> {
        let connection: &mut PgConnection = &mut establish_connection();
        let mut query = tables_dsl::tables.into_boxed();
        query = query.filter(tables_dsl::name.eq(name.as_ref()));
        match query.first::<Table>(connection) {
            Ok(results) => return Ok(results),
            Err(err) => {
                return Err(ReturnError {
                    error_msg: err.to_string(),
                    values: Some(name.as_ref().into()),
                }
                .into());
            }
        }
    }
}
