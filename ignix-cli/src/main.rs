// SPDX-License-Identifier: GPL-3.0-only
pub mod boot;
mod cli;
mod commands;
pub mod config; // Here is where all the consts resides centralized
mod errors;
use std::env;
mod utils;
use crate::cli::interface::parse_hook_args;
use crate::errors::IgnixError;
use crate::errors::cmd;
fn main() {
    // This if runs the actual program, if there is any error, it will exit it.
    if let Err(error) = run() {
        eprintln!("{}", error);
        std::process::exit(1);
    }
}

/// The run function is the one that runs the program. If there is some problem it will tell it about it to the main function and the main function will exit the program with a message.
fn run() -> Result<(), IgnixError> {
    let args: Vec<String> = env::args().collect();

    // If there is not any argument, it will show the help. (ignix 1 [argument] 2)
    if args.len() < 2 {
        commands::help::show_help();
        return Ok(());
    }

    match args[1].as_str() {
        "add" => {
            let options = cli::interface::parse_add_args(&args)?;
            commands::add::add_entry(options)?;
        }
        "install" => {
            let options = cli::interface::parse_install_args(&args)?;
            commands::install::install_ignix(options)?;
        }
        "hook" => {
            let options = parse_hook_args(&args)?;
            println!("{}", commands::hook::help_hooks(options)?);
        }
        "uninstall" => {
            let options = cli::interface::parse_remove_args(&args)?;
            commands::uninstall::remove_ignix(options)?;
        }
        "help" => commands::help::show_help(),
        _ => return Err(cmd::Error::InvalidArgument(args[1].to_string()))?,
    }
    Ok(())
}
