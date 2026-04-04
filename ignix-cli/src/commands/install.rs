use crate::boot::disk;
use crate::boot::esp::Operations;
use crate::boot::esp::manage_esp_structure;
use crate::cli;
use crate::config::LOGICAL_BLOCK;
use crate::IgnixError;
/// This function should install the ignixx64.efi binary in the current ESP partition.
pub fn install_ignix(options: cli::InstallOptions) -> Result<(), IgnixError>{
    
    // If the user did not used the flags '-y' or '--yes' in the execution, ask confirmation prompt
    if !options.force{
        cli::ask_user_confirmation("install")?;
    }
    
    let esp_target = if let Some(route) = &options.install_route{
        route.to_string_lossy().to_string()
    } else {
        let disks = disk::get_system_disks(LOGICAL_BLOCK, &options)?;
        disk::compatible_esp_partition(disks)?
    };
 
    // Manages the structure detecting the disks and creating the basic structure
    manage_esp_structure(Operations::Create, &esp_target, &options)?;

    Ok(())
}
