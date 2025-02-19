use anyhow::Result;

use diesel::delete;
use diesel::insert_into;
use diesel::prelude::*;
use diesel::sql_query;
use diesel::update;
use serde_json::json;
use serde_json::Value;

use super::structs::Create;
use super::structs::CreateTableRequest;
use super::structs::Update;
use crate::controller::fields::structs::CreateField;
use crate::controller::fields::utils::validate_fields;
use crate::controller::Controller;
use crate::controller::GenericValue;
use crate::controller::QueryParams;
use crate::controller::API_LIMIT;
use crate::models::db::connection::establish_connection;

use crate::models::table_model::Table;
use crate::routes::utils::reponses::ReturnError;
use crate::schema::fields::dsl as fields_dsl;
use crate::schema::tables::dsl as tables_dsl;
use crate::utils::sql::TableQueryBuilder;

pub struct TableController;

impl Controller<Table, CreateTableRequest> for TableController {
    fn delete(id: i32) -> Result<Table, ReturnError> {
        let connection = &mut establish_connection();

        let transaction: std::result::Result<Table, ReturnError> = connection.transaction(|conn| {
            let query = delete(tables_dsl::tables).filter(tables_dsl::id.eq(&id));

            match query.get_result::<Table>(conn) {
                Ok(res) => {
                    let drop_sql = TableQueryBuilder::drop_table(&res.name);
                    let drop_table = sql_query(drop_sql).execute(conn);

                    match drop_table {
                        Ok(_) => {
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
                Err(err) => {
                    return Err(ReturnError {
                        error_msg: err.to_string(),
                        values: Some(id.into()),
                    }
                    .into());
                }
            }
        });

        transaction
    }
    fn create(new_table: CreateTableRequest) -> Result<Table, ReturnError> {
        // let new_table = new_table.to::<CreateTableRequest>();

        // if new_table.is_err() {
        //     return Err(new_table.unwrap_err());
        // }

        // let new_table = new_table.unwrap();
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

                    let query = insert_into(fields_dsl::fields)
                        .values(&fields)
                        .execute(conn);

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
    fn update(table_id: i32, new_table: GenericValue) -> Result<Table, ReturnError> {
        // cast Any to Update
        let new_table = new_table.to::<Update>();
        if new_table.is_err() {
            return Err(ReturnError {
                error_msg: new_table.unwrap_err().to_string(),
                values: None,
            }
            .into());
        }
        let new_table = new_table.unwrap();
        let connection = &mut establish_connection();
        match update(tables_dsl::tables)
            .set(&new_table)
            .filter(tables_dsl::id.eq(table_id))
            .get_result::<Table>(connection)
        {
            Ok(res) => {
                // TODO: Adicionar SQL para realizar o update da tabela
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
        if let Some(limit) = query_params.limit {
            query = query.limit(limit); // Define user tables per page
        } else {
            query = query.limit(API_LIMIT) // Default limit
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
    pub fn delete_by_name<S: AsRef<str>>(name: S) -> Result<Value, ReturnError> {
        let connection = &mut establish_connection();
        let name = name.as_ref();
        let name = name.trim();

        if name.is_empty() {
            return Err(ReturnError {
                error_msg: "Name is empty".to_string(),
                values: None,
            }
            .into());
        }
        let id = Self::find_by_name(name)?.id;
        let transaction = connection.transaction(|conn| {
            let query = delete(tables_dsl::tables).filter(tables_dsl::id.eq(&id));

            match query.execute(conn) {
                Ok(_) => {
                    let drop_sql = TableQueryBuilder::drop_table(&name);
                    let delete_sql = TableQueryBuilder::delete_fields(&name);
                    let delete_fields = sql_query(delete_sql).execute(conn);

                    match delete_fields {
                        Ok(_) => {
                            let drop_table = sql_query(drop_sql).execute(conn);
                            match drop_table {
                                Ok(_) => {
                                    let json = json!({"status":"Ok","table":&name});
                                    return Ok(json);
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
                        Err(err) => {
                            return Err(ReturnError {
                                error_msg: err.to_string(),
                                values: Some(id.into()),
                            }
                            .into());
                        }
                    }
                }
                Err(err) => {
                    return Err(ReturnError {
                        error_msg: err.to_string(),
                        values: Some(id.into()),
                    }
                    .into());
                }
            }
        });

        transaction
    }
}
