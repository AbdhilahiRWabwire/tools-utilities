use std::{
    io::{stdout, StdoutLock, Write},
    primitive::str,
};

// Print Version Number
pub fn print_version_number() -> () {
    let mut standard_output: StdoutLock = stdout().lock();
    let version_number: &str = "1.0.0";

    writeln!(standard_output, "Git Repository Update Tool").unwrap();
    writeln!(standard_output, "").unwrap();
    writeln!(standard_output, "Version Number:\t {}", version_number).unwrap();

    return ();
}
