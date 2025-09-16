#![allow(dead_code)]

use std::{primitive::str, vec::Vec};

use crate::hypertext_transfer::http_connection::HttpConnection;

use super::{
    http_headers::HttpHeader,
    http_methods::HttpMethod,
    http_security_directives::HttpSecurityDirective,
    http_status_codes::{HttpStatusCode, HttpStatusText},
    http_versions::HttpVersion,
};

// Hypertext Transfer Protocol Request Definition
pub struct HttpRequest {
    pub connection: HttpConnection,
    pub body: &'static str,
    pub headers: Vec<HttpHeader>,
    pub method: HttpMethod,
    pub resource_locator: &'static str,
    pub security_directives: Vec<HttpSecurityDirective>,
    pub status_code: HttpStatusCode,
    pub status_text: HttpStatusText,
    pub version: HttpVersion,
}

// Hypertext Transfer Protocol Request Message
pub fn request_message(request: &HttpRequest) -> &HttpRequest {
    return request;
}
