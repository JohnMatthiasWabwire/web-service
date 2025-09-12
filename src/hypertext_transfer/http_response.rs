#![allow(dead_code)]

use std::{primitive::u8, vec::Vec};

use crate::hypertext_transfer::http_connection::HttpConnection;

use super::{
    http_headers::HttpHeader,
    http_methods::HttpMethod,
    http_security_directives::HttpSecurityDirective,
    http_status_codes::{HttpStatusCode, HttpStatusText},
    http_versions::HttpVersion,
};

// Hypertext Transfer Protocol Response Definition
pub struct HttpResponse {
    pub connection: HttpConnection,
    pub body: &'static [u8],
    pub headers: Vec<HttpHeader>,
    pub method: HttpMethod,
    pub security_directives: HttpSecurityDirective,
    pub status_code: HttpStatusCode,
    pub status_text: HttpStatusText,
    pub version: HttpVersion,
}

// Hypertext Transfer Protocol Response Message
pub fn response_message(response: HttpResponse) -> HttpResponse {
    return response;
}
