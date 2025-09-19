#![allow(dead_code)]

use std::{collections::HashMap, string::String};

use crate::hypertext_transfer::http_response::HttpResponse;

// Hypertext Transfer Protocol Task Result Cache Definition
pub struct HttpTaskResultCache {
    pub cache: HashMap<String, HttpResponse>,
}
