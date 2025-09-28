#![allow(dead_code)]

use std::{
    io::{Stdin, StdoutLock, Write, stdin, stdout},
    process::exit,
    string::String,
};

use crate::arguments::{print_help::print_help_message, print_version::print_version_number};

// Command Prompt: Read, Evaluate, Print, Looop
pub fn command_prompt() -> () {
    let standard_input: Stdin = stdin();
    let mut standard_input_buffer: String = String::new();
    let mut standard_output: StdoutLock = stdout().lock();

    standard_output.flush().unwrap();
    writeln!(standard_output, "Hyaena-Technologies-Web|> ").unwrap();

    while standard_input_buffer.trim() != "" {
        standard_input_buffer.clear();
        standard_input
            .read_line(&mut standard_input_buffer)
            .unwrap();

        match standard_input_buffer.trim() {
            "exit" => {
                writeln!(standard_output, "Exiting Hyaena Technologies Web Service").unwrap();
                exit(0);
            }
            "help" => {
                print_help_message();
            }
            "version" => {
                print_version_number();
            }
            &_ => {
                writeln!(
                    standard_output,
                    "Unknown Command: {}",
                    standard_input_buffer
                )
                .unwrap();
            }
        };

        standard_output.flush().unwrap();
        writeln!(standard_output, "Hyaena-Technologies-Web|> ").unwrap();

        continue;
    }

    return ();
}
