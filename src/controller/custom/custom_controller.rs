use futures::TryStreamExt;
use serde_json::Value;
use tokio_postgres::types::ToSql;
use tokio_postgres::Row;

use super::structs::QueryParams;

use crate::controller::fields::field_controller::FieldController;
use crate::controller::tables::table_controller::TableController;
use crate::models::db::driver_connection::establish_driver_connection;

use crate::routes::utils::reponses::ReturnError;

pub struct CustomController;

impl CustomController {
    pub async fn find_one(
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

    pub async fn create(query_params: QueryParams, data: Value) -> Result<Vec<Value>, ReturnError> {
        if !data.is_object() {
            return Err(ReturnError::without_value("Invalid data".to_owned()));
        }
        let data = data.as_object().unwrap();

        let table_name = query_params.table_name;
        if table_name.is_none() {
            return Err(ReturnError::without_value(
                "Table name is required".to_owned(),
            ));
        }

        let table_name = table_name.unwrap();
        let client = match establish_driver_connection().await {
            Ok(client) => client,
            Err(_) => {
                return Err(ReturnError::without_value(
                    "Error connecting to database".to_owned(),
                ));
            }
        };
        let columns: Vec<&str> = data.keys().into_iter().map(|x| x.as_str()).collect();
        let placeholders: Vec<String> = (1..=data.len()).map(|i| format!("${}", i)).collect();

        let query = format!("INSERT INTO {} ({})", table_name, columns.join(","));

        let query = format!("{query} VALUES ({}) RETURNING *;", placeholders.join(", "));

        let mut inputs: Vec<Box<dyn ToSql + Sync>> = Vec::new();

        for ele in data.values() {
            match ele {
                Value::String(value) => inputs.push(Box::new(value.clone())),
                Value::Number(value) => inputs.push(Box::new(value.as_f64().unwrap())),
                Value::Bool(value) => inputs.push(Box::new(value)),
                Value::Null => inputs.push(Box::new(None::<String>)),
                _ => inputs.push(Box::new("".to_string())),
            };
        }
        let inputs: Vec<_> = inputs.into_iter().map(|x| x).collect();

        let query = client.query_raw(&query, inputs).await;
        match query {
            Ok(res) => {
                let rows = match res.try_collect::<Vec<Row>>().await {
                    Ok(rows) => rows,
                    Err(err) => {
                        return Err(ReturnError::new(err.to_string(), 1));
                    }
                };
                let results = match resolve_rows(rows) {
                    Ok(value) => value,
                    Err(value) => return value,
                };
                return Ok(results);
            }
            Err(err) => {
                return Err(ReturnError::new(err.to_string(), 1));
            }
        }
    }
    pub async fn find_all(
        table_name: String,
        query_params: QueryParams,
    ) -> Result<Vec<Value>, ReturnError> {
        let fields = FieldController::find_all_by_table_name(&table_name);

        if fields.is_err() {
            return Err(ReturnError::without_value(format!(
                "Table \"{table_name}\" not found"
            )));
        }
        let conditions_str = match get_conditions(&query_params) {
            Ok(value) => value,
            Err(value) => return value,
        };

        let client = match establish_driver_connection().await {
            Ok(client) => client,
            Err(_) => {
                return Err(ReturnError::without_value(
                    "Error connecting to database".to_owned(),
                ));
            }
        };

        let query = format!("SELECT * FROM {} {}", table_name, conditions_str);
        println!("{}", query);
        match client.query(&query, &[]).await {
            Ok(rows) => {
                let results = match resolve_rows(rows) {
                    Ok(value) => value,
                    Err(value) => {
                        let value = value;
                        return Err(value.unwrap_err());
                    }
                };
                if results.len() == 0 {
                    return Err(ReturnError::without_value(format!(
                        "No results found for table \"{table_name}\""
                    )));
                }
                return Ok(results);
            }
            Err(err) => {
                return Err(ReturnError {
                    error_msg: err.to_string(),
                    values: Some(table_name.into()),
                }
                .into());
            }
        }
    }
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
