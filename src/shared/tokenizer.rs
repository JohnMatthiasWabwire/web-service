use std::{
    primitive::{bool, char},
    string::String,
    vec::Vec,
};

use crate::tokens::{
    escape_tokens::{EscapeToken, escape_tokens_vector},
    token_type::{TokenType, token_types_vector},
};

use super::token_lexer::{
    Token, TokenLexer, alphabetic_character, flag_character, integer_character, null_character,
    token, unknown_token, whitespace_character,
};

// Tokenize Command Line Arguments
pub fn tokenize(source_tokens: &'static String) {
    let escape_tokens: Vec<EscapeToken> = escape_tokens_vector();
    let token_vector: Vec<Token> = Vec::new();
    let lexer: TokenLexer = TokenLexer {
        characters: source_tokens.chars().collect(),
        lines: source_tokens.lines(),
        source: source_tokens,
        tokens: token_vector,
    };
    let mut characters: Vec<char> = lexer.characters;
    let alphabetic: bool = alphabetic_character(characters[0].to_string());
    let flag: bool = flag_character(characters[0].to_string());
    let integer: bool = integer_character(characters[0].to_string());
    let null: bool = null_character(characters[0].to_string());
    let mut tokens: Vec<Token> = lexer.tokens;
    let token_types: Vec<TokenType> = token_types_vector();
    let whitespace: bool = whitespace_character(characters[0].to_string());

    while characters.len() > 0 {
        if alphabetic == true {
            tokens.push(token(characters.remove(0).to_string(), token_types[0]));
        } else if characters.len() == 0 {
            tokens.push(token(escape_tokens[4].to_string(), token_types[3]));
        } else if flag == true {
            tokens.push(token(characters.remove(0).to_string(), token_types[5]));
        } else if integer == true {
            tokens.push(token(characters.remove(0).to_string(), token_types[7]));
        } else if null == true {
            tokens.push(token(characters.remove(0).to_string(), token_types[8]));
        } else if whitespace == true {
            characters.remove(0);
        } else {
            unknown_token(characters[0].to_string());
        }
    }

    return ();
}
