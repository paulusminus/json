/// Error for writing, reading or converting json
#[derive(Debug)]
pub enum Error {
    IO(std::io::Error),
    Json(serde_json::Error),
}

impl From<std::io::Error> for Error {
    fn from(io_error: std::io::Error) -> Self {
        Self::IO(io_error)
    }
}

impl From<serde_json::Error> for Error {
    fn from(json_error: serde_json::Error) -> Self {
        Self::Json(json_error)
    }
}

impl std::fmt::Display for Error {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let message = match self {
            Self::IO(io_error) => format!("IO error: {}", io_error),
            Self::Json(json_error) => format!("Json error: {}", json_error),
        };
        write!(f, "{}", message)
    }
}

impl std::error::Error for Error {}

pub(crate) trait ErrInto<T> {
    fn err_into(self) -> crate::Result<T>;
}

impl<T, E: Into<Error>> ErrInto<T> for Result<T, E> {
    fn err_into(self) -> crate::Result<T> {
        self.map_err(Into::into)
    }
}
