#![allow(dead_code)]

use std::{primitive::str, vec::Vec};

// Flag Argument Definition
pub type FlagArgument = &'static str;

// Flag Argument
pub const CONFIGURATION_FLAG: FlagArgument = "--c";
pub const HELP_FLAG: FlagArgument = "--h";
pub const SERVE_FLAG: FlagArgument = "--s";
pub const VERSION_FLAG: FlagArgument = "--v";

// Flags Vector
pub fn flags_vector() -> Vec<FlagArgument> {
    let flags: Vec<FlagArgument> =
        Vec::from([CONFIGURATION_FLAG, HELP_FLAG, SERVE_FLAG, VERSION_FLAG]);

    return flags;
}
