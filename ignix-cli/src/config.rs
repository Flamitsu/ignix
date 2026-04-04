// This is the archive where all the config of the program resides. 

// GPT specification and Kernel sysfs interface.
// mountpoints storage with the Kernel interface.
pub const MOUNTPOINTS: &str = "/proc/mounts";
/* Those strings are searched in the uevent file in the sysfs interface
 * to match the gpt.rs UUID.*/ 
pub const DEVNAME: &str = "DEVNAME=";
pub const DEVTYPE: &str = "DEVTYPE=";
pub const PARTUUID: &str = "PARTUUID=";

// This constants are being used in gpt.rs at this moment
/// EFI part signature in array of integer bytes.
pub const EFI_PART_SIGN: [u8;8] = *b"EFI PART";

pub const ESP_GUID_BYTES: [u8;16]= [
        0x28, 0x73, 0x2A, 0xC1, // (LE) DATASET1 -> 28 73 2A C1 -> C12A7328 
        0x1F, 0xF8, // (LE) DATASET2 -> 1F F8 -> 1FF8
        0xD2, 0x11, // (LE) DATASET3 -> 11 D2 -> D211
        0xBA, 0x4B, // (BE) DATASET4 -> BA 4B -> BA4B
        0x00, 0xA0, 0xC9, 0x3E, 0xC9, 0x3B // (BE) DATASET5 -> 00 A0 C9 3E C9 3B -> 00A0C93EC93B  
    ];

/* Measures are in bytes. I don't recommend at all changing this values unless 
 * you're using some diabolical version of GPT and you need it to specifically
 * match your disks sectors. What's wrong with you.
 * Unless the GPT standard changes, (that hasn't been updated since 2015) don't 
 * commit changing this values.
 */
pub const MAX_GPT_PARTITIONS: usize = 128;
const MAX_GPT_PARTITION_ENTRY_SIZE: usize = 128;
const MAX_LBA_SECTOR_SIZE: usize = 4096;
pub const MAX_GPT_HEADER_SIZE: usize = 92;
// This buffer size is here for security. If this was a heap, will potentially make the code run slower and can expose a thread to systems with corrupt disks.
pub const MAX_BUFFER_SIZE: usize = MAX_GPT_PARTITIONS * MAX_GPT_PARTITION_ENTRY_SIZE + MAX_LBA_SECTOR_SIZE;

// Routes 
/// Default efi bin path that defines where the EFI binary should be located (default and fallback)
pub const DEFAULT_EFI_BIN_PATH: &str = "/usr/lib/ignix/ignixx64.efi";
/// Defines the route that looks up the program to search for real block devices.
pub const BLOCK_DEV_ROUTE: &str = "/sys/class/block/";
/// Defines the route '/sys/class/block/{device}/queue/logical_block_size' with format to get the sector block size
pub const LOGICAL_BLOCK: &str = "/queue/logical_block_size";

// Arguments 
/// Neccesary for the confirmation flags in the CLI usage of the tool while the install process. 
pub const SHORT_CONFIRMATION_FLAG: &str = "-y";
/// Neccesary for the confirmation flags in the CLI usage of the tool while the install process.
pub const LONG_CONFIRMATION_FLAG: &str = "--yes";
/// Flag to allow the program to install in a virtual device.
pub const ALLOW_VIRTUAL_FLAG: &str = "--allow-virtual";
/// Flag argument to remove the neccesary NVRAM write variable.
pub const NO_NVRAM: &str = "--no-nvram";
/// Flag argument to say which route should the program be installed in.
pub const INSTALL_ROUTE: &str = "--install-route=";
/// This flag is what the user should put as an argument to give a distinct EFI bin path (install).
pub const EFI_BIN_PATH: &str = "--efi-bin=";
/// Flag argument to install the bootloader in a removable device.
pub const REMOVABLE_FLAG: &str = "--removable";
