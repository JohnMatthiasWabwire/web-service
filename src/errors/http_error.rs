#![allow(dead_code)]

use std::{
    fmt::{Debug, Display, Formatter, Result},
    time::SystemTime,
};

use crate::hypertext_transfer::http_status_codes::{HttpStatusCode, HttpStatusText};

// Hypertext Transfer Error Definition
pub struct HttpError {
    pub code: HttpStatusCode,
    pub message: HttpStatusText,
    pub time: SystemTime,
}

// Implement std::fmt::Debug for Hypertext Transfer Protocol Error
impl Debug for HttpError {
    fn fmt(&self, formatter: &mut Formatter) -> Result {
        write!(
            formatter,
            "Status Code: {}\n Status Text: {}\n Time: {:#?}",
            self.code, self.message, self.time
        )
    }
}

// Implement std::fmt::Display for Hypertext Transfer Protocol Error
impl Display for HttpError {
    fn fmt(&self, formatter: &mut Formatter) -> Result {
        write!(formatter, "Error Event")
    }
}
