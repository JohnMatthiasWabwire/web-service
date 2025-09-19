use std::io::{StdoutLock, Write, stdout};

// Print Help Command Output
pub fn print_help_message() -> () {
    let mut standard_output: StdoutLock = stdout().lock();

    writeln!(standard_output, "Hyaena Technologies Web Service").unwrap();
    writeln!(standard_output, "").unwrap();
    writeln!(standard_output, "Commands:\t\t Description:").unwrap();
    writeln!(standard_output, "").unwrap();
    writeln!(
        standard_output,
        "config\t\t Configure Server with server-configuration.yaml"
    )
    .unwrap();
    writeln!(standard_output, "exit (Command Prompt)\t\t Exit Service").unwrap();
    writeln!(standard_output, "help\t\t Print Commands and Flags").unwrap();
    writeln!(standard_output, "serve\t\t Serve Web Applcation").unwrap();
    writeln!(standard_output, "version\t\t Print Version Number").unwrap();
    writeln!(standard_output, "").unwrap();
    writeln!(standard_output, "Flag:\t\t Description:").unwrap();
    writeln!(standard_output, "").unwrap();
    writeln!(
        standard_output,
        "--c\t\t Configure Server with server-configuration.yaml"
    )
    .unwrap();
    writeln!(standard_output, "--h\t\t Print Commands and Flags").unwrap();
    writeln!(standard_output, "--s\t\t Serve Web Applcation").unwrap();
    writeln!(standard_output, "--v\t\t Print Version Number").unwrap();
    writeln!(standard_output, "").unwrap();

    return ();
}
