#![allow(dead_code)]

use std::vec::Vec;

use crate::hypertext_transfer::{http_connection::HttpConnection, http_request::HttpRequest};

// Hypertext Transfer Protocol Event Definition
pub struct HttpEvent {
    pub source: HttpConnection,
    pub request: HttpRequest,
}

// Hypertext Transfer Protocol Event Queue Definition
pub struct HttpEventQueue {
    pub queue: Vec<HttpEvent>,
}

// Hypertext Transfer Protocol Event
pub fn http_event(event_source: HttpConnection, message: HttpRequest) -> HttpEvent {
    let event: HttpEvent = HttpEvent {
        source: event_source,
        request: message,
    };

    return event;
}
