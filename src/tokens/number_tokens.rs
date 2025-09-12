#![allow(dead_code)]

use std::{primitive::str, vec::Vec};

// Number Token Defintion
pub type NumberToken = &'static str;

// Number Tokens
pub const ZERO_TOKEN: NumberToken = "0";
pub const ONE_TOKEN: NumberToken = "1";
pub const TWO_TOKEN: NumberToken = "2";
pub const THREE_TOKEN: NumberToken = "3";
pub const FOUR_TOKEN: NumberToken = "4";
pub const FIVE_TOKEN: NumberToken = "5";
pub const SIX_TOKEN: NumberToken = "6";
pub const SEVEN_TOKEN: NumberToken = "7";
pub const EIGHT_TOKEN: NumberToken = "8";
pub const NINE_TOKEN: NumberToken = "9";

// Number Token Vector
pub fn numbers_vector() -> Vec<NumberToken> {
    let numbers: Vec<NumberToken> = Vec::from([
        ZERO_TOKEN,
        ONE_TOKEN,
        TWO_TOKEN,
        THREE_TOKEN,
        FOUR_TOKEN,
        FIVE_TOKEN,
        SIX_TOKEN,
        SEVEN_TOKEN,
        EIGHT_TOKEN,
        NINE_TOKEN,
    ]);

    return numbers;
}
