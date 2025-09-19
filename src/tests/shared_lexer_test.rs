use std::{primitive::bool, vec::Vec};

use crate::{
    shared::lexer::{
        alphabetic_character, integer_character, null_character, whitespace_character,
    },
    tokens::{
        character_tokens::{CharacterToken, characters_vector},
        escape_tokens::{EscapeToken, escape_tokens_vector},
        number_tokens::{NumberToken, numbers_vector},
    },
};

// Alphabetic Character Test
#[test]
fn alphabetic_character_test() -> () {
    let characters: Vec<CharacterToken> = characters_vector();
    let escape_sequences: Vec<EscapeToken> = escape_tokens_vector();
    let lowercase: bool = alphabetic_character(characters[0].to_string());
    let null: bool = alphabetic_character(escape_sequences[4].to_string());
    let numbers: Vec<NumberToken> = numbers_vector();
    let number: bool = alphabetic_character(numbers[4].to_string());
    let uppercase: bool = alphabetic_character(characters[26].to_string());
    let whitespace: bool = alphabetic_character(" ".to_string());

    assert_eq!(null, false);
    assert_eq!(number, false);
    assert_eq!(lowercase, true);
    assert_eq!(uppercase, true);
    assert_eq!(whitespace, false);
}

// Integer Character Test
#[test]
fn integer_character_test() -> () {
    let characters: Vec<CharacterToken> = characters_vector();
    let escape_sequences: Vec<EscapeToken> = escape_tokens_vector();
    let lowercase: bool = integer_character(characters[0].to_string());
    let null: bool = integer_character(escape_sequences[4].to_string());
    let numbers: Vec<NumberToken> = numbers_vector();
    let number: bool = integer_character(numbers[4].to_string());
    let uppercase: bool = integer_character(characters[26].to_string());
    let whitespace: bool = integer_character(" ".to_string());

    assert_eq!(null, false);
    assert_eq!(number, true);
    assert_eq!(lowercase, false);
    assert_eq!(uppercase, false);
    assert_eq!(whitespace, false);
}

// Null Character Test
#[test]
fn null_character_test() -> () {
    let characters: Vec<CharacterToken> = characters_vector();
    let escape_sequences: Vec<EscapeToken> = escape_tokens_vector();
    let lowercase: bool = null_character(characters[0].to_string());
    let null: bool = null_character(escape_sequences[4].to_string());
    let numbers: Vec<NumberToken> = numbers_vector();
    let number: bool = null_character(numbers[4].to_string());
    let uppercase: bool = null_character(characters[26].to_string());
    let whitespace: bool = null_character(" ".to_string());

    assert_eq!(null, true);
    assert_eq!(number, false);
    assert_eq!(lowercase, false);
    assert_eq!(uppercase, false);
    assert_eq!(whitespace, false);
}

// Whitespace Character Test
#[test]
fn whitespace_character_test() -> () {
    let characters: Vec<CharacterToken> = characters_vector();
    let escape_sequences: Vec<EscapeToken> = escape_tokens_vector();
    let lowercase: bool = whitespace_character(characters[0].to_string());
    let null: bool = whitespace_character(escape_sequences[4].to_string());
    let numbers: Vec<NumberToken> = numbers_vector();
    let number: bool = whitespace_character(numbers[4].to_string());
    let uppercase: bool = whitespace_character(characters[26].to_string());
    let whitespace: bool = whitespace_character(" ".to_string());

    assert_eq!(null, false);
    assert_eq!(number, false);
    assert_eq!(lowercase, false);
    assert_eq!(uppercase, false);
    assert_eq!(whitespace, true);
}
