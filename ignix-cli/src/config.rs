// This is the archive where all the config of the program resides. 

/// Default efi bin path that defines where the EFI binary should be located (default and fallback)
pub const DEFAULT_EFI_BIN_PATH: &str = "/usr/lib/ignix/ignixx64.efi";

/// This flag is what the user should put as an argument to give a distinct EFI bin path.
pub const EFI_BIN_PATH_FLAG: &str = "--efi-bin=";

/// Defines the route that looks up the program to search for real block devices.
pub const BLOCK_DEV_ROUTE: &str = "/sys/block/";

/// Defines the route like '/sys/block/{device}/queue/logical_block_size' 
pub const LOGICAL_BLOCK: &str = "/queue/logical_block_size";

// Neccesary for the confirmation flags in the CLI usage of the tool while the install process. 
pub const SHORT_CONFIRMATION_FLAG: &str = "-y";
pub const LONG_CONFIRMATION_FLAG: &str = "--yes";

