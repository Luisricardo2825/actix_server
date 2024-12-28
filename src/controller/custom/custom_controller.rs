use std::sync::Arc;

use chrono::{NaiveDate, NaiveDateTime, NaiveTime};
use diesel::pg::Pg;
use diesel::query_builder::BoxedSqlQuery;
use diesel::sql_types::{Binary, Bool, Date, Float, Integer, Json, Time, Timestamp, VarChar};
use diesel::{sql_query, RunQueryDsl};
use serde_json::{Map, Value};

use super::structs::{GenericValue, QueryParams};

use crate::controller::db::establish_connection;
use crate::controller::fields::field_controller::FieldController;
use crate::controller::fields::structs::FieldType;
use crate::controller::tables::table_controller::TableController;
use crate::models::db::connection::DbPool;
use crate::models::db::driver_connection::establish_driver_connection;

use crate::routes::utils::reponses::ReturnError;

pub struct CustomController(pub Arc<DbPool>);

impl CustomController {
    pub async fn find_one(
        &self,
        table_name: String,
        query_params: QueryParams,
    ) -> Result<Vec<Value>, ReturnError> {
        let pk = FieldController::find_pk(&table_name);

        if pk.is_err() {
            return Err(ReturnError::without_value(format!(
                "Table \"{table_name}\" not found"
            )));
        }

        let conditions_str = match get_conditions(&query_params) {
            Ok(value) => value,
            Err(value) => return value,
        };

        let name = table_name;
        let table = TableController::find_by_name(&name);

        if table.is_err() {
            return Err(ReturnError::without_value(format!(
                "Table \"{name}\" not found"
            )));
        }
        let client = match establish_driver_connection().await {
            Ok(client) => client,
            Err(_) => {
                return Err(ReturnError::without_value(
                    "Error connecting to database".to_owned(),
                ));
            }
        };

        let query = format!("SELECT * FROM {} {}", name, conditions_str);
        match client.query(&query, &[]).await {
            Ok(rows) => {
                let results = match resolve_rows(rows) {
                    Ok(value) => value,
                    Err(value) => return value,
                };
                return Ok(results);
            }
            Err(err) => {
                return Err(ReturnError {
                    error_msg: err.to_string(),
                    values: Some(name.into()),
                }
                .into());
            }
        }
    }

    pub async fn find_all(
        table_name: String,
        _query_params: QueryParams,
    ) -> Result<Vec<GenericValue>, ReturnError> {
        let table = TableController::find_by_name(table_name);
        if table.is_err() {
            return Err(ReturnError::without_value("Table not found".to_owned()));
        }
        let table = table.unwrap();
        let table_name = table.name;
        let connection = &mut establish_connection();
        let query = format!(
            "SELECT row_to_json(t) as row FROM (Select * from {}) t;",
            table_name
        );
        let query = sql_query(query);

        match query.get_results::<GenericValue>(connection) {
            Ok(results) => {
                return Ok(results);
            }
            Err(err) => {
                return Err(ReturnError::new(err.to_string(), 1)); // if Successful, return the ID of the inserted post
            }
        }
    }

    pub async fn create(
        table_name: String,
        values: Value,
        _query_params: QueryParams,
    ) -> Result<Vec<GenericValue>, ReturnError> {
        if !values.is_object() {
            return Err(ReturnError::without_value("Invalid data".to_owned()));
        }
        let fields = FieldController::find_all_by_table_name(&table_name);
        if fields.is_err() {
            return Err(ReturnError::without_value("Table not found".to_owned()));
        }
        let fields = fields.unwrap();
        let values_cloned = values.clone();
        for field in fields.clone() {
            let values = values_cloned.as_object().unwrap();
            if field.is_required && !values.contains_key(&field.name) {
                return Err(ReturnError::without_value(format!(
                    "Field \"{}\" is required",
                    field.name
                )));
            }
            if values.contains_key(&field.name) {
                if field.is_primary_key && field.is_auto_increment {
                    return Err(ReturnError::without_value(format!(
                        "Field \"{}\" is serial and cannot be set",
                        field.name
                    )));
                }
            }
        }
        let fields = fields
            .iter()
            .filter(|x| !x.is_auto_increment)
            .map(|x| x.clone())
            .collect::<Vec<crate::models::fields_model::Field>>();
        let query = format!("INSERT INTO {}", table_name);
        match mutate(table_name, values, query, fields) {
            Ok(value) => Ok(value),
            Err(err) => Err(err),
        }
    }
}

