use std::{
    fs::{DirEntry, ReadDir, read_dir},
    io::{Error, StdoutLock, Write, stdout},
    path::PathBuf,
    process::{Command, Output, exit},
    result::{
        Result,
        Result::{Err, Ok},
    },
};

// Main Entry Point
// Clean Logs by Copying the contents of /dev/null to the Log Files
fn main() -> () {
    let log_directory_path: PathBuf = PathBuf::from("/var/log/");
    let log_directory: ReadDir = read_dir(log_directory_path).unwrap();

    for directory_entry in log_directory {
        let entry: DirEntry = directory_entry.unwrap();
        let path: PathBuf = entry.path();

        if path.is_file() {
            let clean_logs: Result<Output, Error> =
                Command::new("cp").arg("/dev/null").arg(path).output();
            let mut standard_output: StdoutLock = stdout().lock();

            match clean_logs {
                Ok(cleaning) => {
                    writeln!(standard_output, "Cleaning Log File...").unwrap();
                    standard_output.write_all(&cleaning.stdout).unwrap();
                    writeln!(standard_output, "{}", cleaning.status).unwrap();
                }
                Err(error) => {
                    eprintln!("Error Cleaning Log Files: {}", error);
                    exit(1);
                }
            };
        } else {
            return ();
        }
    }
}
