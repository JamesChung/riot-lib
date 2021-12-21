use reqwest::StatusCode;

#[derive(Debug)]
pub enum Error {
    Message(String),
    StatusCode(StatusCode),
}

impl Error {
    pub fn new_message(msg: &str) -> Self {
        Self::Message(msg.to_string())
    }
}
