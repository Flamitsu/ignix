// SPDX-License-Identifier: GPL-3.0-only
/*
 * The structure of this archive is: Structs and later the constants with the associated value.
 * Its done this way because Rust does not implements in this date the capability to associate 
 * values directly into the struct so it's a "default."
 *
 * If you wonder why all the config is here, is because the GPT standard does not change but
 * diabolical disks can appear. So I left this here centralized so its easier to patch.
 * 
 * The structure of this archive is normally: comment -> data. This is because in some IDEs you 
 * can have the autocompletion and it will show up the comment that references to that value.
*/
use std::ops::Range;

pub struct EspStructure;
impl EspStructure{
    pub const ESP_DIRECTORIES: [&'static str; 4] = ["EFI/BOOT", "loader/entries", "EFI/ignix", "loader/ignix/"];
    pub const LOADER_CONFIG: &'static str = "#timeout 3\n#console-mode keep";
}
pub struct LinuxTags;
impl LinuxTags{
    pub const DEVNAME: &'static str = "DEVNAME=";
    pub const DEVTYPE: &'static str = "DEVTYPE=";
    pub const PARTUUID: &'static str = "PARTUUID=";
    pub const PRETTY_NAME: &'static str = "PRETTY_NAME=";
    pub const OS_ID: &'static str = "ID=";
}

pub struct GptSpecification;
impl GptSpecification{
    pub const EFI_SIGN: [u8;8] = *b"EFI PART";
    pub const ESP_GUID_SIG: [u8;16] = [0x28, 0x73, 0x2A, 0xC1,
    0x1F, 0xF8,
    0xD2, 0x11,
    0xBA, 0x4B,
    0x00, 0xA0, 0xC9, 0x3E, 0xC9, 0x3B];
}

// Those values follow the instructions of the GPT std at 18/04/2026.
pub struct GptLimits{
    pub gpt_partitions: usize,
    pub partition_entry_size: usize,
    pub lba_sector_size: usize,
    pub header_size: usize,
    pub buffer_size: usize,
    pub header_part_lba: usize,
    pub header_part_size: usize
}
/*
 * Why does this have a constructor and the other ones do not? 
 * It's because parameters like the buffer size should be
 * calculated with the current config and not be hardcoded. You give the gpt_partitions (normally
 * 128), the entry_size (normally 128) and the lba_sector_size (normally 4096 or 512) 
 */
impl GptLimits {
    const fn new(gpt_partitions: usize, partition_entry_size: usize, lba_sector_size: usize) -> Self{
        let buffer_size = (gpt_partitions * partition_entry_size) + lba_sector_size;
        Self {
            gpt_partitions,
            partition_entry_size,
            lba_sector_size,
            header_size: 92,
            buffer_size,
            header_part_lba: 2,
            header_part_size: 128
        }
    }
}
pub const LIMITS: GptLimits = GptLimits::new(128, 128, 4096);

// GPT HEADER OFFSETS (LBA 1)
pub struct GptHeaderOffsets;
impl GptHeaderOffsets{
    // GPT header signature start (Normally "EFI PART" in Little Endian)
    pub const SIG: Range<usize> = 0..8;
    // GPT header size (normally 92)
    pub const SIZE: Range<usize> = 12..16;
    // CRC32 bytes location.
    pub const CRC: Range<usize> = 16..20;
    // Where does the partition array starts
    pub const PART_LBA: Range<usize> = 72..80;
    // How many partitions can the disk have.
    pub const PART_COUNT: Range<usize> = 80..84;
    // Each GPT entry size (normally is 128 bytes).
    pub const PART_SIZE: Range<usize> = 84..88;
    // CRC32 of the partition array.
    pub const PART_CRC: Range<usize> = 88..92;
}

pub struct GptEntryOffsets;
impl GptEntryOffsets{
    // Partition type GUID (Linux Root or Linux x86 for example).
    pub const TYPE_GUID: Range<usize> = 0..16;
    // Partition unique GUID (UUID v4 unique per partition).
    pub const UNIQUE_GUID: Range<usize> = 16..32;
}

pub struct Routes;
impl Routes{
    // route: /usr/lib/ignix/ignix{arch}.efi
    pub const DEFAULT_EFI_BIN_PATH: &'static str = "/usr/lib/ignix/ignixx64.efi";
    // route: /sys/class/block/
    pub const BLOCK_DEV_ROUTE: &'static str = "/sys/class/block/";
    // route: queue/logical_block_size
    pub const LOGICAL_BLOCK_SIZE: &'static str = "queue/logical_block_size";
    // route: /proc/mounts
    pub const MOUNTPOINTS: &'static str = "/proc/mounts";
    // route: /dev/urandom
    pub const RNG_SOURCE: &'static str = "/dev/urandom";
    // route: /proc/cmdline
    pub const CMDLINE: &'static str = "/proc/cmdline";
    // route: /etc/os-release
    pub const OS_RELEASE: &'static str = "/etc/os-release";
    // route: /etc/machine-id
    pub const ETC_MACHINE_ID: &'static str = "/etc/machine-id";
    // route: /var/lib/dbus/machine-id 
    pub const DBUS_MACHINE_ID: &'static str = "/var/lib/dbus/machine-id";
    
    // Atomicall operations
    pub const AT_DEFAULT_EFI_BIN_PATH: &'static str = "/usr/lib/ignix/ignixx64.efi.tmp";
    pub const AT_ETC_MACHINE_ID: &'static str = "/etc/machine-id.tmp";
}

pub struct Flag;
impl Flag {
    pub const FORCE_FLAG: &'static str = "--force";
    pub const ALLOW_VIRTUAL_FLAG: &'static str = "--allow-virtual";
    pub const NO_NVRAM: &'static str = "--no-nvram";
    pub const INSTALL_ROUTE: &'static str = "--install-route=";
    pub const EFI_BIN_PATH: &'static str = "--efi-bin=";
    pub const REMOVABLE_FLAG: &'static str = "--removable";
}

pub struct AddFlag;
impl AddFlag{
    pub const KERNEL_VERSION: &'static str = "--kernel=";
    pub const LINUX: &'static str = "--linux=";
    pub const INITRD: &'static str = "--initrd=";
}
