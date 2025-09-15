#![allow(dead_code)]

use std::{primitive::str, vec::Vec};

// Hypertext Transfer Protocol Header Definition
pub type HttpHeader = &'static str;

// Hypertext Transfer Protocol Headers
pub const HTTP_ACCEPT: HttpHeader = "Accept"
pub const HTTP_ACCEPT_CLIENT_HINT: HttpHeader = "Accept-CH"
pub const HTTP_ACCEPT_ENCODING: HttpHeader = "Accept-Encoding"
pub const HTTP_ACCEPT_LANGUAGE: HttpHeader = "Accept-Language"
pub const HTTP_ACCEPT_PATCH: HttpHeader = "Accept-Patch"
pub const HTTP_ACCEPT_POST: HttpHeader = "Accept-Post"
pub const HTTP_ACCEPT_RANGES: HttpHeader = "Accept-Ranges"
pub const HTTP_ACCESS_CONTROL_ALLOW_CREDENTIALS: HttpHeader =
    "Access-Control-Allow-Credentials"
pub const HTTP_ACCESS_CONTROL_ALLOW_HEADERS: HttpHeader = "Access-Control-Allow-Headers"
pub const HTTP_ACCESS_CONTROL_ALLOW_METHODS: HttpHeader = "Access-Control-Allow-Methods"
pub const HTTP_ACCESS_CONTROL_ALLOW_ORIGIN: HttpHeader = "Access-Control-Allow-Origin"
pub const HTTP_ACCESS_CONTROL_EXPOSE_HEADERS: HttpHeader =
    "Access-Control-Expose-Headers"
pub const HTTP_ACCESS_CONTROL_MAX_AGE: HttpHeader = "Access-Control-Max-Age"
pub const HTTP_ACCESS_CONTROL_REQUEST_HEADERS: HttpHeader =
    "Access-Control-Request-Headers"
pub const HTTP_ACCESS_CONTROL_REQUEST_METHOD: HttpHeader =
    "Access-Control-Request-Method"
pub const HTTP_AGE: HttpHeader = "Age"
pub const HTTP_ALLOW: HttpHeader = "Allow"
pub const HTTP_ALTERNATIVE_SERVICE: HttpHeader = "Alt-Svc"
pub const HTTP_ALTERNATIVE_USED: HttpHeader = "Alt-Used"
pub const HTTP_AUTHORIZATION: HttpHeader = "Authorization"
pub const HTTP_CACHE_CONTROL: HttpHeader = "Cache-Control"
pub const HTTP_CLEAR_SITE_DATA: HttpHeader = "Clear-Site-Data"
pub const HTTP_CONNECTION: HttpHeader = "Connection"
pub const HTTP_CONTENT_DIGEST: HttpHeader = "Content-Digest"
pub const HTTP_CONTENT_DISPOSITION: HttpHeader = "Content-Disposition"
pub const HTTP_CONTENT_ENCODING: HttpHeader = "Content-Encoding"
pub const HTTP_CONTENT_LANGUAGE: HttpHeader = "Content-Language"
pub const HTTP_CONTENT_LENGTH: HttpHeader = "Content-Length"
pub const HTTP_CONTENT_LOCATION: HttpHeader = "Content-Location"
pub const HTTP_CONTENT_RANGE: HttpHeader = "Content-Range"
pub const HTTP_CONTENT_SECURITY_POLICY: HttpHeader = "Content-Security-Policy"
pub const HTTP_CONTENT_SECURITY_POLICY_REPORT_ONLY: HttpHeader =
    "Content-Security-Policy-Report-Only"
