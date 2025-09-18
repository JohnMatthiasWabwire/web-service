#![allow(dead_code)]

use std::vec::Vec;

use crate::hypertext_transfer::{http_connection::HttpConnection, http_request::HttpRequest};

// Hypertext Transfer Protocol Event Definition
pub struct HttpEvent {
    pub connection: HttpConnection,
    pub request: HttpRequest,
}

// Hypertext Transfer Protocol Event Queue Definition
pub struct HttpEventQueue {
    pub queue: Vec<HttpEvent>,
}
