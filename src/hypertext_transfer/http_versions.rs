#![allow(dead_code)]

use std::{primitive::str, vec::Vec};

// Hypertext Transfer Protocol Version Definition
pub type HttpVersion = &'static str;

// Hypertext Transfer Protocol Versions
pub const HTTP_VERSION_ONE: HttpVersion = "HTTP/1.1";
pub const HTTP_VERSION_TWO: HttpVersion = "HTTP/2.0";
pub const HTTP_VERSION_THREE: HttpVersion = "HTTP/3.0";

// Hypertext Transfer Protocol Version Vector
pub fn versions_vector() -> Vec<HttpVersion> {
    let versions: Vec<HttpVersion> =
        Vec::from([HTTP_VERSION_ONE, HTTP_VERSION_TWO, HTTP_VERSION_THREE]);

    return versions;
}
