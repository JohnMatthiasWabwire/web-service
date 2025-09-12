use std::{
    primitive::{bool, str},
    vec::Vec,
};

use crate::tokens::{
    escape_tokens::{EscapeToken, escape_tokens_vector},
    number_tokens::{NumberToken, numbers_vector},
};

// Returns True if Alphabetic Character
pub fn alphabetic_character(source_code: &str) -> bool {
    return source_code.to_lowercase() != source_code.to_uppercase();
}

// Returns True if Integer
pub fn integer_character(source_code: &str) -> bool {
    let numbers: Vec<NumberToken> = numbers_vector();

    return source_code >= numbers[0] && source_code <= numbers[9];
}

// Returns True if Null Character
pub fn null_character(source_code: &str) -> bool {
    let escape_tokens: Vec<EscapeToken> = escape_tokens_vector();

    return source_code == escape_tokens[4];
}

// Returns True if Whitespace
pub fn whitespace_character(source_code: &str) -> bool {
    let escape_tokens: Vec<EscapeToken> = escape_tokens_vector();

    return source_code == " "
        || source_code == escape_tokens[3]
        || source_code == escape_tokens[6];
}
