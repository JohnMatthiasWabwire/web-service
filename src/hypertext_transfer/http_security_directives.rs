#![allow(dead_code)]

use std::{primitive::u8, vec::Vec};

// Hypertext Transfer Protocol Content Security Policy Directive Definition
pub type HttpSecurityDirective = &'static [u8];

// Hypertext Transfer Protocol Content Security Policy Directives
pub const HTTP_BASE_URI: HttpSecurityDirective = "base-uri".as_bytes();
pub const HTTP_CHILD_SOURCE: HttpSecurityDirective = "child-src".as_bytes();
pub const HTTP_CONNECT_SOURCE: HttpSecurityDirective = "connect-src".as_bytes();
pub const HTTP_DEFAULT_SOURCE: HttpSecurityDirective = "default-src".as_bytes();
pub const HTTP_FONT_SOURCE: HttpSecurityDirective = "font-src".as_bytes();
pub const HTTP_FORM_ACTION: HttpSecurityDirective = "form-action".as_bytes();
pub const HTTP_FRAME_ANCESTORS: HttpSecurityDirective = "frame-ancestors".as_bytes();
pub const HTTP_FRAME_SOURCE: HttpSecurityDirective = "frame-src".as_bytes();
pub const HTTP_IMAGE_SOURCE: HttpSecurityDirective = "img-src".as_bytes();
pub const HTTP_MANIFEST_SOURCE: HttpSecurityDirective = "manifest-src".as_bytes();
pub const HTTP_MEDIA_SOURCE: HttpSecurityDirective = "media-src".as_bytes();
pub const HTTP_OBJECT_SOURCE: HttpSecurityDirective = "object-src".as_bytes();
pub const HTTP_REPORT_TO: HttpSecurityDirective = "report-to".as_bytes();
pub const HTTP_REQUIRE_TRUSTED_TYPES_FOR: HttpSecurityDirective =
    "require-trusted-types-for".as_bytes();
pub const HTTP_SANDBOX: HttpSecurityDirective = "sandbox".as_bytes();
pub const HTTP_SCRIPT_SOURCE: HttpSecurityDirective = "script-src".as_bytes();
pub const HTTP_SCRIPT_SOURCE_ATTRIBUTE: HttpSecurityDirective = "script-src-attr".as_bytes();
pub const HTTP_SCRIPT_SOURCE_ELEMENT: HttpSecurityDirective = "script-src-elem".as_bytes();
pub const HTTP_STYLE_SOURCE: HttpSecurityDirective = "style-src".as_bytes();
pub const HTTP_STYLE_SOURCE_ATTRIBUTE: HttpSecurityDirective = "style-src-attr".as_bytes();
pub const HTTP_STYLE_SOURCE_ELEMENT: HttpSecurityDirective = "style-src-elem".as_bytes();
pub const HTTP_TRUSTED_TYPES: HttpSecurityDirective = "trusted-types".as_bytes();
pub const HTTP_UPGRADE_INSECURE_REQUESTS: HttpSecurityDirective =
    "upgrade-insecure-requests".as_bytes();
pub const HTTP_WORKER_SOURCE: HttpSecurityDirective = "worker-src".as_bytes();

// Hypertext Transfer Protocol Content Security Policy Directive Vector
pub fn security_directives_vector() -> Vec<HttpSecurityDirective> {
    let security_directives: Vec<HttpSecurityDirective> = Vec::from([
        HTTP_BASE_URI,
        HTTP_CHILD_SOURCE,
        HTTP_CONNECT_SOURCE,
        HTTP_DEFAULT_SOURCE,
        HTTP_FONT_SOURCE,
        HTTP_FORM_ACTION,
        HTTP_FRAME_ANCESTORS,
        HTTP_FRAME_SOURCE,
        HTTP_IMAGE_SOURCE,
        HTTP_MANIFEST_SOURCE,
        HTTP_MEDIA_SOURCE,
        HTTP_OBJECT_SOURCE,
        HTTP_REPORT_TO,
        HTTP_REQUIRE_TRUSTED_TYPES_FOR,
        HTTP_SANDBOX,
        HTTP_SCRIPT_SOURCE,
        HTTP_SCRIPT_SOURCE_ATTRIBUTE,
        HTTP_SCRIPT_SOURCE_ELEMENT,
        HTTP_STYLE_SOURCE,
        HTTP_STYLE_SOURCE_ATTRIBUTE,
        HTTP_STYLE_SOURCE_ELEMENT,
        HTTP_TRUSTED_TYPES,
        HTTP_UPGRADE_INSECURE_REQUESTS,
        HTTP_WORKER_SOURCE,
    ]);

    return security_directives;
}
