#![allow(dead_code)]

use std::{primitive::u8, vec::Vec};

// Hypertext Transfer Protocol Header Definition
pub type HttpHeader = &'static [u8];

// Hypertext Transfer Protocol Headers
pub const HTTP_ACCEPT: HttpHeader = "Accept".as_bytes();
pub const HTTP_ACCEPT_CLIENT_HINT: HttpHeader = "Accept-CH".as_bytes();
pub const HTTP_ACCEPT_ENCODING: HttpHeader = "Accept-Encoding".as_bytes();
pub const HTTP_ACCEPT_LANGUAGE: HttpHeader = "Accept-Language".as_bytes();
pub const HTTP_ACCEPT_PATCH: HttpHeader = "Accept-Patch".as_bytes();
pub const HTTP_ACCEPT_POST: HttpHeader = "Accept-Post".as_bytes();
pub const HTTP_ACCEPT_RANGES: HttpHeader = "Accept-Ranges".as_bytes();
pub const HTTP_ACCESS_CONTROL_ALLOW_CREDENTIALS: HttpHeader =
    "Access-Control-Allow-Credentials".as_bytes();
pub const HTTP_ACCESS_CONTROL_ALLOW_HEADERS: HttpHeader = "Access-Control-Allow-Headers".as_bytes();
pub const HTTP_ACCESS_CONTROL_ALLOW_METHODS: HttpHeader = "Access-Control-Allow-Methods".as_bytes();
pub const HTTP_ACCESS_CONTROL_ALLOW_ORIGIN: HttpHeader = "Access-Control-Allow-Origin".as_bytes();
pub const HTTP_ACCESS_CONTROL_EXPOSE_HEADERS: HttpHeader =
    "Access-Control-Expose-Headers".as_bytes();
pub const HTTP_ACCESS_CONTROL_MAX_AGE: HttpHeader = "Access-Control-Max-Age".as_bytes();
pub const HTTP_ACCESS_CONTROL_REQUEST_HEADERS: HttpHeader =
    "Access-Control-Request-Headers".as_bytes();
pub const HTTP_ACCESS_CONTROL_REQUEST_METHOD: HttpHeader =
    "Access-Control-Request-Method".as_bytes();
pub const HTTP_AGE: HttpHeader = "Age".as_bytes();
pub const HTTP_ALLOW: HttpHeader = "Allow".as_bytes();
pub const HTTP_ALTERNATIVE_SERVICE: HttpHeader = "Alt-Svc".as_bytes();
pub const HTTP_ALTERNATIVE_USED: HttpHeader = "Alt-Used".as_bytes();
pub const HTTP_AUTHORIZATION: HttpHeader = "Authorization".as_bytes();
pub const HTTP_CACHE_CONTROL: HttpHeader = "Cache-Control".as_bytes();
pub const HTTP_CLEAR_SITE_DATA: HttpHeader = "Clear-Site-Data".as_bytes();
pub const HTTP_CONNECTION: HttpHeader = "Connection".as_bytes();
pub const HTTP_CONTENT_DIGEST: HttpHeader = "Content-Digest".as_bytes();
pub const HTTP_CONTENT_DISPOSITION: HttpHeader = "Content-Disposition".as_bytes();
pub const HTTP_CONTENT_ENCODING: HttpHeader = "Content-Encoding".as_bytes();
pub const HTTP_CONTENT_LANGUAGE: HttpHeader = "Content-Language".as_bytes();
pub const HTTP_CONTENT_LENGTH: HttpHeader = "Content-Length".as_bytes();
pub const HTTP_CONTENT_LOCATION: HttpHeader = "Content-Location".as_bytes();
pub const HTTP_CONTENT_RANGE: HttpHeader = "Content-Range".as_bytes();
pub const HTTP_CONTENT_SECURITY_POLICY: HttpHeader = "Content-Security-Policy".as_bytes();
pub const HTTP_CONTENT_SECURITY_POLICY_REPORT_ONLY: HttpHeader =
    "Content-Security-Policy-Report-Only".as_bytes();
