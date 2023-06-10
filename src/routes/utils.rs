pub mod reponses {
    use serde::Serialize;

    #[derive(Serialize)]
    pub struct ReturnError<T> {
        pub error_msg: String,
        pub values: Option<T>,
    }
}
