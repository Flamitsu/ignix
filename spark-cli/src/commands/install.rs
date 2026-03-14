use std::path::Path;
use crate::cli;
use crate::SparkError;
/// This function should install the sparkx64.efi binary in the current ESP partition.
pub fn install_spark(args: &[String], skip_confirmation: bool) -> Result<(), SparkError>{
    let _efi_bin_path: &Path = cli::get_efi_bin_path(args)?;
    let continue_program = if skip_confirmation{
        true
    } else{
        cli::ask_user_confirmation("install")
    };
    if !continue_program{
        return Err(SparkError::Cmd(crate::cmd::Error::UserAborted));
    }
    Ok(())
}
