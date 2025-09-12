#![allow(dead_code)]

use std::{collections::HashMap, primitive::u8};

// Hypertext Transfer Protocol Status Code Definition
pub type HttpStatusCode = &'static [u8];

// Hypertext Transfer Protocol Status Text Definition
pub type HttpStatusText = &'static [u8];

// Hypertext Transfer Protocol Status Codes
pub const HTTP_ONE_HUNDRED: HttpStatusCode = "100".as_bytes();
pub const HTTP_ONE_HUNDRED_ONE: HttpStatusCode = "101".as_bytes();
pub const HTTP_ONE_HUNDRED_TWO: HttpStatusCode = "102".as_bytes();
pub const HTTP_ONE_HUNDRED_THREE: HttpStatusCode = "103".as_bytes();
pub const HTTP_TWO_HUNDRED: HttpStatusCode = "200".as_bytes();
pub const HTTP_TWO_HUNDRED_ONE: HttpStatusCode = "201".as_bytes();
pub const HTTP_TWO_HUNDRED_TWO: HttpStatusCode = "202".as_bytes();
pub const HTTP_TWO_HUNDRED_THREE: HttpStatusCode = "203".as_bytes();
pub const HTTP_TWO_HUNDRED_FOUR: HttpStatusCode = "204".as_bytes();
pub const HTTP_TWO_HUNDRED_FIVE: HttpStatusCode = "205".as_bytes();
pub const HTTP_TWO_HUNDRED_SIX: HttpStatusCode = "206".as_bytes();
pub const HTTP_TWO_HUNDRED_SEVEN: HttpStatusCode = "207".as_bytes();
pub const HTTP_TWO_HUNDRED_EIGHT: HttpStatusCode = "208".as_bytes();
pub const HTTP_TWO_HUNDRED_TWENTY_SIX: HttpStatusCode = "226".as_bytes();
pub const HTTP_THREE_HUNDRED: HttpStatusCode = "300".as_bytes();
pub const HTTP_THREE_HUNDRED_ONE: HttpStatusCode = "301".as_bytes();
pub const HTTP_THREE_HUNDRED_TWO: HttpStatusCode = "302".as_bytes();
pub const HTTP_THREE_HUNDRED_THREE: HttpStatusCode = "303".as_bytes();
pub const HTTP_THREE_HUNDRED_FOUR: HttpStatusCode = "304".as_bytes();
pub const HTTP_THREE_HUNDRED_SEVEN: HttpStatusCode = "307".as_bytes();
pub const HTTP_THREE_HUNDRED_EIGHT: HttpStatusCode = "308".as_bytes();
pub const HTTP_FOUR_HUNDRED: HttpStatusCode = "400".as_bytes();
pub const HTTP_FOUR_HUNDRED_ONE: HttpStatusCode = "401".as_bytes();
pub const HTTP_FOUR_HUNDRED_TWO: HttpStatusCode = "402".as_bytes();
pub const HTTP_FOUR_HUNDRED_THREE: HttpStatusCode = "403".as_bytes();
pub const HTTP_FOUR_HUNDRED_FOUR: HttpStatusCode = "404".as_bytes();
pub const HTTP_FOUR_HUNDRED_FIVE: HttpStatusCode = "405".as_bytes();
pub const HTTP_FOUR_HUNDRED_SIX: HttpStatusCode = "406".as_bytes();
pub const HTTP_FOUR_HUNDRED_SEVEN: HttpStatusCode = "407".as_bytes();
pub const HTTP_FOUR_HUNDRED_EIGHT: HttpStatusCode = "408".as_bytes();
pub const HTTP_FOUR_HUNDRED_NINE: HttpStatusCode = "409".as_bytes();
pub const HTTP_FOUR_HUNDRED_TEN: HttpStatusCode = "410".as_bytes();
pub const HTTP_FOUR_HUNDRED_ELEVEN: HttpStatusCode = "411".as_bytes();
pub const HTTP_FOUR_HUNDRED_TWELVE: HttpStatusCode = "412".as_bytes();
pub const HTTP_FOUR_HUNDRED_THIRTEEN: HttpStatusCode = "413".as_bytes();
pub const HTTP_FOUR_HUNDRED_FOURTEEN: HttpStatusCode = "414".as_bytes();
pub const HTTP_FOUR_HUNDRED_FIFTEEN: HttpStatusCode = "415".as_bytes();
pub const HTTP_FOUR_HUNDRED_SIXTEEN: HttpStatusCode = "416".as_bytes();
pub const HTTP_FOUR_HUNDRED_SEVENTEEN: HttpStatusCode = "417".as_bytes();
pub const HTTP_FOUR_HUNDRED_EIGHTEEN: HttpStatusCode = "418".as_bytes();
pub const HTTP_FOUR_HUNDRED_TWENTY_ONE: HttpStatusCode = "421".as_bytes();
pub const HTTP_FOUR_HUNDRED_TWENTY_TWO: HttpStatusCode = "422".as_bytes();
pub const HTTP_FOUR_HUNDRED_TWENTY_THREE: HttpStatusCode = "423".as_bytes();
pub const HTTP_FOUR_HUNDRED_TWENTY_FOUR: HttpStatusCode = "424".as_bytes();
pub const HTTP_FOUR_HUNDRED_TWENTY_FIVE: HttpStatusCode = "425".as_bytes();
pub const HTTP_FOUR_HUNDRED_TWENTY_SIX: HttpStatusCode = "426".as_bytes();
pub const HTTP_FOUR_HUNDRED_TWENTY_EIGHT: HttpStatusCode = "428".as_bytes();
pub const HTTP_FOUR_HUNDRED_TWENTY_NINE: HttpStatusCode = "429".as_bytes();
pub const HTTP_FOUR_HUNDRED_THIRTY_ONE: HttpStatusCode = "431".as_bytes();
pub const HTTP_FOUR_HUNDRED_FIFTEY_ONE: HttpStatusCode = "451".as_bytes();
pub const HTTP_FIVE_HUNDRED: HttpStatusCode = "500".as_bytes();
pub const HTTP_FIVE_HUNDRED_ONE: HttpStatusCode = "501".as_bytes();
pub const HTTP_FIVE_HUNDRED_TWO: HttpStatusCode = "502".as_bytes();
pub const HTTP_FIVE_HUNDRED_THREE: HttpStatusCode = "503".as_bytes();
pub const HTTP_FIVE_HUNDRED_FOUR: HttpStatusCode = "504".as_bytes();
pub const HTTP_FIVE_HUNDRED_FIVE: HttpStatusCode = "505".as_bytes();
pub const HTTP_FIVE_HUNDRED_SIX: HttpStatusCode = "506".as_bytes();
pub const HTTP_FIVE_HUNDRED_SEVEN: HttpStatusCode = "507".as_bytes();
pub const HTTP_FIVE_HUNDRED_EIGHT: HttpStatusCode = "508".as_bytes();
pub const HTTP_FIVE_HUNDRED_TEN: HttpStatusCode = "510".as_bytes();
pub const HTTP_FIVE_HUNDRED_ELEVEN: HttpStatusCode = "511".as_bytes();

