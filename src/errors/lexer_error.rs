#![allow(dead_code)]

use std::{
    io::{StdoutLock, Write, stdout},
    primitive::str,
    time::SystemTime,
};

// Lexer Error Definition
pub struct LexerError {
    pub current_time: SystemTime,
    pub error_message: &'static str,
}

// Print Lexer Error to Standard Output
pub fn print_error(lexer_error: &LexerError) -> () {
    let mut standard_output: StdoutLock = stdout().lock();

    writeln!(standard_output, "{}", lexer_error.error_message).unwrap();
    writeln!(standard_output, "Time: {:#?}", lexer_error.current_time).unwrap();

    return ();
}
