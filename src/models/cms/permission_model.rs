use crate::controller::tables::permissions::structs::CreateTablePermission;

use super::table_model::Table;
use diesel::prelude::*;
use serde::{Deserialize, Serialize};

use diesel::Insertable;
use diesel::{
    deserialize::{self, FromSql, FromSqlRow},
    pg::PgValue,
    serialize::{self, Output, ToSql},
    sql_types::{Text, VarChar},
    AsExpression,
};
use serde::{
    de::{self, Visitor},
    Deserializer,
};

use crate::routes::utils::reponses::ReturnError;
use std::fmt;
#[derive(
    Identifiable,
    Associations,
    Queryable,
    PartialEq,
    Debug,
    Selectable,
    Serialize,
    Deserialize,
    Insertable,
    Clone,
)]
#[diesel(table_name = crate::schema::tables_permissions)]
#[diesel(check_for_backend(diesel::pg::Pg))]
#[serde(rename_all = "camelCase")]
#[diesel(belongs_to(Table))]
pub struct TablePermissions {
    pub id: i32,
    pub table_id: i32,
    pub permission: String,
    pub allow: bool,
}

impl TablePermissions {
    pub fn from(create: CreateTablePermission) -> TablePermissions {
        TablePermissions {
            id: 0,
            table_id: create.table_id.unwrap(),
            permission: create.permission.to_string(),
            allow: create.allow,
        }
    }

    pub fn check(permissions: Vec<TablePermissions>, permission: PermissionType) -> bool {
        let mut allow = false;
        for p in permissions {
            let table_permission = PermissionType::from_string(&p.permission).unwrap();
            if table_permission == permission {
                allow = p.allow;
            }
        }
        allow
    }

    pub fn resolve_method<S: AsRef<str>>(method: S) -> Result<PermissionType, ReturnError> {
        let method = method.as_ref().to_uppercase();
        for (sql, http) in PERMISSION_TYPES_HTTP_EQUIVALENT {
            if http == &method {
                return Ok(PermissionType::from_string(sql).unwrap());
            }
        }
        Err(ReturnError::without_value(format!(
            "Invalid method `{}`, expected one of {}",
            method,
            PERMISSION_TYPES_HTTP_EQUIVALENT
                .into_iter()
                .map(|x| format!("`{}`", x.1))
                .collect::<Vec<String>>()
                .join(", ")
        )))
    }
}

pub const PERMISSION_TYPES_HTTP_EQUIVALENT: [(&str, &str); 5] = [
    ("QUERY", "GET"),
    ("CREATE", "POST"),
    ("REPLACE", "PUT"),
    ("UPDATE", "PATCH"),
    ("DELETE", "DELETE"),
];

pub const PERMISSION_TYPES: &[(&str, bool); 5] = &[
    ("QUERY", true),
    ("CREATE", true),
    ("REPLACE", true),
    ("UPDATE", true),
    ("DELETE", true),
];

#[derive(FromSqlRow, Debug, AsExpression, Clone, Copy, PartialEq, Eq)]
#[diesel(sql_type = VarChar)]
pub enum PermissionType {
    Create,
    Replace,
    Update,
    Delete,
    Query,
}

impl Serialize for PermissionType {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        serializer.serialize_str(&self.to_string())
    }
}
impl PermissionType {
    pub fn from_string(s: &str) -> Result<Self, ReturnError> {
        match s.to_lowercase().as_str() {
            "create" => Ok(PermissionType::Create),
            "replace" => Ok(PermissionType::Replace),
            "update" => Ok(PermissionType::Update),
            "delete" => Ok(PermissionType::Delete),
            "query" => Ok(PermissionType::Query),
            e => Err(ReturnError::without_value(format!(
                "Invalid type `{}`, expected one of {}",
                e,
                PERMISSION_TYPES
                    .into_iter()
                    .map(|x| format!("`{}`", x.0))
                    .collect::<Vec<String>>()
                    .join(", ")
            ))),
        }
    }
}

impl TryFrom<&str> for PermissionType {
    type Error = String;

    fn try_from(value: &str) -> Result<Self, Self::Error> {
        Self::from_string(value).map_err(|e| e.to_string())
    }
}

impl ToString for PermissionType {
    fn to_string(&self) -> String {
        match self {
            PermissionType::Create => "Create".to_string(),
            PermissionType::Replace => "Replace".to_string(),
            PermissionType::Update => "Update".to_string(),
            PermissionType::Delete => "Delete".to_string(),
            PermissionType::Query => "Query".to_string(),
        }
    }
}

impl ToSql<Text, diesel::pg::Pg> for PermissionType
where
    String: ToSql<Text, diesel::pg::Pg>,
{
    fn to_sql<'b>(&'b self, out: &mut Output<'b, '_, diesel::pg::Pg>) -> serialize::Result {
        let v = self.to_string();
        <String as ToSql<Text, diesel::pg::Pg>>::to_sql(&v, &mut out.reborrow())
    }
}

impl FromSql<Text, diesel::pg::Pg> for PermissionType {
    fn from_sql(bytes: PgValue<'_>) -> deserialize::Result<Self> {
        let t = <String as FromSql<Text, diesel::pg::Pg>>::from_sql(bytes)?;
        let value = PermissionType::from_string(serde_json::from_str(&t)?)?;
        Ok(value)
    }
}

impl<'de> Deserialize<'de> for PermissionType {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        struct PermissionTypeVisitor;

        impl<'de> Visitor<'de> for PermissionTypeVisitor {
            type Value = PermissionType;

            fn expecting(&self, formatter: &mut fmt::Formatter) -> fmt::Result {
                formatter.write_str("a valid PermissionType variant")
            }

            fn visit_str<E>(self, value: &str) -> Result<Self::Value, E>
            where
                E: de::Error,
            {
                match value.to_lowercase().as_str() {
                    "create" => Ok(PermissionType::Create),
                    "replace" => Ok(PermissionType::Replace),
                    "update" => Ok(PermissionType::Update),
                    "delete" => Ok(PermissionType::Delete),
                    "query" => Ok(PermissionType::Query),
                    _ => Err(de::Error::custom(format!(
                        "Invalid type `{}`, expected one of {}",
                        value,
                        PERMISSION_TYPES
                            .into_iter()
                            .map(|x| format!("`{}`", x.0))
                            .collect::<Vec<String>>()
                            .join(", ")
                    ))),
                }
            }
        }

        deserializer.deserialize_any(PermissionTypeVisitor)
    }
}

impl Into<String> for PermissionType {
    fn into(self) -> String {
        PermissionType::to_string(&self)
    }
}

impl Into<PermissionType> for String {
    fn into(self) -> PermissionType {
        let permission = PermissionType::from_string(self.to_lowercase().as_str());
        match permission {
            Ok(permission) => permission,
            Err(e) => {
                panic!("{}", e);
            }
        }
    }
}

pub trait DefaultPermissions {
    fn default_permissions(table_id: i32) -> Vec<CreateTablePermission>;
}
