#![allow(dead_code)]

use std::{primitive::str, vec::Vec};

// Hypertext Transfer Protocol Status Text Definition
pub type HttpStatusText = &'static str;

// Hypertext Transfer Protocol Status Texts
pub const HTTP_CONTINUE: HttpStatusText = "Continue"
pub const HTTP_SWITCHING_PROTOCOLS: HttpStatusText = "Switching Protocols"
pub const HTTP_PROCESSING: HttpStatusText = "Processing"
pub const HTTP_EARLY_HINTS: HttpStatusText = "Early Hints"
pub const HTTP_OK: HttpStatusText = "OK"
pub const HTTP_CREATED: HttpStatusText = "Created"
pub const HTTP_ACCEPTED: HttpStatusText = "Accepted"
pub const HTTP_NON_AUTHORITATIVE_INFORMATION: HttpStatusText =
    "Non-Authoritative Information"
pub const HTTP_NO_CONTENT: HttpStatusText = "No Content"
pub const HTTP_RESET_CONTENT: HttpStatusText = "Reset Content"
pub const HTTP_PARTIAL_CONTENT: HttpStatusText = "Partial Content"
pub const HTTP_MULTI_STATUS: HttpStatusText = "Multi-Status"
pub const HTTP_ALREADY_REPORTED: HttpStatusText = "Already Reported"
pub const HTTP_IM_USED: HttpStatusText = "IM Used"
pub const HTTP_MULTIPLE_CHOICES: HttpStatusText = "Multiple Choices"
pub const HTTP_MOVED_PERMANENTLY: HttpStatusText = "Moved Permanently"
pub const HTTP_FOUND: HttpStatusText = "Found"
pub const HTTP_SEE_OTHER: HttpStatusText = "See Other"
pub const HTTP_NOT_MODIFIED: HttpStatusText = "Not Modified"
pub const HTTP_TEMPORARY_REDIRECT: HttpStatusText = "Temporary Redirect"
pub const HTTP_PREMANENT_REDIRECT: HttpStatusText = "Permanent Redirect"
pub const HTTP_BAD_REQUEST: HttpStatusText = "Bad Request"
pub const HTTP_UNAUTHORIZED: HttpStatusText = "Unauthorized"
pub const HTTP_PAYMENT_REQUIRED: HttpStatusText = "Payment Required"
pub const HTTP_FORBIDDEN: HttpStatusText = "Forbidden"
pub const HTTP_NOT_FOUND: HttpStatusText = "Not Found"
pub const HTTP_METHOD_NOT_ALLOWED: HttpStatusText = "Method Not Allowed"
pub const HTTP_NOT_ACCEPTABLE: HttpStatusText = "Not Acceptable"
pub const HTTP_PROXY_AUTHENTICATION_REQUIRED: HttpStatusText =
    "Proxy Authentication Required"
pub const HTTP_REQUEST_TIMEOUT: HttpStatusText = "Request Timeout"
pub const HTTP_CONFLICT: HttpStatusText = "Conflict"
pub const HTTP_GONE: HttpStatusText = "Gone"
pub const HTTP_LENGTH_REQUIRED: HttpStatusText = "Length Required"
pub const HTTP_PRECONDITION_FAILED: HttpStatusText = "Precondition Failed"
pub const HTTP_CONTENT_TOO_LARGE: HttpStatusText = "Content Too Large"
pub const HTTP_URI_TOO_LONG: HttpStatusText = "URI Too Long"
pub const HTTP_UNSUPPORTED_MEDIA_TYPE: HttpStatusText = "Unsupported Media Type"
pub const HTTP_RANGE_NOT_SATISFIABLE: HttpStatusText = "Range Not Satisfiable"
pub const HTTP_EXPECTATION_FAILED: HttpStatusText = "Expectation Failed"
pub const HTTP_TEAPOT: HttpStatusText = "I'm a teapot"
pub const HTTP_MISDIRECTED_REQUEST: HttpStatusText = "Misdirected Request"
pub const HTTP_UNPROCESSABLE_CONTENT: HttpStatusText = "Unprocessable Content"
pub const HTTP_LOCKED: HttpStatusText = "Locked"
pub const HTTP_FAILED_DEPENDENCY: HttpStatusText = "Failed Dependency"
pub const HTTP_TOO_EARLY: HttpStatusText = "Too Early"
pub const HTTP_UPGRADE_REQUIRED: HttpStatusText = "Upgrade Required"
pub const HTTP_PRECONDITION_REQUIRED: HttpStatusText = "Precondition Required"
pub const HTTP_TOO_MANY_REQUESTS: HttpStatusText = "Too Many Requests"
pub const HTTP_REQUEST_HEADER_FIELDS_TOO_LARGE: HttpStatusText =
    "Request Header Fields Too Large"
