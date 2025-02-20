use anyhow::Result;

use diesel::delete;
use diesel::insert_into;
use diesel::prelude::*;
use diesel::update;
use serde_json::json;

use super::structs::CreateTablePermission;
use super::structs::UpdateTablePermission;
use crate::controller::tables::table_controller::TableController;
use crate::controller::Controller;
use crate::controller::GenericValue;
use crate::controller::QueryParams;
use crate::controller::API_LIMIT;
use crate::models::cms::permission_model::DefaultPermissions;
use crate::models::cms::permission_model::PermissionType;
use crate::models::cms::permission_model::TablePermissions;
use crate::models::cms::permission_model::PERMISSION_TYPES;
use crate::models::db::connection::establish_connection;

use crate::routes::utils::reponses::ReturnError;
use crate::schema::tables_permissions::dsl as permissions_dsl;

pub struct TablePermissionsController;

impl DefaultPermissions for TablePermissions {
    fn default_permissions(table_id: i32) -> Vec<CreateTablePermission> {
        let mut permissions = Vec::new();
        for ele in PERMISSION_TYPES {
            let name = ele.0.to_string();
            let permission = PermissionType::from_string(&name);
            let permission = permission.unwrap();

            let allow = ele.1;

            permissions.push(CreateTablePermission::new(table_id, permission, allow));
        }
        permissions
    }
}

impl Controller<TablePermissions, CreateTablePermission> for TablePermissionsController {
    fn delete(id: i32) -> Result<TablePermissions, ReturnError> {
        let connection = &mut establish_connection();

        let transaction: std::result::Result<TablePermissions, ReturnError> = connection
            .transaction(|conn| {
                let query =
                    delete(permissions_dsl::tables_permissions).filter(permissions_dsl::id.eq(&id));

                query
                    .get_result::<TablePermissions>(conn)
                    .map_err(|err| ReturnError {
                        error_msg: err.to_string(),
                        values: Some(json!(id)),
                    })
            });

        transaction
    }
    fn create(permission: CreateTablePermission) -> Result<TablePermissions, ReturnError> {
        let permission = TablePermissions::from(permission);

        let table = TableController::find(permission.table_id);

        if table.is_err() {
            return Err(ReturnError {
                error_msg: "Table not found".to_string(),
                values: None,
            }
            .into());
        }
        let table = table.unwrap();
        let exists =
            TablePermissionsController::exists(permission.table_id, &permission.permission);
        if exists {
            return Err(ReturnError {
                error_msg: format!(
                    "Permission \"{}\" already exists for table \"{}\".",
                    &permission.permission, table.name
                ),
                values: None,
            }
            .into());
        }
        let connection = &mut establish_connection();

        let transaction: std::result::Result<TablePermissions, ReturnError> = connection
            .transaction(|conn| {
                let query = insert_into(permissions_dsl::tables_permissions).values(&permission);
                return query
                    .get_result::<TablePermissions>(conn)
                    .map_err(|res| ReturnError {
                        error_msg: res.to_string(),
                        values: Some(json!(permission)),
                    });
            });

        transaction
    }
    fn update(table_id: i32, permission: GenericValue) -> Result<TablePermissions, ReturnError> {
        // cast Any to Update
        let permission = permission.to::<UpdateTablePermission>();
        if permission.is_err() {
            return Err(ReturnError {
                error_msg: permission.unwrap_err().to_string(),
                values: None,
            }
            .into());
        }
        let permission = permission.unwrap();
        let connection = &mut establish_connection();
        match update(permissions_dsl::tables_permissions)
            .set(&permission)
            .filter(permissions_dsl::id.eq(table_id))
            .get_result::<TablePermissions>(connection)
        {
            Ok(res) => {
                return Ok(res); // if Successful, return the ID of the inserted table
            }
            Err(err) => {
                return Err(ReturnError {
                    error_msg: err.to_string(),
                    values: Some(serde_json::to_value(permission).unwrap()),
                }
                .into()); // if Successful, return the ID of the inserted table
            }
        }
    }
    fn find_all(query_params: QueryParams) -> Result<Vec<TablePermissions>, ReturnError> {
        let connection = &mut establish_connection();
        let mut query = permissions_dsl::tables_permissions.into_boxed();

        if let Some(id_query) = query_params.id {
            query = query.filter(permissions_dsl::id.eq(id_query)); // Search for a unique table
        };
        if let Some(limit) = query_params.limit {
            query = query.limit(limit); // Define user tables per page
        } else {
            query = query.limit(API_LIMIT) // Default limit
        }

        match query.load::<TablePermissions>(connection) {
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
    fn find(id: i32) -> Result<TablePermissions, ReturnError> {
        let connection: &mut PgConnection = &mut establish_connection();
        let mut query = permissions_dsl::tables_permissions.into_boxed();
        query = query.filter(permissions_dsl::id.eq(id)); // Search for a unique table
        match query.first::<TablePermissions>(connection) {
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

impl TablePermissionsController {
    pub fn find_by_table_id(table_id: i32) -> Result<Vec<TablePermissions>, ReturnError> {
        let connection = &mut establish_connection();
        let mut query = permissions_dsl::tables_permissions.into_boxed();

        query = query.filter(permissions_dsl::table_id.eq(table_id)); // Search for a unique table
        match query.load::<TablePermissions>(connection) {
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
    pub fn exists<S: AsRef<str>>(table_id: i32, permission_name: S) -> bool {
        let connection = &mut establish_connection();
        let mut query = permissions_dsl::tables_permissions.into_boxed();
        let permission_name = permission_name.as_ref();
        query = query
            .filter(permissions_dsl::table_id.eq(table_id))
            .filter(permissions_dsl::permission.eq(permission_name)); // Search for a unique table
        let result = query.first::<TablePermissions>(connection).is_ok();

        result
    }

    pub fn create_default(table_id: i32) -> Result<usize, ReturnError> {
        let connection = &mut establish_connection();
        let transaction = connection.transaction(|conn| {
            let values = TablePermissions::default_permissions(table_id);
            let query = insert_into(permissions_dsl::tables_permissions).values(&values);
            return query.execute(conn).map_err(|res| ReturnError {
                error_msg: res.to_string(),
                values: Some(serde_json::to_value(values).unwrap()),
            });
        });

        transaction
    }
}
