use std::io::Error as IOError;

#[derive(Debug, thiserror::Error)]
pub enum Error {
    #[error("IO: {0}")]
    IO(#[from] IOError),

    #[error("Json: {0}")]
    Json(#[from] serde_json::Error),
}
