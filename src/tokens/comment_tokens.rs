#![allow(dead_code)]

use std::{primitive::str, vec::Vec};

// Comment Token Defintion
pub type CommentToken = &'static str;

// Comment Tokens
pub const BLOCK_TOKEN: CommentToken = "/*";
pub const LINE_TOKEN: CommentToken = "//";
pub const INNER_LINE_TOKEN: CommentToken = "//!";
pub const INNER_BLOCK_TOKEN: CommentToken = "/*!";
pub const OUTER_LINE_TOKEN: CommentToken = "///";
pub const OUTER_BLOCK_TOKEN: CommentToken = "/**";
pub const OCTOTHORPE_BLOCK_TOKEN: CommentToken = "#*";
pub const OCTOTHORPE_LINE_TOKEN: CommentToken = "#";
pub const OCTOTHROPE_INNER_LINE_TOKEN: CommentToken = "#!";
pub const OCTOTHORPE_INNER_BLOCK_TOKEN: CommentToken = "#*!";
pub const OCTOTHROPE_OUTER_LINE_TOKEN: CommentToken = "###";
pub const OCTOTHORPE_OUTER_BLOCK_TOKEN: CommentToken = "#**";

// Comment Token Vector
pub fn comments_vector() -> Vec<CommentToken> {
    let comments: Vec<CommentToken> = Vec::from([
        BLOCK_TOKEN,
        LINE_TOKEN,
        INNER_LINE_TOKEN,
        INNER_BLOCK_TOKEN,
        OUTER_LINE_TOKEN,
        OUTER_BLOCK_TOKEN,
        OCTOTHORPE_BLOCK_TOKEN,
        OCTOTHORPE_LINE_TOKEN,
        OCTOTHROPE_INNER_LINE_TOKEN,
        OCTOTHORPE_INNER_BLOCK_TOKEN,
        OCTOTHROPE_OUTER_LINE_TOKEN,
        OCTOTHORPE_OUTER_BLOCK_TOKEN,
    ]);

    return comments;
}
