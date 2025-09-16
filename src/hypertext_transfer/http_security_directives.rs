#![allow(dead_code)]

use std::{primitive::str, vec::Vec};

// Hypertext Transfer Protocol Content Security Policy Directive Definition
pub type HttpSecurityDirective = &'static str;

// Hypertext Transfer Protocol Content Security Policy Directives
pub const HTTP_BASE_URI: HttpSecurityDirective = "base-uri";
pub const HTTP_CHILD_SOURCE: HttpSecurityDirective = "child-src";
pub const HTTP_CONNECT_SOURCE: HttpSecurityDirective = "connect-src";
pub const HTTP_DEFAULT_SOURCE: HttpSecurityDirective = "default-src";
pub const HTTP_FONT_SOURCE: HttpSecurityDirective = "font-src";
pub const HTTP_FORM_ACTION: HttpSecurityDirective = "form-action";
pub const HTTP_FRAME_ANCESTORS: HttpSecurityDirective = "frame-ancestors";
pub const HTTP_FRAME_SOURCE: HttpSecurityDirective = "frame-src";
pub const HTTP_IMAGE_SOURCE: HttpSecurityDirective = "img-src";
pub const HTTP_MANIFEST_SOURCE: HttpSecurityDirective = "manifest-src";
pub const HTTP_MEDIA_SOURCE: HttpSecurityDirective = "media-src";
pub const HTTP_OBJECT_SOURCE: HttpSecurityDirective = "object-src";
pub const HTTP_REPORT_TO: HttpSecurityDirective = "report-to";
pub const HTTP_REQUIRE_TRUSTED_TYPES_FOR: HttpSecurityDirective =
    "require-trusted-types-for";
pub const HTTP_SANDBOX: HttpSecurityDirective = "sandbox";
pub const HTTP_SCRIPT_SOURCE: HttpSecurityDirective = "script-src";
pub const HTTP_SCRIPT_SOURCE_ATTRIBUTE: HttpSecurityDirective = "script-src-attr";
pub const HTTP_SCRIPT_SOURCE_ELEMENT: HttpSecurityDirective = "script-src-elem";
pub const HTTP_STYLE_SOURCE: HttpSecurityDirective = "style-src";
pub const HTTP_STYLE_SOURCE_ATTRIBUTE: HttpSecurityDirective = "style-src-attr";
pub const HTTP_STYLE_SOURCE_ELEMENT: HttpSecurityDirective = "style-src-elem";
pub const HTTP_TRUSTED_TYPES: HttpSecurityDirective = "trusted-types";
pub const HTTP_UPGRADE_INSECURE_REQUESTS: HttpSecurityDirective =
    "upgrade-insecure-requests";
pub const HTTP_WORKER_SOURCE: HttpSecurityDirective = "worker-src";

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
