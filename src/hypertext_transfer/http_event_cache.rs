#![allow(dead_code)]

use std::{collections::HashMap, result::Result, string::String};

use crate::{errors::http_error::HttpError, hypertext_transfer::http_request::HttpRequest};

// Hypertext Transfer Protocol Event Result Cache Definition
pub struct HttpEventResultCache {
    pub cache: HashMap<String, Result<HttpRequest, HttpError>>,
}
