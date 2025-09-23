#![allow(dead_code)]

use std::{
    fs::File,
    io::{Error, StdoutLock, Write, stdout},
    path::PathBuf,
    process::exit,
    result::{
        Result,
        Result::{Err, Ok},
    },
    string::String,
    time::SystemTime,
    vec::Vec,
};

// Structured Log Level Definition
pub type LogLevel = &'static str;

// Structured Log Levels
pub const LOG_LEVEL_DEBUG: LogLevel = "DEBUG";
pub const LOG_LEVEL_ERROR: LogLevel = "ERROR";
pub const LOG_LEVEL_INFO: LogLevel = "INFO";
pub const LOG_LEVEL_OFF: LogLevel = "OFF";
pub const LOG_LEVEL_TRACE: LogLevel = "TRACE";
pub const LOG_LEVEL_WARN: LogLevel = "WARN";

// Structured Log Definition
pub struct StructuredLog {
    pub level: LogLevel,
    pub message: String,
    pub time: SystemTime,
}

// Create Log File and Write to the Log File
pub fn create_log(structured_log: &StructuredLog, log_path: PathBuf) -> () {
    let mut log_file: Result<File, Error> = File::create(log_path);

    match &mut log_file {
        Ok(log) => {
            writeln!(log, "Log Level: {}", structured_log.level).unwrap();
            writeln!(log, "{}", structured_log.message).unwrap();
            writeln!(log, "Time: {:#?}", structured_log.time).unwrap();
        }
        Err(error) => {
            eprintln!("Error Creating Log File: {}", error);
            exit(1);
        }
    }
    return ();
}

// Log Level Vector
pub fn log_levels_vector() -> Vec<LogLevel> {
    let log_levels: Vec<LogLevel> = Vec::from([
        LOG_LEVEL_DEBUG,
        LOG_LEVEL_ERROR,
        LOG_LEVEL_INFO,
        LOG_LEVEL_OFF,
        LOG_LEVEL_TRACE,
        LOG_LEVEL_WARN,
    ]);

    return log_levels;
}

// Print Log to Standard Output
pub fn print_log(structured_log: &StructuredLog) -> () {
    let mut standard_output: StdoutLock = stdout().lock();

    writeln!(standard_output, "Log Level: {}", structured_log.level).unwrap();
    writeln!(standard_output, "{}", structured_log.message).unwrap();
    writeln!(standard_output, "Time: {:#?}", structured_log.time).unwrap();

    return ();
}

// Open Log File and Write to the Log File
pub fn write_log(structured_log: &StructuredLog, log_path: PathBuf) -> () {
    let mut log_file: Result<File, Error> = File::open(log_path);

    match &mut log_file {
        Ok(log) => {
            writeln!(log, "").unwrap();
            writeln!(log, "").unwrap();
            writeln!(log, "Log Level: {}", structured_log.level).unwrap();
            writeln!(log, "{}", structured_log.message).unwrap();
            writeln!(log, "Time: {:#?}", structured_log.time).unwrap();
        }
        Err(error) => {
            eprintln!("Error Writing to Log File: {}", error);
            exit(1);
        }
    }
    return ();
}
