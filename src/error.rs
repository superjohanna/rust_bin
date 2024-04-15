#[derive(Debug, thiserror::Error)]
pub enum Error {
    #[error("Generic error '{0}'")]
    Generic(String),
    #[error("3rd party error '{0}'")]
    Other(Box<dyn std::error::Error>),
}

pub trait ToCrateError {
    fn to_crate_error(self) -> Error;
}

impl<T: std::error::Error + std::fmt::Debug + std::fmt::Display + 'static> ToCrateError for T {
    fn to_crate_error(self) -> Error {
        Error::Other(Box::new(self))
    }
}

impl From<&str> for Error {
    fn from(value: &str) -> Self {
        Self::Generic(value.to_string())
    }
}
