use std::{
    io::{Error, StdoutLock, Write, stdout},
    process::{Command, Output, exit},
    result::{
        Result,
        Result::{Err, Ok},
    },
};

// DNF Upgrade
pub fn upgrade_dnf() -> () {
    let dnf_upgrade: Result<Output, Error> = Command::new("dnf").arg("-y").arg("upgrade").output();
    let mut standard_output: StdoutLock = stdout().lock();

    match dnf_upgrade {
        Ok(upgrade) => {
            standard_output.write_all(&upgrade.stdout).unwrap();
            writeln!(standard_output, "{}", upgrade.status).unwrap();
        }
        Err(error) => {
            eprintln!("Error Executing DNF Upgrade: {}", error);
            exit(1);
        }
    };

    return ();
}
