use std::{
    env::args,
    io::{StdoutLock, Write, stdout},
    primitive::{bool, char, str, usize},
    process::exit,
    str::Lines,
    vec::Vec,
};

use crate::tokens::{
    operator_tokens::{OperatorToken, operators_vector},
    token_type::TokenType,
};

use super::print_help::print_help_message;

// Argument Token Definition
pub struct ArgumentToken {
    pub lexeme: &'static str,
    pub token_type: TokenType,
}

// Argument Lexer Definition
pub struct ArgumentLexer {
    pub argument_characters: Vec<char>,
    pub argument_lines: Lines<'static>,
    pub argument_source: &'static str,
    pub argument_tokens: Vec<ArgumentToken>,
}

// Advance to a Position of the Argument Lexer
pub fn advance_to_position(argument_lexer: &ArgumentLexer, index: usize) -> char {
    let characters: Vec<char> = argument_lexer.argument_source.chars().collect();

    return characters[index];
}

// Current Position of the Argument Lexer
pub fn current_position(argument_lexer: &ArgumentLexer) -> char {
    let characters: Vec<char> = argument_lexer.argument_source.chars().collect();

    return characters[0];
}

// Returns True if Lexer Position is at End of File
pub fn end_of_file(argument_lexer: &ArgumentLexer) -> bool {
    let source: Vec<char> = argument_lexer.argument_source.chars().collect();

    return source.len() == 0;
}

// Returns True if Flag Character
pub fn flag_character(source_arguments: &str) -> bool {
    let operators: Vec<OperatorToken> = operators_vector();

    if source_arguments.starts_with(operators[39]) || source_arguments.starts_with(operators[11]) {
        return true;
    } else {
        return false;
    }
}

// Advance to the Next Position of the Argument Lexer
pub fn next_position(argument_lexer: &ArgumentLexer) -> char {
    let characters: Vec<char> = argument_lexer.argument_source.chars().collect();

    return characters.iter().next().unwrap().clone();
}

// Print Argument Token
pub fn print_argument_token(argument_token: &ArgumentToken) -> () {
    let mut standard_output: StdoutLock = stdout().lock();

    writeln!(standard_output, "{}", argument_token.lexeme).unwrap();
    writeln!(standard_output, "{}", argument_token.token_type).unwrap();

    return ();
}

// Unknow Argument Error
pub fn unknown_argument() -> () {
    let mut standard_output: StdoutLock = stdout().lock();
    let arguments: Vec<String> = args().collect();

    writeln!(standard_output, "Uknown Command or Flag: {}", arguments[1]).unwrap();
    print_help_message();
    eprintln!("Error(1) - Exiting Hyaena Technologies Web Service");
    exit(1)
}
