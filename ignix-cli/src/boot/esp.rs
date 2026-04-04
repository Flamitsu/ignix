use crate::{cli::InstallOptions, errors::{IgnixError, io}};
use std::{fs::read_to_string, path::PathBuf};
use crate::config::MOUNTPOINTS;
/// This enum is used for dir operations. Such as create or delete them
pub enum Operations{
    Create,
    Delete
}

/// This function is the one that manages the ESP structure. (delete or create it.)
pub fn manage_esp_structure(operation: Operations, partition: &str, options: &InstallOptions) 
    -> Result<(),IgnixError>{
    let esp_mountpoint = if let Some(m) = get_esp_mountpoint(partition)?{ m } else {
        Err(io::Error::NotFound("{partition} not found in {MOUNTPOINTS}. Is it mounted?".into()))?
    };

    match operation{
        Operations::Create => { },
        Operations::Delete => { }
    }
    Ok(())
}

/// This function returns the mountpoint provided by compatible_esp_device function.
pub fn get_esp_mountpoint(partition_name: &str) -> Result<Option<String>, IgnixError>{
    let route = PathBuf::from(MOUNTPOINTS);
    
    if !route.exists(){
        Err(io::Error::NotFound(route.display().to_string()))?
    }

    let file_content = read_to_string(route)?;
    for line in file_content.lines(){
        if !line.starts_with(partition_name){
            continue;
        }

        if let Some(mountpoint) = line.split_whitespace().nth(1){
            return Ok(Some(mountpoint.to_string()));
        }
    }
    Ok(None)
}
