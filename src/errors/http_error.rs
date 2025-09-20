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
};

use time::Time;

use crate::{
    hypertext_transfer::http_status_codes::{HttpStatusCode, HttpStatusText},
    logs::logger::LogLevel,
};

// Hypertext Transfer Error Definition
pub struct HttpError {
    pub current_time: Time,
    pub log_level: LogLevel,
    pub status_code: HttpStatusCode,
    pub status_text: HttpStatusText,
}

// Print Hypertext Transfer Error to Standard Output
pub fn print_error(http_error: &HttpError) -> () {
    let mut standard_output: StdoutLock = stdout().lock();

    writeln!(standard_output, "{}", http_error.status_code).unwrap();
    writeln!(standard_output, "{}", http_error.status_text).unwrap();
    writeln!(standard_output, "Time: {}", http_error.current_time).unwrap();

    return ();
}

// Create Log File and Write an Error to the Log File
pub fn create_error_log(http_error: &HttpError, log_path: PathBuf) -> () {
    let log_file: Result<File, Error> = File::create(log_path);

    match log_file {
        Ok(mut file) => {
            writeln!(file, "Log Leve: {}", http_error.log_level).unwrap();
            writeln!(file, "{}", http_error.status_code).unwrap();
            writeln!(file, "{}", http_error.status_text).unwrap();
            writeln!(file, "Time: {}", http_error.current_time).unwrap();
        }
        Err(error) => {
            eprintln!("Error Creating Log File: {}", error);
            exit(1);
        }
    }
    return ();
}

// Open Log File and Write an Error to the Log File
pub fn write_error_log(http_error: &HttpError, log_path: PathBuf) -> () {
    let log_file: Result<File, Error> = File::open(log_path);

    match log_file {
        Ok(mut file) => {
            writeln!(file, "").unwrap();
            writeln!(file, "").unwrap();
            writeln!(file, "Log Level: {}", http_error.log_level).unwrap();
            writeln!(file, "{}", http_error.status_code).unwrap();
            writeln!(file, "{}", http_error.status_text).unwrap();
            writeln!(file, "Time: {}", http_error.current_time).unwrap();
        }
        Err(error) => {
            eprintln!("Error Writing To Log File: {}", error);
            exit(1);
        }
    }
    return ();
}
