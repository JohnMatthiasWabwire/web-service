#![allow(dead_code)]

use std::{string::String, time::SystemTime};

// Lexer Error Definition
pub struct LexerError {
    pub current_time: SystemTime,
    pub error_message: String,
}

// Print Lexer Error to Standard Output
pub fn print_error(lexer_error: &LexerError) -> () {
    eprintln!("{}", lexer_error.error_message);
    eprintln!("Time: {:#?}", lexer_error.current_time);

    return ();
}
