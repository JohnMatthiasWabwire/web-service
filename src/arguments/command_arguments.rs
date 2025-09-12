#![allow(dead_code)]

use std::{primitive::str, vec::Vec};

// Command Argument Definition
pub type CommandArgument = &'static str;

// Command Argument
pub const CONFIGURATION_COMMAND: CommandArgument = "config";
pub const EXIT_COMMAND: CommandArgument = "exit";
pub const HELP_COMMAND: CommandArgument = "help";
pub const SERVE_COMMAND: CommandArgument = "serve";
pub const VERSION_COMMAND: CommandArgument = "version";

// Commands Vector
pub fn commands_vector() -> Vec<CommandArgument> {
    let commands: Vec<CommandArgument> = Vec::from([
        CONFIGURATION_COMMAND,
        EXIT_COMMAND,
        HELP_COMMAND,
        SERVE_COMMAND,
        VERSION_COMMAND,
    ]);

    return commands;
}
