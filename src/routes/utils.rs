pub mod reponses {
    use derive_more::{Display, Error};
    use serde::Serialize;
    #[derive(Serialize, Debug, Display, Error)]
    #[display(fmt = "{{\"error_msg\": \"{}\"}}", error_msg)]
    pub struct ReturnError<T> {
        pub error_msg: String,
        pub values: Option<T>,
    }
}
