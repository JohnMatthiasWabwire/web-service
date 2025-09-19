use std::{primitive::bool, vec::Vec};

use crate::{
    arguments::argument_lexer::flag_character,
    tokens::{
        character_tokens::{CharacterToken, characters_vector},
        escape_tokens::{EscapeToken, escape_tokens_vector},
        number_tokens::{NumberToken, numbers_vector},
        operator_tokens::{OperatorToken, operators_vector},
    },
};

// Flag Character Test
#[test]
pub fn flag_character_test() -> () {
    let characters: Vec<CharacterToken> = characters_vector();
    let escape_sequences: Vec<EscapeToken> = escape_tokens_vector();
    let lowercase: bool = flag_character(characters[0].to_string());
    let null: bool = flag_character(escape_sequences[4].to_string());
    let numbers: Vec<NumberToken> = numbers_vector();
    let number: bool = flag_character(numbers[4].to_string());
    let operators: Vec<OperatorToken> = operators_vector();
    let decrement: bool = flag_character(operators[11].to_string());
    let subtraction: bool = flag_character(operators[39].to_string());
    let uppercase: bool = flag_character(characters[26].to_string());
    let whitespace: bool = flag_character(" ".to_string());

    assert_eq!(decrement, true);
    assert_eq!(null, false);
    assert_eq!(number, false);
    assert_eq!(lowercase, false);
    assert_eq!(subtraction, true);
    assert_eq!(uppercase, false);
    assert_eq!(whitespace, false);
}
