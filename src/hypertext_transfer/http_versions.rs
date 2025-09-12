#![allow(dead_code)]

use std::{primitive::u8, vec::Vec};

// Hypertext Transfer Protocol Version Definition
pub type HttpVersion = &'static [u8];

// Hypertext Transfer Protocol Versions
pub const HTTP_VERSION_ONE: HttpVersion = "HTTP/1.1".as_bytes();
pub const HTTP_VERSION_TWO: HttpVersion = "HTTP/2.0".as_bytes();
pub const HTTP_VERSION_THREE: HttpVersion = "HTTP/3.0".as_bytes();

// Hypertext Transfer Protocol Version Vector
pub fn versions_vector() -> Vec<HttpVersion> {
    let versions: Vec<HttpVersion> =
        Vec::from([HTTP_VERSION_ONE, HTTP_VERSION_TWO, HTTP_VERSION_THREE]);

    return versions;
}
