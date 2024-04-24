use serde::Deserialize;

#[rustfmt::skip]
#[derive(Debug, Deserialize)]
pub struct CodeMessage {
    pub code: u32,
    pub message: String
}

#[rustfmt::skip]
#[derive(Debug, Deserialize)]
pub struct Response<T> {
    pub success:     bool,
    pub errors:      Vec<CodeMessage>,
    pub messages:    Option<Vec<CodeMessage>>,
    pub result:      Option<T>,
}
