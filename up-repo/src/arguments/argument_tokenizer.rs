use std::{
    env::args,
    io::{StdoutLock, Write, stdout},
    process::ExitCode,
    string::String,
    vec::Vec,
};

use super::{print_help::print_help_message, print_version::print_version_number};

use crate::git::{clean_repository::clean_repo, update_repository::up_repo};

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
        eprintln!("Error(1) - Exiting Git Repository Update Tool");
        return ExitCode::FAILURE;
    } else {
        match command_line_arguments[1].trim() {
            "clean" | "--c" => {
                clean_repo();
            }
            "update" | "--u" => {
                up_repo();
            }
            "help" | "--h" => {
                print_help_message();
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
                eprintln!("Error(1) - Exiting Git Repository Update Tool");
                return ExitCode::FAILURE;
            }
        };
    };

    return ExitCode::SUCCESS;
}
