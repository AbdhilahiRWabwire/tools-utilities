use std::{
    env::args,
    io::{StdoutLock, Write, stdout},
    process::ExitCode,
    string::String,
    vec::Vec,
};

use super::{print_help::print_help_message, print_version::print_version_number};

use crate::upgrades::{
    apt_upgrade::upgrade_apt, dnf_release::release_dnf, dnf_upgrade::upgrade_dnf,
    snap_refresh::refresh_snap, ubuntu_release::release_ubuntu,
};

// Command Line Argument Tokenizer
pub fn tokenize_arguments() -> ExitCode {
    let command_line_arguments: Vec<String> = args().collect();
    let mut standard_output: StdoutLock = stdout().lock();

    if command_line_arguments.len() != 2 {
        writeln!(
            standard_output,
            "Command or Flag Required but not Both: {:#?}",
            command_line_arguments
        )
        .unwrap();
        print_help_message();
        eprintln!("Error(1) - Exiting System Update Tool");
        return ExitCode::FAILURE;
    } else {
        match command_line_arguments[1].trim() {
            "au" | "--au" => {
                upgrade_apt();
            }
            "dr" | "--dr" => {
                release_dnf();
            }
            "du" | "--du" => {
                upgrade_dnf();
            }
            "help" | "--h" => {
                print_help_message();
            }
            "sr" | "--sr" => {
                refresh_snap();
            }
            "ur" | "--ur" => {
                release_ubuntu();
            }
            "version" | "--v" => {
                print_version_number();
            }
            &_ => {
                writeln!(
                    standard_output,
                    "Uknown Command or Flag: {:#?}",
                    command_line_arguments[1].trim()
                )
                .unwrap();
                print_help_message();
                eprintln!("Error(1) - Exiting System Update Tool");
                return ExitCode::FAILURE;
            }
        };
    };

    return ExitCode::SUCCESS;
}
