use std::fmt::Debug;

use diesel::{
    backend::Backend,
    deserialize::{self, FromSql},
    row::NamedRow,
    sql_types::Json,
    QueryableByName,
};
use serde::{de::DeserializeOwned, Deserialize, Serialize};
use serde_json::{Map, Number, Value};

use crate::{routes::utils::reponses::ReturnError, utils::string_utils::to_camel_case};

pub mod custom;
pub mod db;
pub mod deno;
pub mod fields;
pub mod login;
pub mod posts;
pub mod tables;
pub mod users;
pub mod utils;

pub type Result<T, E = ReturnError> = core::result::Result<T, E>;

pub trait Controller<ReturnType, Create = GenericValue, Update = GenericValue>
where
    ReturnType: Serialize + Deserialize<'static>,
{
    fn delete(id: i32) -> Result<ReturnType>;
    fn create(new_data: Create) -> Result<ReturnType>;
    fn update(id: i32, new_data: Update) -> Result<ReturnType>;
    fn find_all(query_params: QueryParams) -> Result<Vec<ReturnType>>;
    fn find(id: i32) -> Result<ReturnType>;
}

#[derive(Debug, Deserialize, Serialize, Clone)]
pub struct GenericValue(pub Value);

impl<DB> QueryableByName<DB> for GenericValue
where
    DB: Backend,
    Value: FromSql<Json, DB>,
{
    fn build<'a>(row: &impl NamedRow<'a, DB>) -> deserialize::Result<Self> {
        // Row::
        let internal_table_name: Value = NamedRow::get::<Json, _>(row, "row")?;
        let map: &serde_json::Map<String, Value> = internal_table_name.as_object().unwrap();
        let mut new_map = serde_json::Map::new();
        // Convert snake case to camel case
        for (key, value) in map {
            let camel_case_key = to_camel_case(&key);
            new_map.insert(camel_case_key, value.clone());
        }
        let internal_table_name = serde_json::to_value(new_map).unwrap();
        let value = GenericValue(internal_table_name);
        Ok(value)
    }
}

impl GenericValue {
    pub fn to<T: Serialize + DeserializeOwned>(&self) -> Result<T, ReturnError> {
        let value = self.0.clone();
        serde_json::from_value(value)
            .map_err(|e| ReturnError::new(e.to_string(), serde_json::to_value(&self.0).unwrap()))
    }
    pub fn from<T: Serialize>(value: T) -> Result<Self, ReturnError> {
        let value =
            serde_json::to_value(value).map_err(|e| ReturnError::without_value(e.to_string()));

        value.map(|v| GenericValue(v))
    }
}

pub const API_LIMIT: i64 = 100;
#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(rename_all(serialize = "camelCase"))]
pub struct QueryParams {
    pub id: Option<i32>,
    pub limit: Option<i64>,
    // aditional props
    #[serde(flatten)]
    pub extra: Map<String, Value>,
}

impl QueryParams {
    pub fn new(id: Option<i32>, limit: Option<i64>) -> Self {
        Self {
            id,
            limit,
            extra: Map::new(),
        }
    }
    pub fn from_json(json: &Value) -> Result<Self, ReturnError> {
        let mut params = Self::new(None, None);
        let mut extra = Map::new();
        if let Some(id) = json.get("id") {
            params.id = Some(id.as_i64().unwrap() as i32);
        }
        if let Some(limit) = json.get("limit") {
            params.limit = Some(limit.as_i64().unwrap());
        }
        for (key, value) in json.as_object().unwrap() {
            if key == "id" || key == "limit" {
                continue;
            }
            extra.insert(key.to_string(), value.clone());
        }
        params.extra = extra;
        Ok(params)
    }
    pub fn to_json(&self) -> Value {
        let mut json = serde_json::to_value(self).unwrap();
        for (key, value) in &self.extra {
            json[key] = value.clone();
        }
        json
    }

