use std::fs;
use std::ops::Deref;
use fern::Dispatch;
use fern::colors::Color;
use fern::colors::ColoredLevelConfig;
use log::LevelFilter;
use thiserror::Error;

pub type Result<T> = core::result::Result<T, Box<dyn std::error::Error>>;

const LOGGER_FILE_NAME: &str = "BetterWX-UI.log";

#[derive(Debug, Error)]
pub enum LoggerError {
    #[error("无效的日志级别")]
    LoggerLevelInvalid(String),

    #[error("初始化日志失败:{0}")]
    LoggerInifFailed(String),
}

pub fn init<S:Deref<Target = str>>(level: Option<S>) -> Result<()> {
    if let Some(level) = level {
       let _ = match level.to_lowercase().as_str() {
            "debug" => init_log(LevelFilter::Debug),
            "info" => init_log(LevelFilter::Info),
            "warn" => init_log(LevelFilter::Warn),
            "error" => init_log(LevelFilter::Error),
            "trace" => init_log(LevelFilter::Trace),
            _ => Ok(()),
        };
    }
    Ok(())
}

pub fn init_log(level: LevelFilter) -> Result<()> {
    println!("初始化日志级别: {:?}", level);
    let colors = ColoredLevelConfig::new()
        .error(Color::Red)
        .warn(Color::Yellow)
        .info(Color::Green)
        .debug(Color::Blue)
        .trace(Color::Cyan);
    
    let path = std::env::current_dir()?.join(LOGGER_FILE_NAME);
    if path.exists() {
        fs::remove_file(&path)?;
    }
    let logger_file = fern::log_file(path)?;

    let file_logger = Dispatch::new()
        .format(move |out, message, record| {
            out.finish(format_args!(
                "[{}] {}",
                record.level(),
                message
            ))
        })
        .chain(logger_file);

    let console_logger = Dispatch::new()
        .format(move |out, message, record| {
            out.finish(format_args!(
                "[{}] [{}:{}] {}",
                colors.color(record.level()),
                record.file().unwrap_or("UNKONW"),
                record.line().unwrap_or(0),
                message
            ))
        })
        .chain(std::io::stderr());

    Dispatch::new()
        .level(level)
        .chain(file_logger)
        .chain(console_logger)
        .apply()?;
    Ok(())
}

