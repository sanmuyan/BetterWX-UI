use config::errors::ConfigError;
use thiserror::Error;
use utils::errors::UtilsError;
use winsys::errors::WinsysError;
pub type Result<T> = core::result::Result<T, ServicesError>;

#[derive(Debug, Error)]
pub enum ServicesError {
    #[error("获取配置错误")]
    GetConfigError,

    #[error("获取配置初始化错误{0}")]
    ConfigInitError(String),

    #[error("序列化错误: {0}")]
    SerializationError(#[from] serde_json::Error),

    #[error("关闭app失败，无效的路径或者不是有效的exe文件，路径：{0}")]
    CloseAppFailed(String),

    #[error("运行App列表为空")]
    RunAppListIsEmpty,

    #[error("运行App失败，路径：{0}")]
    RunAppFailed(String),

    #[error("发现新版本：{0}，请升级")]
    ForceUpdate(String),

    #[error("计划运行 {0} 个app，运行成功 {1} 了，请重试")]
    RunAppError(usize,usize),
    
    #[error(transparent)]
    IoError(#[from] std::io::Error),

    #[error(transparent)]
    WinsysError(#[from] WinsysError),

    #[error(transparent)]
    UtilsError(#[from] UtilsError),

    #[error(transparent)]
    ConfigError(#[from] ConfigError),
}
