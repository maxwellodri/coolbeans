use thiserror::Error;

#[derive(Debug, Error)]
pub enum NetworkError {
    #[error("Connection invald")]
    Connection,
    #[error("Invald Credentials")]
    InvalidCredentials,
    #[error("Invald API Token")]
    InvalidAPIToken,
}
