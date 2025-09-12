use std::{
    io::{Error, StdoutLock, Write, stdout},
    process::{Command, Output, exit},
    result::{
        Result,
        Result::{Err, Ok},
    },
};

// Snap Refresh
pub fn refresh_snap() -> () {
    let snap_refresh: Result<Output, Error> = Command::new("snap").arg("refresh").output();
    let mut standard_output: StdoutLock = stdout().lock();

    match snap_refresh {
        Ok(refresh) => {
            standard_output.write_all(&refresh.stdout).unwrap();
            writeln!(standard_output, "{}", refresh.status).unwrap();
        }
        Err(error) => {
            eprintln!("Error Executing Snap Refresh: {}", error);
            exit(1);
        }
    };

    return ();
}
