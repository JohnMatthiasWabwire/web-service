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

// Hypertext Transfer Protocol Task
pub fn http_task(finished: bool, message: HttpResponse, worker: Thread) -> HttpTask {
    let task: HttpTask = HttpTask {
        completed: finished,
        response: message,
        thread: worker,
    };

    return task;
}
