use diesel::{
    deserialize::{self, FromSql},
    pg::PgValue,
    serialize::{self, Output, ToSql},
    sql_types::{Text, VarChar},
    AsExpression, FromSqlRow,
};
use serde::{
    de::{self, Visitor},
    Deserialize, Deserializer, Serialize,
};

use std::fmt;

use crate::routes::utils::reponses::ReturnError;

#[derive(FromSqlRow, Debug, AsExpression, Serialize, Clone, Copy, PartialEq, Eq)]
#[diesel(sql_type = VarChar)]
#[serde(untagged)]
#[serde(rename_all_fields = "lowercase")]
pub enum FieldType {
    Varchar,
    Integer,
    Float,
    Boolean,
    Date,
    Time,
    Timestamp,
    Text,
    Json,
    Binary,
}

const TYPES: &[&str; 10] = &[
    "String",
    "Integer",
    "Float",
    "Boolean",
    "Date",
    "Time",
    "Timestamp",
    "Text",
    "Json",
    "Binary",
];

impl FieldType {
    pub fn to_pg_type(&self) -> String {
        match self {
            FieldType::Varchar => "varchar".to_string(),
            FieldType::Integer => "integer".to_string(),
            FieldType::Float => "float".to_string(),
            FieldType::Boolean => "bool".to_string(),
            FieldType::Date => "date".to_string(),
            FieldType::Time => "time".to_string(),
            FieldType::Timestamp => "timestamp".to_string(),
            FieldType::Text => "text".to_string(),
            FieldType::Json => "json".to_string(),
            FieldType::Binary => "bytea".to_string(),
        }
    }
    pub fn from_string(s: &str) -> Result<Self, ReturnError> {
        match s.to_lowercase().as_str() {
            "varchar" => Ok(FieldType::Varchar),
            "integer" => Ok(FieldType::Integer),
            "float" => Ok(FieldType::Float),
            "boolean" => Ok(FieldType::Boolean),
            "date" => Ok(FieldType::Date),
            "time" => Ok(FieldType::Time),
            "timestamp" => Ok(FieldType::Timestamp),
            "text" => Ok(FieldType::Text),
            "json" => Ok(FieldType::Json),
            "binary" => Ok(FieldType::Binary),
            e => Err(ReturnError::without_value(format!(
                "Invalid type `{}`, expected one of {}",
                e,
                TYPES
                    .into_iter()
                    .map(|x| format!("`{}`", x))
                    .collect::<Vec<String>>()
                    .join(", ")
            ))),
        }
    }
}

impl TryFrom<&str> for FieldType {
    type Error = String;

    fn try_from(value: &str) -> Result<Self, Self::Error> {
        Self::from_string(value).map_err(|e| e.to_string())
    }
}

impl ToString for FieldType {
    fn to_string(&self) -> String {
        match self {
            FieldType::Varchar => "String".to_string(),
            FieldType::Integer => "Integer".to_string(),
            FieldType::Float => "Float".to_string(),
            FieldType::Boolean => "Boolean".to_string(),
            FieldType::Date => "Date".to_string(),
            FieldType::Time => "Time".to_string(),
            FieldType::Timestamp => "Timestamp".to_string(),
            FieldType::Text => "Text".to_string(),
            FieldType::Json => "Json".to_string(),
            FieldType::Binary => "Binary".to_string(),
        }
    }
}

impl ToSql<Text, diesel::pg::Pg> for FieldType
where
    String: ToSql<Text, diesel::pg::Pg>,
{
    fn to_sql<'b>(&'b self, out: &mut Output<'b, '_, diesel::pg::Pg>) -> serialize::Result {
        let v = self.to_string();
        <String as ToSql<Text, diesel::pg::Pg>>::to_sql(&v, &mut out.reborrow())
    }
}

impl FromSql<Text, diesel::pg::Pg> for FieldType {
    fn from_sql(bytes: PgValue<'_>) -> deserialize::Result<Self> {
        let t = <String as FromSql<Text, diesel::pg::Pg>>::from_sql(bytes)?;
        let value = FieldType::from_string(serde_json::from_str(&t)?)?;
        Ok(value)
    }
}

impl<'de> Deserialize<'de> for FieldType {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        struct FieldTypeVisitor;

        impl<'de> Visitor<'de> for FieldTypeVisitor {
            type Value = FieldType;

            fn expecting(&self, formatter: &mut fmt::Formatter) -> fmt::Result {
                formatter.write_str("a valid FieldType variant")
            }

            fn visit_str<E>(self, value: &str) -> Result<Self::Value, E>
            where
                E: de::Error,
            {
                match value.to_lowercase().as_str() {
                    "string" => Ok(FieldType::Varchar),
                    "integer" => Ok(FieldType::Integer),
                    "float" => Ok(FieldType::Float),
                    "boolean" => Ok(FieldType::Boolean),
                    "date" => Ok(FieldType::Date),
                    "time" => Ok(FieldType::Time),
                    "timestamp" => Ok(FieldType::Timestamp),
                    "text" => Ok(FieldType::Text),
                    "json" => Ok(FieldType::Json),
                    "binary" => Ok(FieldType::Binary),
                    _ => Err(de::Error::custom(format!(
                        "Invalid type `{}`, expected one of {}",
                        value,
                        TYPES
                            .into_iter()
                            .map(|x| format!("`{}`", x))
                            .collect::<Vec<String>>()
                            .join(", ")
                    ))),
                }
            }
        }

        deserializer.deserialize_any(FieldTypeVisitor)
    }
}

impl Into<String> for FieldType {
    fn into(self) -> String {
        match self {
            FieldType::Varchar => "string".to_string(),
            FieldType::Integer => "integer".to_string(),
            FieldType::Float => "float".to_string(),
            FieldType::Boolean => "boolean".to_string(),
            FieldType::Date => "date".to_string(),
            FieldType::Time => "time".to_string(),
            FieldType::Timestamp => "Timestamp".to_string(),
            FieldType::Text => "text".to_string(),
            FieldType::Json => "json".to_string(),
            FieldType::Binary => "binary".to_string(),
        }
    }
}

impl Into<FieldType> for String {
    fn into(self) -> FieldType {
        match self.to_lowercase().as_str() {
            "string" => FieldType::Varchar,
            "integer" => FieldType::Integer,
            "float" => FieldType::Float,
            "boolean" => FieldType::Boolean,
            "date" => FieldType::Date,
            "time" => FieldType::Time,
            "timestamp" => FieldType::Timestamp,
            "text" => FieldType::Text,
            "json" => FieldType::Json,
            "binary" => FieldType::Binary,
            _ => panic!("Invalid type"),
        }
    }
}
