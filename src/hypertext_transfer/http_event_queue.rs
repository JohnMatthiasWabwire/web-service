#![allow(dead_code)]

use std::{collections::HashMap, result::Result, string::String, vec::Vec};

use crate::{
    errors::http_error::HttpError,
    hypertext_transfer::{http_connection::HttpConnection, http_request::HttpRequest},
};

// Hypertext Transfer Protocol Event Definition
pub struct HttpEvent {
    pub connection: HttpConnection,
    pub request: HttpRequest,
}

// Hypertext Transfer Protocol Event Queue Definition
pub struct HttpEventQueue {
    pub queue: Vec<HttpEvent>,
}

// Hypertext Transfer Protocol Event Result Storage Cache Definition
pub struct HttpEventResultStore {
    pub cache: HashMap<String, Result<HttpRequest, HttpError>>,
}

// Hypertext Transfer Protocol Event Scheduler Definition
pub struct HttpEventScheduler {}
