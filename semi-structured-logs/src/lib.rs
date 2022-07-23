/// Const for message
const INFO: &str = "[INFO]: ";
const WARNING: &str = "[WARNING]: ";
const ERROR: &str = "[ERROR]: ";

/// various log levels
#[derive(Clone, PartialEq, Eq, Debug)]
pub enum LogLevel {
    Info,
    Warning,
    Error,
}

/// primary function for emitting logs
pub fn log(level: LogLevel, message: &str) -> String {
    (match level {

        LogLevel::Info => INFO,
        LogLevel::Warning => WARNING,
        LogLevel::Error => ERROR,

    }).to_owned() + message
}

pub fn info(message: &str) -> String {
    INFO.to_owned() + message
}

pub fn warn(message: &str) -> String {
    WARNING.to_owned() + message
}

pub fn error(message: &str) -> String {
    ERROR.to_owned() + message
}