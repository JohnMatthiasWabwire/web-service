#![allow(dead_code)]

use std::{primitive::bool, thread::Thread, vec::Vec};

use crate::hypertext_transfer::http_response::HttpResponse;

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