pub const HTTP_UNAVAILABLE_FOR_LEGAL_REASONS: HttpStatusText =
    "Unavailable For Legal Reasons"
pub const HTTP_INTERNAL_SERVER_ERROR: HttpStatusText = "Internal Server Error"
pub const HTTP_NOT_IMPLEMENTED: HttpStatusText = "Not Implemented"
pub const HTTP_BAD_GATEWAY: HttpStatusText = "Bad Gateway"
pub const HTTP_SERVICE_UNAVAILABLE: HttpStatusText = "Service Unavailable"
pub const HTTP_GATEWAY_TIMEOUT: HttpStatusText = "Gateway Timeout"
pub const HTTP_VERSION_NOT_SUPPORTED: HttpStatusText = "HTTP Version Not Supported"
pub const HTTP_VARIANT_ALSO_NEGOTIATES: HttpStatusText = "Variant Also Negotiates"
pub const HTTP_INSUFFICENT_STORAGE: HttpStatusText = "Insufficient Storage"
pub const HTTP_LOOP_DETECTED: HttpStatusText = "Loop Detected"
pub const HTTP_NOT_EXTENDED: HttpStatusText = "Not Extended"
pub const HTTP_NETWORK_AUTHENTICATION_REQUIRED: HttpStatusText =
    "Network Authentication Required"

// Hypertext Transfer Protocol Status Text Vector
pub fn status_texts_vector() -> Vec<HttpStatusText> {
    let http_status_texts: Vec<HttpStatusText> = Vec::from([
        HTTP_CONTINUE,
        HTTP_SWITCHING_PROTOCOLS,
        HTTP_PROCESSING,
        HTTP_EARLY_HINTS,
        HTTP_OK,
        HTTP_CREATED,
        HTTP_ACCEPTED,
        HTTP_NON_AUTHORITATIVE_INFORMATION,
        HTTP_NO_CONTENT,
        HTTP_RESET_CONTENT,
        HTTP_PARTIAL_CONTENT,
        HTTP_MULTI_STATUS,
        HTTP_ALREADY_REPORTED,
        HTTP_IM_USED,
        HTTP_MULTIPLE_CHOICES,
        HTTP_MOVED_PERMANENTLY,
        HTTP_FOUND,
        HTTP_SEE_OTHER,
        HTTP_NOT_MODIFIED,
        HTTP_TEMPORARY_REDIRECT,
        HTTP_PREMANENT_REDIRECT,
        HTTP_BAD_REQUEST,
        HTTP_UNAUTHORIZED,
        HTTP_PAYMENT_REQUIRED,
        HTTP_FORBIDDEN,
        HTTP_NOT_FOUND,
        HTTP_METHOD_NOT_ALLOWED,
        HTTP_NOT_ACCEPTABLE,
        HTTP_PROXY_AUTHENTICATION_REQUIRED,
        HTTP_REQUEST_TIMEOUT,
        HTTP_CONFLICT,
        HTTP_GONE,
        HTTP_LENGTH_REQUIRED,
        HTTP_PRECONDITION_FAILED,
        HTTP_CONTENT_TOO_LARGE,
        HTTP_URI_TOO_LONG,
        HTTP_UNSUPPORTED_MEDIA_TYPE,
        HTTP_RANGE_NOT_SATISFIABLE,
        HTTP_EXPECTATION_FAILED,
        HTTP_TEAPOT,
        HTTP_MISDIRECTED_REQUEST,
        HTTP_UNPROCESSABLE_CONTENT,
        HTTP_LOCKED,
        HTTP_FAILED_DEPENDENCY,
        HTTP_TOO_EARLY,
        HTTP_UPGRADE_REQUIRED,
        HTTP_PRECONDITION_REQUIRED,
        HTTP_TOO_MANY_REQUESTS,
        HTTP_REQUEST_HEADER_FIELDS_TOO_LARGE,
        HTTP_UNAVAILABLE_FOR_LEGAL_REASONS,
        HTTP_INTERNAL_SERVER_ERROR,
        HTTP_NOT_IMPLEMENTED,
        HTTP_BAD_GATEWAY,
        HTTP_SERVICE_UNAVAILABLE,
        HTTP_GATEWAY_TIMEOUT,
        HTTP_VERSION_NOT_SUPPORTED,
        HTTP_VARIANT_ALSO_NEGOTIATES,
        HTTP_INSUFFICENT_STORAGE,
        HTTP_LOOP_DETECTED,
        HTTP_NOT_EXTENDED,
        HTTP_NETWORK_AUTHENTICATION_REQUIRED,
    ]);

    return http_status_texts;
}
