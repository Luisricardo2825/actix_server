use std::fmt::Debug;

use serde::{Deserialize, Serialize};

use crate::routes::utils::reponses::ReturnError;

pub mod custom;
pub mod db;
pub mod deno;
pub mod login;
pub mod posts;
pub mod users;
pub mod fields;
pub mod tables;

pub type Result<T, E = ReturnError> = core::result::Result<T, E>;

pub trait Controller<ReturnType, QueryParams, Create, Update>
where
    QueryParams: Serialize + Deserialize<'static> + Debug + Clone,
    Create: Serialize + Deserialize<'static>,
    Update: Serialize + Deserialize<'static>,
    ReturnType: Serialize + Deserialize<'static>,
{
    fn delete(id: i32) -> Result<ReturnType>;
    fn create(new_data: Create) -> Result<ReturnType>;
    fn update(id: i32, new_data: Update) -> Result<ReturnType>;
    fn find_all(query_params: QueryParams) -> Result<Vec<ReturnType>>;
    fn find(id: i32) -> Result<ReturnType>;
}


