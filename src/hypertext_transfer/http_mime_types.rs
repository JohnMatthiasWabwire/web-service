#![allow(dead_code)]

use std::{primitive::str, vec::Vec};

// Hypertext Transfer Protocol MIME Type Definition
pub type HttpMimeType = &'static str;

// Hypertext Transfer Protocol MIME Types
pub const HTTP_JSON_MIME_TYPE: HttpMimeType = "application/json;charset=UTF-8";
pub const HTTP_JAVASCRIPT_MIME_TYPE: HttpMimeType = "text/javascript;charset=UTF-8";
pub const HTTP_TEXT_MIME_TYPE: HttpMimeType = "text/plain;charset=UTF-8";

// Hypertext Transfer Protocol MIME Type Vector
pub fn mime_types_vector() -> Vec<HttpMimeType> {
    let mime_types: Vec<HttpMimeType> = Vec::from([
        HTTP_JSON_MIME_TYPE,
        HTTP_JAVASCRIPT_MIME_TYPE,
        HTTP_TEXT_MIME_TYPE,
    ]);

    return mime_types;
}