pub const HTTP_CONTENT_TYPE: HttpHeader = "Content-Type".as_bytes();
pub const HTTP_COOKIE: HttpHeader = "Cookie".as_bytes();
pub const HTTP_CROSS_ORIGIN_EMBEDDER_POLICY: HttpHeader = "Cross-Origin-Embedder-Policy".as_bytes();
pub const HTTP_CROSS_ORIGIN_OPENER_POLICY: HttpHeader = "Cross-Origin-Opener-Policy".as_bytes();
pub const HTTP_CROSS_ORIGIN_RESOURCE_POLICY: HttpHeader = "Cross-Origin-Resource-Policy".as_bytes();
pub const HTTP_DATE: HttpHeader = "Date".as_bytes();
pub const HTTP_DEVICE_MEMORY: HttpHeader = "Device-Memory".as_bytes();
pub const HTTP_ENTITY_TAG: HttpHeader = "ETag".as_bytes();
pub const HTTP_EXPECT: HttpHeader = "Expect".as_bytes();
pub const HTTP_EXPIRES: HttpHeader = "Expires".as_bytes();
pub const HTTP_FORWARDED: HttpHeader = "Forwarded".as_bytes();
pub const HTTP_FROM: HttpHeader = "From".as_bytes();
pub const HTTP_HOST: HttpHeader = "Host".as_bytes();
pub const HTTP_IF_MATCH: HttpHeader = "If-Match".as_bytes();
pub const HTTP_IF_MODIFIED_SINCE: HttpHeader = "If-Modified-Since".as_bytes();
pub const HTTP_IF_NONE_MATCH: HttpHeader = "If-None-Match".as_bytes();
pub const HTTP_IF_RANGE: HttpHeader = "If-Range".as_bytes();
pub const HTTP_IF_UNMODIFIED_SINCE: HttpHeader = "If-Unmodified-Since".as_bytes();
pub const HTTP_KEEP_ALIVE: HttpHeader = "Keep-Alive".as_bytes();
pub const HTTP_LAST_MODIFIED: HttpHeader = "Last-Modified".as_bytes();
pub const HTTP_LINK: HttpHeader = "Link".as_bytes();
pub const HTTP_LOCATION: HttpHeader = "Location".as_bytes();
pub const HTTP_MAXIMUM_FORWARDS: HttpHeader = "Max-Forwards".as_bytes();
pub const HTTP_ORIGIN: HttpHeader = "Origin".as_bytes();
pub const HTTP_PRIORITY: HttpHeader = "Priority".as_bytes();
pub const HTTP_PROXY_AUTHENTICATION: HttpHeader = "Proxy-Authenticate".as_bytes();
pub const HTTP_PROXY_AUTHORIZATION: HttpHeader = "Proxy-Authorization".as_bytes();
pub const HTTP_RANGE: HttpHeader = "Range".as_bytes();
pub const HTTP_REFERER: HttpHeader = "Referer".as_bytes();
pub const HTTP_REFERRER_POLICY: HttpHeader = "Referrer-Policy".as_bytes();
pub const HTTP_REFRESH: HttpHeader = "Refresh".as_bytes();
pub const HTTP_REPR_DIGEST: HttpHeader = "Repr-Digest".as_bytes();
pub const HTTP_RETRY_AFTER: HttpHeader = "Retry-After".as_bytes();
pub const HTTP_SECURE_FETCH_DESTINATION: HttpHeader = "Sec-Fetch-Dest".as_bytes();
pub const HTTP_SECURE_FETCH_MODE: HttpHeader = "Sec-Fetch-Mode".as_bytes();
pub const HTTP_SECURE_FETCH_SITE: HttpHeader = "Sec-Fetch-Site".as_bytes();
pub const HTTP_SECURE_FETCH_USER: HttpHeader = "Sec-Fetch-User".as_bytes();
pub const HTTP_SECURE_PURPOSE: HttpHeader = "Sec-Purpose".as_bytes();
pub const HTTP_SECURE_WEBSOCKET_ACCEPT: HttpHeader = "Sec-WebSocket-Accept".as_bytes();
pub const HTTP_SECURE_WEBSOCKET_EXTENTIONS: HttpHeader = "Sec-WebSocket-Extensions".as_bytes();
pub const HTTP_SECURE_WEBSOCKET_KEY: HttpHeader = "Sec-WebSocket-Key".as_bytes();
pub const HTTP_SECURE_WEBSOCKET_PROTOCOL: HttpHeader = "Sec-WebSocket-Protocol".as_bytes();
pub const HTTP_SECURE_WEBSOCKET_VERSION: HttpHeader = "Sec-WebSocket-Version".as_bytes();
pub const HTTP_SERVER: HttpHeader = "Server".as_bytes();
pub const HTTP_SERVER_TIMING: HttpHeader = "Server-Timing".as_bytes();
pub const HTTP_SERVICE_WORKER: HttpHeader = "Service-Worker".as_bytes();
pub const HTTP_SERVICE_WORKER_ALLOWED: HttpHeader = "Service-Worker-Allowed".as_bytes();
pub const HTTP_SERVICE_WORKER_NAVIGATION_PRELOAD: HttpHeader =
    "Service-Worker-Navigation-Preload".as_bytes();
pub const HTTP_SET_COOKIE: HttpHeader = "Set-Cookie".as_bytes();
pub const HTTP_SOURCEMAP: HttpHeader = "SourceMap".as_bytes();
pub const HTTP_STRICT_TRANSPORT_SECURITY: HttpHeader = "Strict-Transport-Security".as_bytes();
pub const HTTP_TRANSFER_ENCODING_TYPE: HttpHeader = "TE".as_bytes();
pub const HTTP_TIMING_ALLOW_ORIGIN: HttpHeader = "Timing-Allow-Origin".as_bytes();
pub const HTTP_TRAILER: HttpHeader = "Trailer".as_bytes();
pub const HTTP_TRANSFER_ENCODING: HttpHeader = "Transfer-Encoding".as_bytes();
pub const HTTP_UPGRADE: HttpHeader = "Upgrade".as_bytes();
pub const HTTP_UPGRADE_REQUESTS: HttpHeader = "Upgrade-Insecure-Requests".as_bytes();
pub const HTTP_USER_AGENT: HttpHeader = "User-Agent".as_bytes();
pub const HTTP_VARY: HttpHeader = "Vary".as_bytes();
pub const HTTP_VIA_PROXY: HttpHeader = "Via".as_bytes();
pub const HTTP_WANT_CONTENT_DIGEST: HttpHeader = "Want-Content-Digest".as_bytes();
pub const HTTP_WANT_REPRESENTATION_DIGEST: HttpHeader = "Want-Repr-Digest".as_bytes();
pub const HTTP_WWW_AUTHENTICATE: HttpHeader = "WWW-Authenticate".as_bytes();
pub const HTTP_X_CONTENT_TYPE_OPTIONS: HttpHeader = "X-Content-Type-Options".as_bytes();
pub const HTTP_X_FRAME_OPTIONS: HttpHeader = "X-Frame-Options".as_bytes();

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
