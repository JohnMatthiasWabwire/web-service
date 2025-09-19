use std::{
    primitive::{bool, char},
    string::String,
    vec::Vec,
};

use crate::{
    shared::lexer::alphabetic_character,
    tokens::token_type::{COMMAND_TOKEN, END_OF_FILE_TOKEN, FLAG_TOKEN},
};

use super::argument_lexer::{
    ArgumentLexer, ArgumentToken, end_of_file, flag_character, token, unknown_argument,
};

// Tokenize Command Line Arguments
pub fn tokenize(source_arguments: &'static String) {
    let argument_tokens: Vec<ArgumentToken> = Vec::new();
    let lexer: ArgumentLexer = ArgumentLexer {
        characters: source_arguments.chars().collect(),
        lines: source_arguments.lines(),
        source: source_arguments,
        tokens: argument_tokens,
    };
    let eof: bool = end_of_file(&lexer);
    let mut characters: Vec<char> = lexer.characters;
    let alphabetic: bool = alphabetic_character(characters[0].to_string());
    let flag: bool = flag_character(characters[0].to_string());
    let mut tokens: Vec<ArgumentToken> = lexer.tokens;

    while characters.len() > 0 {
        if alphabetic == true {
            tokens.push(token(characters.remove(0).to_string(), COMMAND_TOKEN));
        } else if flag == true {
            tokens.push(token(characters.remove(0).to_string(), FLAG_TOKEN));
        } else {
            unknown_argument();
        }
    }

    tokens.push(token(END_OF_FILE_TOKEN.to_string(), END_OF_FILE_TOKEN));
    return ();
}
