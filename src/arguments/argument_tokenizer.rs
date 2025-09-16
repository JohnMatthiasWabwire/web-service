use std::{
    primitive::{bool, char, str},
    str::Lines,
    string::String,
    vec::Vec,
};

use crate::{
    shared::lexer::{
        alphabetic_character, integer_character, null_character, whitespace_character,
    },
    tokens::token_type::TokenType,
};

use super::argument_lexer::{
    ArgumentLexer, ArgumentToken, end_of_file, flag_character, print_argument_token,
    unknown_argument,
};

// Tokenize Command Line Arguments
pub fn tokenize(source_arguments: &'static str) {
    let lines: Lines<'static> = source_arguments.lines();
    let tokens: Vec<ArgumentToken> = Vec::new();
    let lexer: ArgumentLexer = ArgumentLexer {
        argument_characters: source_arguments.chars().collect(),
        argument_lines: lines,
        argument_source: source_arguments,
        argument_tokens: tokens,
    };
    let eof: bool = end_of_file(&lexer);
    let characters: Vec<char> = lexer.argument_characters;
    let character: bool = alphabetic_character(characters[0].to_string().as_str());
    let flag: bool = flag_character(characters[0].to_string().as_str());
    let integer: bool = integer_character(characters[0].to_string().as_str());
    let null: bool = null_character(characters[0].to_string().as_str());
    let whitespace: bool = whitespace_character(characters[0].to_string().as_str());

    while characters.len() > 0 {}
}
