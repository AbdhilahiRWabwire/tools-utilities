use std::io::{StdoutLock, Write, stdout};

// Print Help Command Output
pub fn print_help_message() -> () {
    let mut standard_output: StdoutLock = stdout().lock();

    writeln!(standard_output, "Systsem Update Tool").unwrap();
    writeln!(standard_output, "").unwrap();
    writeln!(standard_output, "Commands:\t\t Description:").unwrap();
    writeln!(standard_output, "").unwrap();
    writeln!(standard_output, "au\t\t APT Upgrade").unwrap();
    writeln!(standard_output, "dr\t\t DNF System Release Upgrade").unwrap();
    writeln!(standard_output, "du\t\t DNF Upgrade").unwrap();
    writeln!(standard_output, "help\t\t Print Commands and Flags").unwrap();
    writeln!(standard_output, "sr\t\t Snap Refresh").unwrap();
    writeln!(standard_output, "ur\t\t Ubuntu System Releade Upgrade").unwrap();
    writeln!(standard_output, "version\t\t Print Version Number").unwrap();
    writeln!(standard_output, "").unwrap();
    writeln!(standard_output, "Flags:\t\t Description:").unwrap();
    writeln!(standard_output, "").unwrap();
    writeln!(standard_output, "--au\t\t APT Upgrade").unwrap();
    writeln!(standard_output, "--dr\t\t DNF System Release Upgrade").unwrap();
    writeln!(standard_output, "--du\t\t DNF Upgrade").unwrap();
    writeln!(standard_output, "--h\t\t Print Commands and Flags").unwrap();
    writeln!(standard_output, "--sr\t\t Snap Refresh").unwrap();
    writeln!(standard_output, "--ur\t\t Ubuntu System Releade Upgrade").unwrap();
    writeln!(standard_output, "--v\t\t Print Version Number").unwrap();
    writeln!(standard_output, "").unwrap();

    return ();
}