    pub fn convert_extra_values(&mut self) {
        for (key, value) in &self.extra.clone() {
            if Self::is_array(key) {
                let value = Self::get_array_value(value);
                println!("{} is array", serde_json::to_string(&value).unwrap());
                let _ = &mut self.extra.insert(key.to_string(), Value::Array(value));
                continue;
            }

            if Self::is_i64(value) {
                let _ = &mut self
                    .extra
                    .insert(key.to_string(), Value::Number(Self::to_i64(value).into()));
                continue;
            }

            if Self::is_f64(value) {
                let _ = &mut self.extra.insert(
                    key.to_string(),
                    Number::from_f64(Self::to_f64(value)).into(),
                );
                continue;
            }
            if Self::is_bool(value) {
                let _ = &mut self
                    .extra
                    .insert(key.to_string(), Value::Bool(Self::to_bool(value)));
                continue;
            }

            let _ = &mut self.extra.insert(key.to_string(), value.clone());
        }
    }

    pub fn get_extras(&mut self) -> Map<String, Value> {
        for (key, value) in &self.extra.clone() {
            if Self::is_array(key) {
                let value = Self::get_array_value(value);
                let _ = &mut self.extra.insert(key.to_string(), Value::Array(value));
                continue;
            }

            if Self::is_i64(value) {
                let _ = &mut self
                    .extra
                    .insert(key.to_string(), Value::Number(Self::to_i64(value).into()));
                continue;
            }

            if Self::is_f64(value) {
                let _ = &mut self.extra.insert(
                    key.to_string(),
                    Number::from_f64(Self::to_f64(value)).into(),
                );
                continue;
            }
            if Self::is_bool(value) {
                let _ = &mut self
                    .extra
                    .insert(key.to_string(), Value::Bool(Self::to_bool(value)));
                continue;
            }
            let _ = &mut self.extra.insert(key.to_string(), value.clone());
        }
        let _ = &mut self.keys_to_lowercase();
        return self.extra.clone();
    }

    pub fn is_array(key: &str) -> bool {
        key.ends_with("[]")
    }
    pub fn get_array_key(key: &str) -> String {
        key.trim_end_matches("[]").to_string()
    }
    pub fn get_array_value(value: &Value) -> Vec<Value> {
        let value = value.as_str().unwrap();
        // Remove first and last character
        let value = &value[1..value.len() - 1];
        let value: Vec<Value> = value
            .split(",")
            .map(|x| Value::String(x.to_string()))
            .collect::<Vec<Value>>();
        value
    }
    pub fn is_i64(value: &Value) -> bool {
        if value.as_i64().is_some() {
            return true;
        } else {
            value.as_str().map_or(false, |x| x.parse::<i64>().is_ok())
        }
    }
    pub fn to_i64(value: &Value) -> i64 {
        value
            .as_i64()
            .unwrap_or_else(|| value.as_str().unwrap().parse::<i64>().unwrap())
    }
    pub fn to_i32(value: &Value) -> i32 {
        Self::to_i64(value) as i32
    }

    pub fn is_f64(value: &Value) -> bool {
        value.as_f64().is_some()
    }
    pub fn is_i32(value: &Value) -> bool {
        Self::is_f64(value)
    }
    pub fn to_f64(value: &Value) -> f64 {
        value.as_f64().unwrap()
    }
    pub fn to_f32(value: &Value) -> f32 {
        Self::to_f64(value) as f32
    }

    pub fn is_bool(value: &Value) -> bool {
        value.as_bool().is_some()
    }
    pub fn to_bool(value: &Value) -> bool {
        value.as_bool().unwrap()
    }

    pub fn keys_to_lowercase(&mut self) {
        let mut extra = Map::new();
        for (key, value) in &self.extra {
            let key = key.to_lowercase();
            extra.insert(key, value.clone());
        }
        self.extra = extra;
    }
}

pub struct RouteAuth {
    pub get: bool,
    pub post: bool,
    pub delete: bool,
    pub patch: bool,

    pub put: bool,
    pub head: bool,
    pub options: bool,
    pub trace: bool,
    pub connect: bool,
}