// Hypertext Transfer Protocol Status Texts
pub const HTTP_CONTINUE: HttpStatusText = "Continue".as_bytes();
pub const HTTP_SWITCHING_PROTOCOLS: HttpStatusText = "Switching Protocols".as_bytes();
pub const HTTP_PROCESSING: HttpStatusText = "Processing".as_bytes();
pub const HTTP_EARLY_HINTS: HttpStatusText = "Early Hints".as_bytes();
pub const HTTP_OK: HttpStatusText = "OK".as_bytes();
pub const HTTP_CREATED: HttpStatusText = "Created".as_bytes();
pub const HTTP_ACCEPTED: HttpStatusText = "Accepted".as_bytes();
pub const HTTP_NON_AUTHORITATIVE_INFORMATION: HttpStatusText =
    "Non-Authoritative Information".as_bytes();
pub const HTTP_NO_CONTENT: HttpStatusText = "No Content".as_bytes();
pub const HTTP_RESET_CONTENT: HttpStatusText = "Reset Content".as_bytes();
pub const HTTP_PARTIAL_CONTENT: HttpStatusText = "Partial Content".as_bytes();
pub const HTTP_MULTI_STATUS: HttpStatusText = "Multi-Status".as_bytes();
pub const HTTP_ALREADY_REPORTED: HttpStatusText = "Already Reported".as_bytes();
pub const HTTP_IM_USED: HttpStatusText = "IM Used".as_bytes();
pub const HTTP_MULTIPLE_CHOICES: HttpStatusText = "Multiple Choices".as_bytes();
pub const HTTP_MOVED_PERMANENTLY: HttpStatusText = "Moved Permanently".as_bytes();
pub const HTTP_FOUND: HttpStatusText = "Found".as_bytes();
pub const HTTP_SEE_OTHER: HttpStatusText = "See Other".as_bytes();
pub const HTTP_NOT_MODIFIED: HttpStatusText = "Not Modified".as_bytes();
pub const HTTP_TEMPORARY_REDIRECT: HttpStatusText = "Temporary Redirect".as_bytes();
pub const HTTP_PREMANENT_REDIRECT: HttpStatusText = "Permanent Redirect".as_bytes();
pub const HTTP_BAD_REQUEST: HttpStatusText = "Bad Request".as_bytes();
pub const HTTP_UNAUTHORIZED: HttpStatusText = "Unauthorized".as_bytes();
pub const HTTP_PAYMENT_REQUIRED: HttpStatusText = "Payment Required".as_bytes();
pub const HTTP_FORBIDDEN: HttpStatusText = "Forbidden".as_bytes();
pub const HTTP_NOT_FOUND: HttpStatusText = "Not Found".as_bytes();
pub const HTTP_METHOD_NOT_ALLOWED: HttpStatusText = "Method Not Allowed".as_bytes();
pub const HTTP_NOT_ACCEPTABLE: HttpStatusText = "Not Acceptable".as_bytes();
pub const HTTP_PROXY_AUTHENTICATION_REQUIRED: HttpStatusText =
    "Proxy Authentication Required".as_bytes();
