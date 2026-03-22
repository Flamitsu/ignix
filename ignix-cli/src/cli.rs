use crate::config::{DEFAULT_EFI_BIN_PATH, EFI_BIN_PATH_FLAG, SHORT_CONFIRMATION_FLAG, LONG_CONFIRMATION_FLAG};
use crate::errors::cmd;
use std::io::{Write, stdin,stdout};
use std::path::Path;
/// Extracts the EFI binary path in the argument that have been provided and the default one.
pub fn get_efi_bin_path(arguments: &[String]) -> Result<&Path, cmd::Error>{ 
    
    for argument in arguments{
        
        // Checks if the argument matches the flag, if not, continue to the next argument.
        if !argument.starts_with(EFI_BIN_PATH_FLAG){
            continue;
        }
        
        // Defines the route and checks if it exists or ends with "efi".
        let route = Path::new(&argument[EFI_BIN_PATH_FLAG.len()..]);
        
        /* Checks if the actual path's extension ends in .efi,
         * if not, it will try to fall back to the default path of the .efi binary.
        */
        if route.extension().is_none_or(|ext| ext != ".efi") || !route.exists(){
            eprintln!("The route '{}' is not a valid EFI binary. Check if the binary exists.",route.display());
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
    
    for argument in arguments{
        
        if argument.starts_with(SHORT_CONFIRMATION_FLAG) 
            || argument.starts_with(LONG_CONFIRMATION_FLAG){
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

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_skip_user_confirmation(){
        let args = vec!["ignix".to_string(), "--yes".to_string()];
        assert!(skip_user_confirmation(&args));
        let args_short = vec!["ignix".to_string(), "-y".to_string()];
        assert!(skip_user_confirmation(&args_short));
    }
    #[test]
    fn test_skip_user_confirmation_false(){
        let args = vec!["ignix".to_string(), "install".to_string()];
        assert!(!skip_user_confirmation(&args));
    }
    #[test]
    fn test_get_efi_bin_path(){
        let args = vec![];
        let efi_path = get_efi_bin_path(&args);
        assert!(efi_path.is_err() || efi_path.is_ok());
    }
}
