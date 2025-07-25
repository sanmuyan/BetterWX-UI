pub type Result<T> = core::result::Result<T, AppError>;
use config::errors::ConfigError;
use services::errors::ServicesError;
use thiserror::Error;
use utils::errors::UtilsError;
use winsys::errors::WinsysError;

#[derive(Error, Debug)]
pub enum AppError {
    #[error(transparent)]
    ConfigError(#[from] ConfigError),

    #[error(transparent)]
    ServicesError(#[from] ServicesError),

    #[error(transparent)]
    UtilsError(#[from] UtilsError),

    #[error(transparent)]
    WinsysError(#[from] WinsysError),

    #[error(transparent)]
    IoError(#[from] std::io::Error), 

    #[error("发生了错误")]
    SomeError,
}

impl serde::Serialize for AppError {
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::ser::Serializer,
    {   
        serializer.serialize_str(self.to_string().as_ref())
    }
}
