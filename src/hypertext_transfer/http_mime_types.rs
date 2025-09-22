#![allow(dead_code)]

use std::{primitive::str, vec::Vec};

// Hypertext Transfer Protocol MIME Type Definition
pub type HttpMimeType = &'static str;

// Hypertext Transfer Protocol MIME Types
pub const HTTP_JSON_MIME_TYPE: HttpMimeType = "application/json;charset=UTF-8";
pub const HTTP_JWT_MIME_TYPE: HttpMimeType = "application/jwt;charset=UTF-8";
pub const HTTP_MATHML_MIME_TYPE: HttpMimeType = "application/mathml+xml;charset=UTF-8";
pub const HTTP_PDF_MIME_TYPE: HttpMimeType = "application/pdf;charset=UTF-8";
pub const HTTP_TOML_MIME_TYPE: HttpMimeType = "application/toml;charset=UTF-8";
pub const HTTP_YAML_MIME_TYPE: HttpMimeType = "application/yaml;charset=UTF-8";
pub const HTTP_ZIP_MIME_TYPE: HttpMimeType = "application/zip";
pub const HTTP_ZLIB_MIME_TYPE: HttpMimeType = "application/zlib";
pub const HTTP_ZSTD_MIME_TYPE: HttpMimeType = "application/zstd";
pub const HTTP_MP4_MIME_TYPE: HttpMimeType = "audio/mp4";
pub const HTTP_MPEG_MIME_TYPE: HttpMimeType = "audio/mpeg";
pub const HTTP_AVIF_MIME_TYPE: HttpMimeType = "image/avif";
pub const HTTP_GIF_MIME_TYPE: HttpMimeType = "image/gif";
pub const HTTP_JPEG_MIME_TYPE: HttpMimeType = "image/jpeg";
pub const HTTP_PNG_MIME_TYPE: HttpMimeType = "image/png";
pub const HTTP_TIFF_MIME_TYPE: HttpMimeType = "image/tiff";
pub const HTTP_WEBP_MIME_TYPE: HttpMimeType = "image/webp";
pub const HTTP_TTF_MIME_TYPE: HttpMimeType = "font/ttf";
pub const HTTP_CSS_MIME_TYPE: HttpMimeType = "text/css;charset=UTF-8";
pub const HTTP_CSV_MIME_TYPE: HttpMimeType = "text/csv;charset=UTF-8";
pub const HTTP_HTML_MIME_TYPE: HttpMimeType = "text/html;charset=UTF-8";
pub const HTTP_JAVASCRIPT_MIME_TYPE: HttpMimeType = "text/javascript;charset=UTF-8";
pub const HTTP_MARKDOWN_MIME_TYPE: HttpMimeType = "text/markdown;charset=UTF-8";
pub const HTTP_PLAIN_MIME_TYPE: HttpMimeType = "text/plain;charset=UTF-8";
pub const HTTP_XML_MIME_TYPE: HttpMimeType = "text/xml;charset=UTF-8";

// Hypertext Transfer Protocol MIME Type Vector
pub fn mime_types_vector() -> Vec<HttpMimeType> {
    let mime_types: Vec<HttpMimeType> = Vec::from([
        HTTP_JSON_MIME_TYPE,
        HTTP_JWT_MIME_TYPE,
        HTTP_MATHML_MIME_TYPE,
        HTTP_PDF_MIME_TYPE,
        HTTP_TOML_MIME_TYPE,
        HTTP_YAML_MIME_TYPE,
        HTTP_ZIP_MIME_TYPE,
        HTTP_ZLIB_MIME_TYPE,
        HTTP_ZSTD_MIME_TYPE,
        HTTP_MP4_MIME_TYPE,
        HTTP_MPEG_MIME_TYPE,
        HTTP_AVIF_MIME_TYPE,
        HTTP_GIF_MIME_TYPE,
        HTTP_JPEG_MIME_TYPE,
        HTTP_PNG_MIME_TYPE,
        HTTP_TIFF_MIME_TYPE,
        HTTP_WEBP_MIME_TYPE,
        HTTP_TTF_MIME_TYPE,
        HTTP_CSS_MIME_TYPE,
        HTTP_CSV_MIME_TYPE,
        HTTP_HTML_MIME_TYPE,
        HTTP_JAVASCRIPT_MIME_TYPE,
        HTTP_MARKDOWN_MIME_TYPE,
        HTTP_PLAIN_MIME_TYPE,
        HTTP_XML_MIME_TYPE,
    ]);

    return mime_types;
}
