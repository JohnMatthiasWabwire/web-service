#![allow(dead_code)]

use std::{collections::HashMap, string::String};

use crate::hypertext_transfer::http_request::HttpRequest;

// Hypertext Transfer Protocol Event Result Cache Definition
pub struct HttpEventResultCache {
    pub cache: HashMap<String, HttpRequest>,
}
