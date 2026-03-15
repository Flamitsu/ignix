// The porpouse of this file is to prepare and configure the EFI System Partition.
use crate::errors::SparkError;
/// This enum is used for dir operations. Such as create or delete them
pub enum Operations{
    Create,
    Delete
}

/// This function is the one that manages the ESP structure. (delete or create it.)
pub fn manage_esp_structure(operation: Operations) -> Result<(), SparkError>{
    Ok(())
}