pub const HTTP_REQUEST_TIMEOUT: HttpStatusText = "Request Timeout".as_bytes();
pub const HTTP_CONFLICT: HttpStatusText = "Conflict".as_bytes();
pub const HTTP_GONE: HttpStatusText = "Gone".as_bytes();
pub const HTTP_LENGTH_REQUIRED: HttpStatusText = "Length Required".as_bytes();
pub const HTTP_PRECONDITION_FAILED: HttpStatusText = "Precondition Failed".as_bytes();
pub const HTTP_CONTENT_TOO_LARGE: HttpStatusText = "Content Too Large".as_bytes();
pub const HTTP_URI_TOO_LONG: HttpStatusText = "URI Too Long".as_bytes();
pub const HTTP_UNSUPPORTED_MEDIA_TYPE: HttpStatusText = "Unsupported Media Type".as_bytes();
pub const HTTP_RANGE_NOT_SATISFIABLE: HttpStatusText = "Range Not Satisfiable".as_bytes();
pub const HTTP_EXPECTATION_FAILED: HttpStatusText = "Expectation Failed".as_bytes();
pub const HTTP_TEAPOT: HttpStatusText = "I'm a teapot".as_bytes();
pub const HTTP_MISDIRECTED_REQUEST: HttpStatusText = "Misdirected Request".as_bytes();
pub const HTTP_UNPROCESSABLE_CONTENT: HttpStatusText = "Unprocessable Content".as_bytes();
pub const HTTP_LOCKED: HttpStatusText = "Locked".as_bytes();
pub const HTTP_FAILED_DEPENDENCY: HttpStatusText = "Failed Dependency".as_bytes();
pub const HTTP_TOO_EARLY: HttpStatusText = "Too Early".as_bytes();
pub const HTTP_UPGRADE_REQUIRED: HttpStatusText = "Upgrade Required".as_bytes();
pub const HTTP_PRECONDITION_REQUIRED: HttpStatusText = "Precondition Required".as_bytes();
pub const HTTP_TOO_MANY_REQUESTS: HttpStatusText = "Too Many Requests".as_bytes();
pub const HTTP_REQUEST_HEADER_FIELDS_TOO_LARGE: HttpStatusText =
    "Request Header Fields Too Large".as_bytes();
pub const HTTP_UNAVAILABLE_FOR_LEGAL_REASONS: HttpStatusText =
    "Unavailable For Legal Reasons".as_bytes();
