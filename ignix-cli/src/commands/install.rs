use std::path::Path;
use crate::cli;
use crate::errors::cmd;
use crate::IgnixError;
/// This function should install the ignixx64.efi binary in the current ESP partition.
pub fn install_ignix(args: &[String], force_install: bool) -> Result<(), IgnixError>{
    let _efi_bin_path: &Path = cli::get_efi_bin_path(args)?;
    
    // If the user did not used the flags '-y' or '--yes' in the execution, ask confirmation prompt
    let continue_program = if force_install{
        true
    } else{
        cli::ask_user_confirmation("install")
    };

    // Aborts the program immediately 
    if !continue_program{
        Err(cmd::Error::UserAborted)?;
    }

    Ok(())
}
