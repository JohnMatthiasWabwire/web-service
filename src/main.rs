mod arguments;
mod authentication;
mod authorization;
mod configuration;
mod errors;
mod hypertext_transfer;
mod json;
mod jwt;
mod logs;
mod shared;

mod service;
use service::application_service::web_service;

#[cfg(test)]
mod tests;

mod tokens;
mod yaml;

// Main Entry Point
fn main() -> () {
    web_service();

    return ();
}
