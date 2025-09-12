#![allow(dead_code)]

use std::{primitive::str, vec::Vec};

// Escape Token Defintion
pub type EscapeToken = &'static str;

// Escape Tokens
pub const APOSTROPHE_TOKEN: EscapeToken = "\'";
pub const BACKSLASH_TOKEN: EscapeToken = "\\";
pub const CARRIAGE_RETURN_TOKEN: EscapeToken = "\r";
pub const NEW_LINE_TOKEN: EscapeToken = "\n";
pub const NULL_TOKEN: EscapeToken = "\0";
pub const QUOTATION_MARK_TOKEN: EscapeToken = "\"";
pub const TAB_TOKEN: EscapeToken = "\t";

// Escape Token Vector
pub fn escape_tokens_vector() -> Vec<EscapeToken> {
    let strings: Vec<EscapeToken> = Vec::from([
        APOSTROPHE_TOKEN,
        BACKSLASH_TOKEN,
        CARRIAGE_RETURN_TOKEN,
        NEW_LINE_TOKEN,
        NULL_TOKEN,
        QUOTATION_MARK_TOKEN,
        TAB_TOKEN,
    ]);

    return strings;
}
