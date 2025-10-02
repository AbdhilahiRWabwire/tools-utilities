use std::io::{StdoutLock, Write, stdout};

// Print Help Command Output
pub fn print_help_message() -> () {
    let mut standard_output: StdoutLock = stdout().lock();

    writeln!(standard_output, "Git Respository Update Tool").unwrap();
    writeln!(standard_output, "").unwrap();
    writeln!(standard_output, "Commands:\t\t Description:").unwrap();
    writeln!(standard_output, "").unwrap();
    writeln!(standard_output, "clean\t\t Git Garbage Collection").unwrap();
    writeln!(standard_output, "update\t\t Git Pull").unwrap();
    writeln!(standard_output, "help\t\t Print Commands and Flags").unwrap();
    writeln!(standard_output, "version\t\t Print Version Number").unwrap();
    writeln!(standard_output, "").unwrap();
    writeln!(standard_output, "Flags:\t\t Description:").unwrap();
    writeln!(standard_output, "").unwrap();
    writeln!(standard_output, "--c\t\t Git Garbage Collection").unwrap();
    writeln!(standard_output, "--u\t\t Git Pull").unwrap();
    writeln!(standard_output, "--h\t\t Print Commands and Flags").unwrap();
    writeln!(standard_output, "--v\t\t Print Version Number").unwrap();
    writeln!(standard_output, "").unwrap();

    return ();
}
