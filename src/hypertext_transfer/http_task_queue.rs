#![allow(dead_code)]

use std::{
    collections::HashMap, primitive::bool, result::Result, string::String, thread::Thread, vec::Vec,
};

use crate::{errors::http_error::HttpError, hypertext_transfer::http_response::HttpResponse};

// Hypertext Transfer Protocol Task Definition
pub struct HttpTask {
    pub completed: bool,
    pub response: HttpResponse,
    pub thread: Thread,
}

// Hypertext Transfer Protocol Task Queue Definition
pub struct HttpTaskQueue {
    pub queue: Vec<HttpTask>,
}

// Hypertext Transfer Protocol Task Result Storage Cache Definition
pub struct HttpTaskResultStore {
    pub cache: HashMap<String, Result<HttpResponse, HttpError>>,
}

// Hypertext Transfer Protocol Task Scheduler Definition
pub struct HttpTaskScheduler {}
