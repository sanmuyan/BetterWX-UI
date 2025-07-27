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

    #[error("计划启动 {0} 个，成功启动 {1} 个，请重试。失败原因：{2}")]
    RunAppError(usize,usize,String),

    #[error("一键启动失败，排列窗口失败，请重试。")]
    ArrangeWindowError,

    #[error("一键启动失败，发送点击事件失败，请重试。")]
    SendClickEventError,

    #[error("未知错误")]
    UnkonwError,
    
    #[error(transparent)]
    IoError(#[from] std::io::Error),

    #[error(transparent)]
    WinsysError(#[from] WinsysError),

    #[error(transparent)]
    UtilsError(#[from] UtilsError),

    #[error(transparent)]
    ConfigError(#[from] ConfigError),
}
