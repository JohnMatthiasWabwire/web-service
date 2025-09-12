#![allow(dead_code)]

use std::{primitive::str, vec::Vec};

// Delimiter Token Defintion
pub type DelimiterToken = &'static str;

// Delimiter Tokens
pub const LEFT_BRACKET_TOKEN: DelimiterToken = "{";
pub const LEFT_PARENTHESIS_TOKEN: DelimiterToken = "(";
pub const LEFT_SQUARE_BRACKET_TOKEN: DelimiterToken = "[";
pub const RIGHT_BRACKET_TOKEN: DelimiterToken = "}";
pub const RIGHT_PARENTHESIS_TOKEN: DelimiterToken = ")";
pub const RIGHT_SQUARE_BRACKET_TOKEN: DelimiterToken = "]";

// Delimiter Token Vector
pub fn delimiters_vector() -> Vec<DelimiterToken> {
    let delimiters: Vec<DelimiterToken> = Vec::from([
        LEFT_BRACKET_TOKEN,
        LEFT_PARENTHESIS_TOKEN,
        LEFT_SQUARE_BRACKET_TOKEN,
        RIGHT_BRACKET_TOKEN,
        RIGHT_PARENTHESIS_TOKEN,
        RIGHT_SQUARE_BRACKET_TOKEN,
    ]);

    return delimiters;
}