fn mutate(
    table_name: String,
    values: Value,
    query: String,
    fields: Vec<crate::models::fields_model::Field>,
) -> Result<Vec<GenericValue>, ReturnError> {
    let fields = fields.iter();
    let values = values.as_object().unwrap();
    if values.is_empty() {
        return Err(ReturnError::without_value("Invalid data".to_owned()));
    }
    let mut columns = String::new();
    let mut placeholders = String::new();
    for (i, (key, _)) in values.iter().enumerate() {
        if i > 0 {
            columns.push_str(", ");
            placeholders.push_str(", ");
        }
        columns.push_str(&format!("\"{}\"", key));
        placeholders.push_str(&format!("${}", i + 1));
    }
    let connection = &mut establish_connection();
    let query = format!(
        "{} ({}) VALUES ({}) RETURNING row_to_json({}.*) as row;",
        query, columns, placeholders, table_name
    );
    let query = sql_query(query);
    let query = add_params(fields, values, query);
    if query.is_err() {
        return Err(query.err().unwrap());
    }
    let query = query.unwrap();
    match query.get_results::<GenericValue>(connection) {
        Ok(results) => {
            return Ok(results);
        }
        Err(err) => {
            return Err(ReturnError::new(err.to_string(), values)); // if Successful, return the ID of the inserted post
        }
    }
}

fn add_params<'a>(
    fields: std::slice::Iter<'a, crate::models::fields_model::Field>,
    key_value: &'a Map<String, Value>,
    query: diesel::query_builder::SqlQuery,
) -> Result<BoxedSqlQuery<'a, Pg, diesel::query_builder::SqlQuery>, ReturnError> {
    let mut query = query.into_boxed::<Pg>();
    let exist_in_fields: Vec<String> = fields
        .clone()
        .map(|x| {
            let name = x.name.to_lowercase();
            name
        })
        .collect();
    for (key, value) in key_value {
        if !exist_in_fields.contains(&key.to_lowercase()) {
            return Err(ReturnError::without_value(format!(
                "Column \"{}\" not found",
                key
            )));
        }

        let field = fields.clone().find(|x| {
            let name = x.name.to_lowercase();
            let key = key.to_lowercase();
            name.eq(&key)
        });
        if field.is_none() {
            return Err(ReturnError::without_value(format!(
                "Column \"{}\" not found",
                key
            )));
        }

        let field = field.unwrap();
        let field_type_str = field.field_type.clone();
        let field_type = FieldType::from_string(&field_type_str);
        if field_type.is_err() {
            return Err(ReturnError::without_value(format!(
                "Invalid field type \"{}\"",
                field_type_str
            )));
        }
        let field_type = field_type.unwrap();
        match field_type {
            FieldType::String => {
                query = query.bind::<VarChar, String>(value.as_str().unwrap().to_owned());
            }
            FieldType::Integer => {
                query = query.bind::<Integer, i32>(value.as_i64().unwrap() as i32);
            }
            FieldType::Float => {
                query = query.bind::<Float, f32>(value.as_f64().unwrap() as f32);
            }
            FieldType::Boolean => {
                query = query.bind::<Bool, bool>(value.as_bool().unwrap());
            }
            FieldType::Timestamp => {
                query = query
                    .bind::<Timestamp, NaiveDateTime>(value.as_str().unwrap().parse().unwrap());
            }
            FieldType::Date => {
                query = query.bind::<Date, NaiveDate>(value.as_str().unwrap().parse().unwrap());
            }
            FieldType::Binary => {
                query = query.bind::<Binary, Vec<u8>>(value.as_str().unwrap().as_bytes().to_vec());
            }
            FieldType::Time => {
                query = query.bind::<Time, NaiveTime>(value.as_str().unwrap().parse().unwrap());
            }
            FieldType::Json => {
                query = query.bind::<Json, Value>(value.clone());
            }
            FieldType::Text => {
                query = query.bind::<VarChar, String>(value.as_str().unwrap().to_owned());
            }
        };
    }

    return Ok(query);
}

