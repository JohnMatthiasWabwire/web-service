#![allow(dead_code)]

use std::{primitive::str, vec::Vec};

// Token Type Defintion
pub type TokenType = &'static str;

// Token Types
pub const CHARACTER_TOKEN: TokenType = "CHARACTER";
pub const COMMAND_TOKEN: TokenType = "COMMAND";
pub const COMMENT_TOKEN: TokenType = "COMMENT";
pub const DELIMITER_TOKEN: TokenType = "DELIMITER";
pub const FLAG_TOKEN: TokenType = "FLAG";
pub const ESCAPE_TOKEN: TokenType = "ESCAPE";
pub const KEYWORD_TOKEN: TokenType = "KEYWORD";
pub const NUMBER_TOKEN: TokenType = "NUMBER";
pub const OPERATOR_TOKEN: TokenType = "OPERATOR";
pub const UNKNOWN_TOKEN: TokenType = "UNKNOWN";

// Token Type Vector
pub fn token_types_vector() -> Vec<TokenType> {
    let token_types: Vec<TokenType> = Vec::from([
        CHARACTER_TOKEN,
        COMMAND_TOKEN,
        COMMENT_TOKEN,
        DELIMITER_TOKEN,
        ESCAPE_TOKEN,
        FLAG_TOKEN,
        KEYWORD_TOKEN,
        NUMBER_TOKEN,
        OPERATOR_TOKEN,
        UNKNOWN_TOKEN,
    ]);

    return token_types;
}