pub const HTTP_CONTENT_TYPE: HttpHeader = "Content-Type"
pub const HTTP_COOKIE: HttpHeader = "Cookie"
pub const HTTP_CROSS_ORIGIN_EMBEDDER_POLICY: HttpHeader = "Cross-Origin-Embedder-Policy"
pub const HTTP_CROSS_ORIGIN_OPENER_POLICY: HttpHeader = "Cross-Origin-Opener-Policy"
pub const HTTP_CROSS_ORIGIN_RESOURCE_POLICY: HttpHeader = "Cross-Origin-Resource-Policy"
pub const HTTP_DATE: HttpHeader = "Date"
pub const HTTP_DEVICE_MEMORY: HttpHeader = "Device-Memory"
pub const HTTP_ENTITY_TAG: HttpHeader = "ETag"
pub const HTTP_EXPECT: HttpHeader = "Expect"
pub const HTTP_EXPIRES: HttpHeader = "Expires"
pub const HTTP_FORWARDED: HttpHeader = "Forwarded"
pub const HTTP_FROM: HttpHeader = "From"
pub const HTTP_HOST: HttpHeader = "Host"
pub const HTTP_IF_MATCH: HttpHeader = "If-Match"
pub const HTTP_IF_MODIFIED_SINCE: HttpHeader = "If-Modified-Since"
pub const HTTP_IF_NONE_MATCH: HttpHeader = "If-None-Match"
pub const HTTP_IF_RANGE: HttpHeader = "If-Range"
pub const HTTP_IF_UNMODIFIED_SINCE: HttpHeader = "If-Unmodified-Since"
pub const HTTP_KEEP_ALIVE: HttpHeader = "Keep-Alive"
pub const HTTP_LAST_MODIFIED: HttpHeader = "Last-Modified"
pub const HTTP_LINK: HttpHeader = "Link"
pub const HTTP_LOCATION: HttpHeader = "Location"
pub const HTTP_MAXIMUM_FORWARDS: HttpHeader = "Max-Forwards"
pub const HTTP_ORIGIN: HttpHeader = "Origin"
pub const HTTP_PRIORITY: HttpHeader = "Priority"
pub const HTTP_PROXY_AUTHENTICATION: HttpHeader = "Proxy-Authenticate"
pub const HTTP_PROXY_AUTHORIZATION: HttpHeader = "Proxy-Authorization"
pub const HTTP_RANGE: HttpHeader = "Range"
pub const HTTP_REFERER: HttpHeader = "Referer"
pub const HTTP_REFERRER_POLICY: HttpHeader = "Referrer-Policy"
pub const HTTP_REFRESH: HttpHeader = "Refresh"
pub const HTTP_REPR_DIGEST: HttpHeader = "Repr-Digest"
pub const HTTP_RETRY_AFTER: HttpHeader = "Retry-After"
pub const HTTP_SECURE_FETCH_DESTINATION: HttpHeader = "Sec-Fetch-Dest"
pub const HTTP_SECURE_FETCH_MODE: HttpHeader = "Sec-Fetch-Mode"
pub const HTTP_SECURE_FETCH_SITE: HttpHeader = "Sec-Fetch-Site"
pub const HTTP_SECURE_FETCH_USER: HttpHeader = "Sec-Fetch-User"
pub const HTTP_SECURE_PURPOSE: HttpHeader = "Sec-Purpose"
pub const HTTP_SECURE_WEBSOCKET_ACCEPT: HttpHeader = "Sec-WebSocket-Accept"
pub const HTTP_SECURE_WEBSOCKET_EXTENTIONS: HttpHeader = "Sec-WebSocket-Extensions"
pub const HTTP_SECURE_WEBSOCKET_KEY: HttpHeader = "Sec-WebSocket-Key"
pub const HTTP_SECURE_WEBSOCKET_PROTOCOL: HttpHeader = "Sec-WebSocket-Protocol"
pub const HTTP_SECURE_WEBSOCKET_VERSION: HttpHeader = "Sec-WebSocket-Version"
pub const HTTP_SERVER: HttpHeader = "Server"
pub const HTTP_SERVER_TIMING: HttpHeader = "Server-Timing"
pub const HTTP_SERVICE_WORKER: HttpHeader = "Service-Worker"
pub const HTTP_SERVICE_WORKER_ALLOWED: HttpHeader = "Service-Worker-Allowed"
pub const HTTP_SERVICE_WORKER_NAVIGATION_PRELOAD: HttpHeader =
    "Service-Worker-Navigation-Preload"
pub const HTTP_SET_COOKIE: HttpHeader = "Set-Cookie"
pub const HTTP_SOURCEMAP: HttpHeader = "SourceMap"
pub const HTTP_STRICT_TRANSPORT_SECURITY: HttpHeader = "Strict-Transport-Security"
pub const HTTP_TRANSFER_ENCODING_TYPE: HttpHeader = "TE"
pub const HTTP_TIMING_ALLOW_ORIGIN: HttpHeader = "Timing-Allow-Origin"
pub const HTTP_TRAILER: HttpHeader = "Trailer"
pub const HTTP_TRANSFER_ENCODING: HttpHeader = "Transfer-Encoding"
pub const HTTP_UPGRADE: HttpHeader = "Upgrade"
pub const HTTP_UPGRADE_REQUESTS: HttpHeader = "Upgrade-Insecure-Requests"
pub const HTTP_USER_AGENT: HttpHeader = "User-Agent"
pub const HTTP_VARY: HttpHeader = "Vary"
pub const HTTP_VIA_PROXY: HttpHeader = "Via"
pub const HTTP_WANT_CONTENT_DIGEST: HttpHeader = "Want-Content-Digest"
pub const HTTP_WANT_REPRESENTATION_DIGEST: HttpHeader = "Want-Repr-Digest"
pub const HTTP_WWW_AUTHENTICATE: HttpHeader = "WWW-Authenticate"
pub const HTTP_X_CONTENT_TYPE_OPTIONS: HttpHeader = "X-Content-Type-Options"
pub const HTTP_X_FRAME_OPTIONS: HttpHeader = "X-Frame-Options"