fn get_conditions(query_params: &QueryParams) -> Result<String, Result<Vec<Value>, ReturnError>> {
    let conditions = &query_params.conditions;
    if conditions.is_none() {
        return Err(Err(ReturnError::without_value(
            "At least one condition is required".to_owned(),
        )));
    }
    let conditions = conditions.clone().unwrap();
    let conditions = conditions.as_object().unwrap();
    let mut conditions_str = String::new();
    if !conditions.is_empty() {
        conditions_str.push_str("where ");
        for (i, (key, value)) in conditions.iter().enumerate() {
            if i > 0 {
                conditions_str.push_str(" AND ");
            }
            conditions_str.push_str(&format!("{} = {}", key, value.as_str().unwrap()));
        }
    }
    Ok(conditions_str)
}
fn resolve_rows(
    rows: Vec<tokio_postgres::Row>,
) -> Result<Vec<Value>, Result<Vec<Value>, ReturnError>> {
    let mut results: Vec<Value> = vec![];
    for row in rows {
        let mut result: Value = Value::Object(Default::default());
        for (i, column) in row.columns().iter().enumerate() {
            let column_name = column.name();
            let column_type = column.type_();
            match column_type.name() {
                "int4" => {
                    let column_value: Option<i32> = row.get(i);
                    let column_value = match serde_json::to_value(column_value) {
                        Ok(value) => value,
                        Err(error) => {
                            return Err(Err(ReturnError::without_value(error.to_string())))
                        }
                    };
                    result[column_name] = column_value;
                }
                "text" => {
                    let column_value: Option<String> = row.get(i);
                    let column_value = match serde_json::to_value(column_value) {
                        Ok(value) => value,
                        Err(error) => {
                            return Err(Err(ReturnError::without_value(error.to_string())))
                        }
                    };
                    result[column_name] = column_value;
                }
                "varchar" => {
                    let column_value: Option<String> = row.get(i);
                    let column_value = match serde_json::to_value(column_value) {
                        Ok(value) => value,
                        Err(error) => {
                            return Err(Err(ReturnError::without_value(error.to_string())))
                        }
                    };
                    result[column_name] = column_value;
                }
                "bool" => {
                    let column_value: Option<bool> = row.get(i);
                    let column_value = match serde_json::to_value(column_value) {
                        Ok(value) => value,
                        Err(error) => {
                            return Err(Err(ReturnError::without_value(error.to_string())))
                        }
                    };
                    result[column_name] = column_value;
                }
                "timestamp" => {
                    let column_value: Option<chrono::NaiveDateTime> = row.get(i);
                    let column_value = match serde_json::to_value(column_value) {
                        Ok(value) => value,
                        Err(error) => {
                            return Err(Err(ReturnError::without_value(error.to_string())))
                        }
                    };
                    result[column_name] = column_value;
                }
                "float4" => {
                    let column_value: Option<f32> = row.get(i);
                    let column_value = match serde_json::to_value(column_value) {
                        Ok(value) => value,
                        Err(error) => {
                            return Err(Err(ReturnError::without_value(error.to_string())))
                        }
                    };
                    result[column_name] = column_value;
                }
                "float8" => {
                    let column_value: Option<f64> = row.get(i);
                    let column_value = match serde_json::to_value(column_value) {
                        Ok(value) => value,
                        Err(error) => {
                            return Err(Err(ReturnError::without_value(error.to_string())))
                        }
                    };
                    result[column_name] = column_value;
                }
                "json" => {
                    let column_value: Option<serde_json::Value> = row.get(i);
                    let column_value = match serde_json::to_value(column_value) {
                        Ok(value) => value,
                        Err(error) => {
                            return Err(Err(ReturnError::without_value(error.to_string())))
                        }
                    };
                    result[column_name] = column_value;
                }
                "jsonb" => {
                    let column_value: Option<serde_json::Value> = row.get(i);
                    let column_value = match serde_json::to_value(column_value) {
                        Ok(value) => value,
                        Err(error) => {
                            return Err(Err(ReturnError::without_value(error.to_string())))
                        }
                    };
                    result[column_name] = column_value;
                }
                "bytea" => {
                    let column_value: Option<Vec<u8>> = row.get(i);
                    let column_value = match serde_json::to_value(column_value) {
                        Ok(value) => value,
                        Err(error) => {
                            return Err(Err(ReturnError::without_value(error.to_string())))
                        }
                    };
                    result[column_name] = column_value;
                }
                _ => {
                    let column_value: Option<String> = row.get(i);
                    let column_value = match serde_json::to_value(column_value) {
                        Ok(value) => value,
                        Err(error) => {
                            return Err(Err(ReturnError::without_value(error.to_string())))
                        }
                    };
                    result[column_name] = column_value;
                }
            }
        }
        results.push(result);
    }
    Ok(results)
}