pub const HTTP_INTERNAL_SERVER_ERROR: HttpStatusText = "Internal Server Error".as_bytes();
pub const HTTP_NOT_IMPLEMENTED: HttpStatusText = "Not Implemented".as_bytes();
pub const HTTP_BAD_GATEWAY: HttpStatusText = "Bad Gateway".as_bytes();
pub const HTTP_SERVICE_UNAVAILABLE: HttpStatusText = "Service Unavailable".as_bytes();
pub const HTTP_GATEWAY_TIMEOUT: HttpStatusText = "Gateway Timeout".as_bytes();
pub const HTTP_VERSION_NOT_SUPPORTED: HttpStatusText = "HTTP Version Not Supported".as_bytes();
pub const HTTP_VARIANT_ALSO_NEGOTIATES: HttpStatusText = "Variant Also Negotiates".as_bytes();
pub const HTTP_INSUFFICENT_STORAGE: HttpStatusText = "Insufficient Storage".as_bytes();
pub const HTTP_LOOP_DETECTED: HttpStatusText = "Loop Detected".as_bytes();
pub const HTTP_NOT_EXTENDED: HttpStatusText = "Not Extended".as_bytes();
pub const HTTP_NETWORK_AUTHENTICATION_REQUIRED: HttpStatusText =
    "Network Authentication Required".as_bytes();

