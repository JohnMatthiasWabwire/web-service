#![allow(dead_code)]

use std::{
    fs::File,
    io::{Error, StdoutLock, Write, stdout},
    path::PathBuf,
    result::{Result, Result::Ok},
    time::SystemTime,
};

use crate::{
    hypertext_transfer::http_status_codes::{HttpStatusCode, HttpStatusText},
    logs::logger::LogLevel,
};

// Hypertext Transfer Error Definition
pub struct HttpError {
    pub current_time: SystemTime,
    pub log_level: LogLevel,
    pub status_code: HttpStatusCode,
    pub status_text: HttpStatusText,
}

// Print Hypertext Transfer Error to Standard Output
pub fn print_error(http_error: HttpError) -> () {
    let mut standard_output: StdoutLock = stdout().lock();

    writeln!(standard_output, "{}", http_error.status_code).unwrap();
    writeln!(standard_output, "{}", http_error.status_text).unwrap();
    writeln!(standard_output, "Time: {:#?}", http_error.current_time).unwrap();

    return ();
}

// Create Log File and Write an Error to the Log File
pub fn create_error_log(http_error: HttpError, log_path: PathBuf) -> Result<File, Error> {
    let mut log_file: File = File::create(log_path)?;

    writeln!(log_file, "Log Leve: {:#?}", http_error.log_level).unwrap();
    writeln!(log_file, "{}", http_error.status_code).unwrap();
    writeln!(log_file, "{}", http_error.status_text).unwrap();
    writeln!(log_file, "Time: {:#?}", http_error.current_time).unwrap();

    Ok(log_file)
}

// Open Log File and Write an Error to the Log File
pub fn write_error_log(http_error: HttpError, log_path: PathBuf) -> Result<File, Error> {
    let mut log_file: File = File::open(log_path)?;

    writeln!(log_file, "").unwrap();
    writeln!(log_file, "").unwrap();
    writeln!(log_file, "Log Level: {:#?}", http_error.log_level).unwrap();
    writeln!(log_file, "{}", http_error.status_code).unwrap();
    writeln!(log_file, "{}", http_error.status_text).unwrap();
    writeln!(log_file, "Time: {:#?}", http_error.current_time).unwrap();

    Ok(log_file)
}
