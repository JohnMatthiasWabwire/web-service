use std::{
    io::{StdoutLock, Write, stdout},
    primitive::{bool, char, usize},
    str::Lines,
    string::String,
    vec::Vec,
};

use crate::tokens::{
    escape_tokens::{EscapeToken, escape_tokens_vector},
    number_tokens::{NumberToken, numbers_vector},
    operator_tokens::{OperatorToken, operators_vector},
    token_type::TokenType,
};

// Token Definition
pub struct Token {
    pub lexeme: String,
    pub token_type: TokenType,
}

// Token Lexer Definition
pub struct TokenLexer {
    pub characters: Vec<char>,
    pub lines: Lines<'static>,
    pub source: &'static String,
    pub tokens: Vec<Token>,
}

// Token
pub fn token(value: String, kind: TokenType) -> Token {
    let token: Token = Token {
        lexeme: value,
        token_type: kind,
    };

    return token;
}

// Advance to a Position of the Lexer
pub fn advance_to_position(token_lexer: &TokenLexer, character_index: usize) -> char {
    return token_lexer.characters[character_index];
}

// Returns True if Alphabetic Character
pub fn alphabetic_character(source_tokens: String) -> bool {
    return source_tokens.to_lowercase() != source_tokens.to_uppercase();
}

// Current Position of the Lexer
pub fn current_position(token_lexer: &TokenLexer) -> char {
    return token_lexer.characters[0];
}

// Returns True if Lexer Position is at the End of the File
pub fn end_of_file(token_lexer: &TokenLexer) -> bool {
    return token_lexer.characters.len() == 0;
}

// Returns True if Flag Character
pub fn flag_character(source_tokens: String) -> bool {
    let operators: Vec<OperatorToken> = operators_vector();

    if source_tokens.starts_with(operators[39]) || source_tokens.starts_with(operators[11]) {
        return true;
    } else {
        return false;
    }
}

// Returns True if Integer
pub fn integer_character(source_tokens: String) -> bool {
    let numbers: Vec<NumberToken> = numbers_vector();

    return source_tokens >= numbers[0].to_string() && source_tokens <= numbers[9].to_string();
}

// Advance to the Next Position of the Lexer
pub fn next_position(token_lexer: &TokenLexer) -> char {
    return token_lexer.characters.iter().next().unwrap().clone();
}

// Returns True if Null Character
pub fn null_character(source_tokens: String) -> bool {
    let escape_tokens: Vec<EscapeToken> = escape_tokens_vector();

    return source_tokens == escape_tokens[4].to_string();
}

// Print Token
pub fn print_token(source_token: &Token) -> () {
    let mut standard_output: StdoutLock = stdout().lock();

    writeln!(standard_output, "{}", source_token.lexeme).unwrap();
    writeln!(standard_output, "{}", source_token.token_type).unwrap();

    return ();
}

// Unknow Token
pub fn unknown_token(source_token: String) -> () {
    let mut standard_output: StdoutLock = stdout().lock();

    writeln!(standard_output, "Uknown Token: {}", source_token).unwrap();
}

// Returns True if Whitespace
pub fn whitespace_character(source_tokens: String) -> bool {
    let escape_tokens: Vec<EscapeToken> = escape_tokens_vector();

    return source_tokens == " "
        || source_tokens == escape_tokens[3].to_string()
        || source_tokens == escape_tokens[6].to_string();
}
