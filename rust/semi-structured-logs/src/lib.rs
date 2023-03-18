/// various log levels
#[derive(Clone, PartialEq, Eq, Debug)]
pub enum LogLevel {
    Info,
    Warning,
    Error,
    Debug,
}

fn level_to_str(level: LogLevel) -> &'static str {
    return match level {
        LogLevel::Info => "INFO",
        LogLevel::Warning => "WARNING",
        LogLevel::Error => "ERROR",
        LogLevel::Debug => "DEBUG",
    };
}

/// primary function for emitting logs
pub fn log(level: LogLevel, message: &str) -> String {
    return format!("[{}]: {}", level_to_str(level), message);
}

pub fn info(message: &str) -> String {
    return log(LogLevel::Info, message);
}

pub fn warn(message: &str) -> String {
    return log(LogLevel::Warning, message);
}

pub fn error(message: &str) -> String {
    return log(LogLevel::Error, message);
}
