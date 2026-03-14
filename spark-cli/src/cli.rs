use crate::errors::cmd;
use std::io::{Write, stdin,stdout};
use std::path::Path;

/// Extracts the EFI binary path in the argument that have been provided and the default one.
pub fn get_efi_bin_path(arguments: &[String]) -> Result<&Path, cmd::Error>{ 
    const DEFAULT_EFI_BIN_PATH: &str = "/usr/lib/spark/sparkx64.efi";
    const EFI_BIN_PATH_FLAG: &str = "--efi-bin=";
    for argument in arguments{
        // Checks if the argument matches the flag, if not, continue to the next argument.
        if !argument.starts_with(EFI_BIN_PATH_FLAG){
            continue;
        }
        // Defines the route and checks if it exists or ends with "efi".
        let route = Path::new(&argument[EFI_BIN_PATH_FLAG.len()..]);
        if !route.ends_with(".efi") || !route.exists(){
            eprintln!("The route '{}' is not valid. Check if the binary exists.",route.display());
            eprintln!("Fallback to the default '{}' path",DEFAULT_EFI_BIN_PATH);
            break;
        }
        return Ok(route);
    }
    // Converts the string from the constant to a path type
    let default_route = Path::new(DEFAULT_EFI_BIN_PATH);
    if default_route.exists(){
        return Ok(default_route)
    }
    // If there is not any right path, neither default neither the given one, it will throw an error.
    Err(cmd::Error::EFINotFound(DEFAULT_EFI_BIN_PATH.to_string()))
}

/// This function provides an argument to skip the confirmation in the installation process of the program with the flag '-y' or '--yes'
pub fn skip_user_confirmation(arguments: &[String]) -> bool{
    const SHORT_FLAG: &str = "-y";
    const LONG_FLAG: &str = "--yes";
    for argument in arguments{
        if argument.starts_with(SHORT_FLAG) || argument.starts_with(LONG_FLAG){
            println!("Skipping confirmation");
            return true;
        }
    }
    false
}

/// This function ask user confirmation. If the user types 'YES' it returns true, if not, false.
pub fn ask_user_confirmation(context: &str) -> bool{
    println!("Remember to use capital letters as shown:");
    println!("Type 'YES' to {} or 'NO' to cancel.",context);
    // Cleans the current stdout buffer 
    stdout().flush().ok();
    let mut user_input = String::new();
    stdin().read_line(&mut user_input).ok();
    match user_input.trim(){
        "YES" => true,
        "NO" => false,
        _ => {
            eprintln!("The program did not understoot the input '{}', assuming 'NO'.",user_input);
            false
        }
    }
}
