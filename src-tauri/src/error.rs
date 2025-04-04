use thiserror::Error;
// 自定义错误类型的定义
#[derive(Error, Debug)]
pub enum MyError {
    #[error("{0}")]
    Anyhow(String),
}

/**
 * @description: 从anyhow::Error转换为MyError
 */
impl From<anyhow::Error> for MyError {
    fn from(error: anyhow::Error) -> Self {
        MyError::Anyhow(error.to_string())
    }
}

/**
 * @description: 实现serde::Serialize trait，用于序列化MyError为字符串
 */
impl serde::Serialize for MyError {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::ser::Serializer,
    {
        serializer.serialize_str(self.to_string().as_ref())
    }
}
