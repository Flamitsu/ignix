use std::env;
mod errors;
use errors::SparkError;
use crate::errors::cmd;
mod cli;
mod commands;
fn main(){
    // This if runs the actual program, if there is any error, it will exit it.
    if let Err(e) = run(){
        eprintln!("ERROR: {}",e);
        std::process::exit(1);
    }
}
/// The run function is the one that runs the program. If there is some problem it will tell it to the main function and the main function will exit the program with a message.
fn run() -> Result<(), SparkError> {
    let args: Vec<String> = env::args().collect();
    // If there is not any argument, it will show the help.
    if args.len() < 2{
        commands::help::show_help();
        return Ok(());
    }
    // let _efi_bin_path = cli::get_efi_bin_path()?;
    // Converts the second argument into string and starts matching
    match args[1].as_str() {
        "install" => commands::install::install_spark()?,
        "remove" => commands::remove::remove_spark_installation()?,
        "update" => commands::update::update_entries()?,
        "clean" => commands::clean::clean_entries()?,
        "help" => commands::help::show_help(),
        "test" => commands::help::show_help(), // This option is for WIP options. Do not execute.
        _ => return Err(cmd::Error::InvalidArgument(args[1].to_string()))?
    }
    Ok(())
}
