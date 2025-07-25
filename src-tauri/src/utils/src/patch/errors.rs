use thiserror::Error;

#[derive(Debug, Error)]
pub enum UPatchError {
    #[error("使用mmap打开文件失败，文件：{0}")]
    ReadWithMmapError(String),

    #[error("使用mmapMut打开文件失败，文件：{0}")]
    WriteWithMmapMutError(String),

    #[error("只读模式打开无法修改数据")]
    ReadOnlyError,

    #[error("写入位置超出文件大小")]
    OutRangePos1Error,

    #[error("写入位置超出文件大小")]
    OutRangePos2Error,

    #[error("无效特征码")]
    PatternBuilderError,

    #[error("未搜索到特征码")]
    PatternNotFindError,

    #[error("找不到{0}缓存")]
    PatchWithCacheNotFind(String),

    #[error("无效的utf8数据")]
    InvalidUtf8Data,

    #[error("无效的16进制数据")]
    InvalidHexData,
}
