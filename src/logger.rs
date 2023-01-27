use serde::Deserialize;
use serde::Serialize;

use crate::wrap::imported::logger_log_level::*;
use crate::wrap::imported::logger_module::*;

pub fn print(msg: String) -> () {
    LoggerModule::log(&ArgsLog {
        message: msg,
        level: LoggerLogLevel::INFO,
    }).unwrap();
}

pub fn print_error(msg: String) -> () {
    LoggerModule::log(&ArgsLog {
        message: msg,
        level: LoggerLogLevel::ERROR,
    }).unwrap();
}

#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct Logger {
    pub name: String,
    pub level: u32,
}

impl Logger {
    pub fn new(name: String, level: LoggerLogLevel) -> Logger {
        Logger {
            name,
            level: level as u32,
        }
    }

    pub fn sub_logger(&self, name: String) -> Logger {
        Logger {
            name: format!("{}.{}", self.name, name),
            level: self.level,
        }
    }

    pub fn debug(&self, msg: String) -> () {
        if self.level <= LoggerLogLevel::DEBUG as u32 {
            LoggerModule::log(&ArgsLog {
                message: format!("{}: DEBUG - {}", self.name, msg),
                level: LoggerLogLevel::DEBUG,
            }).unwrap();
        }
    }

    pub fn info(&self, msg: String) -> () {
        if self.level <= LoggerLogLevel::INFO as u32 {
            LoggerModule::log(&ArgsLog {
                message: format!("{}: INFO - {}", self.name, msg),
                level: LoggerLogLevel::INFO,
            }).unwrap();
        }
    }

    pub fn warn(&self, msg: String) -> () {
        if self.level <= LoggerLogLevel::WARN as u32 {
            LoggerModule::log(&ArgsLog {
                message: format!("{}: WARN - {}", self.name, msg),
                level: LoggerLogLevel::WARN,
            }).unwrap();
        }
    }

    pub fn error(&self, msg: String) -> () {
        if self.level <= LoggerLogLevel::ERROR as u32 {
            LoggerModule::log(&ArgsLog {
                message: format!("{}: ERROR - {}", self.name, msg),
                level: LoggerLogLevel::ERROR,
            }).unwrap();
        }
    }
}