// Hypertext Transfer Protocol Status Code and Status Text Hash Map
pub fn status_codes_map() -> HashMap<HttpStatusCode, HttpStatusText> {
    let http_status_codes: HashMap<HttpStatusCode, HttpStatusText> = HashMap::from([
        (HTTP_ONE_HUNDRED, HTTP_CONTINUE),
        (HTTP_ONE_HUNDRED_ONE, HTTP_SWITCHING_PROTOCOLS),
        (HTTP_ONE_HUNDRED_TWO, HTTP_PROCESSING),
        (HTTP_ONE_HUNDRED_THREE, HTTP_EARLY_HINTS),
        (HTTP_TWO_HUNDRED, HTTP_OK),
        (HTTP_TWO_HUNDRED_ONE, HTTP_CREATED),
        (HTTP_TWO_HUNDRED_TWO, HTTP_ACCEPTED),
        (HTTP_TWO_HUNDRED_THREE, HTTP_NON_AUTHORITATIVE_INFORMATION),
        (HTTP_TWO_HUNDRED_FOUR, HTTP_NO_CONTENT),
        (HTTP_TWO_HUNDRED_FIVE, HTTP_RESET_CONTENT),
        (HTTP_TWO_HUNDRED_SIX, HTTP_PARTIAL_CONTENT),
        (HTTP_TWO_HUNDRED_SEVEN, HTTP_MULTI_STATUS),
        (HTTP_TWO_HUNDRED_EIGHT, HTTP_ALREADY_REPORTED),
        (HTTP_TWO_HUNDRED_TWENTY_SIX, HTTP_IM_USED),
        (HTTP_THREE_HUNDRED, HTTP_MULTIPLE_CHOICES),
        (HTTP_THREE_HUNDRED_ONE, HTTP_MOVED_PERMANENTLY),
        (HTTP_THREE_HUNDRED_TWO, HTTP_FOUND),
        (HTTP_THREE_HUNDRED_THREE, HTTP_SEE_OTHER),
        (HTTP_THREE_HUNDRED_FOUR, HTTP_NOT_MODIFIED),
        (HTTP_THREE_HUNDRED_SEVEN, HTTP_TEMPORARY_REDIRECT),
        (HTTP_THREE_HUNDRED_EIGHT, HTTP_PREMANENT_REDIRECT),
        (HTTP_FOUR_HUNDRED, HTTP_BAD_REQUEST),
        (HTTP_FOUR_HUNDRED_ONE, HTTP_UNAUTHORIZED),
        (HTTP_FOUR_HUNDRED_TWO, HTTP_PAYMENT_REQUIRED),
        (HTTP_FOUR_HUNDRED_THREE, HTTP_FORBIDDEN),
        (HTTP_FOUR_HUNDRED_FOUR, HTTP_NOT_FOUND),
        (HTTP_FOUR_HUNDRED_FIVE, HTTP_METHOD_NOT_ALLOWED),
        (HTTP_FOUR_HUNDRED_SIX, HTTP_NOT_ACCEPTABLE),
        (HTTP_FOUR_HUNDRED_SEVEN, HTTP_PROXY_AUTHENTICATION_REQUIRED),
        (HTTP_FOUR_HUNDRED_EIGHT, HTTP_REQUEST_TIMEOUT),
        (HTTP_FOUR_HUNDRED_NINE, HTTP_CONFLICT),
        (HTTP_FOUR_HUNDRED_TEN, HTTP_GONE),
        (HTTP_FOUR_HUNDRED_ELEVEN, HTTP_LENGTH_REQUIRED),
        (HTTP_FOUR_HUNDRED_TWELVE, HTTP_PRECONDITION_FAILED),
        (HTTP_FOUR_HUNDRED_THIRTEEN, HTTP_CONTENT_TOO_LARGE),
        (HTTP_FOUR_HUNDRED_FOURTEEN, HTTP_URI_TOO_LONG),
        (HTTP_FOUR_HUNDRED_FIFTEEN, HTTP_UNSUPPORTED_MEDIA_TYPE),
        (HTTP_FOUR_HUNDRED_SIXTEEN, HTTP_RANGE_NOT_SATISFIABLE),
        (HTTP_FOUR_HUNDRED_SEVENTEEN, HTTP_EXPECTATION_FAILED),
        (HTTP_FOUR_HUNDRED_EIGHTEEN, HTTP_TEAPOT),
        (HTTP_FOUR_HUNDRED_TWENTY_ONE, HTTP_MISDIRECTED_REQUEST),
        (HTTP_FOUR_HUNDRED_TWENTY_TWO, HTTP_UNPROCESSABLE_CONTENT),
        (HTTP_FOUR_HUNDRED_TWENTY_THREE, HTTP_LOCKED),
        (HTTP_FOUR_HUNDRED_TWENTY_FOUR, HTTP_FAILED_DEPENDENCY),
        (HTTP_FOUR_HUNDRED_TWENTY_FIVE, HTTP_TOO_EARLY),
        (HTTP_FOUR_HUNDRED_TWENTY_SIX, HTTP_UPGRADE_REQUIRED),
        (HTTP_FOUR_HUNDRED_TWENTY_EIGHT, HTTP_PRECONDITION_REQUIRED),
        (HTTP_FOUR_HUNDRED_TWENTY_NINE, HTTP_TOO_MANY_REQUESTS),
        (
            HTTP_FOUR_HUNDRED_THIRTY_ONE,
            HTTP_REQUEST_HEADER_FIELDS_TOO_LARGE,
        ),
        (
            HTTP_FOUR_HUNDRED_FIFTEY_ONE,
            HTTP_UNAVAILABLE_FOR_LEGAL_REASONS,
        ),
        (HTTP_FIVE_HUNDRED, HTTP_INTERNAL_SERVER_ERROR),
        (HTTP_FIVE_HUNDRED_ONE, HTTP_NOT_IMPLEMENTED),
        (HTTP_FIVE_HUNDRED_TWO, HTTP_BAD_GATEWAY),
        (HTTP_FIVE_HUNDRED_THREE, HTTP_SERVICE_UNAVAILABLE),
        (HTTP_FIVE_HUNDRED_FOUR, HTTP_GATEWAY_TIMEOUT),
        (HTTP_FIVE_HUNDRED_FIVE, HTTP_VERSION_NOT_SUPPORTED),
        (HTTP_FIVE_HUNDRED_SIX, HTTP_VARIANT_ALSO_NEGOTIATES),
        (HTTP_FIVE_HUNDRED_SEVEN, HTTP_INSUFFICENT_STORAGE),
        (HTTP_FIVE_HUNDRED_EIGHT, HTTP_LOOP_DETECTED),
        (HTTP_FIVE_HUNDRED_TEN, HTTP_NOT_EXTENDED),
        (
            HTTP_FIVE_HUNDRED_ELEVEN,
            HTTP_NETWORK_AUTHENTICATION_REQUIRED,
        ),
    ]);

    return http_status_codes;
}
