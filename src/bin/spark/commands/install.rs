use std::path::Path;
use crate::SparkError;
/// This function should install the sparkx64.efi binary in the current ESP partition.
pub fn install_spark(_efi_bin_path: &Path, _skip_confirmation: bool) -> Result<(), SparkError>{
    
    todo!("This should install the binary.")
}
