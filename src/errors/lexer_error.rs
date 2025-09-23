#![allow(dead_code)]

use std::{
    fmt::{Debug, Display, Formatter, Result},
    string::String,
    time::SystemTime,
};

// Lexer Error Definition
pub struct LexerError {
    pub message: String,
    pub time: SystemTime,
}

// Implement std::fmt::Debug for Lexer Error
impl Debug for LexerError {
    fn fmt(&self, formatter: &mut Formatter) -> Result {
        write!(formatter, "{}\n Time: {:#?}", self.message, self.time)
    }
}

// Implement std::fmt::Display for Lexer Error
impl Display for LexerError {
    fn fmt(&self, formatter: &mut Formatter) -> Result {
        write!(formatter, "Unkown Token")
    }
}
