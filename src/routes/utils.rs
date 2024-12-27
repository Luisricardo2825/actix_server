pub mod reponses {

    use derive_more::{Display, Error};
    use diesel::result::Error as DieselError;
    use serde::Serialize;
    use serde_json::Value;
    #[derive(Serialize, Debug, Display, Error, Clone)]
    #[display("{}", error_msg)]
    pub struct ReturnError {
        pub error_msg: String,
        pub values: Option<Value>,
    }

    impl ReturnError {
        pub fn new<T: Serialize>(error_msg: String, values: T) -> Self {
            let values = Some(serde_json::to_value(values).unwrap());
            Self { error_msg, values }
        }

        pub fn without_value(error_msg: String) -> Self {
            // Put null
            let values = Some(serde_json::Value::Null);
            Self { error_msg, values }
        }

        pub fn from<T: Serialize, E: Error>(error: E, values: T) -> Self {
            let values = Some(serde_json::to_value(values).unwrap());
            let error_msg = format!("{}", error.to_string());
            Self { error_msg, values }
        }
    }

    impl From<DieselError> for ReturnError {
        fn from(value: DieselError) -> Self {
            value.into()
        }
    }
}
