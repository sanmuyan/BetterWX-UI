use anyhow::Result;
use flexi_logger::{Cleanup, Criterion, Duplicate, FileSpec, Logger, Naming};

pub fn init(level: Option<&str>) -> Result<()> {
    if cfg!(debug_assertions) {
        return init_log("debug");
    }
    if let Some(level) = level {
        return match level {
            "debug" | "info" | "warn" | "error" | "trace" => init_log(level),
            "" => Ok(()),
            _ => Err(anyhow::anyhow!("无效的日志级别: {}", level)),
        };
    }
    Ok(())
}

pub fn init_log(level: &str) -> Result<()> {
    let path = std::env::current_exe()?
        .parent()
        .ok_or_else(|| std::io::Error::new(std::io::ErrorKind::Other, "无法获取程序所在目录"))?
        .to_path_buf();
    Logger::try_with_env_or_str(level)
        .unwrap()
        .log_to_file(
            FileSpec::default()
                .directory(path)
                .basename("BetterWX-UI")
                .suffix("log"),
        )
        .rotate(
            Criterion::Size(5_000000),
            Naming::Timestamps,
            Cleanup::KeepLogFiles(1),
        )
        .duplicate_to_stderr(Duplicate::Warn)
        .format_for_files(|w, now, record| {
            write!(
                w,
                "[{}] [{}] {}",
                now.now().format("%Y-%m-%d %H:%M:%S"),
                record.level(),
                record.args()
            )
        })
        .start()
        .map_err(|e| anyhow::anyhow!("初始化日志失败：{}", e))?;
    Ok(())
}
