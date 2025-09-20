#![allow(dead_code)]

use std::{
    io::{StdoutLock, Write, stdout},
    string::String,
};

use time::Time;

// Lexer Error Definition
pub struct LexerError {
    pub current_time: Time,
    pub error_message: String,
}

// Print Lexer Error to Standard Output
pub fn print_error(lexer_error: &LexerError) -> () {
    let mut standard_output: StdoutLock = stdout().lock();

    writeln!(standard_output, "{}", lexer_error.error_message).unwrap();
    writeln!(standard_output, "Time: {}", lexer_error.current_time).unwrap();

    return ();
}
