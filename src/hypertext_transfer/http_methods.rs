#![allow(dead_code)]

use std::{primitive::u8, vec::Vec};

// Hypertext Transfer Protocol Method Defintion
pub type HttpMethod = &'static [u8];

// Hypertext Transfer Protocol Methods
pub const HTTP_CONNECT: HttpMethod = "CONNECT".as_bytes();
pub const HTTP_DELETE: HttpMethod = "DELETE".as_bytes();
pub const HTTP_GET: HttpMethod = "GET".as_bytes();
pub const HTTP_HEAD: HttpMethod = "HEAD".as_bytes();
pub const HTTP_OPTIONS: HttpMethod = "OPTIONS".as_bytes();
pub const HTTP_PATCH: HttpMethod = "PATCH".as_bytes();
pub const HTTP_POST: HttpMethod = "POST".as_bytes();
pub const HTTP_PUT: HttpMethod = "PUT".as_bytes();
pub const HTTP_TRACE: HttpMethod = "TRACE".as_bytes();

// Hypertext Transfer Protocol Method Vector
pub fn methods_vector() -> Vec<HttpMethod> {
    let methods: Vec<HttpMethod> = Vec::from([
        HTTP_CONNECT,
        HTTP_DELETE,
        HTTP_GET,
        HTTP_HEAD,
        HTTP_OPTIONS,
        HTTP_PATCH,
        HTTP_POST,
        HTTP_PUT,
        HTTP_TRACE,
    ]);

    return methods;
}
