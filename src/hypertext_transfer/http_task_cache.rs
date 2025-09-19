#![allow(dead_code)]

use std::{collections::HashMap, result::Result, string::String};

use crate::{errors::http_error::HttpError, hypertext_transfer::http_response::HttpResponse};

// Hypertext Transfer Protocol Task Result Cache Definition
pub struct HttpTaskResultCache {
    pub cache: HashMap<String, Result<HttpResponse, HttpError>>,
}
