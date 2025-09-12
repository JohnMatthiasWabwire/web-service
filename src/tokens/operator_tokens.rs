#![allow(dead_code)]

use std::{primitive::str, vec::Vec};

// Operator Token Defintion
pub type OperatorToken = &'static str;

// Operator Tokens
pub const ADDITION_TOKEN: OperatorToken = "+";
pub const ADDITION_ASSIGNMENT_TOKEN: OperatorToken = "+=";
pub const AND_TOKEN: OperatorToken = "&";
pub const ANNOTATION_TOKEN: OperatorToken = "@";
pub const ASSIGNMENT_TOKEN: OperatorToken = "=";
pub const BITWISE_SHIFT_LEFT_TOKEN: OperatorToken = "<<";
pub const BITWISE_SHIFT_RIGHT_TOKEN: OperatorToken = ">>";
pub const BITWISE_SHIFT_LEFT_ASSIGNMENT_TOKEN: OperatorToken = "<<=";
pub const BITWISE_SHIFT_RIGHT_ASSIGNMENT_TOKEN: OperatorToken = ">>=";
pub const COLON_TOKEN: OperatorToken = ":";
pub const COMMA_TOKEN: OperatorToken = ",";
pub const DECREMENT_TOKEN: OperatorToken = "--";
pub const DIVISION_TOKEN: OperatorToken = "/";
pub const DIVISION_ASSIGNMENT_TOKEN: OperatorToken = "/=";
pub const DOLLAR_SIGN_TOKEN: OperatorToken = "$";
pub const EQUALITY_TOKEN: OperatorToken = "==";
pub const EXCLUSIVE_OR_TOKEN: OperatorToken = "^";
pub const FIELD_ACCESS_TOKEN: OperatorToken = ".";
pub const GRAVE_ACCENT_TOKEN: OperatorToken = "`";
pub const GREATER_THAN_TOKEN: OperatorToken = ">";
pub const GREATER_OR_EQUAL_TOKEN: OperatorToken = ">=";
pub const INCLUSIVE_RANGE_TOKEN: OperatorToken = "..=";
pub const INCREMENT_TOKEN: OperatorToken = "++";
pub const LAMBDA_TOKEN: OperatorToken = "=>";
pub const LEFT_ARROW_TOKEN: OperatorToken = "<-";
pub const LESS_THAN_TOKEN: OperatorToken = "<";
pub const LESS_OR_EQUAL_TOKEN: OperatorToken = "<=";
pub const MODULUS_TOKEN: OperatorToken = "%";
pub const MODULUS_ASSIGNMENT_TOKEN: OperatorToken = "%=";
pub const MULTIPLICATION_TOKEN: OperatorToken = "*";
pub const MULTIPLICATION_ASSIGNMENT_TOKEN: OperatorToken = "*=";
pub const NOT_TOKEN: OperatorToken = "!";
pub const NOT_EQUAL_TOKEN: OperatorToken = "!=";
pub const NULL_COALESCING_TOKEN: OperatorToken = "?";
pub const OR_TOKEN: OperatorToken = "|";
pub const PATH_SEPARATOR_TOKEN: OperatorToken = "::";
pub const RANGE_TOKEN: OperatorToken = "..";
pub const RETURN_TYPE_TOKEN: OperatorToken = "->";
pub const SEMI_COLON_TOKEN: OperatorToken = ";";
pub const SUBTRACTION_TOKEN: OperatorToken = "-";
pub const SUBTRACTION_ASSIGNMENT_TOKEN: OperatorToken = "-=";
pub const TILDE_TOKEN: OperatorToken = "~";
pub const UNDERSCORE_TOKEN: OperatorToken = "_";
pub const VARIADIC_RANGE_TOKEN: OperatorToken = "...";

// Operator Token Vector
pub fn operators_vector() -> Vec<OperatorToken> {
    let operators: Vec<OperatorToken> = Vec::from([
        ADDITION_TOKEN,
        ADDITION_ASSIGNMENT_TOKEN,
        AND_TOKEN,
        ANNOTATION_TOKEN,
        ASSIGNMENT_TOKEN,
        BITWISE_SHIFT_LEFT_TOKEN,
        BITWISE_SHIFT_RIGHT_TOKEN,
        BITWISE_SHIFT_LEFT_ASSIGNMENT_TOKEN,
        BITWISE_SHIFT_RIGHT_ASSIGNMENT_TOKEN,
        COLON_TOKEN,
        COMMA_TOKEN,
        DECREMENT_TOKEN,
        DIVISION_TOKEN,
        DIVISION_ASSIGNMENT_TOKEN,
        DOLLAR_SIGN_TOKEN,
        EQUALITY_TOKEN,
        EXCLUSIVE_OR_TOKEN,
        FIELD_ACCESS_TOKEN,
        GRAVE_ACCENT_TOKEN,
        GREATER_THAN_TOKEN,
        GREATER_OR_EQUAL_TOKEN,
        INCLUSIVE_RANGE_TOKEN,
        INCREMENT_TOKEN,
        LAMBDA_TOKEN,
        LEFT_ARROW_TOKEN,
        LESS_THAN_TOKEN,
        LESS_OR_EQUAL_TOKEN,
        MODULUS_TOKEN,
        MODULUS_ASSIGNMENT_TOKEN,
        MULTIPLICATION_TOKEN,
        MULTIPLICATION_ASSIGNMENT_TOKEN,
        NOT_TOKEN,
        NOT_EQUAL_TOKEN,
        NULL_COALESCING_TOKEN,
        OR_TOKEN,
        PATH_SEPARATOR_TOKEN,
        RANGE_TOKEN,
        RETURN_TYPE_TOKEN,
        SEMI_COLON_TOKEN,
        SUBTRACTION_TOKEN,
        SUBTRACTION_ASSIGNMENT_TOKEN,
        TILDE_TOKEN,
        UNDERSCORE_TOKEN,
        VARIADIC_RANGE_TOKEN,
    ]);

    return operators;
}
