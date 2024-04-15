use crate::error::ToCrateError;

pub type Result<T> = std::result::Result<T, crate::error::Error>;

pub trait ToCrateResult<T> {
    fn to_crate_result(self) -> Result<T>;
}

impl<T, E: std::error::Error + std::fmt::Debug + std::fmt::Display + 'static> ToCrateResult<T>
    for std::result::Result<T, E>
{
    fn to_crate_result(self) -> Result<T> {
        match self {
            Ok(t) => Result::Ok(t),
            Err(e) => Result::Err(e.to_crate_error()),
        }
    }
}

// Re-exports
pub use crate::error::Error;
