use chrono::NaiveDateTime;
#[macro_use]
use serde::{Deserialize, Serialize};
use thiserror::Error;

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct TodoEntry {
    pub name: String,
    pub message: String,
    pub client_name: String,
    pub time_modified: NaiveDateTime,
    pub client_identifier: u64,
    pub image: Option<String>, //TODO Change this to an image type -> png?
}

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct Board {
    pub name: String,
    pub identifier: u64,
    pub entry_vector: Vec<TodoEntry>,
}

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct App {
    pub name: String,
    pub identifier: u64,
    api_token: Option<String>,
    pub list_vector: Vec<Board>,
    pub server: Option<Server>,
}

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct Server {
    pub url: String,
}

impl App {
    pub fn empty() -> Self {
        Self {
            name: "Default App".to_string(),
            identifier: 0,
            api_token: None,
            list_vector: Vec::new(),
            server: None,
        }
    }
}

#[derive(Debug, Error)]
pub enum NetworkError {
    #[error("Connection invald")]
    Connection,
    #[error("Invald Credentials")]
    InvalidCredentials,
    #[error("Invald API Token")]
    InvalidAPIToken,
}

pub fn sync_with_server(api_token: String, server: &Server) -> Result<Vec<Board>, NetworkError> {
    Err(NetworkError::Connection)
}

pub fn request_api_token(
    server: &Server,
    username: &str,
    password: &str,
) -> Result<String, NetworkError> {
    Err(NetworkError::InvalidCredentials)
}
pub fn revoke_api_token(server: &Server, api_token: String) -> Result<(), NetworkError> {
    Err(NetworkError::InvalidAPIToken)
}
