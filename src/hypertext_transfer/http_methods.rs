#![allow(dead_code)]

use std::{primitive::str, vec::Vec};

// Hypertext Transfer Protocol Method Defintion
pub type HttpMethod = &'static str;

// Hypertext Transfer Protocol Methods
pub const HTTP_CONNECT: HttpMethod = "CONNECT";
pub const HTTP_DELETE: HttpMethod = "DELETE";
pub const HTTP_GET: HttpMethod = "GET";
pub const HTTP_HEAD: HttpMethod = "HEAD";
pub const HTTP_OPTIONS: HttpMethod = "OPTIONS";
pub const HTTP_PATCH: HttpMethod = "PATCH";
pub const HTTP_POST: HttpMethod = "POST";
pub const HTTP_PUT: HttpMethod = "PUT";
pub const HTTP_TRACE: HttpMethod = "TRACE";

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