// Hypertext Transfer Protocol Header Vector
pub fn headers_vector() -> Vec<HttpHeader> {
    let headers: Vec<HttpHeader> = Vec::from([
        HTTP_ACCEPT,
        HTTP_ACCEPT_CLIENT_HINT,
        HTTP_ACCEPT_ENCODING,
        HTTP_ACCEPT_LANGUAGE,
        HTTP_ACCEPT_PATCH,
        HTTP_ACCEPT_POST,
        HTTP_ACCEPT_RANGES,
        HTTP_ACCESS_CONTROL_ALLOW_CREDENTIALS,
        HTTP_ACCESS_CONTROL_ALLOW_HEADERS,
        HTTP_ACCESS_CONTROL_ALLOW_METHODS,
        HTTP_ACCESS_CONTROL_ALLOW_ORIGIN,
        HTTP_ACCESS_CONTROL_EXPOSE_HEADERS,
        HTTP_ACCESS_CONTROL_MAX_AGE,
        HTTP_ACCESS_CONTROL_REQUEST_HEADERS,
        HTTP_ACCESS_CONTROL_REQUEST_METHOD,
        HTTP_AGE,
        HTTP_ALLOW,
        HTTP_ALTERNATIVE_SERVICE,
        HTTP_ALTERNATIVE_USED,
        HTTP_AUTHORIZATION,
        HTTP_CACHE_CONTROL,
        HTTP_CLEAR_SITE_DATA,
        HTTP_CONNECTION,
        HTTP_CONTENT_DIGEST,
        HTTP_CONTENT_DISPOSITION,
        HTTP_CONTENT_ENCODING,
        HTTP_CONTENT_LANGUAGE,
        HTTP_CONTENT_LENGTH,
        HTTP_CONTENT_LOCATION,
        HTTP_CONTENT_RANGE,
        HTTP_CONTENT_SECURITY_POLICY,
        HTTP_CONTENT_SECURITY_POLICY_REPORT_ONLY,
        HTTP_CONTENT_TYPE,
        HTTP_COOKIE,
        HTTP_CROSS_ORIGIN_EMBEDDER_POLICY,
        HTTP_CROSS_ORIGIN_OPENER_POLICY,
        HTTP_CROSS_ORIGIN_RESOURCE_POLICY,
        HTTP_DATE,
        HTTP_DEVICE_MEMORY,
        HTTP_ENTITY_TAG,
        HTTP_EXPECT,
        HTTP_EXPIRES,
        HTTP_FORWARDED,
        HTTP_FROM,
        HTTP_HOST,
        HTTP_IF_MATCH,
        HTTP_IF_MODIFIED_SINCE,
        HTTP_IF_NONE_MATCH,
        HTTP_IF_RANGE,
        HTTP_IF_UNMODIFIED_SINCE,
        HTTP_KEEP_ALIVE,
        HTTP_LAST_MODIFIED,
        HTTP_LINK,
        HTTP_LOCATION,
        HTTP_MAXIMUM_FORWARDS,
        HTTP_ORIGIN,
        HTTP_PRIORITY,
        HTTP_PROXY_AUTHENTICATION,
        HTTP_PROXY_AUTHORIZATION,
        HTTP_RANGE,
        HTTP_REFERER,
        HTTP_REFERRER_POLICY,
        HTTP_REFRESH,
        HTTP_REPR_DIGEST,
        HTTP_RETRY_AFTER,
        HTTP_SECURE_FETCH_DESTINATION,
        HTTP_SECURE_FETCH_MODE,
        HTTP_SECURE_FETCH_SITE,
        HTTP_SECURE_FETCH_USER,
        HTTP_SECURE_PURPOSE,
        HTTP_SECURE_WEBSOCKET_ACCEPT,
        HTTP_SECURE_WEBSOCKET_EXTENTIONS,
        HTTP_SECURE_WEBSOCKET_KEY,
        HTTP_SECURE_WEBSOCKET_PROTOCOL,
        HTTP_SECURE_WEBSOCKET_VERSION,
        HTTP_SERVER,
        HTTP_SERVER_TIMING,
        HTTP_SERVICE_WORKER,
        HTTP_SERVICE_WORKER_ALLOWED,
        HTTP_SERVICE_WORKER_NAVIGATION_PRELOAD,
        HTTP_SET_COOKIE,
        HTTP_SOURCEMAP,
        HTTP_STRICT_TRANSPORT_SECURITY,
        HTTP_TRANSFER_ENCODING_TYPE,
        HTTP_TIMING_ALLOW_ORIGIN,
        HTTP_TRAILER,
        HTTP_TRANSFER_ENCODING,
        HTTP_UPGRADE,
        HTTP_UPGRADE_REQUESTS,
        HTTP_USER_AGENT,
        HTTP_VARY,
        HTTP_VIA_PROXY,
        HTTP_WANT_CONTENT_DIGEST,
        HTTP_WANT_REPRESENTATION_DIGEST,
        HTTP_WWW_AUTHENTICATE,
        HTTP_X_CONTENT_TYPE_OPTIONS,
        HTTP_X_FRAME_OPTIONS,
    ]);

    return headers;
}
